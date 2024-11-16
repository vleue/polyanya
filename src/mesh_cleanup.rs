use glam::Vec2;
#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::{instance::U32Layer, Layer, Mesh};

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
                polygons.sort_unstable_by_key(|p| {
                    let vertices =
                        &self.layers[p.layer() as usize].polygons[p.polygon() as usize].vertices;
                    let center = vertices
                        .iter()
                        .map(|v| self.layers[p.layer() as usize].vertices[*v as usize].coords)
                        .sum::<Vec2>()
                        / vertices.len() as f32
                        + self.layers[p.layer() as usize].offset;
                    let direction = center - vertex_coords;
                    let angle = Vec2::Y.angle_to(direction);
                    (angle * 100000.0) as i32
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
                            .flat_map(|pair| {
                                let layer0 = &self.layers[pair[0].layer() as usize];
                                let layer1 = &self.layers[pair[1].layer() as usize];
                                let mut polygon0 =
                                    layer0.polygons[pair[0].polygon() as usize].vertices.clone();
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
                                    return vec![pair[0], u32::MAX];
                                };
                                let polygon1 =
                                    &layer1.polygons[pair[1].polygon() as usize].vertices;
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
                                    return vec![pair[0], u32::MAX];
                                };

                                if layer0.vertices[*previous0 as usize].coords + layer0.offset
                                    != layer1.vertices[*next1 as usize].coords + layer1.offset
                                {
                                    vec![pair[0], u32::MAX]
                                } else {
                                    vec![pair[0]]
                                }
                            })
                            .collect();
                    }
                    reordered_neighbors_in_layer.push(polygons);
                }
            }

            reordered_neighbors.push(reordered_neighbors_in_layer);
        }
        for (layer, new) in self.layers.iter_mut().zip(reordered_neighbors.into_iter()) {
            for (vertex, new) in layer.vertices.iter_mut().zip(new.into_iter()) {
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
        removed
    }
}
