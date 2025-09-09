use std::collections::HashMap;

use glam::Vec2;

use crate::{instance::U32Layer, Mesh};

type StitchVertices = Vec<((u8, u8), Vec<(usize, usize)>)>;
type StitchPoints = Vec<((u8, u8), Vec<Vec2>)>;

impl Mesh {
    fn stitch_internals(
        &mut self,
        target_layer: Option<u8>,
        stitch_vertices: StitchVertices,
        one_way: bool,
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
                    if *polygon_index != u32::MAX {
                        *polygon_index += (layer_index as u32) << 24;
                    }
                }
            }
        }
        for ((from, to), stitch_vertices) in stitch_vertices {
            if let Some(target_layer) = target_layer {
                if target_layer != from && target_layer != to {
                    continue;
                }
            }
            for (stitch_from, stitch_to) in stitch_vertices {
                let mut neighbors_to = {
                    let vertex_from = self.layers[from as usize]
                        .vertices
                        .get(stitch_from)
                        .unwrap();
                    let mut neighbors_from = vertex_from
                        .polygons
                        .iter()
                        .filter(|n| **n != u32::MAX && n.layer() == from)
                        .cloned()
                        .collect::<Vec<_>>();
                    let vertex_to = self
                        .layers
                        .get_mut(to as usize)
                        .unwrap()
                        .vertices
                        .get_mut(stitch_to)
                        .unwrap();
                    let neighbors_to = vertex_to
                        .polygons
                        .iter()
                        .filter(|n| **n != u32::MAX && n.layer() == to)
                        .cloned()
                        .collect::<Vec<_>>();
                    if !one_way {
                        vertex_to.polygons.append(&mut neighbors_from);
                    }
                    neighbors_to
                };
                let vertex_from = self
                    .layers
                    .get_mut(from as usize)
                    .unwrap()
                    .vertices
                    .get_mut(stitch_from)
                    .unwrap();
                vertex_from.polygons.append(&mut neighbors_to);
            }
        }
        self.reorder_neighbors_ccw_and_fix_corners();
    }

    /// Stitch points between layers. After, the polygons neighboring the stitch points will be
    /// marked as neighbors in both layers.
    pub fn stitch_at_points(&mut self, stitch_points: StitchPoints, one_way: bool) {
        let stitch_vertices = stitch_points
            .into_iter()
            .map(|((from, to), points)| {
                let mut stitch_vertices = Vec::new();
                for point in points {
                    let stitch_from = self.layers[from as usize]
                        .vertices
                        .iter()
                        .position(|v| v.coords == point)
                        .unwrap();
                    let stitch_to = self.layers[to as usize]
                        .vertices
                        .iter()
                        .position(|v| v.coords == point)
                        .unwrap();
                    stitch_vertices.push((stitch_from, stitch_to));
                }
                ((from, to), stitch_vertices)
            })
            .collect();
        self.stitch_internals(None, stitch_vertices, one_way);
        // Stitching may change the effective connectivity, invalidate caches
        self.invalidate_polygon_count_cache();
    }

    /// Stitch vertices between layers. After, the polygons neighboring the stitch points will be
    /// marked as neighbors in both layers.
    pub fn stitch_at_vertices(&mut self, stitch_vertices: StitchVertices, one_way: bool) {
        self.stitch_internals(None, stitch_vertices, one_way);
        // Stitching may change the effective connectivity, invalidate caches
        self.invalidate_polygon_count_cache();
    }

    /// Remove all stitches between layers.
    ///
    /// This can be useful when updating the NavMesh after obstacles changed, and stitches need to be redone.
    pub fn remove_stitches(&mut self) {
        for (layer_index, layer) in self.layers.iter_mut().enumerate() {
            for vertex in layer.vertices.iter_mut() {
                vertex.polygons.retain_mut(|p| {
                    if *p == u32::MAX {
                        return true;
                    }
                    if p.layer() == layer_index as u8 {
                        *p = p.polygon();
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
                        if *p == u32::MAX {
                            return true;
                        }
                        if p.layer() == layer_index as u8 {
                            *p = p.polygon();
                            true
                        } else {
                            false
                        }
                    })
                } else {
                    // other layers, drop all references to target layer
                    vertex.polygons.retain(|p| {
                        if *p == u32::MAX {
                            return true;
                        }
                        p.layer() != target_layer
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
        stitch_points: StitchPoints,
        one_way: bool,
    ) {
        let stitch_vertices = stitch_points
            .into_iter()
            .map(|((from, to), points)| {
                let mut stitch_vertices = Vec::new();
                for point in points {
                    let stitch_from = self.layers[from as usize]
                        .vertices
                        .iter()
                        .position(|v| v.coords == point)
                        .unwrap();
                    let stitch_to = self.layers[to as usize]
                        .vertices
                        .iter()
                        .position(|v| v.coords == point)
                        .unwrap();
                    stitch_vertices.push((stitch_from, stitch_to));
                }
                ((from, to), stitch_vertices)
            })
            .collect();

        self.stitch_internals(Some(target_layer), stitch_vertices, one_way);
    }

    /// Restitch vertices targeting a specific layer.
    ///
    /// This can be useful when updating the NavMesh after obstacles changed, and changes are limited to a single layer.
    ///
    /// This will produce an invalid NavMesh if one of the stitch points target a layer that hasn't been stitched already.
    pub fn restitch_layer_at_vertices(
        &mut self,
        target_layer: u8,
        stitch_vertices: StitchVertices,
        one_way: bool,
    ) {
        self.stitch_internals(Some(target_layer), stitch_vertices, one_way);
    }

    /// Find stitch points between layers.
    ///
    /// This can be slow, as every vertex of every layer need to be compared with every others.
    ///
    /// It can also produce invalid results if some layers are overlapping, for example when they represent different levels/floors of the same area.
    pub fn find_stitch_points(&mut self) -> StitchPoints {
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
        stitch_points.sort_unstable_by_key(|((a, b), _)| (*a, *b));
        stitch_points
    }
}

#[cfg(test)]
mod tests {
    use crate::{Layer, Mesh, Path, Polygon, Triangulation, Vertex};
    use glam::{vec2, Vec2};

    fn basic_mesh_with_layers() -> Mesh {
        Mesh {
            layers: vec![
                Layer {
                    vertices: vec![
                        Vertex::new(Vec2::new(1., 0.), vec![0, u32::MAX]),
                        Vertex::new(Vec2::new(2., 0.), vec![0, u32::MAX]),
                        Vertex::new(Vec2::new(1., 1.), vec![0, u32::MAX]),
                        Vertex::new(Vec2::new(2., 1.), vec![0, u32::MAX]),
                    ],
                    polygons: vec![Polygon::new(vec![0, 1, 3, 2], true)],
                    ..Default::default()
                },
                Layer {
                    vertices: vec![
                        Vertex::new(Vec2::new(0., 0.), vec![0, u32::MAX]),
                        Vertex::new(Vec2::new(1., 0.), vec![0, u32::MAX]),
                        Vertex::new(Vec2::new(0., 1.), vec![0, u32::MAX]),
                        Vertex::new(Vec2::new(1., 1.), vec![0, u32::MAX]),
                    ],
                    polygons: vec![Polygon::new(vec![0, 1, 3, 2], true)],
                    ..Default::default()
                },
                Layer {
                    vertices: vec![
                        Vertex::new(Vec2::new(1., 1.), vec![0, u32::MAX]),
                        Vertex::new(Vec2::new(2., 1.), vec![0, u32::MAX]),
                        Vertex::new(Vec2::new(1., 2.), vec![0, u32::MAX]),
                        Vertex::new(Vec2::new(2., 2.), vec![0, u32::MAX]),
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
        mesh.stitch_at_points(
            vec![
                ((0, 1), vec![Vec2::new(1., 0.), Vec2::new(1., 1.)]),
                ((0, 2), vec![Vec2::new(1., 1.), Vec2::new(2., 1.)]),
                ((1, 2), vec![Vec2::new(1., 1.)]),
            ],
            false,
        );
        assert_eq!(
            mesh.layers[0].vertices[0].polygons,
            vec![0, 16777216, u32::MAX]
        );
        assert_eq!(mesh.layers[0].vertices[1].polygons, vec![0, u32::MAX]);
        assert_eq!(
            mesh.layers[0].vertices[2].polygons,
            vec![0, 33554432, u32::MAX, 16777216]
        );
        assert_eq!(
            mesh.layers[0].vertices[3].polygons,
            vec![33554432, 0, u32::MAX]
        );

        assert_eq!(
            mesh.layers[1].vertices[0].polygons,
            vec![16777216, u32::MAX]
        );
        assert_eq!(
            mesh.layers[1].vertices[1].polygons,
            vec![0, 16777216, u32::MAX]
        );
        assert_eq!(
            mesh.layers[1].vertices[2].polygons,
            vec![16777216, u32::MAX]
        );
        assert_eq!(
            mesh.layers[1].vertices[3].polygons,
            vec![0, 33554432, u32::MAX, 16777216]
        );

        assert_eq!(
            mesh.layers[2].vertices[0].polygons,
            vec![0, 33554432, u32::MAX, 16777216]
        );
        assert_eq!(
            mesh.layers[2].vertices[1].polygons,
            vec![33554432, 0, u32::MAX]
        );
        assert_eq!(
            mesh.layers[2].vertices[2].polygons,
            vec![33554432, u32::MAX]
        );
        assert_eq!(
            mesh.layers[2].vertices[3].polygons,
            vec![33554432, u32::MAX]
        );
    }

    #[test]
    fn remove_stitches() {
        let mut mesh = basic_mesh_with_layers();
        mesh.stitch_at_points(
            vec![
                ((0, 1), vec![Vec2::new(1., 0.), Vec2::new(1., 1.)]),
                ((0, 2), vec![Vec2::new(1., 1.), Vec2::new(2., 1.)]),
                ((1, 2), vec![Vec2::new(1., 1.)]),
            ],
            false,
        );
        mesh.remove_stitches();
        assert_eq!(mesh.layers[0].vertices[0].polygons, vec![0, u32::MAX]);
        assert_eq!(mesh.layers[0].vertices[1].polygons, vec![0, u32::MAX]);
        assert_eq!(mesh.layers[0].vertices[2].polygons, vec![0, u32::MAX]);
        assert_eq!(mesh.layers[0].vertices[3].polygons, vec![0, u32::MAX]);

        assert_eq!(mesh.layers[1].vertices[0].polygons, vec![0, u32::MAX]);
        assert_eq!(mesh.layers[1].vertices[1].polygons, vec![0, u32::MAX]);
        assert_eq!(mesh.layers[1].vertices[2].polygons, vec![0, u32::MAX]);
        assert_eq!(mesh.layers[1].vertices[3].polygons, vec![u32::MAX, 0]);

        assert_eq!(mesh.layers[2].vertices[0].polygons, vec![0, u32::MAX]);
        assert_eq!(mesh.layers[2].vertices[1].polygons, vec![0, u32::MAX]);
        assert_eq!(mesh.layers[2].vertices[2].polygons, vec![0, u32::MAX]);
        assert_eq!(mesh.layers[2].vertices[3].polygons, vec![0, u32::MAX]);
    }

    #[test]
    fn unstitch_layer() {
        let mut mesh = basic_mesh_with_layers();
        mesh.stitch_at_points(
            vec![
                ((0, 1), vec![Vec2::new(1., 0.), Vec2::new(1., 1.)]),
                ((0, 2), vec![Vec2::new(1., 1.), Vec2::new(2., 1.)]),
                ((1, 2), vec![Vec2::new(1., 1.)]),
            ],
            false,
        );
        mesh.remove_stitches_to_layer(1);
        assert_eq!(mesh.layers[0].vertices[0].polygons, vec![0, u32::MAX]);
        assert_eq!(mesh.layers[0].vertices[1].polygons, vec![0, u32::MAX]);
        assert_eq!(
            mesh.layers[0].vertices[2].polygons,
            vec![0, 33554432, u32::MAX]
        );
        assert_eq!(
            mesh.layers[0].vertices[3].polygons,
            vec![33554432, 0, u32::MAX]
        );

        assert_eq!(mesh.layers[1].vertices[0].polygons, vec![0, u32::MAX]);
        assert_eq!(mesh.layers[1].vertices[1].polygons, vec![0, u32::MAX]);
        assert_eq!(mesh.layers[1].vertices[2].polygons, vec![0, u32::MAX]);
        assert_eq!(mesh.layers[1].vertices[3].polygons, vec![u32::MAX, 0]);

        assert_eq!(
            mesh.layers[2].vertices[0].polygons,
            vec![0, 33554432, u32::MAX]
        );
        assert_eq!(
            mesh.layers[2].vertices[1].polygons,
            vec![33554432, 0, u32::MAX]
        );
        assert_eq!(
            mesh.layers[2].vertices[2].polygons,
            vec![33554432, u32::MAX]
        );
        assert_eq!(
            mesh.layers[2].vertices[3].polygons,
            vec![33554432, u32::MAX]
        );
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
            false,
        );

        assert_eq!(
            mesh.layers[0].vertices[0].polygons,
            vec![0, 16777216, u32::MAX]
        );
        assert_eq!(mesh.layers[0].vertices[1].polygons, vec![0, u32::MAX]);
        assert_eq!(
            mesh.layers[0].vertices[2].polygons,
            vec![0, u32::MAX, 16777216]
        );
        assert_eq!(mesh.layers[0].vertices[3].polygons, vec![0, u32::MAX]);

        assert_eq!(
            mesh.layers[1].vertices[0].polygons,
            vec![16777216, u32::MAX]
        );
        assert_eq!(
            mesh.layers[1].vertices[1].polygons,
            vec![0, 16777216, u32::MAX]
        );
        assert_eq!(
            mesh.layers[1].vertices[2].polygons,
            vec![16777216, u32::MAX]
        );
        assert_eq!(
            mesh.layers[1].vertices[3].polygons,
            vec![0, u32::MAX, 16777216]
        );

        // these are not logical, `restitch_layer_at_points` should not be used to stitch a layer with other layers that haven't been stitched in already
        assert_eq!(
            mesh.layers[2].vertices[0].polygons,
            vec![0, u32::MAX, 16777216]
        );
        assert_eq!(mesh.layers[2].vertices[1].polygons, vec![0, u32::MAX]);
        assert_eq!(mesh.layers[2].vertices[2].polygons, vec![0, u32::MAX]);
        assert_eq!(mesh.layers[2].vertices[3].polygons, vec![0, u32::MAX]);
    }

    #[test]
    fn restitch_single_layer() {
        let mut mesh = basic_mesh_with_layers();
        mesh.stitch_at_points(
            vec![
                ((0, 1), vec![Vec2::new(1., 0.), Vec2::new(1., 1.)]),
                ((0, 2), vec![Vec2::new(1., 1.), Vec2::new(2., 1.)]),
                ((1, 2), vec![Vec2::new(1., 1.)]),
            ],
            false,
        );
        mesh.remove_stitches_to_layer(1);
        mesh.restitch_layer_at_points(
            1,
            vec![
                ((0, 1), vec![Vec2::new(1., 0.), Vec2::new(1., 1.)]),
                ((0, 2), vec![Vec2::new(1., 1.), Vec2::new(2., 1.)]),
                ((1, 2), vec![Vec2::new(1., 1.)]),
            ],
            false,
        );

        assert_eq!(
            mesh.layers[0].vertices[0].polygons,
            vec![0, 16777216, u32::MAX]
        );
        assert_eq!(mesh.layers[0].vertices[1].polygons, vec![0, u32::MAX]);
        assert_eq!(
            mesh.layers[0].vertices[2].polygons,
            vec![0, 33554432, u32::MAX, 16777216]
        );
        assert_eq!(
            mesh.layers[0].vertices[3].polygons,
            vec![33554432, 0, u32::MAX]
        );

        assert_eq!(
            mesh.layers[1].vertices[0].polygons,
            vec![16777216, u32::MAX]
        );
        assert_eq!(
            mesh.layers[1].vertices[1].polygons,
            vec![0, 16777216, u32::MAX]
        );
        assert_eq!(
            mesh.layers[1].vertices[2].polygons,
            vec![16777216, u32::MAX]
        );
        assert_eq!(
            mesh.layers[1].vertices[3].polygons,
            vec![0, 33554432, u32::MAX, 16777216]
        );

        assert_eq!(
            mesh.layers[2].vertices[0].polygons,
            vec![0, 33554432, u32::MAX, 16777216]
        );
        assert_eq!(
            mesh.layers[2].vertices[1].polygons,
            vec![33554432, 0, u32::MAX]
        );
        assert_eq!(
            mesh.layers[2].vertices[2].polygons,
            vec![33554432, u32::MAX]
        );
        assert_eq!(
            mesh.layers[2].vertices[3].polygons,
            vec![33554432, u32::MAX]
        );
    }

    #[test]
    fn auto_stitch() {
        let mut mesh = basic_mesh_with_layers();
        let points = mesh.find_stitch_points();
        mesh.stitch_at_points(points.clone(), false);

        assert_eq!(
            points,
            vec![
                ((0, 1), vec![Vec2::new(1., 0.), Vec2::new(1., 1.)]),
                ((0, 2), vec![Vec2::new(1., 1.), Vec2::new(2., 1.)]),
                ((1, 2), vec![Vec2::new(1., 1.)]),
            ]
        );

        assert_eq!(
            mesh.layers[0].vertices[0].polygons,
            vec![0, 16777216, u32::MAX]
        );
        assert_eq!(mesh.layers[0].vertices[1].polygons, vec![0, u32::MAX]);
        assert_eq!(
            mesh.layers[0].vertices[2].polygons,
            vec![0, 33554432, u32::MAX, 16777216]
        );
        assert_eq!(
            mesh.layers[0].vertices[3].polygons,
            vec![33554432, 0, u32::MAX]
        );

        assert_eq!(
            mesh.layers[1].vertices[0].polygons,
            vec![16777216, u32::MAX]
        );
        assert_eq!(
            mesh.layers[1].vertices[1].polygons,
            vec![0, 16777216, u32::MAX]
        );
        assert_eq!(
            mesh.layers[1].vertices[2].polygons,
            vec![16777216, u32::MAX]
        );
        assert_eq!(
            mesh.layers[1].vertices[3].polygons,
            vec![0, 33554432, u32::MAX, 16777216]
        );

        assert_eq!(
            mesh.layers[2].vertices[0].polygons,
            vec![0, 33554432, u32::MAX, 16777216]
        );
        assert_eq!(
            mesh.layers[2].vertices[1].polygons,
            vec![33554432, 0, u32::MAX]
        );
        assert_eq!(
            mesh.layers[2].vertices[2].polygons,
            vec![33554432, u32::MAX]
        );
        assert_eq!(
            mesh.layers[2].vertices[3].polygons,
            vec![33554432, u32::MAX]
        );
    }

    fn layers_different_coordinates() -> Mesh {
        let layer_a = Layer {
            vertices: vec![
                Vertex::new(vec2(0., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(0., 1.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 1.), vec![0, u32::MAX]),
            ],
            polygons: vec![Polygon::new(vec![0, 1, 3, 2], false)],
            ..Default::default()
        };
        let layer_b = Layer {
            vertices: vec![
                Vertex::new(vec2(0., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(0., 1.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 1.), vec![0, u32::MAX]),
            ],
            polygons: vec![Polygon::new(vec![0, 1, 3, 2], false)],
            offset: vec2(1.0, 0.0),
            ..Default::default()
        };
        Mesh {
            layers: vec![layer_a, layer_b],
            ..Default::default()
        }
    }

    #[test]
    fn stitch_layers_different_coordinates() {
        let mut mesh = layers_different_coordinates();
        let indices_from = mesh.layers[0].get_vertices_on_segment(vec2(1.0, 0.0), vec2(1.0, 1.0));
        let indices_to = mesh.layers[1].get_vertices_on_segment(vec2(0.0, 0.0), vec2(0.0, 1.0));

        let stitch_indices = indices_from
            .into_iter()
            .zip(indices_to.into_iter())
            .collect();

        mesh.stitch_at_vertices(vec![((0, 1), stitch_indices)], false);

        assert_eq!(mesh.layers[0].vertices[0].polygons, vec![0, u32::MAX]);
        assert_eq!(
            mesh.layers[0].vertices[1].polygons,
            vec![1 << 24, 0, u32::MAX]
        );
        assert_eq!(mesh.layers[0].vertices[2].polygons, vec![0, u32::MAX,]);
        assert_eq!(
            mesh.layers[0].vertices[3].polygons,
            vec![1 << 24, u32::MAX, 0]
        );

        assert_eq!(
            mesh.layers[1].vertices[0].polygons,
            vec![1 << 24, 0, u32::MAX]
        );
        assert_eq!(mesh.layers[1].vertices[1].polygons, vec![1 << 24, u32::MAX]);
        assert_eq!(
            mesh.layers[1].vertices[2].polygons,
            vec![1 << 24, u32::MAX, 0]
        );
        assert_eq!(mesh.layers[1].vertices[3].polygons, vec![1 << 24, u32::MAX]);

        assert!(mesh.point_in_mesh(vec2(0.5, 0.5)));
        assert!(mesh.point_in_mesh(vec2(1.5, 0.5)));
    }

    #[test]
    fn path_stitch_layers_different_coordinates() {
        let mut mesh = layers_different_coordinates();
        let indices_from = mesh.layers[0].get_vertices_on_segment(vec2(1.0, 0.0), vec2(1.0, 1.0));
        let indices_to = mesh.layers[1].get_vertices_on_segment(vec2(0.0, 0.0), vec2(0.0, 1.0));

        let stitch_indices = indices_from
            .into_iter()
            .zip(indices_to.into_iter())
            .collect();

        mesh.stitch_at_vertices(vec![((0, 1), stitch_indices)], false);

        let path = mesh.path(vec2(0.5, 0.5), vec2(1.5, 0.5)).unwrap();
        assert_eq!(
            path,
            Path {
                length: 1.0,
                path: vec![vec2(1.5, 0.5)],
                #[cfg(feature = "detailed-layers")]
                path_with_layers: vec![(vec2(1.0, 0.5), 1), (vec2(1.5, 0.5), 1)],
                path_through_polygons: vec![0, 16777216],
            }
        );
        let path = mesh.path(vec2(1.5, 0.5), vec2(0.5, 0.5)).unwrap();
        assert_eq!(
            path,
            Path {
                length: 1.0,
                path: vec![vec2(0.5, 0.5)],
                #[cfg(feature = "detailed-layers")]
                path_with_layers: vec![(vec2(1.0, 0.5), 0), (vec2(0.5, 0.5), 0)],
                path_through_polygons: vec![16777216, 0],
            }
        );
    }

    #[test]
    fn path_with_obstacle_stitch_layers_different_coordinates() {
        let base_mesh = layers_different_coordinates();

        let mut triangulation_a = Triangulation::from_mesh(&base_mesh, 0);
        let mut triangulation_b = Triangulation::from_mesh(&base_mesh, 1);
        for obstacle in [
            // on the boundary
            vec![
                vec2(0.75, 0.25),
                vec2(0.75, 0.75),
                vec2(1.25, 0.75),
                vec2(1.25, 0.25),
            ],
            // on layer 0
            vec![vec2(0.0, 0.0), vec2(0.0, 0.25), vec2(0.25, 0.0)],
            vec![vec2(0.0, 1.0), vec2(0.0, 0.75), vec2(0.25, 1.0)],
            // on layer 1
            vec![vec2(2.0, 0.0), vec2(2.0, 0.25), vec2(1.75, 0.0)],
            vec![vec2(2.0, 2.0), vec2(2.0, 1.75), vec2(1.75, 2.0)],
        ] {
            triangulation_a.add_obstacle(obstacle.clone());
            triangulation_b.add_obstacle(obstacle.into_iter().map(|v| v - vec2(1.0, 0.0)));
        }
        let mut mesh = Mesh::default();
        let mut layer_a = triangulation_a.as_layer();
        layer_a.remove_useless_vertices();
        mesh.layers.push(layer_a);
        let mut layer_b = triangulation_b.as_layer();
        layer_b.remove_useless_vertices();
        layer_b.offset = vec2(1.0, 0.0);
        mesh.layers.push(layer_b);

        for layer in &mesh.layers {
            println!("====================");
            println!(
                "{:?}",
                layer.vertices.iter().map(|v| v.coords).collect::<Vec<_>>()
            );
            println!(
                "{:?}",
                layer
                    .polygons
                    .iter()
                    .map(|p| &p.vertices)
                    .collect::<Vec<_>>()
            );
        }

        let indices_from = mesh.layers[0].get_vertices_on_segment(vec2(1.0, 0.0), vec2(1.0, 1.0));
        let indices_to = mesh.layers[1].get_vertices_on_segment(vec2(0.0, 0.0), vec2(0.0, 1.0));

        let stitch_indices = indices_from
            .into_iter()
            .zip(indices_to.into_iter())
            .collect();

        mesh.stitch_at_vertices(vec![((0, 1), stitch_indices)], false);

        let path = mesh.path(vec2(0.5, 0.5), vec2(1.5, 0.5)).unwrap();
        assert_eq!(
            path.path,
            vec![vec2(0.75, 0.75), vec2(1.25, 0.75), vec2(1.5, 0.5)]
        );
        let path = mesh.path(vec2(1.5, 0.5), vec2(0.5, 0.5)).unwrap();
        assert_eq!(
            path.path,
            vec![vec2(1.25, 0.75), vec2(0.75, 0.75), vec2(0.5, 0.5)]
        );
    }

    #[test]
    fn path_stitch_layers_different_coordinates_with_scale() {
        let base_mesh = layers_different_coordinates();
        let triangulation_a = Triangulation::from_mesh(&base_mesh, 0);
        let triangulation_b = Triangulation::from_mesh(&base_mesh, 0);
        let triangulation_c = Triangulation::from_mesh(&base_mesh, 0);

        let mut mesh = Mesh::default();

        let mut layer_a = triangulation_a.as_layer();
        layer_a.remove_useless_vertices();
        layer_a.merge_polygons();
        mesh.layers.push(layer_a);

        let mut layer_b = triangulation_b.as_layer();
        layer_b.remove_useless_vertices();
        layer_b.offset = vec2(1.0, 0.0);
        #[cfg(feature = "detailed-layers")]
        {
            layer_b.scale = vec2(0.5, 1.0);
        }
        layer_b.merge_polygons();
        mesh.layers.push(layer_b);

        let mut layer_c = triangulation_c.as_layer();
        layer_c.remove_useless_vertices();
        layer_c.offset = vec2(2.0, 0.0);
        layer_c.merge_polygons();
        mesh.layers.push(layer_c);

        let indices_from = mesh.layers[0].get_vertices_on_segment(vec2(1.0, 0.0), vec2(1.0, 1.0));
        let indices_to = mesh.layers[1].get_vertices_on_segment(vec2(0.0, 0.0), vec2(0.0, 1.0));
        let stitch_indices_0_1 = indices_from
            .into_iter()
            .zip(indices_to.into_iter())
            .collect();

        let indices_from = mesh.layers[1].get_vertices_on_segment(vec2(1.0, 0.0), vec2(1.0, 1.0));
        let indices_to = mesh.layers[2].get_vertices_on_segment(vec2(0.0, 0.0), vec2(0.0, 1.0));
        let stitch_indices_1_2 = indices_from
            .into_iter()
            .zip(indices_to.into_iter())
            .collect();

        mesh.stitch_at_vertices(
            vec![
                ((0, 1), dbg!(stitch_indices_0_1)),
                ((1, 2), dbg!(stitch_indices_1_2)),
            ],
            false,
        );

        assert_eq!(mesh.get_point_location(vec2(0.5, 0.5)), 0);
        assert_eq!(mesh.get_point_location(vec2(1.5, 0.5)), 1 << 24);
        assert_eq!(mesh.get_point_location(vec2(2.5, 0.5)), 2 << 24);

        for layer in &mesh.layers {
            println!(
                "{:?}",
                // layer.vertices.iter().map(|v| v.coords).collect::<Vec<_>>()
                layer.vertices
            );
            println!(
                "{:?}",
                layer
                    .polygons
                    .iter()
                    .map(|p| &p.vertices)
                    .collect::<Vec<_>>()
            );
        }

        assert_eq!(
            mesh.path(vec2(0.5, 0.5), vec2(1.25, 0.5)).unwrap(),
            Path {
                #[cfg(not(feature = "detailed-layers"))]
                length: 0.75,
                path: vec![vec2(1.25, 0.5)],
                #[cfg(feature = "detailed-layers")]
                length: 0.625,
                #[cfg(feature = "detailed-layers")]
                path_with_layers: vec![(vec2(1.0, 0.5), 1), (vec2(1.25, 0.5), 1)],
                path_through_polygons: vec![0, 16777216],
            }
        );
        assert_eq!(
            mesh.path(vec2(1.25, 0.5), vec2(1.75, 0.5)).unwrap(),
            Path {
                length: 0.5,
                path: vec![vec2(1.75, 0.5)],
                #[cfg(feature = "detailed-layers")]
                path_with_layers: vec![(vec2(1.75, 0.5), 1)],
                path_through_polygons: vec![16777216],
            }
        );
        assert_eq!(
            mesh.path(vec2(0.5, 0.5), vec2(1.75, 0.5)).unwrap(),
            Path {
                #[cfg(not(feature = "detailed-layers"))]
                length: 1.25,
                path: vec![vec2(1.75, 0.5)],
                #[cfg(feature = "detailed-layers")]
                length: 0.875,
                #[cfg(feature = "detailed-layers")]
                path_with_layers: vec![(vec2(1.0, 0.5), 1), (vec2(1.75, 0.5), 1)],
                path_through_polygons: vec![0, 16777216],
            }
        );
        assert_eq!(
            mesh.path(vec2(1.75, 0.5), vec2(0.5, 0.5)).unwrap(),
            Path {
                #[cfg(not(feature = "detailed-layers"))]
                length: 1.25,
                path: vec![vec2(0.5, 0.5)],
                #[cfg(feature = "detailed-layers")]
                length: 0.875,
                #[cfg(feature = "detailed-layers")]
                path_with_layers: vec![(vec2(1.0, 0.5), 0), (vec2(0.5, 0.5), 0)],
                path_through_polygons: vec![16777216, 0],
            }
        );
    }

    #[test]
    fn path_with_different_scales() {
        let _scale = vec2(1.0, 1.1);
        let layer_start = Layer {
            vertices: vec![
                Vertex::new(vec2(0., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(2., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(0., 1.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 1.), vec![0, u32::MAX]),
                Vertex::new(vec2(2., 1.), vec![0, u32::MAX]),
            ],
            polygons: vec![Polygon::new(vec![0, 1, 4, 3, 2], false)],
            ..Default::default()
        };
        let layer_up = Layer {
            vertices: vec![
                Vertex::new(vec2(0., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(0., 1.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 1.), vec![0, u32::MAX]),
            ],
            polygons: vec![Polygon::new(vec![0, 1, 3, 2], false)],
            offset: vec2(0.0, 1.0),
            #[cfg(feature = "detailed-layers")]
            scale: _scale,
            ..Default::default()
        };
        let layer_down = Layer {
            vertices: vec![
                Vertex::new(vec2(0., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(0., 1.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 1.), vec![0, u32::MAX]),
            ],
            polygons: vec![Polygon::new(vec![0, 1, 3, 2], false)],
            offset: vec2(0.0, 2.0),
            #[cfg(feature = "detailed-layers")]
            scale: _scale,
            ..Default::default()
        };
        let layer_flat = Layer {
            vertices: vec![
                Vertex::new(vec2(0., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(0., 2.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 2.), vec![0, u32::MAX]),
            ],
            polygons: vec![Polygon::new(vec![0, 1, 3, 2], false)],
            offset: vec2(1.0, 1.0),
            ..Default::default()
        };
        let layer_end = Layer {
            vertices: vec![
                Vertex::new(vec2(0., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(2., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(0., 1.), vec![0, u32::MAX]),
                Vertex::new(vec2(2., 1.), vec![0, u32::MAX]),
            ],
            polygons: vec![Polygon::new(vec![0, 1, 2, 4, 3], false)],
            offset: vec2(0.0, 3.0),
            ..Default::default()
        };
        let mut mesh = Mesh {
            layers: vec![layer_start, layer_up, layer_down, layer_flat, layer_end],
            ..Default::default()
        };
        mesh.stitch_at_vertices(
            vec![
                ((0, 1), vec![(2, 0), (3, 1)]),
                ((1, 2), vec![(2, 0), (3, 1)]),
                ((0, 3), vec![(3, 0), (4, 1)]),
                ((2, 4), vec![(2, 0), (3, 1)]),
                ((3, 4), vec![(2, 1), (3, 2)]),
            ],
            false,
        );

        println!("========================================");
        for layer in &mesh.layers {
            println!("{:?}", layer.vertices);
            println!(
                "{:?}",
                layer
                    .polygons
                    .iter()
                    .map(|p| &p.vertices)
                    .collect::<Vec<_>>()
            );
        }
        println!("========================================");

        for i in 0..20 {
            let x = i as f32 / 10.0;
            let path = mesh.path(vec2(x, 0.1), vec2(x, 3.9)).unwrap();
            #[cfg(feature = "detailed-layers")]
            {
                println!("{:?}", x);
                println!("  - {:?}", path.length);
                println!("  - {:?}", path.path);
                println!("  - {:?}", path.path_with_layers);

                println!(
                    "   - direct length: {:?}",
                    vec2(x, 0.1).distance(vec2(x, 3.9))
                );
                println!("   - with hill: {:?}", 0.9 * 2.0 + 1.0 * _scale.y * 2.0,);
                println!(
                    "   - with long way: {:?}",
                    vec2(x, 0.1).distance(Vec2::ONE) * 2.0 + 2.0,
                );
                println!(
                    "  - should take: {:?}",
                    if x >= 1.0 {
                        "direct path"
                    } else if 0.9 * 2.0 + 1.0 * _scale.y * 2.0
                        < vec2(x, 0.1).distance(Vec2::ONE) * 2.0 + 2.0
                    {
                        "hill"
                    } else {
                        "long way"
                    }
                );
                if x < 0.6 {
                    println!(" -> taking hill");
                    // direct path with hill
                    assert_eq!(
                        path.path_with_layers
                            .iter()
                            .map(|(v, l)| (v.y, *l))
                            .collect::<Vec<_>>(),
                        vec![(1.0, 1), (2.0, 2), (3.0, 4), (3.9, 4)]
                    );
                    assert!((dbg!(path.length) - 4.0).abs() < 0.1);
                } else if x < 1.0 {
                    println!(" -> taking long way");
                    // path that avoids hill
                    assert_eq!(
                        path.path_with_layers
                            .iter()
                            .map(|(v, l)| (v.y, *l))
                            .collect::<Vec<_>>(),
                        vec![(1.0, 3), (3.0, 4), (3.9, 4)]
                    );
                    assert!(
                        (dbg!(path.length) - (vec2(x, 0.1).distance(Vec2::ONE) * 2.0 + 2.0)).abs()
                            < 0.01
                    );
                } else {
                    println!(" -> taking direct path");
                    // direct flat path
                    assert_eq!(
                        path.path_with_layers
                            .iter()
                            .map(|(v, l)| (v.y, *l))
                            .collect::<Vec<_>>(),
                        vec![(1.0, 3), (3.0, 4), (3.9, 4)]
                    );
                    assert!((dbg!(path.length) - 3.8).abs() < 0.01);
                }
            }
            #[cfg(not(feature = "detailed-layers"))]
            {
                assert!((dbg!(path.length) - 3.8).abs() < 0.01);
                assert_eq!(path.path, vec![vec2(x, 3.9)]);
            }

            let path = mesh.path(vec2(x, 3.9), vec2(x, 0.1)).unwrap();
            #[cfg(feature = "detailed-layers")]
            {
                println!("{:?}", x);
                println!("  - {:?}", path.length);
                println!("  - {:?}", path.path);
                println!("  - {:?}", path.path_with_layers);

                println!(
                    "   - direct length: {:?}",
                    vec2(x, 0.1).distance(vec2(x, 3.9))
                );
                println!("   - with hill: {:?}", 0.9 * 2.0 + 1.0 * _scale.y * 2.0,);
                println!(
                    "   - with long way: {:?}",
                    vec2(x, 0.1).distance(Vec2::ONE) * 2.0 + 2.0,
                );
                println!(
                    "  - should take: {:?}",
                    if x >= 1.0 {
                        "direct path"
                    } else if 0.9 * 2.0 + 1.0 * _scale.y * 2.0
                        < vec2(x, 0.1).distance(Vec2::ONE) * 2.0 + 2.0
                    {
                        "hill"
                    } else {
                        "long way"
                    }
                );
                if x < 0.6 {
                    println!(" -> taking hill");
                    // direct path with hill
                    assert_eq!(
                        path.path_with_layers
                            .iter()
                            .map(|(v, l)| (v.y, *l))
                            .collect::<Vec<_>>(),
                        vec![(3.0, 2), (2.0, 1), (1.0, 0), (0.1, 0)]
                    );
                    assert!((dbg!(path.length) - 4.0).abs() < 0.01);
                } else if x < 1.0 {
                    println!(" -> taking long way");
                    // path that avoids hill
                    assert_eq!(
                        path.path_with_layers
                            .iter()
                            .map(|(v, l)| (v.y, *l))
                            .collect::<Vec<_>>(),
                        vec![(3.0, 3), (1.0, 0), (0.1, 0)]
                    );
                    assert!(
                        (dbg!(path.length) - (vec2(x, 0.1).distance(Vec2::ONE) * 2.0 + 2.0)).abs()
                            < 0.01
                    );
                } else {
                    println!(" -> taking direct path");
                    // direct flat path
                    assert_eq!(
                        path.path_with_layers
                            .iter()
                            .map(|(v, l)| (v.y, *l))
                            .collect::<Vec<_>>(),
                        vec![(3.0, 3), (1.0, 0), (0.1, 0)]
                    );
                    assert!((dbg!(path.length) - 3.8).abs() < 0.01);
                }
            }
            #[cfg(not(feature = "detailed-layers"))]
            {
                assert!((dbg!(path.length) - 3.8).abs() < 0.01);
                assert_eq!(path.path, vec![vec2(x, 0.1)]);
            }
        }
    }
}
