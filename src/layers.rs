#[cfg(feature = "tracing")]
use tracing::instrument;

use glam::{vec2, Vec2};
use rstar::RTree;
use smallvec::SmallVec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    helpers::Vec2Helper,
    instance::{EdgeSide, U32Layer},
    BoundedPolygon, MeshError, Polygon, Vertex,
};

/// Layer of a NavMesh
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Layer {
    /// List of `Vertex` in this mesh
    pub vertices: Vec<Vertex>,
    /// List of `Polygons` in this mesh
    pub polygons: Vec<Polygon>,
    /// Offset of the layer
    pub offset: Vec2,
    /// Scale of the layer
    #[cfg(feature = "detailed-layers")]
    #[cfg_attr(docsrs, doc(cfg(feature = "detailed-layers")))]
    pub scale: Vec2,
    pub(crate) baked_polygons: Option<RTree<BoundedPolygon>>,
    pub(crate) islands: Option<Vec<usize>>,
    /// Height of each vertex. Must either have zero elements to ignore heights, or the same length as vertices.
    pub height: Vec<f32>,
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            vertices: vec![],
            polygons: vec![],
            offset: Vec2::ZERO,
            #[cfg(feature = "detailed-layers")]
            scale: Vec2::ONE,
            baked_polygons: None,
            islands: None,
            height: vec![],
        }
    }
}

impl Layer {
    /// Remove pre-computed optimizations from the mesh. Call this if you modified the [`Mesh`].
    #[inline]
    pub fn unbake(&mut self) {
        self.baked_polygons = None;
        self.islands = None;
    }

    /// Pre-compute optimizations on the mesh
    ///
    /// Must be called on an unstitched layer.
    ///
    /// Optimisations available are:
    /// - [`Self::bake_polygon_finder`]
    /// - [`Self::bake_islands_detection`]
    pub fn bake(&mut self) {
        self.bake_polygon_finder();
        self.bake_islands_detection()
    }

    /// Speed up bailing out if two points are not reachable.
    ///
    /// This is useful if there are isolated zones in the mesh, and you need to check for a path
    /// between them.
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub fn bake_islands_detection(&mut self) {
        let mut islands = vec![usize::MAX; self.polygons.len()];
        while let Some((root, _)) = islands
            .iter()
            .enumerate()
            .find(|(_, island)| **island == usize::MAX)
        {
            let mut to_visit = Vec::new();
            to_visit.push(root);
            while let Some(next) = to_visit.pop() {
                if islands[next] == usize::MAX {
                    let polygon = &mut self.polygons[next];
                    islands[next] = root;
                    to_visit.extend(
                        polygon
                            .vertices
                            .iter()
                            .flat_map(|v| self.vertices[*v as usize].polygons.iter())
                            .filter_map(|i| {
                                if *i != u32::MAX {
                                    Some(*i as usize)
                                } else {
                                    None
                                }
                            }),
                    );
                }
            }
        }
        self.islands = Some(islands);
    }

    /// Speed up finding which polygon, if any, contains a point in the mesh.
    ///
    /// Uses a BVH. This is useful at the start of the pathfinding, to get the containing polygons
    /// for the start and end point. It can also be used through [`Self::point_in_mesh`] to check
    /// if a point is in the mesh.
    ///
    /// A layer without polygons is left unbaked: there is no tree to build, and every reader of
    /// the BVH already falls back to the linear scan when it is missing.
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub fn bake_polygon_finder(&mut self) {
        if self.polygons.is_empty() {
            self.baked_polygons = None;
            return;
        }
        let bounded_polygons = self
            .polygons
            .iter()
            .enumerate()
            .map(|(index, polygon)| {
                let (min, max) = polygon.vertices.iter().fold(
                    (vec2(f32::MAX, f32::MAX), vec2(f32::MIN, f32::MIN)),
                    |mut aabb, v| {
                        if let Some(v) = self.vertices.get(*v as usize) {
                            aabb.0.x = aabb.0.x.min(v.coords.x);
                            aabb.0.y = aabb.0.y.min(v.coords.y);
                            aabb.1.x = aabb.1.x.max(v.coords.x);
                            aabb.1.y = aabb.1.y.max(v.coords.y);
                        }
                        aabb
                    },
                );
                BoundedPolygon {
                    index,
                    aabb_min: [min.x, min.y],
                    aabb_max: [max.x, max.y],
                }
            })
            .collect::<Vec<_>>();

        self.baked_polygons = Some(RTree::bulk_load(bounded_polygons));
    }

