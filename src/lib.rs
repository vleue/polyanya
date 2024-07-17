#![doc = include_str!("../README.md")]
#![warn(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]

const PRECISION: f32 = 1000.0;

#[cfg(feature = "stats")]
use std::{cell::Cell, time::Instant};
use std::{
    cmp::Ordering,
    fmt::{self, Debug, Display},
    hash::Hash,
};

use bvh2d::{
    aabb::{Bounded, AABB},
    bvh2d::BVH2d,
};
use glam::Vec2;

use helpers::Vec2Helper;
use instance::{EdgeSide, InstanceStep};
use log::error;
use thiserror::Error;
#[cfg(feature = "tracing")]
use tracing::instrument;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "async")]
mod async_helpers;
mod helpers;
mod input;
mod instance;
mod merger;
mod primitives;

#[cfg(feature = "async")]
pub use async_helpers::FuturePath;
pub use input::polyanya_file::PolyanyaFile;
pub use input::triangulation::Triangulation;
pub use input::trimesh::Trimesh;
pub use primitives::{Polygon, Vertex};

use crate::instance::SearchInstance;

/// A path between two points.
#[derive(Debug, PartialEq)]
pub struct Path {
    /// Length of the path.
    pub length: f32,
    /// Coordinates for each step of the path. The destination is the last step.
    pub path: Vec<Vec2>,
}

/// Layer of a NavMesh
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Layer {
    /// List of `Vertex` in this mesh
    pub vertices: Vec<Vertex>,
    /// List of `Polygons` in this mesh
    pub polygons: Vec<Polygon>,
    baked_polygons: Option<BVH2d>,
    islands: Option<Vec<usize>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PolygonInMesh {
    layer: u8,
    polygon: u32,
}

const POLYGON_NOT_FOUND: PolygonInMesh = PolygonInMesh {
    layer: 0,
    polygon: u32::MAX,
};

/// A navigation mesh
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mesh {
    /// Layers of the NavMesh
    pub layers: Vec<Layer>,
    delta: f32,
    #[cfg(feature = "stats")]
    pub(crate) scenarios: Cell<u32>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            layers: vec![],
            delta: 0.1,
            #[cfg(feature = "stats")]
            scenarios: Cell::new(0),
        }
    }
}

impl Mesh {
    /// Create a new single layer NavMesh
    pub fn new(vertices: Vec<Vertex>, polygons: Vec<Polygon>) -> Result<Self, MeshError> {
        let layer = Layer::new(vertices, polygons)?;
        Ok(Mesh {
            layers: vec![layer],
            ..Default::default()
        })
    }
}

struct Root(Vec2);

impl PartialEq for Root {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Root {}

impl Hash for Root {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ((self.0.x * PRECISION) as i32).hash(state);
        ((self.0.y * PRECISION) as i32).hash(state);
        state.finish();
    }
}

struct BoundedPolygon {
    aabb: (Vec2, Vec2),
}

impl Bounded for BoundedPolygon {
    fn aabb(&self) -> AABB {
        AABB::with_bounds(self.aabb.0, self.aabb.1)
    }
}

/// Errors that can happen when working creating a [`Mesh`]
#[derive(Error, Debug, Copy, Clone, PartialEq)]
pub enum MeshError {
    /// The mesh is empty.
    #[error("The mesh is empty")]
    EmptyMesh,
    /// The mesh is invalid, such as having a vertex that does not belong to any polygon.
    #[error("The mesh is invalid")]
    InvalidMesh,
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
                            .filter_map(|i| if *i != -1 { Some(*i as usize) } else { None }),
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
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub fn bake_polygon_finder(&mut self) {
        let bounded_polygons = self
            .polygons
            .iter_mut()
            .map(|polygon| BoundedPolygon {
                aabb: polygon.vertices.iter().fold(
                    (Vec2::new(f32::MAX, f32::MAX), Vec2::ZERO),
                    |mut aabb, v| {
                        if let Some(v) = self.vertices.get(*v as usize) {
                            if v.coords.x < aabb.0.x {
                                aabb.0.x = v.coords.x;
                            }
                            if v.coords.y < aabb.0.y {
                                aabb.0.y = v.coords.y;
                            }
                            if v.coords.x > aabb.1.x {
                                aabb.1.x = v.coords.x;
                            }
                            if v.coords.y > aabb.1.y {
                                aabb.1.y = v.coords.y;
                            }
                        }
                        aabb
                    },
                ),
            })
            .collect::<Vec<_>>();

        self.baked_polygons = Some(BVH2d::build(&bounded_polygons));
    }

    /// Create a `Layer` from a list of [`Vertex`] and [`Polygon`].
    pub fn new(vertices: Vec<Vertex>, polygons: Vec<Polygon>) -> Result<Self, MeshError> {
        if vertices.is_empty() || polygons.is_empty() {
            return Err(MeshError::EmptyMesh);
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
        mesh.unbake();
        Ok(layer)
    }
}

impl Mesh {
    /// Pre-compute optimizations on the mesh
    ///
    /// Call [Layer::bake] on each layer.
    pub fn bake(&mut self) {
        for layer in self.layers.iter_mut() {
            layer.bake();
        }
    }

    /// Remove pre-computed optimizations from the mesh. Call this if you modified the [`Mesh`].
    #[inline]
    pub fn unbake(&mut self) {
        for layer in self.layers.iter_mut() {
            layer.unbake();
        }
    }

    /// Compute a path between two points.
    ///
    /// This method returns a `Future`, to get the path in a blocking way use [`Self::path`].
    #[cfg(feature = "async")]
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub fn get_path(&self, from: Vec2, to: Vec2) -> FuturePath {
        FuturePath {
            from,
            to,
            mesh: self,
            instance: None,
            ending_polygon: POLYGON_NOT_FOUND,
        }
    }

