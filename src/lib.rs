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
#![cfg_attr(docsrs, feature(doc_cfg))]

const PRECISION: f32 = 1000.0;

#[cfg(feature = "stats")]
use std::{cell::Cell, time::Instant};
use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{self, Debug, Display},
};

use glam::{FloatExt, Vec2, Vec3, Vec3Swizzles};

use helpers::{line_intersect_segment, Vec2Helper, EPSILON};
use instance::{InstanceStep, U32Layer};
use smallvec::SmallVec;
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
mod mesh_cleanup;
mod primitives;
mod stitching;

#[cfg(feature = "async")]
pub use async_helpers::FuturePath;
pub use geo;
pub use input::polyanya_file::PolyanyaFile;
#[cfg(feature = "recast")]
pub use input::recast::{RecastError, RecastFullMesh, RecastPolyMesh, RecastPolyMeshDetail};
pub use input::triangulation::Triangulation;
pub use input::trimesh::Trimesh;
pub use layers::Layer;
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
    #[cfg(feature = "detailed-layers")]
    #[cfg_attr(docsrs, doc(cfg(feature = "detailed-layers")))]
    pub path_with_layers: Vec<(Vec2, u8)>,
    /// Indices of the polygons through which the path passes.
    path_through_polygons: Vec<u32>,
}

impl Path {
    /// Returns the path with height information on the Y axis.
    ///
    /// This can add points to the path when needed to follow the terrain height.
    pub fn path_with_height(&self, start: Vec3, end: Vec3, mesh: &Mesh) -> Vec<Vec3> {
        let mut heighted_path = Vec::with_capacity(self.path.len());
        let mut current = start;
        let mut next_i = 0;
        let mut next_coords: Coords = Coords::on_mesh(self.path[next_i]);
        for polygon_index in &self.path_through_polygons {
            let layer = &mesh.layers[polygon_index.layer() as usize];
            let polygon = &layer.polygons[polygon_index.polygon() as usize];
            if polygon.contains(layer, self.path[next_i]) {
                next_coords = Coords {
                    pos: self.path[next_i],
                    layer: Some(polygon_index.layer()),
                    polygon_index: *polygon_index,
                };
                break;
            }
        }
        let mut next = next_coords.position_with_height(mesh);
        for (step, polygon_index) in self
            .path_through_polygons
            .iter()
            .enumerate()
            .take(self.path_through_polygons.len() - 1)
        {
            let layer = &mesh.layers[polygon_index.layer() as usize];

            let polygon = &layer.polygons[polygon_index.polygon() as usize];
            if *polygon_index == next_coords.polygon_index {
                next_i += 1;
                heighted_path.push(next);
                current = next;
                for polygon_index in &self.path_through_polygons[step..] {
                    let layer = &mesh.layers[polygon_index.layer() as usize];
                    let polygon = &layer.polygons[polygon_index.polygon() as usize];
                    // Guard: stop once we've consumed all waypoints.
                    if next_i >= self.path.len() {
                        break;
                    }
                    if polygon.contains(layer, self.path[next_i]) {
                        next_coords = Coords {
                            pos: self.path[next_i],
                            layer: Some(polygon_index.layer()),
                            polygon_index: *polygon_index,
                        };
                        break;
                    }
                }
                next = next_coords.position_with_height(mesh);
            }
            let v0 = polygon.vertices[0] as usize;
            let a = layer.vertices[v0].coords.extend(layer.height[v0]).xzy();
            let v1 = polygon.vertices[1] as usize;
            let b = layer.vertices[v1].coords.extend(layer.height[v1]).xzy();
            let v2 = polygon.vertices[2] as usize;
            let c = layer.vertices[v2].coords.extend(layer.height[v2]).xzy();
            let polygon_normal = (b - a).cross(c - a);
            let path_direction = next - current;
            if path_direction.dot(polygon_normal).abs() > EPSILON {
                let poly_coords = polygon.coords(layer);
                let closing = [*poly_coords.last().unwrap(), *poly_coords.first().unwrap()];

                if let Some(new) = poly_coords
                    .windows(2)
                    .map(|pair| [pair[0], pair[1]])
                    .chain(std::iter::once(closing))
                    .filter_map(|[edge0, edge1]| {
                        line_intersect_segment((current.xz(), next.xz()), (edge0, edge1))
                    })
                    .filter(|p| p.in_bounding_box((current.xz(), next.xz())))
                    .max_by_key(|p| (current.xz().distance_squared(*p) / EPSILON) as u32)
                {
                    if new.distance_squared(current.xz()) > EPSILON {
                        let new = Coords {
                            pos: new,
                            layer: Some(polygon_index.layer()),
                            polygon_index: *polygon_index,
                        }
                        .position_with_height(mesh);
                        heighted_path.push(new);
                        current = new;
                    }
                }
            }
        }
        heighted_path.push(end);
        heighted_path
    }

    /// Returns the polygons that the path goes through.
    pub fn polygons(&self) -> Vec<(u8, u32)> {
        self.path_through_polygons
            .iter()
            .map(|poly_index| (poly_index.layer(), poly_index.polygon()))
            .collect()
    }
}

/// A navigation mesh
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mesh {
    /// Layers of the NavMesh
    pub layers: Vec<Layer>,
    /// Precision used when searching for a point in a mesh
    pub search_delta: f32,
    /// Number of steps before stopping searching for a point in a mesh
    pub search_steps: u32,
    #[cfg(feature = "stats")]
    pub(crate) scenarios: Cell<u32>,
}

/// A point in the navigation mesh
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coords {
    /// The position
    pos: Vec2,
    /// The layer
    ///
    /// If specified, the point will be searched in that layer only.
    layer: Option<u8>,
    /// internal: this coords have been built by a search on the mesh that found the polygon index
    /// if used for a path, this will be used directly instead of searching for it again in the mesh
    /// default value is u32::MAX which means it hasn't been searched
    polygon_index: u32,
}

