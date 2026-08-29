use std::cmp::Ordering;

use glam::Vec2;
#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::{instance::U32Layer, Layer, Mesh};

/// Order two directions by their angle from the `+Y` axis, counter-clockwise, over
/// `(-PI, PI]` -- the same order as `Vec2::Y.angle_to(direction)`, without the `atan2`.
///
/// The quadrant of that angle plus the sign of the cross product within it is the whole
/// comparison, so it uses only multiplication, subtraction and comparison. `atan2` is not
/// required to be correctly rounded and differs between glibc, musl, Apple's libm and MSVC.
/// That matters here and not in most places: this order is `Vertex::polygons`, which decides
/// which neighbour `successors` walks to first, so one ulp of difference near the boundary
/// of the `(angle * 100000.0) as i32` bucket this replaces could change a returned path.
///
/// It is also finer than that bucket, which put angles a hundred-thousandth apart in an
/// arbitrary order, and it does not read the sign of a zero: a direction of exactly
/// `(0, -y)` is `-PI` whichever zero it is, where `angle_to` answers `-PI` for `+0.0` and
/// `PI` for `-0.0`.
fn ccw_from_y(a: Vec2, b: Vec2) -> Ordering {
    // Quadrant of the angle from `+Y`, counter-clockwise: `[-PI, -PI/2)`, `[-PI/2, 0)`,
    // `[0, PI/2]`, `(PI/2, PI)`. Each spans at most a quarter turn, which is what lets the
    // cross product decide within one -- two directions less than a half turn apart are
    // never antipodal, so their cross product is only zero when they are the same.
    fn quadrant(d: Vec2) -> u8 {
        if d.y < 0.0 {
            if d.x >= 0.0 {
                0
            } else {
                3
            }
        } else if d.x > 0.0 {
            1
        } else {
            2
        }
    }

    quadrant(a).cmp(&quadrant(b)).then_with(|| {
        // Less than a half turn apart, so the sign of the cross product is which of the two
        // is reached first going counter-clockwise. Equal directions give zero, and so does
        // a NaN, which sorts as a tie rather than panicking.
        let cross = a.perp_dot(b);
        if cross > 0.0 {
            Ordering::Less
        } else if cross < 0.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    })
}

impl Mesh {
    /// Reorder all the neighboring polygons of all the vertices so that they are CCW ordered, and correctly mark corners.
    pub fn reorder_neighbors_ccw_and_fix_corners(&mut self) {
        let mut reordered_neighbors = vec![];
        for layer in self.layers.iter() {
            let mut reordered_neighbors_in_layer = vec![];
            for vertex in &layer.vertices {
                let vertex_coords = vertex.coords + layer.offset;
                // For each polygon using a vertex, sort them in CCW order
                let mut polygons = vertex
                    .polygons
                    .iter()
                    .filter(|p| **p != u32::MAX)
                    .cloned()
                    .collect::<Vec<_>>();
                // Sort by the angle between the Y axis and the direction from the vertex to the center of the polygon
                let direction_to_center = |p: &u32| {
                    let layer = &self.layers[p.layer() as usize];
                    let vertices = &layer.polygons[p.polygon() as usize].vertices;
                    let center = vertices
                        .iter()
                        .map(|v| layer.vertices[*v as usize].coords)
                        .sum::<Vec2>()
                        / vertices.len() as f32
                        + layer.offset;
                    center - vertex_coords
                };
                polygons.sort_unstable_by(|a, b| {
                    ccw_from_y(direction_to_center(a), direction_to_center(b))
                });
                polygons.dedup_by_key(|p| *p);
                if polygons.is_empty() {
                    reordered_neighbors_in_layer.push(vec![u32::MAX]);
                } else {
                    // Reintroduce empty markers
                    // For two following polygons on a vertex, check their previous / next vertices
                    // If they are different, there is a hole between them
                    let first = polygons[0];
                    let last = *polygons.last().unwrap();
                    if first == last {
                        polygons.push(u32::MAX);
                    } else {
                        polygons = polygons
                            .windows(2)
                            .map(|pair| [pair[0], pair[1]])
                            .chain(std::iter::once([last, first]))
                            .flat_map(|[pair0, pair1]| {
                                let layer0 = &self.layers[pair0.layer() as usize];
                                let layer1 = &self.layers[pair1.layer() as usize];
                                let mut polygon0 =
                                    layer0.polygons[pair0.polygon() as usize].vertices.clone();
                                polygon0.reverse();
                                let mut found = false;
                                let Some(previous0) =
                                    polygon0.iter().cycle().take(polygon0.len() * 2).find(|v| {
                                        if found {
                                            return true;
                                        }
                                        if (layer0.vertices[**v as usize].coords + layer0.offset)
                                            .distance_squared(vertex_coords)
                                            < 0.0001
                                        {
                                            found = true;
                                        }
                                        false
                                    })
                                else {
                                    return vec![pair0, u32::MAX];
                                };
                                let polygon1 = &layer1.polygons[pair1.polygon() as usize].vertices;
                                let mut found = false;
                                let Some(next1) =
                                    polygon1.iter().cycle().take(polygon1.len() * 2).find(|v| {
                                        if found {
                                            return true;
                                        }
                                        if (layer1.vertices[**v as usize].coords + layer1.offset)
                                            .distance_squared(vertex_coords)
                                            < 0.0001
                                        {
                                            found = true;
                                        }
                                        false
                                    })
                                else {
                                    return vec![pair0, u32::MAX];
                                };

                                if layer0.vertices[*previous0 as usize].coords + layer0.offset
                                    != layer1.vertices[*next1 as usize].coords + layer1.offset
                                {
                                    vec![pair0, u32::MAX]
                                } else {
                                    vec![pair0]
                                }
                            })
                            .collect();
                    }
                    reordered_neighbors_in_layer.push(polygons);
                }
            }

            reordered_neighbors.push(reordered_neighbors_in_layer);
        }
        for (layer, new) in self.layers.iter_mut().zip(reordered_neighbors) {
            for (vertex, new) in layer.vertices.iter_mut().zip(new) {
                vertex.is_corner = new.contains(&u32::MAX);
                vertex.polygons = new;
            }
        }
    }