    /// Compute a path between two points.
    ///
    /// This will be a [`Path`] if a path is found, or `None` if not.
    ///
    /// This method is blocking, to get the path in an async way use [`Self::get_path`].
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    pub fn path(&self, from: Vec2, to: Vec2) -> Option<Path> {
        #[cfg(feature = "stats")]
        let start = Instant::now();

        let starting_polygon_index = self.get_point_location(from);
        if starting_polygon_index.polygon == u32::MAX {
            return None;
        }
        let ending_polygon = self.get_point_location(to);
        if ending_polygon.polygon == u32::MAX {
            return None;
        }
        if starting_polygon_index.layer == ending_polygon.layer {
            if let Some(islands) = self.layers[starting_polygon_index.layer as usize]
                .islands
                .as_ref()
            {
                let start_island = islands.get(starting_polygon_index.polygon as usize);
                let end_island = islands.get(ending_polygon.polygon as usize);
                if start_island.is_some() && end_island.is_some() && start_island != end_island {
                    return None;
                }
            }
        }

        if starting_polygon_index == ending_polygon {
            #[cfg(feature = "stats")]
            {
                if self.scenarios.get() == 0 {
                    eprintln!(
                    "index;micros;successor_calls;generated;pushed;popped;pruned_post_pop;length",
                );
                }
                eprintln!(
                    "{};{};0;0;0;0;0;{}",
                    self.scenarios.get(),
                    start.elapsed().as_secs_f32() * 1_000_000.0,
                    from.distance(to),
                );
                self.scenarios.set(self.scenarios.get() + 1);
            }
            return Some(Path {
                length: from.distance(to),
                path: vec![to],
            });
        }

        let mut search_instance = SearchInstance::setup(
            self,
            (from, starting_polygon_index),
            (to, ending_polygon),
            #[cfg(feature = "stats")]
            start,
        );

        // Limit search to avoid an infinite loop.
        for _ in 0..self.layers[0].polygons.len() * 1000 {
            match search_instance.next() {
                InstanceStep::Found(path) => return Some(path),
                InstanceStep::NotFound => return None,
                InstanceStep::Continue => (),
            }
        }

        error!("Search from {from} to {to} failed. Please check the mesh is valid as this should not happen.");
        None
    }

    /// The delta set by [`Mesh::set_delta`]
    pub fn delta(&self) -> f32 {
        self.delta
    }