impl From<Vec2> for Coords {
    fn from(value: Vec2) -> Self {
        Coords {
            pos: value,
            layer: None,
            polygon_index: u32::MAX,
        }
    }
}

impl Coords {
    /// A point on the navigation mesh
    pub fn on_mesh(pos: Vec2) -> Self {
        pos.into()
    }

    /// A point on the navigation mesh on the specified layer
    pub fn on_layer(pos: Vec2, layer: u8) -> Self {
        Coords {
            pos,
            layer: Some(layer),
            polygon_index: u32::MAX,
        }
    }

    /// Position of this point
    #[inline]
    pub fn position(&self) -> Vec2 {
        self.pos
    }

    /// Layer of this point, if known
    #[inline]
    pub fn layer(&self) -> Option<u8> {
        self.layer
    }

    /// Polygon index of this point
    #[inline]
    pub fn polygon(&self) -> u32 {
        self.polygon_index
    }

    /// Height of this point
    pub fn height(&self, mesh: &Mesh) -> f32 {
        if self.polygon_index == u32::MAX {
            return 0.0;
        }
        let layer = &mesh.layers[self.layer().unwrap_or(0) as usize];
        let poly = &layer.polygons[self.polygon_index.polygon() as usize];

        if let Some([segment0, segment1]) = poly.edges_index().find(|[edge0, edge1]| {
            self.pos.on_segment((
                layer.vertices[*edge0 as usize].coords,
                layer.vertices[*edge1 as usize].coords,
            ))
        }) {
            let (a, b) = (
                layer.vertices[segment0 as usize].coords,
                layer.vertices[segment1 as usize].coords,
            );
            let t = (self.pos - a).dot(b - a) / (b - a).dot(b - a);
            return layer.height[segment0 as usize].lerp(layer.height[segment1 as usize], t);
        }

        // TODO: should find the position of the point within the polygon and weight each polygonpoint height based on its distance to the point
        poly.vertices
            .iter()
            .map(|i| *layer.height.get(*i as usize).unwrap_or(&0.0))
            .sum::<f32>()
            / poly.vertices.len() as f32
    }