    /// Remove vertices that are not used by any polygon, and update indexes.
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub fn remove_useless_vertices(&mut self) -> bool {
        !self
            .layers
            .iter_mut()
            .map(|layer| layer.remove_useless_vertices())
            .all(|m| !m)
    }

    /// Update the `is_one_way` flag for each polygon.
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub fn update_is_one_way(&mut self) {
        self.layers
            .iter_mut()
            .for_each(|layer| layer.update_is_one_way());
    }
}

impl Layer {
    /// Remove vertices that are not used by any polygon, and update indexes.
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub fn remove_useless_vertices(&mut self) -> bool {
        let mut removed = false;
        let mut new_indexes = vec![u32::MAX; self.vertices.len()];
        let mut kept = 0;
        for (i, vertex) in self.vertices.iter().enumerate() {
            if vertex.polygons.is_empty() || vertex.polygons == [u32::MAX] {
                removed = true;
            } else {
                new_indexes[i] = kept;
                kept += 1;
            }
        }
        for polygon in self.polygons.iter_mut() {
            for vertex in polygon.vertices.iter_mut() {
                *vertex = new_indexes[*vertex as usize];
            }
        }
        self.vertices = self
            .vertices
            .iter()
            .enumerate()
            .filter_map(|(i, _)| {
                if new_indexes[i] != u32::MAX {
                    Some(self.vertices[i].clone())
                } else {
                    None
                }
            })
            .collect();
        if !self.height.is_empty() {
            self.height = self
                .height
                .iter()
                .enumerate()
                .filter_map(|(i, _)| {
                    if new_indexes[i] != u32::MAX {
                        Some(self.height[i])
                    } else {
                        None
                    }
                })
                .collect();
        }
        removed
    }

    /// Update the `is_one_way` flag for each polygon.
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub fn update_is_one_way(&mut self) {
        for polygon in self.polygons.iter_mut() {
            if polygon.vertices.len() == 3 {
                polygon.is_one_way = polygon
                    .vertices
                    .iter()
                    .any(|vertex| self.vertices[*vertex as usize].polygons.len() == 2);
            } else {
                polygon.is_one_way = polygon
                    .vertices
                    .iter()
                    .filter(|vertex| self.vertices[**vertex as usize].polygons.len() == 2)
                    .count()
                    == polygon.vertices.len() - 2;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use glam::Vec2;

    use super::ccw_from_y;

    /// Integer directions with no common factor: every one is a distinct angle, and the
    /// components are exact, so neither the angle nor the cross product is ambiguous.
    fn reduced_directions() -> Vec<Vec2> {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        let mut directions = vec![];
        for x in -8_i32..=8 {
            for y in -8_i32..=8 {
                if (x, y) != (0, 0) && gcd(x.abs(), y.abs()) == 1 {
                    directions.push(Vec2::new(x as f32, y as f32));
                }
            }
        }
        directions
    }

    /// What the comparator has to be: the order of `Vec2::Y.angle_to`, without calling it.
    #[test]
    fn agrees_with_the_angle_it_replaces() {
        for a in reduced_directions() {
            for b in reduced_directions() {
                let expected = Vec2::Y
                    .angle_to(a)
                    .partial_cmp(&Vec2::Y.angle_to(b))
                    .unwrap();
                assert_eq!(ccw_from_y(a, b), expected, "{a} against {b}");
            }
        }
    }

    /// Only the direction counts, so a longer one of the same heading is a tie.
    #[test]
    fn length_is_not_part_of_the_order() {
        for direction in reduced_directions() {
            assert_eq!(ccw_from_y(direction, direction * 7.0), Ordering::Equal);
        }
    }

    /// The sweep starts straight down and comes back to it, so a vertex's polygons are a
    /// cycle cut at `-Y`.
    #[test]
    fn sweeps_counter_clockwise_from_straight_down() {
        let mut directions = vec![
            Vec2::new(-1.0, 1.0),
            Vec2::new(0.0, -1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(-1.0, 0.0),
            Vec2::new(1.0, -1.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(-1.0, -1.0),
            Vec2::new(1.0, 0.0),
        ];
        directions.sort_unstable_by(|a, b| ccw_from_y(*a, *b));

        assert_eq!(
            directions,
            vec![
                Vec2::new(0.0, -1.0),
                Vec2::new(1.0, -1.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(0.0, 1.0),
                Vec2::new(-1.0, 1.0),
                Vec2::new(-1.0, 0.0),
                Vec2::new(-1.0, -1.0),
            ]
        );
    }
}