    /// Set the delta for search with [`Mesh::path`], [`Mesh::get_path`], and [`Mesh::point_in_mesh`].
    /// A given point (x, y)  will be searched in a square around a delimited by (x ± delta, y ± delta).
    ///
    /// Default is 0.1
    pub fn set_delta(&mut self, delta: f32) -> &mut Self {
        assert!(delta >= 0.0);
        self.delta = delta;
        self
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[cfg(test)]
    fn successors(&self, node: SearchNode, to: Vec2) -> Vec<SearchNode> {
        use hashbrown::HashMap;
        use std::collections::BinaryHeap;

        let mut search_instance = SearchInstance {
            #[cfg(feature = "stats")]
            start: Instant::now(),
            queue: BinaryHeap::new(),
            node_buffer: Vec::new(),
            root_history: HashMap::new(),
            from: Vec2::ZERO,
            to,
            polygon_to: self.get_point_location(to),
            mesh: self,
            #[cfg(feature = "stats")]
            pushed: 0,
            #[cfg(feature = "stats")]
            popped: 0,
            #[cfg(feature = "stats")]
            successors_called: 0,
            #[cfg(feature = "stats")]
            nodes_generated: 0,
            #[cfg(feature = "stats")]
            nodes_pruned_post_pop: 0,
            #[cfg(debug_assertions)]
            debug: false,
            #[cfg(debug_assertions)]
            fail_fast: -1,
        };
        search_instance.successors(node);
        search_instance.queue.drain().collect()
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[cfg(test)]
    fn edges_between(&self, node: &SearchNode) -> Vec<instance::Successor> {
        use hashbrown::HashMap;
        use std::collections::BinaryHeap;

        let search_instance = SearchInstance {
            #[cfg(feature = "stats")]
            start: Instant::now(),
            queue: BinaryHeap::new(),
            node_buffer: Vec::new(),
            root_history: HashMap::new(),
            from: Vec2::ZERO,
            to: Vec2::new(0.0, 0.0),
            polygon_to: self.get_point_location(Vec2::new(0.0, 0.0)),
            mesh: self,
            #[cfg(feature = "stats")]
            pushed: 0,
            #[cfg(feature = "stats")]
            popped: 0,
            #[cfg(feature = "stats")]
            successors_called: 0,
            #[cfg(feature = "stats")]
            nodes_generated: 0,
            #[cfg(feature = "stats")]
            nodes_pruned_post_pop: 0,
            #[cfg(debug_assertions)]
            debug: false,
            #[cfg(debug_assertions)]
            fail_fast: -1,
        };
        search_instance.edges_between(node).to_vec()
    }

    /// Check if a given point is in a `Mesh`
    pub fn point_in_mesh(&self, point: Vec2) -> bool {
        self.get_point_location(point).polygon != u32::MAX
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    fn get_point_location(&self, point: Vec2) -> PolygonInMesh {
        let delta = self.delta;
        [
            Vec2::new(0.0, 0.0),
            Vec2::new(delta, 0.0),
            Vec2::new(delta, delta),
            Vec2::new(0.0, delta),
            Vec2::new(-delta, delta),
            Vec2::new(-delta, 0.0),
            Vec2::new(-delta, -delta),
            Vec2::new(0.0, -delta),
            Vec2::new(delta, -delta),
        ]
        .iter()
        .flat_map(|delta| {
            self.layers
                .iter()
                .enumerate()
                .map(move |(index, layer)| PolygonInMesh {
                    layer: index as u8,
                    polygon: if layer.baked_polygons.is_none() {
                        layer.get_point_location_unit(point + *delta)
                    } else {
                        layer.get_point_location_unit_baked(point + *delta)
                    },
                })
        })
        .find(|poly| poly.polygon != u32::MAX)
        .unwrap_or(POLYGON_NOT_FOUND)
    }

    /// Stitch points between layers. After, the polygons neighboring the stitch points will be
    /// marked as neighbors in both layers.
    pub fn stitch_at_points(&mut self, stitch_points: Vec<((u8, u8), Vec<Vec2>)>) {
        // update indexes of layers
        for (layer_index, layer) in self.layers.iter_mut().enumerate().skip(1) {
            for vertex in layer.vertices.iter_mut() {
                for polygon_index in vertex.polygons.iter_mut() {
                    if *polygon_index != -1 {
                        *polygon_index += ((layer_index as u32) << 24) as isize;
                    }
                }
            }
        }
        for ((from, to), stitch_points) in stitch_points {
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
}

impl Layer {
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    fn get_point_location_unit(&self, point: Vec2) -> u32 {
        for (i, polygon) in self.polygons.iter().enumerate() {
            if self.point_in_polygon(point, polygon) {
                return i as u32;
            }
        }
        u32::MAX
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    fn get_point_location_unit_baked(&self, point: Vec2) -> u32 {
        self.baked_polygons
            .as_ref()
            .unwrap()
            .contains_iterator(&point)
            .find(|index| self.point_in_polygon(point, &self.polygons[*index]))
            .map(|index| index as u32)
            .unwrap_or(u32::MAX)
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    fn point_in_polygon(&self, point: Vec2, polygon: &Polygon) -> bool {
        let mut edged = false;
        for edge in polygon.edges_index().iter() {
            if edge.0.max(edge.1) as usize >= self.vertices.len() {
                return false;
            }
            edged = true;
            // Bounds are checked just before
            #[allow(unsafe_code)]
            let (last, next) = unsafe {
                (
                    self.vertices.get_unchecked(edge.0 as usize).coords,
                    self.vertices.get_unchecked(edge.1 as usize).coords,
                )
            };

            let current_side = point.side((last, next));
            if current_side == EdgeSide::Edge && point.on_segment((last, next)) {
                return true;
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
}

#[derive(PartialEq, Debug)]
struct SearchNode {
    path: Vec<Vec2>,
    root: Vec2,
    interval: (Vec2, Vec2),
    edge: (u32, u32),
    polygon_from: PolygonInMesh,
    polygon_to: PolygonInMesh,
    previous_polygon_layer: u8,
    f: f32,
    g: f32,
}

impl Display for SearchNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("root=({}, {}); ", self.root.x, self.root.y))?;
        f.write_str(&format!(
            "left=({}, {}); ",
            self.interval.1.x, self.interval.1.y
        ))?;
        f.write_str(&format!(
            "right=({}, {}); ",
            self.interval.0.x, self.interval.0.y
        ))?;
        f.write_str(&format!("f={:.2}, g={:.2} ", self.f + self.g, self.f))?;
        Ok(())
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SearchNode {}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.f + self.g).total_cmp(&(other.f + other.g)) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => self.f.total_cmp(&other.f),
            Ordering::Greater => Ordering::Less,
        }
    }
}

#[cfg(test)]
mod tests {
    macro_rules! assert_delta {
        ($x:expr, $y:expr) => {
            let val = $x;
            let expected = $y;
            if !((val - expected).abs() < 0.01) {
                assert_eq!(val, expected);
            }
        };
    }

    use glam::Vec2;

    use crate::{helpers::*, Layer, Mesh, Path, Polygon, PolygonInMesh, SearchNode, Vertex};

    fn mesh_u_grid() -> Mesh {
        let layer = Layer {
            vertices: vec![
                Vertex::new(Vec2::new(0., 0.), vec![0, -1]),
                Vertex::new(Vec2::new(1., 0.), vec![0, 1, -1]),
                Vertex::new(Vec2::new(2., 0.), vec![1, 2, -1]),
                Vertex::new(Vec2::new(3., 0.), vec![2, -1]),
                Vertex::new(Vec2::new(0., 1.), vec![3, 0, -1]),
                Vertex::new(Vec2::new(1., 1.), vec![3, 1, 0, -1]),
                Vertex::new(Vec2::new(2., 1.), vec![4, 2, 1, -1]),
                Vertex::new(Vec2::new(3., 1.), vec![4, 2, -1]),
                Vertex::new(Vec2::new(0., 2.), vec![3, -1]),
                Vertex::new(Vec2::new(1., 2.), vec![3, -1]),
                Vertex::new(Vec2::new(2., 2.), vec![4, -1]),
                Vertex::new(Vec2::new(3., 2.), vec![4, -1]),
            ],
            polygons: vec![
                Polygon::new(vec![0, 1, 5, 4], false),
                Polygon::new(vec![1, 2, 6, 5], false),
                Polygon::new(vec![2, 3, 7, 6], false),
                Polygon::new(vec![4, 5, 9, 8], true),
                Polygon::new(vec![6, 7, 11, 10], true),
            ],
            ..Default::default()
        };
        Mesh {
            layers: vec![layer],
            ..Default::default()
        }
    }

    #[test]
    fn point_in_polygon() {
        let mut mesh = mesh_u_grid();
        mesh.bake();
        assert_eq!(mesh.get_point_location(Vec2::new(0.5, 0.5)).polygon, 0);
        assert_eq!(mesh.get_point_location(Vec2::new(1.5, 0.5)).polygon, 1);
        assert_eq!(mesh.get_point_location(Vec2::new(0.5, 1.5)).polygon, 3);
        assert_eq!(
            mesh.get_point_location(Vec2::new(1.5, 1.5)).polygon,
            u32::MAX
        );
        assert_eq!(mesh.get_point_location(Vec2::new(2.5, 1.5)).polygon, 4);
    }

    #[test]
    fn successors_straight_line_ahead() {
        let mesh = mesh_u_grid();

        let from = Vec2::new(0.1, 0.1);
        let to = Vec2::new(2.9, 0.9);
        let search_node = SearchNode {
            path: vec![],
            root: from,
            interval: (Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0)),
            edge: (1, 5),
            polygon_from: mesh.get_point_location(from),
            polygon_to: PolygonInMesh {
                layer: 0,
                polygon: 1,
            },
            previous_polygon_layer: 0,
            f: from.distance(to),
            g: 0.0,
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].f, from.distance(to));
        assert_eq!(successors[0].g, from.distance(to));
        assert_eq!(successors[0].polygon_from.polygon, 1);
        assert_eq!(successors[0].polygon_to.polygon, 2);
        assert_eq!(
            successors[0].interval,
            (Vec2::new(2.0, 0.0), Vec2::new(2.0, 1.0))
        );
        assert_eq!(successors[0].edge, (2, 6));

        assert_eq!(successors[0].path, Vec::<Vec2>::new());

        assert_eq!(
            mesh.path(from, to).unwrap(),
            Path {
                path: vec![to],
                length: from.distance(to),
            }
        );
    }

    #[test]
    fn successors_straight_line_reversed() {
        let mesh = mesh_u_grid();

        let to = Vec2::new(0.1, 0.1);
        let from = Vec2::new(2.9, 0.9);
        let search_node = SearchNode {
            path: vec![],
            root: from,
            interval: (Vec2::new(2.0, 1.0), Vec2::new(2.0, 0.0)),
            edge: (6, 2),
            polygon_from: mesh.get_point_location(from),
            polygon_to: PolygonInMesh {
                layer: 0,
                polygon: 1,
            },
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].f, 0.0);
        assert_eq!(successors[0].g, to.distance(from));
        assert_eq!(successors[0].polygon_from.polygon, 1);
        assert_eq!(successors[0].polygon_to.polygon, 0);
        assert_eq!(
            successors[0].interval,
            (Vec2::new(1.0, 1.0), Vec2::new(1.0, 0.0))
        );
        assert_eq!(successors[0].edge, (5, 1));
        assert_eq!(successors[0].path, Vec::<Vec2>::new());

        assert_eq!(
            mesh.path(from, to).unwrap(),
            Path {
                path: vec![to],
                length: from.distance(to),
            }
        );
    }

    #[test]
    fn successors_corner_first_step() {
        let mesh = mesh_u_grid();

        let from = Vec2::new(0.1, 1.9);
        let to = Vec2::new(2.1, 1.9);
        let search_node = SearchNode {
            path: vec![],
            root: from,
            interval: (Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0)),
            edge: (4, 5),
            polygon_from: mesh.get_point_location(from),
            polygon_to: PolygonInMesh {
                layer: 0,
                polygon: 0,
            },
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].root, Vec2::new(2.0, 1.0));
        assert_eq!(
            successors[0].f,
            from.distance(Vec2::new(1.0, 1.0)) + Vec2::new(1.0, 1.0).distance(Vec2::new(2.0, 1.0))
        );
        assert_eq!(successors[0].g, Vec2::new(2.0, 1.0).distance(to));
        assert_eq!(successors[0].polygon_from.polygon, 2);
        assert_eq!(successors[0].polygon_to.polygon, 4);
        assert_eq!(
            successors[0].interval,
            (Vec2::new(3.0, 1.0), Vec2::new(2.0, 1.0))
        );
        assert_eq!(successors[0].edge, (7, 6));
        assert_eq!(successors[0].path, vec![from, Vec2::new(1.0, 1.0)]);

        assert_eq!(
            mesh.path(from, to).unwrap(),
            Path {
                path: vec![Vec2::new(1.0, 1.0), Vec2::new(2.0, 1.0), to],
                length: from.distance(Vec2::new(1.0, 1.0))
                    + Vec2::new(1.0, 1.0).distance(Vec2::new(2.0, 1.0))
                    + Vec2::new(2.0, 1.0).distance(to),
            }
        );
    }