    /// Position of the point within the mesh, including its height on the Y axis.
    pub fn position_with_height(&self, mesh: &Mesh) -> Vec3 {
        Vec3::new(self.pos.x, self.height(mesh), self.pos.y)
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
            search_delta: 0.1,
            search_steps: 2,
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

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct BoundedPolygon {
    index: usize,
    aabb_min: [f32; 2],
    aabb_max: [f32; 2],
}

impl rstar::RTreeObject for BoundedPolygon {
    type Envelope = rstar::AABB<[f32; 2]>;

    // Called many times per element during `RTree::bulk_load`; leaving it out of line makes
    // baking the polygon finder about 8x slower.
    #[inline(always)]
    fn envelope(&self) -> Self::Envelope {
        rstar::AABB::from_corners(self.aabb_min, self.aabb_max)
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
    pub fn get_path(&self, from: Vec2, to: Vec2) -> FuturePath<'_> {
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
        self.path_on_layers(from, to, HashSet::default())
    }

    /// Compute a path between two points.
    ///
    /// This will be a [`Path`] if a path is found, or `None` if not.
    ///
    /// A point given without a polygon or a layer can sit over more than one polygon, on a
    /// mesh whose layers or polygons overlap: a spot under a balcony is on the ground floor
    /// and on the balcony both. Every polygon it resolves to is a valid reading of the
    /// query, so all of them are searched together and the shortest path is returned. Give
    /// a [`Coords`] with a layer, or use [`Self::path_with_height`], to pick one instead.
    ///
    /// This method is blocking, to get the path in an async way use [`Self::get_path`].
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    pub fn path_on_layers(
        &self,
        from: impl Into<Coords>,
        to: impl Into<Coords>,
        blocked_layers: HashSet<u8>,
    ) -> Option<Path> {
        let from = from.into();
        let to = to.into();

        let starting_polygons = self.candidate_polygons(from, &blocked_layers);
        if starting_polygons.is_empty() {
            return None;
        }
        let ending_polygons = self.candidate_polygons(to, &blocked_layers);
        if ending_polygons.is_empty() {
            return None;
        }

        self.path_between_polygons(
            (from.pos, &starting_polygons),
            (to.pos, &ending_polygons),
            blocked_layers,
        )
    }

    /// Every polygon a query point resolves to.
    ///
    /// A point that already names its polygon has exactly one; one that names a layer is
    /// looked for in that layer only. Otherwise it can land on several overlapping
    /// polygons, and which of them a search picks must not depend on how the mesh is cut
    /// into layers, so they are all returned.
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    pub(crate) fn candidate_polygons(
        &self,
        point: Coords,
        blocked_layers: &HashSet<u8>,
    ) -> SmallVec<[u32; 1]> {
        if point.polygon_index != u32::MAX {
            return smallvec::smallvec![point.polygon_index];
        }
        let mut found = SmallVec::new();
        for step in 0..self.search_steps {
            // Every layer is searched at this step before moving on to the next one:
            // stopping at the first layer with a hit would make the answer depend on how
            // the mesh happens to be partitioned into layers.
            for (layer_index, layer) in self.layers.iter().enumerate() {
                let layer_index = layer_index as u8;
                if point.layer.is_some_and(|only| only != layer_index)
                    || blocked_layers.contains(&layer_index)
                {
                    continue;
                }
                layer.push_point_locations(
                    point.pos - layer.offset,
                    self.search_delta,
                    step,
                    layer_index,
                    &mut found,
                );
            }
            if !found.is_empty() {
                break;
            }
        }
        if found.len() > 1 {
            self.merge_touching_readings(&mut found);
        }
        found
    }

    /// Drop the readings that are not really different.
    ///
    /// A point that lands on an edge is in the polygons on both sides of it, and one on a
    /// vertex is in every polygon around it. Those are neighbours, so a search that starts
    /// in any of them reaches the rest over a zero-length crossing, and seeding from each
    /// would walk the whole mesh once per reading. What has to be kept is a reading the
    /// search cannot get to from another one -- a floor above, a disconnected region.
    fn merge_touching_readings(&self, found: &mut SmallVec<[u32; 1]>) {
        let mut kept: SmallVec<[u32; 1]> = SmallVec::new();
        let mut group: SmallVec<[u32; 1]> = SmallVec::new();
        while !found.is_empty() {
            let first = found.remove(0);
            kept.push(first);
            group.clear();
            group.push(first);
            // Everything the kept reading touches, and everything those touch in turn.
            while let Some(current) = group.pop() {
                let mut other = 0;
                while other < found.len() {
                    if self.share_an_edge(current, found[other]) {
                        group.push(found.remove(other));
                    } else {
                        other += 1;
                    }
                }
            }
        }
        *found = kept;
    }

    /// Is there an edge of `polygon` that `other` is on the other side of?
    fn share_an_edge(&self, polygon: u32, other: u32) -> bool {
        let layer = &self.layers[polygon.layer() as usize];
        layer.polygons[polygon.polygon() as usize]
            .edges_index()
            .any(|[edge0, edge1]| {
                let (Some(start), Some(end)) = (
                    layer.vertices.get(edge0 as usize),
                    layer.vertices.get(edge1 as usize),
                ) else {
                    return false;
                };
                start.polygons.contains(&other) && end.polygons.contains(&other)
            })
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    fn path_between_polygons(
        &self,
        from: (Vec2, &[u32]),
        to: (Vec2, &[u32]),
        blocked_layers: HashSet<u8>,
    ) -> Option<Path> {
        #[cfg(feature = "stats")]
        let start = Instant::now();

        let (from_point, starting_polygons) = from;
        let (to_point, ending_polygons) = to;

        // TODO: fix islands detection with multiple layers, even if start and end are on the same layer
        if self.layers.len() == 1 {
            if let Some(islands) = self.layers[0].islands.as_ref() {
                let connected = starting_polygons.iter().any(|starting_polygon| {
                    ending_polygons.iter().any(|ending_polygon| {
                        let start_island = islands.get(starting_polygon.polygon() as usize);
                        let end_island = islands.get(ending_polygon.polygon() as usize);
                        start_island.is_none() || end_island.is_none() || start_island == end_island
                    })
                });
                if !connected {
                    return None;
                }
            }
        }

        // A point that starts in a goal polygon is already there, and nothing beats a
        // straight line, so there is no need to look at the other candidates.
        if let Some(ending_polygon) = starting_polygons
            .iter()
            .find(|starting_polygon| ending_polygons.contains(starting_polygon))
        {
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
                    from_point.distance(to_point),
                );
                self.scenarios.set(self.scenarios.get() + 1);
            }
            return Some(Path {
                length: from_point.distance(to_point),
                path: vec![to_point],
                #[cfg(feature = "detailed-layers")]
                path_with_layers: vec![(to_point, ending_polygon.layer())],
                path_through_polygons: vec![*ending_polygon],
            });
        }

        let mut search_instance = SearchInstance::setup(
            self,
            (from_point, starting_polygons),
            (to_point, ending_polygons),
            blocked_layers,
            #[cfg(feature = "stats")]
            start,
        );

        // Limit search to avoid an infinite loop.
        let iterations = self.layers.iter().map(|l| l.polygons.len()).sum::<usize>() * 10;

        // A path reaches the goal while its `heuristic` still covers the last leg, so the
        // `f` it was popped with is a lower bound on its cost rather than the cost itself.
        // With every layer at the same scale the two are equal and the first path found is
        // returned immediately, exactly as the default build does. When they differ, a
        // cheaper path can still be queued behind it -- but only one whose `f` is below
        // what has been found, and `f` never overestimates, so the search stops as soon as
        // the queue cannot beat it. That is a handful of extra pops, against the full
        // enumeration of the mesh this used to do.
        #[cfg(feature = "detailed-layers")]
        {
            let mut best: Option<Path> = None;
            for _ in 0..iterations {
                match search_instance.next() {
                    InstanceStep::Found(path) => {
                        if best.as_ref().is_none_or(|best| path.length < best.length) {
                            best = Some(path);
                        }
                    }
                    // The queue has run dry and nothing can refill it, so every further
                    // step is a pop from an empty heap.
                    InstanceStep::NotFound => break,
                    InstanceStep::Continue => (),
                }
                if let Some(best) = &best {
                    match search_instance.queued_lower_bound() {
                        Some(bound) if bound < best.length => (),
                        _ => break,
                    }
                }
            }
            best
        }

        #[cfg(not(feature = "detailed-layers"))]
        {
            for _ in 0..iterations {
                match search_instance.next() {
                    // Nothing left in the queue can beat the node that just came off it,
                    // so the first path to the goal is the shortest one.
                    InstanceStep::Found(path) => return Some(path),
                    InstanceStep::NotFound => break,
                    InstanceStep::Continue => (),
                }
            }
            None
        }
    }

    /// The cheapest a unit of travel can be anywhere on this mesh: the smallest component
    /// of any layer's [`Layer::scale`].
    ///
    /// Measuring the heuristic at this rate is what keeps it from overestimating, whatever
    /// layers the path ends up crossing. Recomputed per query rather than baked, because
    /// `scale` is a public field callers are free to change after the mesh is built.
    #[cfg(feature = "detailed-layers")]
    #[cfg_attr(docsrs, doc(cfg(feature = "detailed-layers")))]
    pub(crate) fn min_scale(&self) -> f32 {
        self.layers
            .iter()
            .map(|layer| layer.scale.x.min(layer.scale.y))
            .fold(f32::INFINITY, f32::min)
    }

    /// The delta set by [`Mesh::set_delta`]
    pub fn search_delta(&self) -> f32 {
        self.search_delta
    }

    /// Set the delta for search with [`Mesh::path`], [`Mesh::get_path`], and [`Mesh::point_in_mesh`].
    /// A given point P(x, y) will be searched in concentric circles around P of radius `delta` * ([`Mesh::search_steps`] - 1).
    ///
    /// Default is 0.1
    pub fn set_search_delta(&mut self, delta: f32) -> &mut Self {
        assert!(delta >= 0.0);
        self.search_delta = delta;
        self
    }

    /// The steps set by [`Mesh::set_steps`]
    pub fn search_steps(&self) -> u32 {
        self.search_steps
    }

    /// Set the steps for search with [`Mesh::path`], [`Mesh::get_path`], and [`Mesh::point_in_mesh`].
    /// A given point P(x, y) will be searched in concentric circles around P of radius [`Mesh::search_delta`] * (`steps` - 1).
    ///
    /// Default is 2
    pub fn set_search_steps(&mut self, steps: u32) -> &mut Self {
        assert!(steps != 0);
        self.search_steps = steps;
        self
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[cfg(test)]
    fn successors(&self, node: SearchNode, to: Vec2) -> (Vec<SearchNode>, Vec<PathArenaNode>) {
        use hashbrown::HashMap;
        use std::collections::BinaryHeap;

        let mut search_instance = SearchInstance {
            #[cfg(feature = "stats")]
            start: Instant::now(),
            queue: BinaryHeap::new(),
            node_buffer: Vec::new(),
            root_history: HashMap::new(),
            seen_nodes: hashbrown::HashSet::new(),
            last_f: 0.0,
            stalled_pops: 0,
            stall_limit: u32::MAX,
            recording: false,
            path_arena: Vec::new(),
            from: (node.root, 0),
            to,
            polygon_to: self.get_point_location(to),
            other_polygons_to: Vec::new(),
            mesh: self,
            blocked_layers: HashSet::default(),
            #[cfg(feature = "detailed-layers")]
            min_scale: self.min_scale(),
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
        let nodes: Vec<SearchNode> = search_instance.queue.drain().collect();
        (nodes, search_instance.path_arena)
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
            seen_nodes: hashbrown::HashSet::new(),
            last_f: 0.0,
            stalled_pops: 0,
            stall_limit: u32::MAX,
            recording: false,
            path_arena: Vec::new(),
            from: (Vec2::ZERO, 0),
            to: Vec2::ZERO,
            polygon_to: self.get_point_location(vec2(0.0, 0.0)),
            other_polygons_to: Vec::new(),
            mesh: self,
            blocked_layers: HashSet::default(),
            #[cfg(feature = "detailed-layers")]
            min_scale: self.min_scale(),
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

    /// Get the positions of a point, including its layer.
    ///
    /// If the point can be in multiple layers, in case of overlapping layers, returns all possible layers.
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub fn get_point_layer(&self, point: impl Into<Coords>) -> Vec<Coords> {
        let coords = point.into();
        self.get_point_locations(coords)
            .iter()
            .map(|p| Coords {
                pos: coords.pos,
                layer: Some(p.layer()),
                polygon_index: *p,
            })
            .collect()
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
                        layer.get_point_location(point.pos - layer.offset, self.search_delta)?,
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
                        layer.get_point_location(point.pos - layer.offset, self.search_delta)?,
                    ))
                })
                .find(|poly| poly != &u32::MAX)
                .unwrap_or(u32::MAX)
        }
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    fn get_point_locations(&self, point: impl Into<Coords>) -> Vec<u32> {
        let point = point.into();
        if let Some(layer_index) = point.layer {
            self.layers
                .get(layer_index as usize)
                .and_then(|layer| {
                    Some(U32Layer::from_layer_and_polygon(
                        layer_index,
                        layer.get_point_location(point.pos - layer.offset, self.search_delta)?,
                    ))
                })
                .into_iter()
                .collect()
        } else {
            self.layers
                .iter()
                .enumerate()
                .flat_map(|(index, layer)| {
                    Some(U32Layer::from_layer_and_polygon(
                        index as u8,
                        layer.get_point_location(point.pos - layer.offset, self.search_delta)?,
                    ))
                })
                .filter(|poly| poly != &u32::MAX)
                .collect()
        }
    }