    /// Create a `Layer` from a list of [`Vertex`] and [`Polygon`].
    pub fn new(vertices: Vec<Vertex>, polygons: Vec<Polygon>) -> Result<Self, MeshError> {
        if vertices.is_empty() || polygons.is_empty() {
            return Err(MeshError::EmptyMesh);
        }
        if polygons.len() > (2_i32.pow(24) - 1) as usize {
            return Err(MeshError::TooManyPolygons);
        }
        let mut layer = Layer {
            vertices,
            polygons,
            ..Default::default()
        };
        #[cfg(not(feature = "no-default-baking"))]
        layer.bake();
        // just to not get a warning on the mut borrow. should be pretty much free anyway
        #[cfg(feature = "no-default-baking")]
        layer.unbake();
        Ok(layer)
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub(crate) fn get_point_locations_unit(
        &self,
        point: Vec2,
    ) -> impl Iterator<Item = u32> + use<'_> {
        self.polygons
            .iter()
            .enumerate()
            .filter_map(move |(index, polygon)| {
                self.point_in_polygon(point, polygon)
                    .then_some(index as u32)
            })
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub(crate) fn get_point_locations_unit_baked<'a>(
        &'a self,
        point: &'a Vec2,
    ) -> impl Iterator<Item = u32> + use<'a> {
        let query_point = [point.x, point.y];
        self.baked_polygons
            .as_ref()
            .unwrap()
            .locate_in_envelope_intersecting(rstar::AABB::from_point(query_point))
            .filter_map(|bp| {
                self.point_in_polygon(*point, &self.polygons[bp.index])
                    .then_some(bp.index as u32)
            })
    }

    /// Find the first polygon containing the point using internal iteration (no SmallVec allocation).
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    pub(crate) fn find_first_point_location_baked(&self, point: &Vec2) -> Option<u32> {
        use core::ops::ControlFlow;
        let query_point = [point.x, point.y];
        let mut result = None;
        let _ = self
            .baked_polygons
            .as_ref()
            .unwrap()
            .locate_in_envelope_intersecting_int(rstar::AABB::from_point(query_point), |bp| {
                if self.point_in_polygon(*point, &self.polygons[bp.index]) {
                    result = Some(bp.index as u32);
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            });
        result
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    fn point_in_polygon(&self, point: Vec2, polygon: &Polygon) -> bool {
        let mut edged = false;
        for [edge0, edge1] in polygon.edges_index() {
            if edge0.max(edge1) as usize >= self.vertices.len() {
                return false;
            }
            edged = true;
            // Bounds are checked just before
            #[allow(unsafe_code)]
            let (last, next) = unsafe {
                (
                    self.vertices.get_unchecked(edge0 as usize).coords,
                    self.vertices.get_unchecked(edge1 as usize).coords,
                )
            };

            let current_side = point.side((last, next));
            if current_side == EdgeSide::Edge {
                if point.in_bounding_box((last, next)) {
                    return true;
                }
                continue;
            }
            if current_side != EdgeSide::Left {
                return false;
            }
        }
        if edged {
            return true;
        }
        false
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub(crate) fn get_point_location(&self, point: Vec2, delta: f32) -> Option<u32> {
        [
            vec2(0.0, 0.0),
            vec2(delta, 0.0),
            vec2(delta, delta),
            vec2(0.0, delta),
            vec2(-delta, delta),
            vec2(-delta, 0.0),
            vec2(-delta, -delta),
            vec2(0.0, -delta),
            vec2(delta, -delta),
        ]
        .iter()
        .map(|delta| {
            let point = point + *delta;
            if self.baked_polygons.is_none() {
                self.get_point_locations_unit(point).next()
            } else {
                self.find_first_point_location_baked(&point)
            }
            .unwrap_or(u32::MAX)
        })
        .find(|poly| *poly != u32::MAX)
    }

    /// Get all the vertices in a layer that are on a segment.
    pub fn get_vertices_on_segment(&self, start: Vec2, end: Vec2) -> Vec<usize> {
        let mut vertices = self
            .vertices
            .iter()
            .enumerate()
            .filter_map(|(idx, v)| {
                if v.coords.on_segment((start, end)) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        vertices.sort_unstable_by(|a, b| {
            self.vertices[*a]
                .coords
                .distance(start)
                .partial_cmp(&self.vertices[*b].coords.distance(start))
                .unwrap()
        });
        vertices
    }

    /// Find the closest point in the layer
    ///
    /// This will stop after searching in circle of radius up to `delta` * `steps` distance
    pub fn get_closest_point(&self, point: Vec2, delta: f32, steps: u32) -> Option<Vec2> {
        for step in 0..=steps {
            if let Some((new_point, _)) = self.get_closest_point_inner(point, delta, step) {
                return Some(new_point);
            }
        }
        None
    }

    /// Find the closest points in the layer.
    ///
    /// If there are several points at the same distance, all of them will be returned.
    /// This can happen when a layer have overlapping polygons.
    ///
    /// This will stop after searching in circle of radius up to `delta` * `steps` distance
    pub fn get_closest_points(&self, point: Vec2, delta: f32, steps: u32) -> Vec<(Vec2, u32)> {
        for step in 0..=steps {
            let points = self.get_closest_points_inner(point, delta, step);
            if !points.is_empty() {
                return points;
            }
        }
        vec![]
    }

    #[inline(always)]
    pub(crate) fn get_closest_point_inner(
        &self,
        point: Vec2,
        delta: f32,
        step: u32,
    ) -> Option<(Vec2, u32)> {
        let sample = 10;
        for i in 0..=(sample * step) {
            let angle = i as f32 * std::f32::consts::TAU / (sample * (step + 1)) as f32;
            let (x, y) = angle.sin_cos();
            let new_point = point + vec2(x, y) * delta * step as f32;
            let poly = if self.baked_polygons.is_none() {
                self.get_point_locations_unit(new_point).next()
            } else {
                self.find_first_point_location_baked(&new_point)
            }
            .unwrap_or(u32::MAX);

            if poly != u32::MAX {
                return Some((new_point, poly));
            }
        }
        None
    }

    /// Push every polygon of this layer that the point lands in, tagged with the layer
    /// index.
    ///
    /// Same walk as [`Self::get_closest_points_inner`], without building the intermediate
    /// `Vec` per layer and per step: a query over overlapping layers does this once per
    /// layer, and only the polygon indices are ever used.
    #[inline(always)]
    pub(crate) fn push_point_locations(
        &self,
        point: Vec2,
        delta: f32,
        step: u32,
        layer_index: u8,
        found: &mut SmallVec<[u32; 1]>,
    ) {
        // A mesh can carry layers holding nothing -- a chunk entirely covered by an obstacle,
        // or a recast area id that nothing was tagged with. Point location visits every layer
        // on every query, so it is worth one load not to walk into the rest of this.
        if self.polygons.is_empty() {
            return;
        }
        let sample = 10;
        for i in 0..=(sample * step) {
            let angle = i as f32 * std::f32::consts::TAU / (sample * (step + 1)) as f32;
            let (x, y) = angle.sin_cos();
            let new_point = point + vec2(x, y) * delta * step as f32;
            let before = found.len();
            if self.baked_polygons.is_none() {
                found.extend(
                    self.get_point_locations_unit(new_point)
                        .map(|polygon| U32Layer::from_layer_and_polygon(layer_index, polygon)),
                );
            } else {
                // Internal iteration: the lazy `locate_in_envelope_intersecting` costs
                // noticeably more per hit, and this runs once per layer per query.
                let query_point = [new_point.x, new_point.y];
                let _ = self
                    .baked_polygons
                    .as_ref()
                    .unwrap()
                    .locate_in_envelope_intersecting_int(
                        rstar::AABB::from_point(query_point),
                        |bp| {
                            if self.point_in_polygon(new_point, &self.polygons[bp.index]) {
                                found.push(U32Layer::from_layer_and_polygon(
                                    layer_index,
                                    bp.index as u32,
                                ));
                            }
                            core::ops::ControlFlow::<()>::Continue(())
                        },
                    );
            }
            if found.len() != before {
                return;
            }
        }
    }

    #[inline(always)]
    pub(crate) fn get_closest_points_inner(
        &self,
        point: Vec2,
        delta: f32,
        step: u32,
    ) -> Vec<(Vec2, u32)> {
        let sample = 10;
        for i in 0..=(sample * step) {
            let angle = i as f32 * std::f32::consts::TAU / (sample * (step + 1)) as f32;
            let (x, y) = angle.sin_cos();
            let new_point = point + vec2(x, y) * delta * step as f32;
            let poly: Vec<(Vec2, u32)> = if self.baked_polygons.is_none() {
                self.get_point_locations_unit(new_point)
                    .map(|p| (new_point, p))
                    .collect()
            } else {
                self.get_point_locations_unit_baked(&new_point)
                    .map(|p| (new_point, p))
                    .collect()
            };
            if !poly.is_empty() {
                return poly;
            }
        }
        vec![]
    }

    /// Find the closest point in the layer in the given direction
    ///
    /// This will stop after going `delta` * `steps` distance in the `towards` direction
    pub fn get_closest_point_towards(
        &self,
        point: Vec2,
        delta: f32,
        steps: u32,
        towards: Vec2,
    ) -> Option<Vec2> {
        let direction = -(point - towards).normalize();
        for step in 0..steps {
            if let Some((new_point, _)) =
                self.get_closest_point_towards_inner(point, delta, direction, step)
            {
                return Some(new_point);
            }
        }
        None
    }

    #[inline(always)]
    pub(crate) fn get_closest_point_towards_inner(
        &self,
        point: Vec2,
        delta: f32,
        direction: Vec2,
        step: u32,
    ) -> Option<(Vec2, u32)> {
        if self.polygons.is_empty() {
            return None;
        }
        let point = point + direction * delta * step as f32;
        let poly = if self.baked_polygons.is_none() {
            self.get_point_locations_unit(point).next()
        } else {
            self.find_first_point_location_baked(&point)
        }
        .unwrap_or(u32::MAX);
        if poly != u32::MAX {
            return Some((point, poly));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[cfg(feature = "detailed-layers")]
    use crate::helpers::line_intersect_segment;
    use crate::{instance::U32Layer, Coords, Layer, Mesh, Path, Polygon, SearchNode, Vertex};
    #[cfg(feature = "detailed-layers")]
    use glam::IVec2;
    use glam::{vec2, Vec2};

    fn mesh_u_grid() -> Mesh {
        let main_layer = Layer {
            vertices: vec![
                Vertex::new(vec2(0., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 0.), vec![0, 1, u32::MAX]),
                Vertex::new(vec2(2., 0.), vec![1, 2, u32::MAX]),
                Vertex::new(vec2(3., 0.), vec![2, u32::MAX]),
                Vertex::new(vec2(0., 1.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 1.), vec![1, 0, u32::MAX]),
                Vertex::new(vec2(2., 1.), vec![2, 1, u32::MAX]),
                Vertex::new(vec2(3., 1.), vec![2, u32::MAX]),
            ],
            polygons: vec![
                Polygon::new(vec![0, 1, 5, 4], false),
                Polygon::new(vec![1, 2, 6, 5], false),
                Polygon::new(vec![2, 3, 7, 6], false),
            ],
            ..Default::default()
        };
        let mut mesh = Mesh {
            layers: vec![
                main_layer,
                Layer {
                    vertices: vec![
                        Vertex::new(vec2(0., 1.), vec![0, u32::MAX]),
                        Vertex::new(vec2(1., 1.), vec![0, u32::MAX]),
                        Vertex::new(vec2(0., 2.), vec![0, u32::MAX]),
                        Vertex::new(vec2(1., 2.), vec![0, u32::MAX]),
                    ],
                    polygons: vec![Polygon::new(vec![0, 1, 3, 2], true)],
                    ..Default::default()
                },
                Layer {
                    vertices: vec![
                        Vertex::new(vec2(2., 1.), vec![0, u32::MAX]),
                        Vertex::new(vec2(3., 1.), vec![0, u32::MAX]),
                        Vertex::new(vec2(2., 2.), vec![0, u32::MAX]),
                        Vertex::new(vec2(3., 2.), vec![0, u32::MAX]),
                    ],
                    polygons: vec![Polygon::new(vec![0, 1, 3, 2], true)],
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        mesh.bake();
        mesh.stitch_at_points(
            vec![
                ((0, 1), vec![vec2(0., 1.), vec2(1., 1.)]),
                ((0, 2), vec![vec2(2., 1.), vec2(3., 1.)]),
            ],
            false,
        );
        mesh
    }

    #[test]
    fn point_in_polygon() {
        let mesh = mesh_u_grid();
        assert_eq!(mesh.get_point_location(vec2(0.5, 0.5)), 0);
        assert_eq!(mesh.get_point_location(vec2(1.5, 0.5)), 1);
        assert_eq!(
            mesh.get_point_location(vec2(0.5, 1.5)),
            u32::from_layer_and_polygon(1, 0)
        );
        assert_eq!(mesh.get_point_location(vec2(1.5, 1.5)), u32::MAX);
        assert_eq!(
            mesh.get_point_location(vec2(2.5, 1.5)),
            u32::from_layer_and_polygon(2, 0)
        );
    }

    #[test]
    fn successors_straight_line() {
        let mesh = mesh_u_grid();

        let from: Vec2 = vec2(0.1, 1.1);
        let to = vec2(1.1, 0.1);
        let search_node = SearchNode {
            arena_parent: u32::MAX,
            root: from,
            interval: (vec2(0.0, 1.0), vec2(1.0, 1.0)),
            edge: (0, 1),
            polygon_from: mesh.get_point_location(from),
            polygon_to: mesh.get_point_location(to),
            previous_polygon_layer: 0,
            distance_start_to_root: 0.0,
            heuristic: from.distance(to),
        };
        let (successors, _arena) = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 0);
        #[cfg(not(feature = "detailed-layers"))]
        assert_eq!(
            mesh.path(from, to).unwrap(),
            Path {
                path: vec![to],
                length: from.distance(to),
                path_through_polygons: vec![16777216, 0, 1],
            }
        );
        #[cfg(feature = "detailed-layers")]
        {
            let path = mesh.path(from, to).unwrap();
            assert_eq!(path.path, vec![to]);
            assert!((path.length - from.distance(to)).abs() < 0.0001);
            assert!(path.path_with_layers[0].0.distance(vec2(0.2, 1.0)) < 0.0001);
            assert_eq!(path.path_with_layers[0].1, 0);
            assert!(path.path_with_layers[1].0.distance(to) < 0.0001);
            assert_eq!(path.path_with_layers[1].1, 0);
        }
    }

    #[test]
    fn successors_corner_first_step() {
        let mesh = mesh_u_grid();

        let from = vec2(0.1, 1.9);
        let to = vec2(2.1, 1.9);
        let search_node = SearchNode {
            arena_parent: u32::MAX,
            root: from,
            interval: (vec2(0.0, 1.0), vec2(1.0, 1.0)),
            edge: (4, 5),
            polygon_from: mesh.get_point_location(from),
            polygon_to: 0,
            previous_polygon_layer: 0,
            distance_start_to_root: 0.0,
            heuristic: from.distance(to),
        };
        let (successors, arena) = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].root, vec2(2.0, 1.0));
        assert_eq!(
            successors[0].distance_start_to_root,
            from.distance(vec2(1.0, 1.0)) + vec2(1.0, 1.0).distance(vec2(2.0, 1.0))
        );
        assert_eq!(successors[0].heuristic, vec2(2.0, 1.0).distance(to));
        assert_eq!(successors[0].polygon_from.polygon(), 2);
        assert_eq!(successors[0].polygon_to, u32::from_layer_and_polygon(2, 0));
        assert_eq!(successors[0].interval, (vec2(3.0, 1.0), vec2(2.0, 1.0)));
        assert_eq!(successors[0].edge, (7, 6));
        assert_eq!(
            crate::reconstruct_test_path(&arena, successors[0].arena_parent),
            vec![vec2(1.0, 1.0), vec2(2.0, 1.0)]
        );

        assert_eq!(
            mesh.path(from, to).unwrap(),
            Path {
                path: vec![vec2(1.0, 1.0), vec2(2.0, 1.0), to],
                length: from.distance(vec2(1.0, 1.0))
                    + vec2(1.0, 1.0).distance(vec2(2.0, 1.0))
                    + vec2(2.0, 1.0).distance(to),
                #[cfg(feature = "detailed-layers")]
                path_with_layers: vec![(vec2(1.0, 1.0), 0), (vec2(2.0, 1.0), 2), (to, 2)],
                path_through_polygons: vec![16777216, 0, 1, 2, 33554432],
            }
        );
    }

    /// layer 1:
    /// 000
    ///   1
    ///   222
    ///
    /// layer 2:
    ///
    /// 00000
    ///
    fn mesh_overlapping_layers() -> Mesh {
        let main_layer = Layer {
            vertices: vec![
                Vertex::new(vec2(0., 3.), vec![0, u32::MAX]),
                Vertex::new(vec2(3., 3.), vec![0, u32::MAX]),
                Vertex::new(vec2(0., 2.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 2.), vec![0, u32::MAX]),
                Vertex::new(vec2(2., 2.), vec![0, 1, u32::MAX]),
                Vertex::new(vec2(3., 2.), vec![0, 1, u32::MAX]),
                Vertex::new(vec2(2., 1.), vec![1, 2, u32::MAX]),
                Vertex::new(vec2(3., 1.), vec![1, 2, u32::MAX]),
                Vertex::new(vec2(4., 1.), vec![2, u32::MAX]),
                Vertex::new(vec2(5., 1.), vec![2, u32::MAX]),
                Vertex::new(vec2(2., 0.), vec![2, u32::MAX]),
                Vertex::new(vec2(5., 0.), vec![2, u32::MAX]),
            ],
            polygons: vec![
                Polygon::new(vec![2, 3, 4, 5, 1, 0], false),
                Polygon::new(vec![6, 7, 5, 4], false),
                Polygon::new(vec![10, 11, 9, 8, 7, 6], false),
            ],
            ..Default::default()
        };
        let overlapping_layer = Layer {
            vertices: vec![
                Vertex::new(vec2(0., 2.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 2.), vec![0, u32::MAX]),
                Vertex::new(vec2(5., 2.), vec![0, u32::MAX]),
                Vertex::new(vec2(0., 1.), vec![0, u32::MAX]),
                Vertex::new(vec2(4., 1.), vec![0, u32::MAX]),
                Vertex::new(vec2(5., 1.), vec![0, u32::MAX]),
            ],
            polygons: vec![Polygon::new(vec![3, 4, 5, 2, 1, 0], false)],
            ..Default::default()
        };
        let mut mesh = Mesh {
            layers: vec![main_layer, overlapping_layer],
            ..Default::default()
        };
        mesh.bake();
        let points = dbg!(mesh.find_stitch_points());
        mesh.stitch_at_points(points, false);
        mesh
    }

    #[cfg(feature = "detailed-layers")]
    fn reduce_path_precision(path: Vec<(Vec2, u8)>) -> Vec<(IVec2, u8)> {
        path.into_iter()
            .map(|(point, layer)| ((point * 100000.0).as_ivec2(), layer))
            .collect()
    }

    /// Compare a path against where its points are meant to be, rather than against the
    /// exact `f32` that came out last time.
    ///
    /// The points where a path crosses between layers are computed, not copied off a
    /// vertex, so the last few bits of them move whenever the arithmetic reaching them
    /// changes. Writing those bits into the test pins noise: it fails for changes that move
    /// a crossing by a millionth of a unit and says nothing about it landing in the right
    /// place.
    #[cfg(feature = "detailed-layers")]
    #[track_caller]
    fn assert_path_with_layers(actual: &[(Vec2, u8)], expected: &[(Vec2, u8)]) {
        assert_eq!(
            actual.len(),
            expected.len(),
            "expected {expected:?}, got {actual:?}"
        );
        for ((point, layer), (expected_point, expected_layer)) in actual.iter().zip(expected) {
            assert_eq!(
                layer, expected_layer,
                "expected {expected:?}, got {actual:?}"
            );
            assert!(
                point.distance(*expected_point) < 1.0e-4,
                "expected {expected:?}, got {actual:?}"
            );
        }
    }

    #[test]
    fn shortcut_blocked() {
        let mesh = mesh_overlapping_layers();
        for i in 0..15 {
            let from = vec2(i as f32 / 10.0, 2.1);
            let to = vec2(5.0 - i as f32 / 10.0, 0.9);
            let mut blocked = HashSet::default();
            blocked.insert(1);
            let path = dbg!(mesh.path_on_layers(from, to, blocked).unwrap());
            assert_eq!(path.path, vec![vec2(2.0, 2.0), vec2(3.0, 1.0), to]);
            #[cfg(feature = "detailed-layers")]
            assert_eq!(
                path.path_with_layers,
                vec![(vec2(2.0, 2.0), 0), (vec2(3.0, 1.0), 0), (to, 0)]
            );
        }
    }

    #[test]
    fn take_shortcut() {
        let mesh = mesh_overlapping_layers();
        for i in 0..6 {
            let from = vec2(i as f32 / 10.0, 2.1);
            let to = vec2(5.0 - i as f32 / 10.0, 0.9);
            let path = dbg!(mesh.path(from, to).unwrap());
            assert_eq!(path.path, vec![to]);
            #[cfg(feature = "detailed-layers")]
            assert_eq!(
                reduce_path_precision(path.path_with_layers),
                reduce_path_precision(vec![
                    (
                        line_intersect_segment((from, to), (vec2(0.0, 2.0), vec2(5.0, 2.0)))
                            .unwrap(),
                        1
                    ),
                    (
                        line_intersect_segment((from, to), (vec2(0.0, 1.0), vec2(5.0, 1.0)))
                            .unwrap(),
                        0
                    ),
                    (to, 0)
                ]),
            );
        }
    }

    #[test]
    fn take_shortcut_back() {
        let mesh = mesh_overlapping_layers();
        for i in 0..6 {
            let from = vec2(5.0 - i as f32 / 10.0, 0.9);
            let to = vec2(i as f32 / 10.0, 2.1);
            let path = dbg!(mesh.path(from, to).unwrap());
            assert_eq!(path.path, vec![to]);
            #[cfg(feature = "detailed-layers")]
            assert_eq!(
                reduce_path_precision(path.path_with_layers),
                reduce_path_precision(vec![
                    (
                        line_intersect_segment((from, to), (vec2(0.0, 1.0), vec2(5.0, 1.0)))
                            .unwrap(),
                        1
                    ),
                    (
                        line_intersect_segment((from, to), (vec2(0.0, 2.0), vec2(5.0, 2.0)))
                            .unwrap(),
                        0
                    ),
                    (to, 0)
                ]),
            );
        }
    }

    #[test]
    fn shortcut_with_corner() {
        let mesh = mesh_overlapping_layers();
        for i in 7..8 {
            let from = vec2(i as f32 / 10.0, 2.1);
            let to = vec2(5.0 - i as f32 / 10.0, 0.9);
            let path = dbg!(mesh.path(from, to).unwrap());
            match i {
                7 => {
                    assert_eq!(path.path, vec![vec2(1.0, 2.0), to]);
                    #[cfg(feature = "detailed-layers")]
                    assert_path_with_layers(
                        &path.path_with_layers,
                        &[(vec2(1.0, 2.0), 1), (vec2(4.0, 1.0), 0), (to, 0)],
                    );
                }
                _ if i < 11 => {
                    assert_eq!(path.path, vec![vec2(1.0, 2.0), vec2(4.0, 1.0), to]);
                    #[cfg(feature = "detailed-layers")]
                    assert_eq!(
                        path.path_with_layers,
                        vec![(vec2(1.0, 2.0), 1), (vec2(4.0, 1.0), 0), (to, 0)]
                    );
                }
                _ if i < 15 => {
                    assert_eq!(path.path, vec![vec2(2.0, 2.0), vec2(3.0, 1.0), to]);
                    #[cfg(feature = "detailed-layers")]
                    assert_eq!(
                        path.path_with_layers,
                        vec![(vec2(2.0, 2.0), 0), (vec2(3.0, 1.0), 0), (to, 0)]
                    );
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn shortcut_with_corner_back() {
        let mesh = mesh_overlapping_layers();
        for i in 7..15 {
            let from = vec2(5.0 - i as f32 / 10.0, 0.9);
            let to = vec2(i as f32 / 10.0, 2.1);
            let path = dbg!(mesh.path(from, to).unwrap());
            match i {
                7 => {
                    assert_eq!(path.path, vec![vec2(4.0, 1.0), to]);
                    #[cfg(feature = "detailed-layers")]
                    assert_path_with_layers(
                        &path.path_with_layers,
                        &[(vec2(4.0, 1.0), 1), (vec2(1.0, 2.0), 0), (to, 0)],
                    );
                }
                _ if i < 11 => {
                    assert_eq!(path.path, vec![vec2(4.0, 1.0), vec2(1.0, 2.0), to]);
                    #[cfg(feature = "detailed-layers")]
                    assert_eq!(
                        path.path_with_layers,
                        vec![(vec2(4.0, 1.0), 1), (vec2(1.0, 2.0), 0), (to, 0)]
                    );
                }
                _ if i < 15 => {
                    assert_eq!(path.path, vec![vec2(3.0, 1.0), vec2(2.0, 2.0), to]);
                    #[cfg(feature = "detailed-layers")]
                    assert_eq!(
                        path.path_with_layers,
                        vec![(vec2(3.0, 1.0), 0), (vec2(2.0, 2.0), 0), (to, 0)]
                    );
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn from_one_to_the_other() {
        let mesh = mesh_overlapping_layers();
        let path = dbg!(mesh
            .path(
                Coords::on_layer(vec2(2.5, 1.5), 0),
                Coords::on_layer(vec2(2.5, 1.5), 1),
            )
            .unwrap());
        assert_eq!(
            path.path,
            vec![vec2(3.0, 1.0,), vec2(4.0, 1.0,), vec2(2.5, 1.5,),],
        );
        #[cfg(feature = "detailed-layers")]
        assert_eq!(
            path.path_with_layers,
            vec![
                (vec2(3.0, 1.0), 0),
                (vec2(4.0, 1.0), 1),
                (vec2(2.5, 1.5), 1),
            ],
        );

        let path_back = dbg!(mesh
            .path(
                Coords::on_layer(vec2(2.5, 1.5), 1),
                Coords::on_layer(vec2(2.5, 1.5), 0),
            )
            .unwrap());
        assert_eq!(
            path_back.path,
            vec![vec2(4.0, 1.0,), vec2(3.0, 1.0,), vec2(2.5, 1.5,),],
        );
        #[cfg(feature = "detailed-layers")]
        assert_eq!(
            path_back.path_with_layers,
            vec![
                (vec2(4.0, 1.0), 0),
                (vec2(3.0, 1.0), 0),
                (vec2(2.5, 1.5), 0),
            ],
        );
    }

    #[test]
    fn find_point_on_layer() {
        let mesh = mesh_overlapping_layers();
        assert_eq!(
            mesh.get_point_location(Coords::on_layer(vec2(2.5, 1.5), 0)),
            1
        );
        assert_eq!(
            mesh.get_point_location(Coords::on_layer(vec2(2.5, 1.5), 1)),
            u32::from_layer_and_polygon(1, 0)
        );
    }

    #[test]
    fn find_vertices_on_segment() {
        let mesh = mesh_u_grid();
        assert_eq!(
            mesh.layers[0].get_vertices_on_segment(vec2(0.0, 0.0), vec2(0.0, 1.0)),
            vec![0, 4]
        );
        assert_eq!(
            mesh.layers[0].get_vertices_on_segment(vec2(0.0, 0.0), vec2(4.0, 0.0)),
            vec![0, 1, 2, 3]
        );
    }

    #[test]
    fn get_closest_point() {
        let mut mesh = mesh_u_grid();
        mesh.search_steps = 100;
        mesh.search_delta = 0.01;

        assert_eq!(
            mesh.layers[0].get_closest_point(vec2(1.5, 1.5), 0.1, 10),
            Some(vec2(1.5, 1.0))
        );
        assert_eq!(
            mesh.get_closest_point(vec2(1.5, 1.5)),
            Some(Coords {
                pos: vec2(1.5, 1.0),
                layer: Some(0),
                polygon_index: U32Layer::from_layer_and_polygon(0, 1),
            })
        );

        assert_eq!(
            mesh.layers[0].get_closest_point(vec2(1.25, 1.5), 0.01, 100),
            Some(vec2(1.25, 1.0))
        );
        assert_eq!(
            mesh.layers[1].get_closest_point(vec2(1.25, 1.5), 0.01, 100),
            Some(vec2(1.0, 1.5))
        );
        assert_eq!(
            mesh.get_closest_point(vec2(1.25, 1.5)),
            Some(Coords {
                pos: vec2(1.0, 1.5),
                layer: Some(1),
                polygon_index: U32Layer::from_layer_and_polygon(1, 0),
            })
        );
    }

    #[test]
    fn get_closest_point_towards() {
        let mut mesh = mesh_u_grid();
        mesh.search_steps = 10;

        assert_eq!(
            mesh.layers[0].get_closest_point_towards(vec2(1.5, 1.5), 0.1, 10, vec2(1.5, 0.5)),
            Some(vec2(1.5, 1.0))
        );
        assert_eq!(
            mesh.get_closest_point_towards(vec2(1.5, 1.5), vec2(1.5, 0.5)),
            Some(Coords {
                pos: vec2(1.5, 1.0),
                layer: Some(0),
                polygon_index: 1,
            })
        );

        assert_eq!(
            mesh.layers[0].get_closest_point_towards(vec2(1.5, 1.5), 0.1, 10, vec2(0.5, 1.5)),
            None
        );
        assert_eq!(
            mesh.get_closest_point_towards(vec2(1.5, 1.5), vec2(0.5, 1.5)),
            Some(Coords {
                pos: vec2(1.0, 1.5),
                layer: Some(1),
                polygon_index: U32Layer::from_layer_and_polygon(1, 0),
            })
        );

        assert_eq!(
            mesh.layers[0].get_closest_point_towards(vec2(1.5, 1.5), 0.2, 10, vec2(1.5, 0.5)),
            Some(vec2(1.5, 0.9))
        );
    }
}