    #[test]
    fn successors_corner_observable_second_step() {
        let mesh = mesh_u_grid();

        let from = Vec2::new(0.1, 1.9);
        let to = Vec2::new(2.1, 1.9);
        let search_node = SearchNode {
            path: vec![],
            root: from,
            interval: (Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0)),
            edge: (1, 5),
            polygon_from: PolygonInMesh {
                layer: 0,
                polygon: 0,
            },
            polygon_to: PolygonInMesh {
                layer: 0,
                polygon: 1,
            },
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].root, Vec2::new(2.0, 1.0));
        assert_eq!(
            successors[0].f,
            from.distance(Vec2::new(1.0, 1.0)) + Vec2::new(1.0, 1.0).distance(Vec2::new(2.0, 1.0))
        );
        assert_eq!(successors[0].g, Vec2::new(2.0, 1.0).distance(to));
        assert_eq!(successors[0].polygon_from.polygon, 2);
        assert_eq!(successors[0].polygon_to.polygon, 4);
        assert_eq!(
            successors[0].interval,
            (Vec2::new(3.0, 1.0), Vec2::new(2.0, 1.0))
        );
        assert_eq!(successors[0].edge, (7, 6));
        assert_eq!(successors[0].path, vec![from, Vec2::new(1.0, 1.0)]);