    /// Find the closest point in the mesh
    ///
    /// This will search in circles up to `Mesh::delta` * `Mesh::steps` distance away from the point
    pub fn get_closest_point(&self, point: impl Into<Coords>) -> Option<Coords> {
        self.get_closest_point_on_layers(point, HashSet::default())
    }

    /// Find the closest point in the mesh
    ///
    /// This will search in circles up to `Mesh::delta` * `Mesh::steps` distance away from the point
    pub fn get_closest_point_on_layers(
        &self,
        point: impl Into<Coords>,
        blocked_layers: HashSet<u8>,
    ) -> Option<Coords> {
        let point = point.into();
        if let Some(layer_index) = point.layer {
            let layer = &self.layers[layer_index as usize];
            for step in 0..self.search_steps {
                if let Some((new_point, polygon)) =
                    layer.get_closest_point_inner(point.pos - layer.offset, self.search_delta, step)
                {
                    return Some(Coords {
                        pos: new_point + layer.offset,
                        layer: Some(layer_index),
                        polygon_index: U32Layer::from_layer_and_polygon(layer_index, polygon),
                    });
                }
            }
        } else {
            for step in 0..self.search_steps {
                for (index, layer) in self
                    .layers
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| !blocked_layers.contains(&(*index as u8)))
                {
                    if let Some((new_point, polygon)) = layer.get_closest_point_inner(
                        point.pos - layer.offset,
                        self.search_delta,
                        step,
                    ) {
                        return Some(Coords {
                            pos: new_point + layer.offset,
                            layer: Some(index as u8),
                            polygon_index: U32Layer::from_layer_and_polygon(index as u8, polygon),
                        });
                    }
                }
            }
        }
        None
    }

    /// Find the closest points in the mesh
    ///
    /// If there are several points at the same distance, all of them will be returned.
    /// This can happen when a layer have overlapping polygons.
    ///
    /// This will search in circles up to `Mesh::delta` * `Mesh::steps` distance away from the point
    pub fn get_closest_points(&self, point: impl Into<Coords>) -> Vec<Coords> {
        self.get_closest_points_on_layers(point, HashSet::default())
    }

    /// Find the closest point in the mesh, discriminating by height if there are several polygon overlapping.
    ///
    /// This will search in circles up to `Mesh::delta` * `Mesh::steps` distance away from the point
    pub fn get_closest_point_at_height(
        &self,
        point: impl Into<Coords>,
        height: f32,
    ) -> Option<Coords> {
        self.get_closest_points_on_layers_at_height(point, HashSet::default(), height)
    }

    /// Find the closest point in the mesh, discriminating by height if there are several polygon overlapping.
    ///
    /// If there are several points at the same distance, all of them will be returned.
    /// This can happen when a layer have overlapping polygons.
    ///
    /// This will search in circles up to `Mesh::delta` * `Mesh::steps` distance away from the point
    pub fn get_closest_points_on_layers_at_height(
        &self,
        point: impl Into<Coords>,
        blocked_layers: HashSet<u8>,
        height: f32,
    ) -> Option<Coords> {
        self.get_closest_points_on_layers(point, blocked_layers)
            .iter()
            .fold(None, |acc: Option<(Coords, f32)>, &coord| {
                let coord_height = coord.height(self);
                if acc
                    .map(|(_, closest_height)| (closest_height - height).abs())
                    .unwrap_or(f32::MAX)
                    > (coord_height - height).abs()
                {
                    Some((coord, coord_height))
                } else {
                    acc
                }
            })
            .map(|acc| acc.0)
    }

    /// Find the closest point in the mesh
    ///
    /// If there are several points at the same distance, all of them will be returned.
    /// This can happen when a layer have overlapping polygons.
    ///
    /// This will search in circles up to `Mesh::delta` * `Mesh::steps` distance away from the point
    pub fn get_closest_points_on_layers(
        &self,
        point: impl Into<Coords>,
        blocked_layers: HashSet<u8>,
    ) -> Vec<Coords> {
        let point = point.into();
        if let Some(layer_index) = point.layer {
            let layer = &self.layers[layer_index as usize];
            for step in 0..self.search_steps {
                let coords: Vec<Coords> = layer
                    .get_closest_points_inner(point.pos - layer.offset, self.search_delta, step)
                    .iter()
                    .map(|(new_point, polygon)| Coords {
                        pos: new_point + layer.offset,
                        layer: Some(layer_index),
                        polygon_index: U32Layer::from_layer_and_polygon(layer_index, *polygon),
                    })
                    .collect();
                if !coords.is_empty() {
                    return coords;
                }
            }
        } else {
            for step in 0..self.search_steps {
                // Every layer is searched at this step before moving on to the next one:
                // stopping at the first layer with a hit would make the answer depend on
                // how the mesh happens to be partitioned into layers.
                let coords: Vec<Coords> = self
                    .layers
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| !blocked_layers.contains(&(*index as u8)))
                    .flat_map(|(layer_index, layer)| {
                        layer
                            .get_closest_points_inner(
                                point.pos - layer.offset,
                                self.search_delta,
                                step,
                            )
                            .into_iter()
                            .map(move |(new_point, polygon)| Coords {
                                pos: new_point + layer.offset,
                                layer: Some(layer_index as u8),
                                polygon_index: U32Layer::from_layer_and_polygon(
                                    layer_index as u8,
                                    polygon,
                                ),
                            })
                    })
                    .collect();
                if !coords.is_empty() {
                    return coords;
                }
            }
        }
        vec![]
    }

    /// Find the closest point in the mesh in the given direction
    ///
    /// This will search in a line up to `Mesh::delta` * `Mesh::steps` distance away from the point
    pub fn get_closest_point_towards(
        &self,
        point: impl Into<Coords>,
        towards: Vec2,
    ) -> Option<Coords> {
        let point = point.into();
        let direction = -(point.pos - towards).normalize();
        if let Some(layer_index) = point.layer {
            let layer = &self.layers[layer_index as usize];
            for step in 0..self.search_steps {
                if let Some((new_point, polygon)) = layer.get_closest_point_towards_inner(
                    point.pos - layer.offset,
                    self.search_delta,
                    direction,
                    step,
                ) {
                    return Some(Coords {
                        pos: new_point + layer.offset,
                        layer: Some(layer_index),
                        polygon_index: U32Layer::from_layer_and_polygon(layer_index, polygon),
                    });
                }
            }
        } else {
            for step in 0..self.search_steps {
                for (index, layer) in self.layers.iter().enumerate() {
                    if let Some((new_point, polygon)) = layer.get_closest_point_towards_inner(
                        point.pos - layer.offset,
                        self.search_delta,
                        direction,
                        step,
                    ) {
                        return Some(Coords {
                            pos: new_point + layer.offset,
                            layer: Some(index as u8),
                            polygon_index: U32Layer::from_layer_and_polygon(index as u8, polygon),
                        });
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub(crate) struct PathArenaNode {
    root: Vec2,
    polygon: u32,
    parent: u32, // u32::MAX = no parent
    root_changed: bool,
    /// The edge this node was reached over, used to place the point at which the path
    /// crosses into `polygon`'s layer. Both layers are read back off `polygon` and the
    /// previous entry's, so neither is stored.
    #[cfg(feature = "detailed-layers")]
    interval: (Vec2, Vec2),
}

#[derive(PartialEq, Debug)]
struct SearchNode {
    arena_parent: u32, // index into path_arena, u32::MAX = no parent
    root: Vec2,
    interval: (Vec2, Vec2),
    edge: (u32, u32),
    polygon_from: u32,
    polygon_to: u32,
    previous_polygon_layer: u8,
    distance_start_to_root: f32,
    heuristic: f32,
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
        f.write_str(&format!(
            "f={:.2}, g={:.2} ",
            self.distance_start_to_root + self.heuristic,
            self.distance_start_to_root
        ))?;
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
        match (self.distance_start_to_root + self.heuristic)
            .total_cmp(&(other.distance_start_to_root + other.heuristic))
        {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => self
                .distance_start_to_root
                .total_cmp(&other.distance_start_to_root),
            Ordering::Greater => Ordering::Less,
        }
    }
}

/// Reconstruct the turning-point path from an arena chain (test helper).
#[cfg(test)]
pub(crate) fn reconstruct_test_path(arena: &[PathArenaNode], arena_parent: u32) -> Vec<Vec2> {
    let mut chain = Vec::new();
    let mut idx = arena_parent;
    while idx != u32::MAX {
        chain.push(idx);
        idx = arena[idx as usize].parent;
    }
    chain.reverse();

    let mut turning_points = Vec::new();
    for &arena_idx in &chain {
        let entry = &arena[arena_idx as usize];
        if entry.root_changed {
            turning_points.push(entry.root);
        }
    }
    turning_points
}

#[cfg(test)]
mod tests {
    macro_rules! assert_delta {
        ($x:expr, $y:expr) => {
            let val = $x;
            let expected = $y;
            if (val - expected).abs() >= 0.01 {
                assert_eq!(val, expected);
            }
        };
    }

    use std::vec;

    use glam::{vec2, Vec2};

    use crate::{helpers::*, Layer, Mesh, Path, PathArenaNode, Polygon, SearchNode, Vertex};

    fn reconstruct_test_path(arena: &[PathArenaNode], arena_parent: u32) -> Vec<Vec2> {
        crate::reconstruct_test_path(arena, arena_parent)
    }

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
            arena_parent: u32::MAX,
            root: from,
            interval: (vec2(1.0, 0.0), vec2(1.0, 1.0)),
            edge: (1, 5),
            polygon_from: mesh.get_point_location(from),
            polygon_to: 1,
            previous_polygon_layer: 0,
            distance_start_to_root: from.distance(to),
            heuristic: 0.0,
        };
        let (successors, arena) = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].distance_start_to_root, from.distance(to));
        assert_eq!(successors[0].heuristic, from.distance(to));
        assert_eq!(successors[0].polygon_from, 1);
        assert_eq!(successors[0].polygon_to, 2);
        assert_eq!(successors[0].interval, (vec2(2.0, 0.0), vec2(2.0, 1.0)));
        assert_eq!(successors[0].edge, (2, 6));

        assert_eq!(
            reconstruct_test_path(&arena, successors[0].arena_parent),
            Vec::<Vec2>::new()
        );

        assert_eq!(
            mesh.path(from, to).unwrap(),
            Path {
                path: vec![to],
                length: from.distance(to),
                #[cfg(feature = "detailed-layers")]
                path_with_layers: vec![(to, 0)],
                path_through_polygons: vec![0, 1, 2],
            }
        );
    }

    #[test]
    fn successors_straight_line_reversed() {
        let mesh = mesh_u_grid();

        let to = vec2(0.1, 0.1);
        let from = vec2(2.9, 0.9);
        let search_node = SearchNode {
            arena_parent: u32::MAX,
            root: from,
            interval: (vec2(2.0, 1.0), vec2(2.0, 0.0)),
            edge: (6, 2),
            polygon_from: mesh.get_point_location(from),
            polygon_to: 1,
            previous_polygon_layer: 0,
            distance_start_to_root: 0.0,
            heuristic: from.distance(to),
        };
        let (successors, arena) = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].distance_start_to_root, 0.0);
        assert_eq!(successors[0].heuristic, to.distance(from));
        assert_eq!(successors[0].polygon_from, 1);
        assert_eq!(successors[0].polygon_to, 0);
        assert_eq!(successors[0].interval, (vec2(1.0, 1.0), vec2(1.0, 0.0)));
        assert_eq!(successors[0].edge, (5, 1));
        assert_eq!(
            reconstruct_test_path(&arena, successors[0].arena_parent),
            Vec::<Vec2>::new()
        );

        assert_eq!(
            mesh.path(from, to).unwrap(),
            Path {
                path: vec![to],
                length: from.distance(to),
                #[cfg(feature = "detailed-layers")]
                path_with_layers: vec![(to, 0)],
                path_through_polygons: vec![2, 1, 0],
            }
        );
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
        assert_eq!(successors[0].polygon_from, 2);
        assert_eq!(successors[0].polygon_to, 4);
        assert_eq!(successors[0].interval, (vec2(3.0, 1.0), vec2(2.0, 1.0)));
        assert_eq!(successors[0].edge, (7, 6));
        assert_eq!(
            reconstruct_test_path(&arena, successors[0].arena_parent),
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
                path_with_layers: vec![(vec2(1.0, 1.0), 0), (vec2(2.0, 1.0), 0), (to, 0)],
                path_through_polygons: vec![3, 0, 1, 2, 4],
            }
        );
    }

    #[test]
    fn successors_corner_observable_second_step() {
        let mesh = mesh_u_grid();

        let from = vec2(0.1, 1.9);
        let to = vec2(2.1, 1.9);
        let search_node = SearchNode {
            arena_parent: u32::MAX,
            root: from,
            interval: (vec2(1.0, 0.0), vec2(1.0, 1.0)),
            edge: (1, 5),
            polygon_from: 0,
            polygon_to: 1,
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
        assert_eq!(successors[0].polygon_from, 2);
        assert_eq!(successors[0].polygon_to, 4);
        assert_eq!(successors[0].interval, (vec2(3.0, 1.0), vec2(2.0, 1.0)));
        assert_eq!(successors[0].edge, (7, 6));
        assert_eq!(
            reconstruct_test_path(&arena, successors[0].arena_parent),
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
                path_with_layers: vec![(vec2(1.0, 1.0), 0), (vec2(2.0, 1.0), 0), (to, 0)],
                path_through_polygons: vec![3, 0, 1, 2, 4],
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
            arena_parent: u32::MAX,
            root: from,
            interval: (vec2(11.0, 3.0), vec2(7.0, 0.0)),
            edge: (16, 15),
            polygon_from: mesh.get_point_location(from),
            polygon_to: 4,
            previous_polygon_layer: 0,
            distance_start_to_root: 0.0,
            heuristic: from.distance(to),
        };
        let (successors, arena) = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 2);

        assert_eq!(successors[1].root, vec2(11.0, 3.0));
        assert_eq!(
            successors[1].distance_start_to_root,
            from.distance(vec2(11.0, 3.0))
        );
        assert_eq!(
            successors[1].heuristic,
            vec2(11.0, 3.0).distance(vec2(9.75, 6.75)) + vec2(9.75, 6.75).distance(to)
        );
        assert_eq!(successors[1].polygon_from, 4);
        assert_eq!(successors[1].polygon_to, 2);
        assert_eq!(successors[1].interval, (vec2(10.0, 7.0), vec2(9.75, 6.75)));
        assert_eq!(successors[1].edge, (11, 10));
        assert_eq!(
            reconstruct_test_path(&arena, successors[1].arena_parent),
            vec![vec2(11.0, 3.0)]
        );

        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].distance_start_to_root, 0.0);
        assert_eq!(successors[0].heuristic, from.distance(to));
        assert_eq!(successors[0].polygon_from, 4);
        assert_eq!(successors[0].polygon_to, 2);
        assert_eq!(successors[0].interval, (vec2(9.75, 6.75), vec2(7.0, 4.0)));
        assert_eq!(successors[0].edge, (11, 10));
        assert_eq!(
            reconstruct_test_path(&arena, successors[0].arena_parent),
            Vec::<Vec2>::new()
        );

        assert_eq!(mesh.path(from, to).unwrap().length, from.distance(to));
        assert_eq!(mesh.path(from, to).unwrap().path, vec![to]);
    }

    #[test]
    fn paper_corner_right() {
        let mesh = mesh_from_paper();

        let from = vec2(12.0, 0.0);
        let to = vec2(13.0, 6.0);
        let search_node = SearchNode {
            arena_parent: u32::MAX,
            root: from,
            interval: (vec2(11.0, 3.0), vec2(7.0, 0.0)),
            edge: (16, 15),
            polygon_from: mesh.get_point_location(from),
            polygon_to: 4,
            previous_polygon_layer: 0,
            distance_start_to_root: 0.0,
            heuristic: from.distance(to),
        };
        let (successors, arena) = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 3);

        assert_eq!(successors[0].root, vec2(11.0, 3.0));
        assert_eq!(
            successors[0].distance_start_to_root,
            from.distance(vec2(11.0, 3.0))
        );
        assert_eq!(
            successors[0].heuristic,
            vec2(11.0, 3.0).distance(vec2(11.0, 5.0)) + vec2(11.0, 5.0).distance(to)
        );
        assert_eq!(successors[0].polygon_from, 4);
        assert_eq!(successors[0].polygon_to, 6);
        assert_eq!(successors[0].interval, (vec2(11.0, 5.0), vec2(10.0, 7.0)));
        assert_eq!(successors[0].edge, (17, 11));
        assert_eq!(
            reconstruct_test_path(&arena, successors[0].arena_parent),
            vec![vec2(11.0, 3.0)]
        );

        assert_eq!(successors[1].root, vec2(11.0, 3.0));
        assert_eq!(
            successors[1].distance_start_to_root,
            from.distance(vec2(11.0, 3.0))
        );
        assert_eq!(
            successors[1].heuristic,
            vec2(11.0, 3.0).distance(to.mirror((vec2(10.0, 7.0), vec2(9.75, 6.75))))
        );
        assert_eq!(successors[1].polygon_from, 4);
        assert_eq!(successors[1].polygon_to, 2);
        assert_eq!(successors[1].interval, (vec2(10.0, 7.0), vec2(9.75, 6.75)));
        assert_eq!(successors[1].edge, (11, 10));
        assert_eq!(
            reconstruct_test_path(&arena, successors[1].arena_parent),
            vec![vec2(11.0, 3.0)]
        );

        assert_eq!(successors[2].root, from);
        assert_eq!(successors[2].distance_start_to_root, 0.0);
        assert_eq!(
            successors[2].heuristic,
            from.distance(vec2(9.75, 6.75))
                + vec2(9.75, 6.75).distance(to.mirror((vec2(9.75, 6.75), vec2(7.0, 4.0))))
        );
        assert_eq!(successors[2].polygon_from, 4);
        assert_eq!(successors[2].polygon_to, 2);
        assert_eq!(successors[2].interval, (vec2(9.75, 6.75), vec2(7.0, 4.0)));
        assert_eq!(successors[2].edge, (11, 10));
        assert_eq!(
            reconstruct_test_path(&arena, successors[2].arena_parent),
            Vec::<Vec2>::new()
        );

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
            arena_parent: u32::MAX,
            root: from,
            interval: (vec2(11.0, 3.0), vec2(7.0, 0.0)),
            edge: (16, 15),
            polygon_from: mesh.get_point_location(from),
            polygon_to: 4,
            previous_polygon_layer: 0,
            distance_start_to_root: 0.0,
            heuristic: from.distance(to),
        };
        let (successors, arena) = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 2);

        assert_eq!(successors[1].root, vec2(11.0, 3.0));
        assert_eq!(
            successors[1].distance_start_to_root,
            from.distance(vec2(11.0, 3.0))
        );
        assert_eq!(
            successors[1].heuristic,
            vec2(11.0, 3.0).distance(vec2(9.75, 6.75)) + vec2(9.75, 6.75).distance(to)
        );
        assert_eq!(successors[1].polygon_from, 4);
        assert_eq!(successors[1].polygon_to, 2);
        assert_eq!(successors[1].interval, (vec2(10.0, 7.0), vec2(9.75, 6.75)));
        assert_eq!(successors[1].edge, (11, 10));
        assert_eq!(
            reconstruct_test_path(&arena, successors[1].arena_parent),
            vec![vec2(11.0, 3.0)]
        );

        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].distance_start_to_root, 0.0);
        assert_eq!(
            successors[0].heuristic,
            from.distance(vec2(7.0, 4.0)) + vec2(7.0, 4.0).distance(to)
        );
        assert_eq!(successors[0].polygon_from, 4);
        assert_eq!(successors[0].polygon_to, 2);
        assert_eq!(successors[0].interval, (vec2(9.75, 6.75), vec2(7.0, 4.0)));
        assert_eq!(successors[0].edge, (11, 10));
        assert_eq!(
            reconstruct_test_path(&arena, successors[0].arena_parent),
            Vec::<Vec2>::new()
        );

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
            arena_parent: u32::MAX,
            root: from,
            interval: (vec2(11.0, 3.0), vec2(7.0, 0.0)),
            edge: (16, 15),
            polygon_from: mesh.get_point_location(from),
            polygon_to: 4,
            previous_polygon_layer: 0,
            distance_start_to_root: 0.0,
            heuristic: from.distance(to),
        };
        let (successors, arena) = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 2);

        assert_eq!(successors[1].root, vec2(11.0, 3.0));
        assert_eq!(
            successors[1].distance_start_to_root,
            from.distance(vec2(11.0, 3.0))
        );
        assert_eq!(
            successors[1].heuristic,
            vec2(11.0, 3.0).distance(vec2(9.75, 6.75)) + vec2(9.75, 6.75).distance(to)
        );
        assert_eq!(successors[1].polygon_from, 4);
        assert_eq!(successors[1].polygon_to, 2);
        assert_eq!(successors[1].interval, (vec2(10.0, 7.0), vec2(9.75, 6.75)));
        assert_eq!(successors[1].edge, (11, 10));
        // assert_eq!(successors[1].path, vec![from]);

        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].distance_start_to_root, 0.0);
        assert_eq!(
            successors[0].heuristic,
            from.distance(vec2(7.0, 4.0)) + vec2(7.0, 4.0).distance(to)
        );
        assert_eq!(successors[0].polygon_from, 4);
        assert_eq!(successors[0].polygon_to, 2);
        assert_eq!(successors[0].interval, (vec2(9.75, 6.75), vec2(7.0, 4.0)));
        assert_eq!(successors[0].edge, (11, 10));
        assert_eq!(
            reconstruct_test_path(&arena, successors[0].arena_parent),
            Vec::<Vec2>::new()
        );

        let successor = successors.into_iter().next().unwrap();
        let (successors, _arena) = dbg!(mesh.successors(successor, to));
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
        assert_eq!(
            mesh.path(from, to).unwrap().path_through_polygons,
            vec![5, 4, 2, 1]
        );
    }

    #[test]
    fn edges_between_simple() {
        let mesh = mesh_from_paper();

        let from = vec2(12.0, 0.0);
        let to = vec2(3.0, 1.0);
        let search_node = SearchNode {
            arena_parent: u32::MAX,
            root: from,
            interval: (vec2(11.0, 3.0), vec2(7.0, 0.0)),
            edge: (16, 15),
            polygon_from: mesh.get_point_location(from),
            polygon_to: 4,
            previous_polygon_layer: 0,
            distance_start_to_root: 0.0,
            heuristic: from.distance(to),
        };

        let successors = mesh.edges_between(&search_node);

        for successor in &successors {
            println!("{successor:?}");
        }

        println!("=========================");

        let search_node = SearchNode {
            arena_parent: u32::MAX,
            root: from,
            interval: (vec2(9.75, 6.75), vec2(7.0, 4.0)),
            edge: (11, 10),
            polygon_from: 4,
            polygon_to: 2,
            previous_polygon_layer: 0,
            distance_start_to_root: 0.0,
            heuristic: from.distance(to),
        };

        let successors = mesh.edges_between(&search_node);

        for successor in &successors {
            println!("{successor:?}");
        }

        println!("=========================");

        let search_node = SearchNode {
            arena_parent: u32::MAX,
            root: vec2(11.0, 3.0),
            interval: (vec2(10.0, 7.0), vec2(7.0, 4.0)),
            edge: (11, 10),
            polygon_from: 4,
            polygon_to: 2,
            previous_polygon_layer: 0,
            distance_start_to_root: 0.0,
            heuristic: from.distance(to),
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
            arena_parent: u32::MAX,
            root: vec2(0.0, 0.0),
            interval: (vec2(1.0, 0.0), vec2(1.0, 1.0)),
            edge: (1, 5),
            polygon_from: 0,
            polygon_to: 1,
            previous_polygon_layer: 0,
            distance_start_to_root: 0.0,
            heuristic: 1.0,
        };

        let successors = mesh.edges_between(&search_node);

        for successor in &successors {
            println!("{successor:?}");
        }
    }

    #[test]
    fn get_closest_point() {
        let mesh = mesh_u_grid();
        let point_location = mesh.get_point_location(vec2(0.5, 0.5));
        let closest_point = mesh.get_closest_point(vec2(0.5, 0.5)).unwrap();
        assert_eq!(point_location, closest_point.polygon_index);
    }

    #[test]
    fn polygon_contains() {
        let mesh = mesh_u_grid();
        let layer = &mesh.layers[0];
        let polygon = &layer.polygons[0];
        assert!(polygon.contains(layer, vec2(0.0, 0.5)));
        assert!(polygon.contains(layer, vec2(0.5, 0.0)));
        assert!(polygon.contains(layer, vec2(0.5, 0.5)));
        assert!(!polygon.contains(layer, vec2(0.5, 1.5)));
        let polygon = &layer.polygons[3];
        assert!(polygon.contains(layer, vec2(0.5, 1.5)));
    }
}
