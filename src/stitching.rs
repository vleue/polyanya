use std::collections::HashMap;

use glam::Vec2;

use crate::Mesh;

impl Mesh {
    fn stitch_internals(
        &mut self,
        target_layer: Option<u8>,
        stitch_points: Vec<((u8, u8), Vec<Vec2>)>,
    ) {
        // update indexes of layers
        for (layer_index, layer) in self.layers.iter_mut().enumerate() {
            if let Some(target_layer) = target_layer {
                if target_layer != layer_index as u8 {
                    continue;
                }
            }
            for vertex in layer.vertices.iter_mut() {
                for polygon_index in vertex.polygons.iter_mut() {
                    if *polygon_index != -1 {
                        *polygon_index += ((layer_index as u32) << 24) as isize;
                    }
                }
            }
        }
        for ((from, to), stitch_points) in stitch_points {
            if let Some(target_layer) = target_layer {
                if target_layer != from && target_layer != to {
                    continue;
                }
            }
            for stitch_point in stitch_points {
                let mut neighbors_to = {
                    let vertex_from = self.layers[from as usize]
                        .vertices
                        .iter()
                        .find(|v| v.coords == stitch_point)
                        .unwrap();
                    let mut neighbors_from = vertex_from
                        .polygons
                        .iter()
                        .filter(|n| **n != -1 && (*n >> 24) as u8 == from)
                        .cloned()
                        .collect::<Vec<_>>();
                    let vertex_to = self
                        .layers
                        .get_mut(to as usize)
                        .unwrap()
                        .vertices
                        .iter_mut()
                        .find(|v| v.coords == stitch_point)
                        .unwrap();
                    let neighbors_to = vertex_to
                        .polygons
                        .iter()
                        .filter(|n| **n != -1 && (*n >> 24) as u8 == to)
                        .cloned()
                        .collect::<Vec<_>>();
                    std::mem::swap(&mut vertex_to.polygons, &mut neighbors_from);
                    vertex_to.polygons.append(&mut neighbors_from);
                    neighbors_to
                };
                let vertex_from = self
                    .layers
                    .get_mut(from as usize)
                    .unwrap()
                    .vertices
                    .iter_mut()
                    .find(|v| v.coords == stitch_point)
                    .unwrap();
                std::mem::swap(&mut vertex_from.polygons, &mut neighbors_to);
                vertex_from.polygons.append(&mut neighbors_to);
            }
        }
    }

    /// Stitch points between layers. After, the polygons neighboring the stitch points will be
    /// marked as neighbors in both layers.
    pub fn stitch_at_points(&mut self, stitch_points: Vec<((u8, u8), Vec<Vec2>)>) {
        self.stitch_internals(None, stitch_points);
    }

    /// Remove all stitches between layers.
    ///
    /// This can be useful when updating the NavMesh after obstacles changed, and stitches need to be redone.
    pub fn remove_stitches(&mut self) {
        for (layer_index, layer) in self.layers.iter_mut().enumerate() {
            for vertex in layer.vertices.iter_mut() {
                vertex.polygons.retain_mut(|p| {
                    if *p == -1 {
                        return true;
                    }
                    if (*p >> 24) as u8 == layer_index as u8 {
                        *p = ((*p & 0b00000000111111111111111111111111) as u32) as isize;
                        true
                    } else {
                        false
                    }
                })
            }
        }
    }

    /// Remove stitches to a single layer.
    ///
    /// This can be useful when updating the NavMesh and changes are known to be contained in a single layer.
    pub fn remove_stitches_to_layer(&mut self, target_layer: u8) {
        for (layer_index, layer) in self.layers.iter_mut().enumerate() {
            for vertex in layer.vertices.iter_mut() {
                if layer_index as u8 == target_layer {
                    // target layer, drop all references to other layers
                    vertex.polygons.retain_mut(|p| {
                        if *p == -1 {
                            return true;
                        }
                        if (*p >> 24) as u8 == layer_index as u8 {
                            *p = ((*p & 0b00000000111111111111111111111111) as u32) as isize;
                            true
                        } else {
                            false
                        }
                    })
                } else {
                    // other layers, drop all references to target layer
                    vertex.polygons.retain(|p| {
                        if *p == -1 {
                            return true;
                        }
                        (*p >> 24) as u8 != target_layer
                    })
                }
            }
        }
    }

    /// Restitch points targeting a specific layer.
    ///
    /// This can be useful when updating the NavMesh after obstacles changed, and changes are limited to a single layer.
    ///
    /// This will produce an invalid NavMesh if one of the stitch points target a layer that hasn't been stitched already.
    pub fn restitch_layer_at_points(
        &mut self,
        target_layer: u8,
        stitch_points: Vec<((u8, u8), Vec<Vec2>)>,
    ) {
        self.stitch_internals(Some(target_layer), stitch_points);
    }