        assert_eq!(
            mesh.path(from, to).unwrap(),
            Path {
                path: vec![Vec2::new(1.0, 1.0), Vec2::new(2.0, 1.0), to],
                length: from.distance(Vec2::new(1.0, 1.0))
                    + Vec2::new(1.0, 1.0).distance(Vec2::new(2.0, 1.0))
                    + Vec2::new(2.0, 1.0).distance(to),
            }
        );
    }

    #[test]
    fn empty_mesh_fails() {
        let layer = Layer::new(vec![], vec![]);
        assert!(matches!(layer, Err(crate::MeshError::EmptyMesh)));
    }

    fn mesh_from_paper() -> Mesh {
        let layer = Layer {
            vertices: vec![
                Vertex::new(Vec2::new(0., 6.), vec![0, -1]),    // 0
                Vertex::new(Vec2::new(2., 5.), vec![0, -1, 2]), // 1
                Vertex::new(Vec2::new(5., 7.), vec![0, 2, -1]), // 2
                Vertex::new(Vec2::new(5., 8.), vec![0, -1]),    // 3
                Vertex::new(Vec2::new(0., 8.), vec![0, -1]),    // 4
                Vertex::new(Vec2::new(1., 4.), vec![1, -1]),    // 5
                Vertex::new(Vec2::new(2., 1.), vec![1, -1]),    // 6
                Vertex::new(Vec2::new(4., 1.), vec![1, -1]),    // 7
                Vertex::new(Vec2::new(4., 2.), vec![1, -1, 2]), // 8
                Vertex::new(Vec2::new(2., 4.), vec![1, 2, -1]), // 9
                Vertex::new(Vec2::new(7., 4.), vec![2, -1, 4]), // 10
                Vertex::new(Vec2::new(10., 7.), vec![2, 4, 6, -1, 3]), // 11
                Vertex::new(Vec2::new(7., 7.), vec![2, 3, -1]), // 12
                Vertex::new(Vec2::new(11., 8.), vec![3, -1]),   // 13
                Vertex::new(Vec2::new(7., 8.), vec![3, -1]),    // 14
                Vertex::new(Vec2::new(7., 0.), vec![5, 4, -1]), // 15
                Vertex::new(Vec2::new(11., 3.), vec![4, 5, -1]), // 16
                Vertex::new(Vec2::new(11., 5.), vec![4, -1, 6]), // 17
                Vertex::new(Vec2::new(12., 0.), vec![5, -1]),   // 18
                Vertex::new(Vec2::new(12., 3.), vec![5, -1]),   // 19
                Vertex::new(Vec2::new(13., 5.), vec![6, -1]),   // 20
                Vertex::new(Vec2::new(13., 7.), vec![6, -1]),   // 21
                Vertex::new(Vec2::new(1., 3.), vec![1, -1]),    // 22
            ],
            polygons: vec![
                Polygon::new(vec![0, 1, 2, 3, 4], true),
                Polygon::new(vec![5, 22, 6, 7, 8, 9], true),
                Polygon::new(vec![1, 9, 8, 10, 11, 12, 2], false),
                Polygon::new(vec![12, 11, 13, 14], true),
                Polygon::new(vec![10, 15, 16, 17, 11], false),
                Polygon::new(vec![15, 18, 19, 16], true),
                Polygon::new(vec![11, 17, 20, 21], true),
            ],
            ..Default::default()
        };
        Mesh {
            layers: vec![layer],
            ..Default::default()
        }
    }

    #[test]
    fn paper_point_in_polygon() {
        let mut mesh = mesh_from_paper();
        mesh.bake();
        assert_eq!(
            mesh.get_point_location(Vec2::new(0.5, 0.5)).polygon,
            u32::MAX
        );
        assert_eq!(mesh.get_point_location(Vec2::new(2.0, 6.0)).polygon, 0);
        assert_eq!(mesh.get_point_location(Vec2::new(2.0, 5.1)).polygon, 0);
        assert_eq!(mesh.get_point_location(Vec2::new(2.0, 1.5)).polygon, 1);
        assert_eq!(mesh.get_point_location(Vec2::new(4.0, 2.1)).polygon, 2);
    }

    #[test]
    fn paper_straight() {
        let mesh = mesh_from_paper();

        let from = Vec2::new(12.0, 0.0);
        let to = Vec2::new(7.0, 6.9);
        let search_node = SearchNode {
            path: vec![],
            root: from,
            interval: (Vec2::new(11.0, 3.0), Vec2::new(7.0, 0.0)),
            edge: (16, 15),
            polygon_from: mesh.get_point_location(from),
            polygon_to: PolygonInMesh {
                layer: 0,
                polygon: 4,
            },
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 2);

        assert_eq!(successors[1].root, Vec2::new(11.0, 3.0));
        assert_eq!(successors[1].f, from.distance(Vec2::new(11.0, 3.0)));
        assert_eq!(
            successors[1].g,
            Vec2::new(11.0, 3.0).distance(Vec2::new(9.75, 6.75))
                + Vec2::new(9.75, 6.75).distance(to)
        );
        assert_eq!(successors[1].polygon_from.polygon, 4);
        assert_eq!(successors[1].polygon_to.polygon, 2);
        assert_eq!(
            successors[1].interval,
            (Vec2::new(10.0, 7.0), Vec2::new(9.75, 6.75))
        );
        assert_eq!(successors[1].edge, (11, 10));
        assert_eq!(successors[1].path, vec![from]);

        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].f, 0.0);
        assert_eq!(successors[0].g, from.distance(to));
        assert_eq!(successors[0].polygon_from.polygon, 4);
        assert_eq!(successors[0].polygon_to.polygon, 2);
        assert_eq!(
            successors[0].interval,
            (Vec2::new(9.75, 6.75), Vec2::new(7.0, 4.0))
        );
        assert_eq!(successors[0].edge, (11, 10));
        assert_eq!(successors[0].path, Vec::<Vec2>::new());

        assert_eq!(mesh.path(from, to).unwrap().length, from.distance(to));
        assert_eq!(mesh.path(from, to).unwrap().path, vec![to]);
    }

    #[test]
    fn paper_corner_right() {
        let mesh = mesh_from_paper();

        let from = Vec2::new(12.0, 0.0);
        let to = Vec2::new(13.0, 6.0);
        let search_node = SearchNode {
            path: vec![],
            root: from,
            interval: (Vec2::new(11.0, 3.0), Vec2::new(7.0, 0.0)),
            edge: (16, 15),
            polygon_from: mesh.get_point_location(from),
            polygon_to: PolygonInMesh {
                layer: 0,
                polygon: 4,
            },
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 3);

        assert_eq!(successors[0].root, Vec2::new(11.0, 3.0));
        assert_eq!(successors[0].f, from.distance(Vec2::new(11.0, 3.0)));
        assert_eq!(
            successors[0].g,
            Vec2::new(11.0, 3.0).distance(Vec2::new(11.0, 5.0)) + Vec2::new(11.0, 5.0).distance(to)
        );
        assert_eq!(successors[0].polygon_from.polygon, 4);
        assert_eq!(successors[0].polygon_to.polygon, 6);
        assert_eq!(
            successors[0].interval,
            (Vec2::new(11.0, 5.0), Vec2::new(10.0, 7.0))
        );
        assert_eq!(successors[0].edge, (17, 11));
        assert_eq!(successors[0].path, vec![from]);

        assert_eq!(successors[1].root, Vec2::new(11.0, 3.0));
        assert_eq!(successors[1].f, from.distance(Vec2::new(11.0, 3.0)));
        assert_eq!(
            successors[1].g,
            Vec2::new(11.0, 3.0).distance(to.mirror((Vec2::new(10.0, 7.0), Vec2::new(9.75, 6.75))))
        );
        assert_eq!(successors[1].polygon_from.polygon, 4);
        assert_eq!(successors[1].polygon_to.polygon, 2);
        assert_eq!(
            successors[1].interval,
            (Vec2::new(10.0, 7.0), Vec2::new(9.75, 6.75))
        );
        assert_eq!(successors[1].edge, (11, 10));
        assert_eq!(successors[1].path, vec![from]);

        assert_eq!(successors[2].root, from);
        assert_eq!(successors[2].f, 0.0);
        assert_eq!(
            successors[2].g,
            from.distance(Vec2::new(9.75, 6.75))
                + Vec2::new(9.75, 6.75)
                    .distance(to.mirror((Vec2::new(9.75, 6.75), Vec2::new(7.0, 4.0))))
        );
        assert_eq!(successors[2].polygon_from.polygon, 4);
        assert_eq!(successors[2].polygon_to.polygon, 2);
        assert_eq!(
            successors[2].interval,
            (Vec2::new(9.75, 6.75), Vec2::new(7.0, 4.0))
        );
        assert_eq!(successors[2].edge, (11, 10));
        assert_eq!(successors[2].path, Vec::<Vec2>::new());

        assert_delta!(
            mesh.path(from, to).unwrap().length,
            from.distance(Vec2::new(11.0, 3.0))
                + Vec2::new(11.0, 3.0).distance(Vec2::new(11.0, 5.0))
                + Vec2::new(11.0, 5.0).distance(to)
        );
        assert_eq!(
            mesh.path(from, to).unwrap().path,
            vec![Vec2::new(11.0, 3.0), Vec2::new(11.0, 5.0), to]
        );
    }

    #[test]
    fn paper_corner_left() {
        let mesh = mesh_from_paper();

        let from = Vec2::new(12.0, 0.0);
        let to = Vec2::new(5.0, 3.0);
        let search_node = SearchNode {
            path: vec![],
            root: from,
            interval: (Vec2::new(11.0, 3.0), Vec2::new(7.0, 0.0)),
            edge: (16, 15),
            polygon_from: mesh.get_point_location(from),
            polygon_to: PolygonInMesh {
                layer: 0,
                polygon: 4,
            },
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 2);

        assert_eq!(successors[1].root, Vec2::new(11.0, 3.0));
        assert_eq!(successors[1].f, from.distance(Vec2::new(11.0, 3.0)));
        assert_eq!(
            successors[1].g,
            Vec2::new(11.0, 3.0).distance(Vec2::new(9.75, 6.75))
                + Vec2::new(9.75, 6.75).distance(to)
        );
        assert_eq!(successors[1].polygon_from.polygon, 4);
        assert_eq!(successors[1].polygon_to.polygon, 2);
        assert_eq!(
            successors[1].interval,
            (Vec2::new(10.0, 7.0), Vec2::new(9.75, 6.75))
        );
        assert_eq!(successors[1].edge, (11, 10));
        assert_eq!(successors[1].path, vec![from]);

        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].f, 0.0);
        assert_eq!(
            successors[0].g,
            from.distance(Vec2::new(7.0, 4.0)) + Vec2::new(7.0, 4.0).distance(to)
        );
        assert_eq!(successors[0].polygon_from.polygon, 4);
        assert_eq!(successors[0].polygon_to.polygon, 2);
        assert_eq!(
            successors[0].interval,
            (Vec2::new(9.75, 6.75), Vec2::new(7.0, 4.0))
        );
        assert_eq!(successors[0].edge, (11, 10));
        assert_eq!(successors[0].path, Vec::<Vec2>::new());

        assert_delta!(
            mesh.path(from, to).unwrap().length,
            from.distance(Vec2::new(7.0, 4.0)) + Vec2::new(7.0, 4.0).distance(to)
        );
        assert_eq!(
            mesh.path(from, to).unwrap().path,
            vec![Vec2::new(7.0, 4.0), to]
        );
    }

    #[test]
    fn paper_going_to_one_way_polygon() {
        let mesh = mesh_from_paper();

        let from = Vec2::new(11., 0.);
        let to = Vec2::new(9., 3.);
        let path = mesh.path(from, to);

        assert_eq!(path.unwrap().path, vec![to]);

        let path = mesh.path(to, from);

        assert_eq!(path.unwrap().path, vec![from]);
    }

    #[test]
    fn paper_corner_left_twice() {
        let mesh = mesh_from_paper();

        let from = Vec2::new(12.0, 0.0);
        let to = Vec2::new(3.0, 1.0);
        let search_node = SearchNode {
            path: vec![],
            root: from,
            interval: (Vec2::new(11.0, 3.0), Vec2::new(7.0, 0.0)),
            edge: (16, 15),
            polygon_from: mesh.get_point_location(from),
            polygon_to: PolygonInMesh {
                layer: 0,
                polygon: 4,
            },
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 2);

        assert_eq!(successors[1].root, Vec2::new(11.0, 3.0));
        assert_eq!(successors[1].f, from.distance(Vec2::new(11.0, 3.0)));
        assert_eq!(
            successors[1].g,
            Vec2::new(11.0, 3.0).distance(Vec2::new(9.75, 6.75))
                + Vec2::new(9.75, 6.75).distance(to)
        );
        assert_eq!(successors[1].polygon_from.polygon, 4);
        assert_eq!(successors[1].polygon_to.polygon, 2);
        assert_eq!(
            successors[1].interval,
            (Vec2::new(10.0, 7.0), Vec2::new(9.75, 6.75))
        );
        assert_eq!(successors[1].edge, (11, 10));
        assert_eq!(successors[1].path, vec![from]);

        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].f, 0.0);
        assert_eq!(
            successors[0].g,
            from.distance(Vec2::new(7.0, 4.0)) + Vec2::new(7.0, 4.0).distance(to)
        );
        assert_eq!(successors[0].polygon_from.polygon, 4);
        assert_eq!(successors[0].polygon_to.polygon, 2);
        assert_eq!(
            successors[0].interval,
            (Vec2::new(9.75, 6.75), Vec2::new(7.0, 4.0))
        );
        assert_eq!(successors[0].edge, (11, 10));
        assert_eq!(successors[0].path, Vec::<Vec2>::new());

        let successor = successors.into_iter().next().unwrap();
        let successors = dbg!(mesh.successors(successor, to));
        dbg!(&successors[0]);
        assert_eq!(successors.len(), 1);

        assert_delta!(
            mesh.path(from, to).unwrap().length,
            from.distance(Vec2::new(7.0, 4.0))
                + Vec2::new(7.0, 4.0).distance(Vec2::new(4.0, 2.0))
                + Vec2::new(4.0, 2.0).distance(to)
        );

        assert_eq!(
            mesh.path(from, to).unwrap().path,
            vec![Vec2::new(7.0, 4.0), Vec2::new(4.0, 2.0), to]
        );
    }

    #[test]
    fn edges_between_simple() {
        let mesh = mesh_from_paper();

        let from = Vec2::new(12.0, 0.0);
        let to = Vec2::new(3.0, 1.0);
        let search_node = SearchNode {
            path: vec![],
            root: from,
            interval: (Vec2::new(11.0, 3.0), Vec2::new(7.0, 0.0)),
            edge: (16, 15),
            polygon_from: mesh.get_point_location(from),
            polygon_to: PolygonInMesh {
                layer: 0,
                polygon: 4,
            },
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };

        let successors = mesh.edges_between(&search_node);

        for successor in &successors {
            println!("{successor:?}");
        }

        println!("=========================");

        let search_node = SearchNode {
            path: vec![],
            root: from,
            interval: (Vec2::new(9.75, 6.75), Vec2::new(7.0, 4.0)),
            edge: (11, 10),
            polygon_from: PolygonInMesh {
                layer: 0,
                polygon: 4,
            },
            polygon_to: PolygonInMesh {
                layer: 0,
                polygon: 2,
            },
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };

        let successors = mesh.edges_between(&search_node);

        for successor in &successors {
            println!("{successor:?}");
        }

        println!("=========================");

        let search_node = SearchNode {
            path: vec![],
            root: Vec2::new(11.0, 3.0),
            interval: (Vec2::new(10.0, 7.0), Vec2::new(7.0, 4.0)),
            edge: (11, 10),
            polygon_from: PolygonInMesh {
                layer: 0,
                polygon: 4,
            },
            polygon_to: PolygonInMesh {
                layer: 0,
                polygon: 2,
            },
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };

        let successors = mesh.edges_between(&search_node);

        for successor in &successors {
            println!("{successor:?}");
        }
    }

    #[test]
    fn edges_between_simple_u() {
        let mesh = mesh_u_grid();

        let search_node = SearchNode {
            path: vec![],
            root: Vec2::new(0.0, 0.0),
            interval: (Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0)),
            edge: (1, 5),
            polygon_from: PolygonInMesh {
                layer: 0,
                polygon: 0,
            },
            polygon_to: PolygonInMesh {
                layer: 0,
                polygon: 1,
            },
            previous_polygon_layer: 0,
            f: 0.0,
            g: 1.0,
        };

        let successors = mesh.edges_between(&search_node);

        for successor in &successors {
            println!("{successor:?}");
        }
    }

    mod layer {
        use glam::Vec2;

        use crate::{
            Layer, Mesh, Path, Polygon, PolygonInMesh, SearchNode, Vertex, POLYGON_NOT_FOUND,
        };

        fn mesh_u_grid() -> Mesh {
            let main_layer = Layer {
                vertices: vec![
                    Vertex::new(Vec2::new(0., 0.), vec![0, -1]),
                    Vertex::new(Vec2::new(1., 0.), vec![0, 1, -1]),
                    Vertex::new(Vec2::new(2., 0.), vec![1, 2, -1]),
                    Vertex::new(Vec2::new(3., 0.), vec![2, -1]),
                    Vertex::new(Vec2::new(0., 1.), vec![0, -1]),
                    Vertex::new(Vec2::new(1., 1.), vec![1, 0, -1]),
                    Vertex::new(Vec2::new(2., 1.), vec![2, 1, -1]),
                    Vertex::new(Vec2::new(3., 1.), vec![2, -1]),
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
                            Vertex::new(Vec2::new(0., 1.), vec![0, -1]),
                            Vertex::new(Vec2::new(1., 1.), vec![0, -1]),
                            Vertex::new(Vec2::new(0., 2.), vec![0, -1]),
                            Vertex::new(Vec2::new(1., 2.), vec![0, -1]),
                        ],
                        polygons: vec![Polygon::new(vec![0, 1, 3, 2], true)],
                        ..Default::default()
                    },
                    Layer {
                        vertices: vec![
                            Vertex::new(Vec2::new(2., 1.), vec![0, -1]),
                            Vertex::new(Vec2::new(3., 1.), vec![0, -1]),
                            Vertex::new(Vec2::new(2., 2.), vec![0, -1]),
                            Vertex::new(Vec2::new(3., 2.), vec![0, -1]),
                        ],
                        polygons: vec![Polygon::new(vec![0, 1, 3, 2], true)],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            };
            mesh.stitch_at_points(vec![
                ((0, 1), vec![Vec2::new(0., 1.), Vec2::new(1., 1.)]),
                ((0, 2), vec![Vec2::new(2., 1.), Vec2::new(3., 1.)]),
            ]);
            mesh
        }

        #[test]
        fn point_in_polygon() {
            let mut mesh = mesh_u_grid();
            // mesh.bake();
            assert_eq!(
                mesh.get_point_location(Vec2::new(0.5, 0.5)),
                PolygonInMesh {
                    layer: 0,
                    polygon: 0
                }
            );
            assert_eq!(
                mesh.get_point_location(Vec2::new(1.5, 0.5)),
                PolygonInMesh {
                    layer: 0,
                    polygon: 1
                }
            );
            assert_eq!(
                mesh.get_point_location(Vec2::new(0.5, 1.5)),
                PolygonInMesh {
                    layer: 1,
                    polygon: 0
                }
            );
            assert_eq!(
                mesh.get_point_location(Vec2::new(1.5, 1.5)),
                POLYGON_NOT_FOUND
            );
            assert_eq!(
                mesh.get_point_location(Vec2::new(2.5, 1.5)),
                PolygonInMesh {
                    layer: 2,
                    polygon: 0
                }
            );
        }

        #[test]
        fn successors_straight_line() {
            let mesh = mesh_u_grid();

            let from: Vec2 = Vec2::new(0.1, 1.1);
            let to = Vec2::new(1.1, 0.1);
            let search_node = SearchNode {
                path: vec![],
                root: from,
                interval: (Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0)),
                edge: (0, 1),
                polygon_from: mesh.get_point_location(from),
                polygon_to: mesh.get_point_location(to),
                previous_polygon_layer: 0,
                f: 0.0,
                g: from.distance(to),
            };
            let successors = dbg!(mesh.successors(search_node, to));
            assert_eq!(successors.len(), 0);
            assert_eq!(
                mesh.path(from, to).unwrap(),
                Path {
                    path: vec![to],
                    length: from.distance(to),
                }
            );
        }

        #[test]
        fn successors_corner_first_step() {
            let mesh = mesh_u_grid();

            let from = Vec2::new(0.1, 1.9);
            let to = Vec2::new(2.1, 1.9);
            let search_node = SearchNode {
                path: vec![],
                root: from,
                interval: (Vec2::new(0.0, 1.0), Vec2::new(1.0, 1.0)),
                edge: (4, 5),
                polygon_from: mesh.get_point_location(from),
                polygon_to: PolygonInMesh {
                    layer: 0,
                    polygon: 0,
                },
                previous_polygon_layer: 0,
                f: 0.0,
                g: from.distance(to),
            };
            let successors = dbg!(mesh.successors(search_node, to));
            assert_eq!(successors.len(), 1);
            assert_eq!(successors[0].root, Vec2::new(2.0, 1.0));
            assert_eq!(
                successors[0].f,
                from.distance(Vec2::new(1.0, 1.0))
                    + Vec2::new(1.0, 1.0).distance(Vec2::new(2.0, 1.0))
            );
            assert_eq!(successors[0].g, Vec2::new(2.0, 1.0).distance(to));
            assert_eq!(successors[0].polygon_from.polygon, 2);
            assert_eq!(
                successors[0].polygon_to,
                PolygonInMesh {
                    layer: 2,
                    polygon: 0
                }
            );
            assert_eq!(
                successors[0].interval,
                (Vec2::new(3.0, 1.0), Vec2::new(2.0, 1.0))
            );
            assert_eq!(successors[0].edge, (7, 6));
            assert_eq!(successors[0].path, vec![from, Vec2::new(1.0, 1.0)]);

            assert_eq!(
                mesh.path(from, to).unwrap(),
                Path {
                    path: vec![Vec2::new(1.0, 1.0), Vec2::new(2.0, 1.0), to],
                    length: from.distance(Vec2::new(1.0, 1.0))
                        + Vec2::new(1.0, 1.0).distance(Vec2::new(2.0, 1.0))
                        + Vec2::new(2.0, 1.0).distance(to),
                }
            );
        }
    }
}
