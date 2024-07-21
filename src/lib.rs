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
};

use bvh2d::aabb::{Bounded, AABB};
use glam::Vec2;

use instance::{InstanceStep, U32Layer};
use layers::Layer;
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
mod layers;
mod merger;
mod primitives;
mod stitching;

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
    /// Coordinates for each step of the path, including when changing layer. The destination is the last step.
    pub path_with_layers: Vec<(Vec2, u8)>,
}
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

/// A point in the navigation mesh
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coords {
    /// The position
    pub pos: Vec2,
    /// The layer
    ///
    /// If specified, the point will be searched in that layer only.
    pub layer: Option<u8>,
}

impl From<Vec2> for Coords {
    fn from(value: Vec2) -> Self {
        Coords {
            pos: value,
            layer: None,
        }
    }
}

impl Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(layer) = self.layer {
            write!(f, "({}, {})[{}]", self.pos.x, self.pos.y, layer)
        } else {
            write!(f, "({}, {})", self.pos.x, self.pos.y)
        }
    }
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
    /// One of the layer has too many polygons (more than 2^24-1).
    #[error("One layer has too many polygons")]
    TooManyPolygons,
}

impl Mesh {
    /// Pre-compute optimizations on the mesh
    ///
    /// Call [Layer::bake] on each layer. If the mesh has several layers, it must be called before stitching.
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
            ending_polygon: u32::MAX,
        }
    }

    /// Compute a path between two points.
    ///
    /// This will be a [`Path`] if a path is found, or `None` if not.
    ///
    /// This method is blocking, to get the path in an async way use [`Self::get_path`].
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    pub fn path(&self, from: impl Into<Coords>, to: impl Into<Coords>) -> Option<Path> {
        #[cfg(feature = "stats")]
        let start = Instant::now();

        let from = from.into();
        let to = to.into();

        let starting_polygon_index = self.get_point_location(from);
        if starting_polygon_index == u32::MAX {
            return None;
        }
        let ending_polygon = self.get_point_location(to);
        if ending_polygon == u32::MAX {
            return None;
        }
        if starting_polygon_index.layer() == ending_polygon.layer() {
            if let Some(islands) = self.layers[starting_polygon_index.layer() as usize]
                .islands
                .as_ref()
            {
                let start_island = islands.get(starting_polygon_index.polygon() as usize);
                let end_island = islands.get(ending_polygon.polygon() as usize);
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
                length: from.pos.distance(to.pos),
                path: vec![to.pos],
                path_with_layers: vec![(to.pos, ending_polygon.layer())],
            });
        }

        let mut search_instance = SearchInstance::setup(
            self,
            (from.pos, starting_polygon_index),
            (to.pos, ending_polygon),
            #[cfg(feature = "stats")]
            start,
        );

        // Limit search to avoid an infinite loop.
        for _ in 0..self.layers.iter().map(|l| l.polygons.len()).sum::<usize>() * 100 {
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
            from: node.root,
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
        use glam::vec2;
        use hashbrown::HashMap;
        use std::collections::BinaryHeap;

        let search_instance = SearchInstance {
            #[cfg(feature = "stats")]
            start: Instant::now(),
            queue: BinaryHeap::new(),
            node_buffer: Vec::new(),
            root_history: HashMap::new(),
            from: Vec2::ZERO,
            to: Vec2::ZERO,
            polygon_to: self.get_point_location(vec2(0.0, 0.0)),
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
    pub fn point_in_mesh(&self, point: impl Into<Coords>) -> bool {
        self.get_point_location(point) != u32::MAX
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    fn get_point_location(&self, point: impl Into<Coords>) -> u32 {
        let point = point.into();
        if let Some(layer_index) = point.layer {
            self.layers
                .get(layer_index as usize)
                .and_then(|layer| {
                    Some(U32Layer::from_layer_and_polygon(
                        layer_index,
                        layer.get_point_location(point.pos, self.delta)?,
                    ))
                })
                .unwrap_or(u32::MAX)
        } else {
            self.layers
                .iter()
                .enumerate()
                .flat_map(|(index, layer)| {
                    Some(U32Layer::from_layer_and_polygon(
                        index as u8,
                        layer.get_point_location(point.pos, self.delta)?,
                    ))
                })
                .find(|poly| poly != &u32::MAX)
                .unwrap_or(u32::MAX)
        }
    }
}

#[derive(PartialEq, Debug)]
struct SearchNode {
    path: Vec<Vec2>,
    path_with_layers: Vec<(Vec2, Vec2, u8)>,
    root: Vec2,
    interval: (Vec2, Vec2),
    edge: (u32, u32),
    polygon_from: u32,
    polygon_to: u32,
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

    use std::vec;

    use glam::{vec2, Vec2};

    use crate::{helpers::*, Layer, Mesh, Path, Polygon, SearchNode, Vertex};

    fn mesh_u_grid() -> Mesh {
        let layer = Layer {
            vertices: vec![
                Vertex::new(vec2(0., 0.), vec![0, u32::MAX]),
                Vertex::new(vec2(1., 0.), vec![0, 1, u32::MAX]),
                Vertex::new(vec2(2., 0.), vec![1, 2, u32::MAX]),
                Vertex::new(vec2(3., 0.), vec![2, u32::MAX]),
                Vertex::new(vec2(0., 1.), vec![3, 0, u32::MAX]),
                Vertex::new(vec2(1., 1.), vec![3, 1, 0, u32::MAX]),
                Vertex::new(vec2(2., 1.), vec![4, 2, 1, u32::MAX]),
                Vertex::new(vec2(3., 1.), vec![4, 2, u32::MAX]),
                Vertex::new(vec2(0., 2.), vec![3, u32::MAX]),
                Vertex::new(vec2(1., 2.), vec![3, u32::MAX]),
                Vertex::new(vec2(2., 2.), vec![4, u32::MAX]),
                Vertex::new(vec2(3., 2.), vec![4, u32::MAX]),
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
        assert_eq!(mesh.get_point_location(vec2(0.5, 0.5)), 0);
        assert_eq!(mesh.get_point_location(vec2(1.5, 0.5)), 1);
        assert_eq!(mesh.get_point_location(vec2(0.5, 1.5)), 3);
        assert_eq!(mesh.get_point_location(vec2(1.5, 1.5)), u32::MAX);
        assert_eq!(mesh.get_point_location(vec2(2.5, 1.5)), 4);
    }

    #[test]
    fn successors_straight_line_ahead() {
        let mesh = mesh_u_grid();

        let from = vec2(0.1, 0.1);
        let to = vec2(2.9, 0.9);
        let search_node = SearchNode {
            path: vec![],
            path_with_layers: vec![],
            root: from,
            interval: (vec2(1.0, 0.0), vec2(1.0, 1.0)),
            edge: (1, 5),
            polygon_from: mesh.get_point_location(from),
            polygon_to: 1,
            previous_polygon_layer: 0,
            f: from.distance(to),
            g: 0.0,
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].f, from.distance(to));
        assert_eq!(successors[0].g, from.distance(to));
        assert_eq!(successors[0].polygon_from, 1);
        assert_eq!(successors[0].polygon_to, 2);
        assert_eq!(successors[0].interval, (vec2(2.0, 0.0), vec2(2.0, 1.0)));
        assert_eq!(successors[0].edge, (2, 6));

        assert_eq!(successors[0].path, Vec::<Vec2>::new());

        assert_eq!(
            mesh.path(from, to).unwrap(),
            Path {
                path: vec![to],
                length: from.distance(to),
                path_with_layers: vec![(to, 0)],
            }
        );
    }

    #[test]
    fn successors_straight_line_reversed() {
        let mesh = mesh_u_grid();

        let to = vec2(0.1, 0.1);
        let from = vec2(2.9, 0.9);
        let search_node = SearchNode {
            path: vec![],
            path_with_layers: vec![],
            root: from,
            interval: (vec2(2.0, 1.0), vec2(2.0, 0.0)),
            edge: (6, 2),
            polygon_from: mesh.get_point_location(from),
            polygon_to: 1,
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].f, 0.0);
        assert_eq!(successors[0].g, to.distance(from));
        assert_eq!(successors[0].polygon_from, 1);
        assert_eq!(successors[0].polygon_to, 0);
        assert_eq!(successors[0].interval, (vec2(1.0, 1.0), vec2(1.0, 0.0)));
        assert_eq!(successors[0].edge, (5, 1));
        assert_eq!(successors[0].path, Vec::<Vec2>::new());

        assert_eq!(
            mesh.path(from, to).unwrap(),
            Path {
                path: vec![to],
                length: from.distance(to),
                path_with_layers: vec![(to, 0)],
            }
        );
    }

    #[test]
    fn successors_corner_first_step() {
        let mesh = mesh_u_grid();

        let from = vec2(0.1, 1.9);
        let to = vec2(2.1, 1.9);
        let search_node = SearchNode {
            path: vec![],
            path_with_layers: vec![],
            root: from,
            interval: (vec2(0.0, 1.0), vec2(1.0, 1.0)),
            edge: (4, 5),
            polygon_from: mesh.get_point_location(from),
            polygon_to: 0,
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].root, vec2(2.0, 1.0));
        assert_eq!(
            successors[0].f,
            from.distance(vec2(1.0, 1.0)) + vec2(1.0, 1.0).distance(vec2(2.0, 1.0))
        );
        assert_eq!(successors[0].g, vec2(2.0, 1.0).distance(to));
        assert_eq!(successors[0].polygon_from, 2);
        assert_eq!(successors[0].polygon_to, 4);
        assert_eq!(successors[0].interval, (vec2(3.0, 1.0), vec2(2.0, 1.0)));
        assert_eq!(successors[0].edge, (7, 6));
        assert_eq!(successors[0].path, vec![vec2(1.0, 1.0), vec2(2.0, 1.0)]);

        assert_eq!(
            mesh.path(from, to).unwrap(),
            Path {
                path: vec![vec2(1.0, 1.0), vec2(2.0, 1.0), to],
                length: from.distance(vec2(1.0, 1.0))
                    + vec2(1.0, 1.0).distance(vec2(2.0, 1.0))
                    + vec2(2.0, 1.0).distance(to),
                path_with_layers: vec![(vec2(1.0, 1.0), 0), (vec2(2.0, 1.0), 0), (to, 0)],
            }
        );
    }

    #[test]
    fn successors_corner_observable_second_step() {
        let mesh = mesh_u_grid();

        let from = vec2(0.1, 1.9);
        let to = vec2(2.1, 1.9);
        let search_node = SearchNode {
            path: vec![],
            path_with_layers: vec![],
            root: from,
            interval: (vec2(1.0, 0.0), vec2(1.0, 1.0)),
            edge: (1, 5),
            polygon_from: 0,
            polygon_to: 1,
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].root, vec2(2.0, 1.0));
        assert_eq!(
            successors[0].f,
            from.distance(vec2(1.0, 1.0)) + vec2(1.0, 1.0).distance(vec2(2.0, 1.0))
        );
        assert_eq!(successors[0].g, vec2(2.0, 1.0).distance(to));
        assert_eq!(successors[0].polygon_from, 2);
        assert_eq!(successors[0].polygon_to, 4);
        assert_eq!(successors[0].interval, (vec2(3.0, 1.0), vec2(2.0, 1.0)));
        assert_eq!(successors[0].edge, (7, 6));
        assert_eq!(successors[0].path, vec![vec2(1.0, 1.0), vec2(2.0, 1.0)]);

        assert_eq!(
            mesh.path(from, to).unwrap(),
            Path {
                path: vec![vec2(1.0, 1.0), vec2(2.0, 1.0), to],
                length: from.distance(vec2(1.0, 1.0))
                    + vec2(1.0, 1.0).distance(vec2(2.0, 1.0))
                    + vec2(2.0, 1.0).distance(to),
                path_with_layers: vec![(vec2(1.0, 1.0), 0), (vec2(2.0, 1.0), 0), (to, 0)],
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
                Vertex::new(vec2(0., 6.), vec![0, u32::MAX]),    // 0
                Vertex::new(vec2(2., 5.), vec![0, u32::MAX, 2]), // 1
                Vertex::new(vec2(5., 7.), vec![0, 2, u32::MAX]), // 2
                Vertex::new(vec2(5., 8.), vec![0, u32::MAX]),    // 3
                Vertex::new(vec2(0., 8.), vec![0, u32::MAX]),    // 4
                Vertex::new(vec2(1., 4.), vec![1, u32::MAX]),    // 5
                Vertex::new(vec2(2., 1.), vec![1, u32::MAX]),    // 6
                Vertex::new(vec2(4., 1.), vec![1, u32::MAX]),    // 7
                Vertex::new(vec2(4., 2.), vec![1, u32::MAX, 2]), // 8
                Vertex::new(vec2(2., 4.), vec![1, 2, u32::MAX]), // 9
                Vertex::new(vec2(7., 4.), vec![2, u32::MAX, 4]), // 10
                Vertex::new(vec2(10., 7.), vec![2, 4, 6, u32::MAX, 3]), // 11
                Vertex::new(vec2(7., 7.), vec![2, 3, u32::MAX]), // 12
                Vertex::new(vec2(11., 8.), vec![3, u32::MAX]),   // 13
                Vertex::new(vec2(7., 8.), vec![3, u32::MAX]),    // 14
                Vertex::new(vec2(7., 0.), vec![5, 4, u32::MAX]), // 15
                Vertex::new(vec2(11., 3.), vec![4, 5, u32::MAX]), // 16
                Vertex::new(vec2(11., 5.), vec![4, u32::MAX, 6]), // 17
                Vertex::new(vec2(12., 0.), vec![5, u32::MAX]),   // 18
                Vertex::new(vec2(12., 3.), vec![5, u32::MAX]),   // 19
                Vertex::new(vec2(13., 5.), vec![6, u32::MAX]),   // 20
                Vertex::new(vec2(13., 7.), vec![6, u32::MAX]),   // 21
                Vertex::new(vec2(1., 3.), vec![1, u32::MAX]),    // 22
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
        assert_eq!(mesh.get_point_location(vec2(0.5, 0.5)), u32::MAX);
        assert_eq!(mesh.get_point_location(vec2(2.0, 6.0)), 0);
        assert_eq!(mesh.get_point_location(vec2(2.0, 5.1)), 0);
        assert_eq!(mesh.get_point_location(vec2(2.0, 1.5)), 1);
        assert_eq!(mesh.get_point_location(vec2(4.0, 2.1)), 2);
    }

    #[test]
    fn paper_straight() {
        let mesh = mesh_from_paper();

        let from = vec2(12.0, 0.0);
        let to = vec2(7.0, 6.9);
        let search_node = SearchNode {
            path: vec![],
            path_with_layers: vec![],
            root: from,
            interval: (vec2(11.0, 3.0), vec2(7.0, 0.0)),
            edge: (16, 15),
            polygon_from: mesh.get_point_location(from),
            polygon_to: 4,
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 2);

        assert_eq!(successors[1].root, vec2(11.0, 3.0));
        assert_eq!(successors[1].f, from.distance(vec2(11.0, 3.0)));
        assert_eq!(
            successors[1].g,
            vec2(11.0, 3.0).distance(vec2(9.75, 6.75)) + vec2(9.75, 6.75).distance(to)
        );
        assert_eq!(successors[1].polygon_from, 4);
        assert_eq!(successors[1].polygon_to, 2);
        assert_eq!(successors[1].interval, (vec2(10.0, 7.0), vec2(9.75, 6.75)));
        assert_eq!(successors[1].edge, (11, 10));
        assert_eq!(successors[1].path, vec![vec2(11.0, 3.0)]);

        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].f, 0.0);
        assert_eq!(successors[0].g, from.distance(to));
        assert_eq!(successors[0].polygon_from, 4);
        assert_eq!(successors[0].polygon_to, 2);
        assert_eq!(successors[0].interval, (vec2(9.75, 6.75), vec2(7.0, 4.0)));
        assert_eq!(successors[0].edge, (11, 10));
        assert_eq!(successors[0].path, Vec::<Vec2>::new());

        assert_eq!(mesh.path(from, to).unwrap().length, from.distance(to));
        assert_eq!(mesh.path(from, to).unwrap().path, vec![to]);
    }

    #[test]
    fn paper_corner_right() {
        let mesh = mesh_from_paper();

        let from = vec2(12.0, 0.0);
        let to = vec2(13.0, 6.0);
        let search_node = SearchNode {
            path: vec![],
            path_with_layers: vec![],
            root: from,
            interval: (vec2(11.0, 3.0), vec2(7.0, 0.0)),
            edge: (16, 15),
            polygon_from: mesh.get_point_location(from),
            polygon_to: 4,
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 3);

        assert_eq!(successors[0].root, vec2(11.0, 3.0));
        assert_eq!(successors[0].f, from.distance(vec2(11.0, 3.0)));
        assert_eq!(
            successors[0].g,
            vec2(11.0, 3.0).distance(vec2(11.0, 5.0)) + vec2(11.0, 5.0).distance(to)
        );
        assert_eq!(successors[0].polygon_from, 4);
        assert_eq!(successors[0].polygon_to, 6);
        assert_eq!(successors[0].interval, (vec2(11.0, 5.0), vec2(10.0, 7.0)));
        assert_eq!(successors[0].edge, (17, 11));
        assert_eq!(successors[0].path, vec![vec2(11.0, 3.0)]);

        assert_eq!(successors[1].root, vec2(11.0, 3.0));
        assert_eq!(successors[1].f, from.distance(vec2(11.0, 3.0)));
        assert_eq!(
            successors[1].g,
            vec2(11.0, 3.0).distance(to.mirror((vec2(10.0, 7.0), vec2(9.75, 6.75))))
        );
        assert_eq!(successors[1].polygon_from, 4);
        assert_eq!(successors[1].polygon_to, 2);
        assert_eq!(successors[1].interval, (vec2(10.0, 7.0), vec2(9.75, 6.75)));
        assert_eq!(successors[1].edge, (11, 10));
        assert_eq!(successors[1].path, vec![vec2(11.0, 3.0)]);

        assert_eq!(successors[2].root, from);
        assert_eq!(successors[2].f, 0.0);
        assert_eq!(
            successors[2].g,
            from.distance(vec2(9.75, 6.75))
                + vec2(9.75, 6.75).distance(to.mirror((vec2(9.75, 6.75), vec2(7.0, 4.0))))
        );
        assert_eq!(successors[2].polygon_from, 4);
        assert_eq!(successors[2].polygon_to, 2);
        assert_eq!(successors[2].interval, (vec2(9.75, 6.75), vec2(7.0, 4.0)));
        assert_eq!(successors[2].edge, (11, 10));
        assert_eq!(successors[2].path, Vec::<Vec2>::new());

        assert_delta!(
            mesh.path(from, to).unwrap().length,
            from.distance(vec2(11.0, 3.0))
                + vec2(11.0, 3.0).distance(vec2(11.0, 5.0))
                + vec2(11.0, 5.0).distance(to)
        );
        assert_eq!(
            mesh.path(from, to).unwrap().path,
            vec![vec2(11.0, 3.0), vec2(11.0, 5.0), to]
        );
    }

    #[test]
    fn paper_corner_left() {
        let mesh = mesh_from_paper();

        let from = vec2(12.0, 0.0);
        let to = vec2(5.0, 3.0);
        let search_node = SearchNode {
            path: vec![],
            path_with_layers: vec![],
            root: from,
            interval: (vec2(11.0, 3.0), vec2(7.0, 0.0)),
            edge: (16, 15),
            polygon_from: mesh.get_point_location(from),
            polygon_to: 4,
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 2);

        assert_eq!(successors[1].root, vec2(11.0, 3.0));
        assert_eq!(successors[1].f, from.distance(vec2(11.0, 3.0)));
        assert_eq!(
            successors[1].g,
            vec2(11.0, 3.0).distance(vec2(9.75, 6.75)) + vec2(9.75, 6.75).distance(to)
        );
        assert_eq!(successors[1].polygon_from, 4);
        assert_eq!(successors[1].polygon_to, 2);
        assert_eq!(successors[1].interval, (vec2(10.0, 7.0), vec2(9.75, 6.75)));
        assert_eq!(successors[1].edge, (11, 10));
        assert_eq!(successors[1].path, vec![vec2(11.0, 3.0)]);

        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].f, 0.0);
        assert_eq!(
            successors[0].g,
            from.distance(vec2(7.0, 4.0)) + vec2(7.0, 4.0).distance(to)
        );
        assert_eq!(successors[0].polygon_from, 4);
        assert_eq!(successors[0].polygon_to, 2);
        assert_eq!(successors[0].interval, (vec2(9.75, 6.75), vec2(7.0, 4.0)));
        assert_eq!(successors[0].edge, (11, 10));
        assert_eq!(successors[0].path, Vec::<Vec2>::new());

        assert_delta!(
            mesh.path(from, to).unwrap().length,
            from.distance(vec2(7.0, 4.0)) + vec2(7.0, 4.0).distance(to)
        );
        assert_eq!(mesh.path(from, to).unwrap().path, vec![vec2(7.0, 4.0), to]);
    }

    #[test]
    fn paper_going_to_one_way_polygon() {
        let mesh = mesh_from_paper();

        let from = vec2(11., 0.);
        let to = vec2(9., 3.);
        let path = mesh.path(from, to);

        assert_eq!(path.unwrap().path, vec![to]);

        let path = mesh.path(to, from);

        assert_eq!(path.unwrap().path, vec![from]);
    }

    #[test]
    fn paper_corner_left_twice() {
        let mesh = mesh_from_paper();

        let from = vec2(12.0, 0.0);
        let to = vec2(3.0, 1.0);
        let search_node = SearchNode {
            path: vec![],
            path_with_layers: vec![],
            root: from,
            interval: (vec2(11.0, 3.0), vec2(7.0, 0.0)),
            edge: (16, 15),
            polygon_from: mesh.get_point_location(from),
            polygon_to: 4,
            previous_polygon_layer: 0,
            f: 0.0,
            g: from.distance(to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 2);

        assert_eq!(successors[1].root, vec2(11.0, 3.0));
        assert_eq!(successors[1].f, from.distance(vec2(11.0, 3.0)));
        assert_eq!(
            successors[1].g,
            vec2(11.0, 3.0).distance(vec2(9.75, 6.75)) + vec2(9.75, 6.75).distance(to)
        );
        assert_eq!(successors[1].polygon_from, 4);
        assert_eq!(successors[1].polygon_to, 2);
        assert_eq!(successors[1].interval, (vec2(10.0, 7.0), vec2(9.75, 6.75)));
        assert_eq!(successors[1].edge, (11, 10));
        // assert_eq!(successors[1].path, vec![from]);

        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].f, 0.0);
        assert_eq!(
            successors[0].g,
            from.distance(vec2(7.0, 4.0)) + vec2(7.0, 4.0).distance(to)
        );
        assert_eq!(successors[0].polygon_from, 4);
        assert_eq!(successors[0].polygon_to, 2);
        assert_eq!(successors[0].interval, (vec2(9.75, 6.75), vec2(7.0, 4.0)));
        assert_eq!(successors[0].edge, (11, 10));
        assert_eq!(successors[0].path, Vec::<Vec2>::new());

        let successor = successors.into_iter().next().unwrap();
        let successors = dbg!(mesh.successors(successor, to));
        dbg!(&successors[0]);
        assert_eq!(successors.len(), 1);

        assert_delta!(
            mesh.path(from, to).unwrap().length,
            from.distance(vec2(7.0, 4.0))
                + vec2(7.0, 4.0).distance(vec2(4.0, 2.0))
                + vec2(4.0, 2.0).distance(to)
        );

        assert_eq!(
            mesh.path(from, to).unwrap().path,
            vec![vec2(7.0, 4.0), vec2(4.0, 2.0), to]
        );
    }

    #[test]
    fn edges_between_simple() {
        let mesh = mesh_from_paper();

        let from = vec2(12.0, 0.0);
        let to = vec2(3.0, 1.0);
        let search_node = SearchNode {
            path: vec![],
            path_with_layers: vec![],
            root: from,
            interval: (vec2(11.0, 3.0), vec2(7.0, 0.0)),
            edge: (16, 15),
            polygon_from: mesh.get_point_location(from),
            polygon_to: 4,
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
            path_with_layers: vec![],
            root: from,
            interval: (vec2(9.75, 6.75), vec2(7.0, 4.0)),
            edge: (11, 10),
            polygon_from: 4,
            polygon_to: 2,
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
            path_with_layers: vec![],
            root: vec2(11.0, 3.0),
            interval: (vec2(10.0, 7.0), vec2(7.0, 4.0)),
            edge: (11, 10),
            polygon_from: 4,
            polygon_to: 2,
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
            path_with_layers: vec![],
            root: vec2(0.0, 0.0),
            interval: (vec2(1.0, 0.0), vec2(1.0, 1.0)),
            edge: (1, 5),
            polygon_from: 0,
            polygon_to: 1,
            previous_polygon_layer: 0,
            f: 0.0,
            g: 1.0,
        };

        let successors = mesh.edges_between(&search_node);

        for successor in &successors {
            println!("{successor:?}");
        }
    }
}