    /// Find stitch points between layers.
    ///
    /// This can be slow, as every vertex of every layer need to be compared with every others.
    ///
    /// It can also produce invalid results if some layers are overlapping, for example when they represent different levels/floors of the same area.
    pub fn find_stitch_points(&mut self) -> Vec<((u8, u8), Vec<Vec2>)> {
        let mut stitch_points: HashMap<(u8, u8), Vec<Vec2>> = HashMap::new();
        for (layer_index, layer) in self.layers.iter().enumerate() {
            for vertex in layer.vertices.iter() {
                for (layer_index_b, layer_b) in self.layers.iter().enumerate().skip(layer_index + 1)
                {
                    for vertex_b in layer_b.vertices.iter() {
                        if vertex.coords == vertex_b.coords {
                            stitch_points
                                .entry((layer_index as u8, layer_index_b as u8))
                                .or_default()
                                .push(vertex.coords);
                        }
                    }
                }
            }
        }
        let mut stitch_points: Vec<_> = stitch_points.into_iter().collect();
        stitch_points.sort_by_key(|((a, b), _)| (*a, *b));
        stitch_points
    }
}

#[cfg(test)]
mod tests {
    use crate::{Layer, Mesh, Polygon, Vertex};
    use glam::Vec2;

    fn basic_mesh_with_layers() -> Mesh {
        Mesh {
            layers: vec![
                Layer {
                    vertices: vec![
                        Vertex::new(Vec2::new(1., 0.), vec![0, -1]),
                        Vertex::new(Vec2::new(2., 0.), vec![0, -1]),
                        Vertex::new(Vec2::new(1., 1.), vec![0, -1]),
                        Vertex::new(Vec2::new(2., 1.), vec![0, -1]),
                    ],
                    polygons: vec![Polygon::new(vec![0, 1, 3, 2], true)],
                    ..Default::default()
                },
                Layer {
                    vertices: vec![
                        Vertex::new(Vec2::new(0., 0.), vec![0, -1]),
                        Vertex::new(Vec2::new(1., 0.), vec![0, -1]),
                        Vertex::new(Vec2::new(0., 1.), vec![0, -1]),
                        Vertex::new(Vec2::new(1., 1.), vec![0, -1]),
                    ],
                    polygons: vec![Polygon::new(vec![0, 1, 3, 2], true)],
                    ..Default::default()
                },
                Layer {
                    vertices: vec![
                        Vertex::new(Vec2::new(1., 1.), vec![0, -1]),
                        Vertex::new(Vec2::new(2., 1.), vec![0, -1]),
                        Vertex::new(Vec2::new(1., 2.), vec![0, -1]),
                        Vertex::new(Vec2::new(2., 2.), vec![0, -1]),
                    ],
                    polygons: vec![Polygon::new(vec![0, 1, 3, 2], true)],
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    }

    #[test]
    fn stitch_at_points() {
        let mut mesh = basic_mesh_with_layers();
        mesh.stitch_at_points(vec![
            ((0, 1), vec![Vec2::new(1., 0.), Vec2::new(1., 1.)]),
            ((0, 2), vec![Vec2::new(1., 1.), Vec2::new(2., 1.)]),
            ((1, 2), vec![Vec2::new(1., 1.)]),
        ]);
        assert_eq!(mesh.layers[0].vertices[0].polygons, vec![16777216, 0, -1]);
        assert_eq!(mesh.layers[0].vertices[1].polygons, vec![0, -1]);
        assert_eq!(
            mesh.layers[0].vertices[2].polygons,
            vec![33554432, 16777216, 0, -1]
        );
        assert_eq!(mesh.layers[0].vertices[3].polygons, vec![33554432, 0, -1]);

        assert_eq!(mesh.layers[1].vertices[0].polygons, vec![16777216, -1]);
        assert_eq!(mesh.layers[1].vertices[1].polygons, vec![0, 16777216, -1]);
        assert_eq!(mesh.layers[1].vertices[2].polygons, vec![16777216, -1]);
        assert_eq!(
            mesh.layers[1].vertices[3].polygons,
            vec![33554432, 0, 16777216, -1]
        );

        assert_eq!(
            mesh.layers[2].vertices[0].polygons,
            vec![16777216, 0, 33554432, -1]
        );
        assert_eq!(mesh.layers[2].vertices[1].polygons, vec![0, 33554432, -1]);
        assert_eq!(mesh.layers[2].vertices[2].polygons, vec![33554432, -1]);
        assert_eq!(mesh.layers[2].vertices[3].polygons, vec![33554432, -1]);
    }

    #[test]
    fn remove_stitches() {
        let mut mesh = basic_mesh_with_layers();
        mesh.stitch_at_points(vec![
            ((0, 1), vec![Vec2::new(1., 0.), Vec2::new(1., 1.)]),
            ((0, 2), vec![Vec2::new(1., 1.), Vec2::new(2., 1.)]),
            ((1, 2), vec![Vec2::new(1., 1.)]),
        ]);
        mesh.remove_stitches();
        assert_eq!(mesh.layers[0].vertices[0].polygons, vec![0, -1]);
        assert_eq!(mesh.layers[0].vertices[1].polygons, vec![0, -1]);
        assert_eq!(mesh.layers[0].vertices[2].polygons, vec![0, -1]);
        assert_eq!(mesh.layers[0].vertices[3].polygons, vec![0, -1]);

        assert_eq!(mesh.layers[1].vertices[0].polygons, vec![0, -1]);
        assert_eq!(mesh.layers[1].vertices[1].polygons, vec![0, -1]);
        assert_eq!(mesh.layers[1].vertices[2].polygons, vec![0, -1]);
        assert_eq!(mesh.layers[1].vertices[3].polygons, vec![0, -1]);

        assert_eq!(mesh.layers[2].vertices[0].polygons, vec![0, -1]);
        assert_eq!(mesh.layers[2].vertices[1].polygons, vec![0, -1]);
        assert_eq!(mesh.layers[2].vertices[2].polygons, vec![0, -1]);
        assert_eq!(mesh.layers[2].vertices[3].polygons, vec![0, -1]);
    }

    #[test]
    fn unstitch_layer() {
        let mut mesh = basic_mesh_with_layers();
        mesh.stitch_at_points(vec![
            ((0, 1), vec![Vec2::new(1., 0.), Vec2::new(1., 1.)]),
            ((0, 2), vec![Vec2::new(1., 1.), Vec2::new(2., 1.)]),
            ((1, 2), vec![Vec2::new(1., 1.)]),
        ]);
        mesh.remove_stitches_to_layer(1);
        assert_eq!(mesh.layers[0].vertices[0].polygons, vec![0, -1]);
        assert_eq!(mesh.layers[0].vertices[1].polygons, vec![0, -1]);
        assert_eq!(mesh.layers[0].vertices[2].polygons, vec![33554432, 0, -1]);
        assert_eq!(mesh.layers[0].vertices[3].polygons, vec![33554432, 0, -1]);

        assert_eq!(mesh.layers[1].vertices[0].polygons, vec![0, -1]);
        assert_eq!(mesh.layers[1].vertices[1].polygons, vec![0, -1]);
        assert_eq!(mesh.layers[1].vertices[2].polygons, vec![0, -1]);
        assert_eq!(mesh.layers[1].vertices[3].polygons, vec![0, -1]);

        assert_eq!(mesh.layers[2].vertices[0].polygons, vec![0, 33554432, -1]);
        assert_eq!(mesh.layers[2].vertices[1].polygons, vec![0, 33554432, -1]);
        assert_eq!(mesh.layers[2].vertices[2].polygons, vec![33554432, -1]);
        assert_eq!(mesh.layers[2].vertices[3].polygons, vec![33554432, -1]);
    }
    #[test]
    fn stitch_single_layer() {
        let mut mesh = basic_mesh_with_layers();
        mesh.restitch_layer_at_points(
            1,
            vec![
                ((0, 1), vec![Vec2::new(1., 0.), Vec2::new(1., 1.)]),
                ((0, 2), vec![Vec2::new(1., 1.), Vec2::new(2., 1.)]),
                ((1, 2), vec![Vec2::new(1., 1.)]),
            ],
        );

        assert_eq!(mesh.layers[0].vertices[0].polygons, vec![16777216, 0, -1]);
        assert_eq!(mesh.layers[0].vertices[1].polygons, vec![0, -1]);
        // order different from previous test
        assert_eq!(mesh.layers[0].vertices[2].polygons, vec![16777216, 0, -1]);
        assert_eq!(mesh.layers[0].vertices[3].polygons, vec![0, -1]);

        assert_eq!(mesh.layers[1].vertices[0].polygons, vec![16777216, -1]);
        assert_eq!(mesh.layers[1].vertices[1].polygons, vec![0, 16777216, -1]);
        assert_eq!(mesh.layers[1].vertices[2].polygons, vec![16777216, -1]);
        assert_eq!(mesh.layers[1].vertices[3].polygons, vec![0, 16777216, -1]);

        // these are not logical, `restitch_layer_at_points` should not be used to stitch a layer with other layers that haven't been stitched in already
        assert_eq!(mesh.layers[2].vertices[0].polygons, vec![16777216, 0, -1]);
        assert_eq!(mesh.layers[2].vertices[1].polygons, vec![0, -1]);
        assert_eq!(mesh.layers[2].vertices[2].polygons, vec![0, -1]);
        assert_eq!(mesh.layers[2].vertices[3].polygons, vec![0, -1]);
    }

    #[test]
    fn restitch_single_layer() {
        let mut mesh = basic_mesh_with_layers();
        mesh.stitch_at_points(vec![
            ((0, 1), vec![Vec2::new(1., 0.), Vec2::new(1., 1.)]),
            ((0, 2), vec![Vec2::new(1., 1.), Vec2::new(2., 1.)]),
            ((1, 2), vec![Vec2::new(1., 1.)]),
        ]);
        mesh.remove_stitches_to_layer(1);
        mesh.restitch_layer_at_points(
            1,
            vec![
                ((0, 1), vec![Vec2::new(1., 0.), Vec2::new(1., 1.)]),
                ((0, 2), vec![Vec2::new(1., 1.), Vec2::new(2., 1.)]),
                ((1, 2), vec![Vec2::new(1., 1.)]),
            ],
        );

        assert_eq!(mesh.layers[0].vertices[0].polygons, vec![16777216, 0, -1]);
        assert_eq!(mesh.layers[0].vertices[1].polygons, vec![0, -1]);
        // order different from previous test
        assert_eq!(
            mesh.layers[0].vertices[2].polygons,
            vec![16777216, 33554432, 0, -1]
        );
        assert_eq!(mesh.layers[0].vertices[3].polygons, vec![33554432, 0, -1]);

        assert_eq!(mesh.layers[1].vertices[0].polygons, vec![16777216, -1]);
        assert_eq!(mesh.layers[1].vertices[1].polygons, vec![0, 16777216, -1]);
        assert_eq!(mesh.layers[1].vertices[2].polygons, vec![16777216, -1]);
        assert_eq!(
            mesh.layers[1].vertices[3].polygons,
            vec![33554432, 0, 16777216, -1]
        );

        assert_eq!(
            mesh.layers[2].vertices[0].polygons,
            vec![16777216, 0, 33554432, -1]
        );
        assert_eq!(mesh.layers[2].vertices[1].polygons, vec![0, 33554432, -1]);
        assert_eq!(mesh.layers[2].vertices[2].polygons, vec![33554432, -1]);
        assert_eq!(mesh.layers[2].vertices[3].polygons, vec![33554432, -1]);
    }

    #[test]
    fn auto_stitch() {
        let mut mesh = basic_mesh_with_layers();
        let points = mesh.find_stitch_points();
        mesh.stitch_at_points(points.clone());

        assert_eq!(
            points,
            vec![
                ((0, 1), vec![Vec2::new(1., 0.), Vec2::new(1., 1.)]),
                ((0, 2), vec![Vec2::new(1., 1.), Vec2::new(2., 1.)]),
                ((1, 2), vec![Vec2::new(1., 1.)]),
            ]
        );

        assert_eq!(mesh.layers[0].vertices[0].polygons, vec![16777216, 0, -1]);
        assert_eq!(mesh.layers[0].vertices[1].polygons, vec![0, -1]);
        assert_eq!(
            mesh.layers[0].vertices[2].polygons,
            vec![33554432, 16777216, 0, -1]
        );
        assert_eq!(mesh.layers[0].vertices[3].polygons, vec![33554432, 0, -1]);

        assert_eq!(mesh.layers[1].vertices[0].polygons, vec![16777216, -1]);
        assert_eq!(mesh.layers[1].vertices[1].polygons, vec![0, 16777216, -1]);
        assert_eq!(mesh.layers[1].vertices[2].polygons, vec![16777216, -1]);
        assert_eq!(
            mesh.layers[1].vertices[3].polygons,
            vec![33554432, 0, 16777216, -1]
        );

        assert_eq!(
            mesh.layers[2].vertices[0].polygons,
            vec![16777216, 0, 33554432, -1]
        );
        assert_eq!(mesh.layers[2].vertices[1].polygons, vec![0, 33554432, -1]);
        assert_eq!(mesh.layers[2].vertices[2].polygons, vec![33554432, -1]);
        assert_eq!(mesh.layers[2].vertices[3].polygons, vec![33554432, -1]);
    }
}
