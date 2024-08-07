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
                // For each polygon using a vertex, sort them in CCW order
                let mut polygons = vertex
                    .polygons
                    .iter()
                    .filter(|p| **p != u32::MAX)
                    .cloned()
                    .collect::<Vec<_>>();
                // Sort by the angle between the Y axis and the direction from the vertex to the center of the polygon
                polygons.sort_by_key(|p| {
                    let vertices =
                        &self.layers[p.layer() as usize].polygons[p.polygon() as usize].vertices;
                    let center = vertices
                        .iter()
                        .map(|v| self.layers[p.layer() as usize].vertices[*v as usize].coords)
                        .sum::<Vec2>()
                        / vertices.len() as f32;
                    let direction = center - vertex.coords;
                    let angle = Vec2::Y.angle_between(direction);
                    (angle * 100000.0) as i32
                });

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
                            let mut polygon_a = self.layers[pair[0].layer() as usize].polygons
                                [pair[0].polygon() as usize]
                                .vertices
                                .clone();
                            polygon_a.reverse();
                            let mut found = false;
                            let previous_a = polygon_a
                                .iter()
                                .cycle()
                                .find(|v| {
                                    if found {
                                        return true;
                                    }
                                    if self.layers[pair[0].layer() as usize].vertices[**v as usize]
                                        .coords
                                        == vertex.coords
                                    {
                                        found = true;
                                    }
                                    false
                                })
                                .unwrap();
                            let polygon_b = &self.layers[pair[1].layer() as usize].polygons
                                [pair[1].polygon() as usize]
                                .vertices;
                            let mut found = false;
                            let next_b = polygon_b
                                .iter()
                                .cycle()
                                .find(|v| {
                                    if found {
                                        return true;
                                    }
                                    if self.layers[pair[1].layer() as usize].vertices[**v as usize]
                                        .coords
                                        == vertex.coords
                                    {
                                        found = true;
                                    }
                                    false
                                })
                                .unwrap();
                            if self.layers[pair[0].layer() as usize].vertices[*previous_a as usize]
                                .coords
                                != self.layers[pair[1].layer() as usize].vertices[*next_b as usize]
                                    .coords
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
