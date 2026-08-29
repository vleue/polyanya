use smallvec::SmallVec;
#[cfg(feature = "tracing")]
use tracing::instrument;

use std::collections::{BinaryHeap, HashSet};

#[cfg(feature = "stats")]
use std::time::Instant;

use glam::Vec2;
use hashbrown::{hash_map::Entry, HashMap};

use crate::{
    helpers::{heuristic, line_intersect_segment, turning_point, Vec2Helper},
    Layer, Mesh, Path, PathArenaNode, Polygon, SearchNode, PRECISION,
};

/// A run of this many pops without `f` going up is taken as the search going in circles,
/// and turns on the bookkeeping in `is_new`.
///
/// It only has to sit above the longest run a healthy search produces, and those are
/// short: over 15000 searches on the bundled meshes the longest was 79, on `aurora.mesh`.
/// Turning the bookkeeping on early costs a little speed and nothing else, so there is no
/// need to leave much more room than that. A mesh with fewer polygons than this uses its
/// polygon count instead, so that a small mesh still gets there well inside the iteration
/// limit `Mesh::path` searches under.
const STALL_LIMIT: usize = 512;

pub(crate) struct Root(Vec2);

impl PartialEq for Root {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Root {}

impl std::hash::Hash for Root {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ((self.0.x * PRECISION) as i32).hash(state);
        ((self.0.y * PRECISION) as i32).hash(state);
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum EdgeSide {
    Left,
    Right,
    Edge,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum SuccessorType {
    LeftNonObservable,
    Observable,
    RightNonObservable,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Successor {
    interval: (Vec2, Vec2),
    edge: [u32; 2],
    ty: SuccessorType,
}

pub(crate) struct SearchInstance<'m> {
    pub(crate) queue: BinaryHeap<SearchNode>,
    pub(crate) node_buffer: Vec<SearchNode>,
    pub(crate) root_history: HashMap<Root, f32>,
    /// Nodes already expanded, keyed on everything that decides what a node expands
    /// into. Empty unless the search has started going in circles: see `is_new`.
    pub(crate) seen_nodes: hashbrown::HashSet<[u32; 10]>,
    /// `f` of the last node popped, and how many pops have gone by without it going up.
    /// This is what going in circles looks like from here: see `is_new`.
    pub(crate) last_f: f32,
    pub(crate) stalled_pops: u32,
    /// How long a run of pops without progress is tolerated before `seen_nodes` starts
    /// recording. See `STALL_LIMIT`.
    pub(crate) stall_limit: u32,
    pub(crate) recording: bool,
    pub(crate) path_arena: Vec<PathArenaNode>,
    pub(crate) from: (Vec2, u8),
    pub(crate) to: Vec2,
    /// A polygon that counts as the goal, and the ones after it.
    ///
    /// A 2D point over overlapping polygons has more than one reading, and any of them
    /// ends the search. One search over all of them beats one search each: they share a
    /// queue, so the first goal reached is the closest one. There is almost always just
    /// the one, and the test for it sits in the hot loop, so it is kept out of the list.
    pub(crate) polygon_to: u32,
    pub(crate) other_polygons_to: Vec<u32>,
    pub(crate) mesh: &'m Mesh,
    pub(crate) blocked_layers: HashSet<u8>,
    /// The cheapest a unit of travel can be on this mesh: the smallest component of any
    /// layer's `scale`. The heuristic is measured at this rate so that it stays a lower
    /// bound however the path ends up routed. See [`SearchInstance::add_node`].
    #[cfg(feature = "detailed-layers")]
    pub(crate) min_scale: f32,
    #[cfg(feature = "stats")]
    pub(crate) start: Instant,
    #[cfg(feature = "stats")]
    pub(crate) pushed: usize,
    #[cfg(feature = "stats")]
    pub(crate) popped: u32,
    #[cfg(feature = "stats")]
    pub(crate) successors_called: u32,
    #[cfg(feature = "stats")]
    pub(crate) nodes_generated: u32,
    #[cfg(feature = "stats")]
    pub(crate) nodes_pruned_post_pop: u32,
    #[cfg(debug_assertions)]
    pub(crate) debug: bool,
    #[cfg(debug_assertions)]
    pub(crate) fail_fast: i32,
}

pub(crate) enum InstanceStep {
    Found(Path),
    NotFound,
    Continue,
}

/// Did a computed interval end land on the vertex it was meant to?
///
/// Interval ends come out of intersecting a ray with an edge. When the ray passes close to
/// a corner it crosses that edge at a shallow angle, the intersection is the ratio of two
/// nearly cancelling quantities, and the answer comes back a good deal further from the
/// corner than `f32`'s precision alone would suggest. The search then has to decide whether
/// that end is the corner or some other point along the edge. Decide it too strictly and it
/// refuses to turn at a corner it can plainly see, takes the long way round, and answers
/// the same query differently depending on which end you start from. Decide it too loosely
/// and it turns at a vertex that was never there.
///
/// The allowance has a floor because that cancellation does not shrink with the ray: on the
/// bundled meshes the ends that should have been corners missed by 1.0 to 1.4 thousandths
/// of a unit alike, over rays of 18, 46 and 239 units. It grows with the ray on top of that,
/// so it does not go tight on a mesh whose coordinates are larger than the ones measured
/// here. In those same searches the nearest end that genuinely was not a corner sat several
/// times further out than the floor.
#[inline(always)]
fn lands_on(computed: Vec2, vertex: Vec2, root: Vec2) -> bool {
    const RELATIVE: f32 = 5.0e-5;
    const FLOOR: f32 = 2.0e-3;
    let allowance = (RELATIVE * root.distance(vertex)).max(FLOOR);
    computed.distance_squared(vertex) < allowance.powi(2)
}

/// Narrow a successor's interval to the part of it the parent could actually see.
///
/// A successor that keeps its parent's root sees through the parent's wedge, so its own
/// wedge has to sit inside that one. Nothing else makes that true. The wedge is carried as
/// two loose points on an edge and every decision about it is a float comparison, so a
/// wedge thin enough that its two bounding rays stop being distinguishable can come back
/// out of the edge walk pointing somewhere else entirely: on the bundled meshes a wedge
/// seen to narrow to a twentieth of a degree reappears in the next polygon sixteen degrees
/// wide and not even overlapping where it came from. A search that believes that walks
/// straight through the wall the wedge had narrowed around.
///
/// Cutting the interval down to the overlap is what keeps a vanishing wedge vanishing.
/// Discarding a successor outright instead is not enough and not safe: most of these
/// overlap their parent in part, and dropping those loses real ways through.
#[inline(always)]
fn clip_to_cone(root: Vec2, cone: (Vec2, Vec2), segment: (Vec2, Vec2)) -> Option<(Vec2, Vec2)> {
    // Interval ends are computed by intersecting a ray with an edge, and on the bundled
    // meshes ends that should have landed exactly on a corner miss it by about a
    // thousandth of a unit. This runs at every polygon along a path, though, so what it has
    // to tolerate is not one of those misses but however far they have wandered by the time
    // a long chain of them has gone by; clip tighter than that and the wedge gets eaten a
    // little at each step until real ways through are gone. There is room to be generous:
    // the widening this exists to stop is not a near miss but a wedge reappearing sixteen
    // degrees wide after narrowing to a twentieth of one.
    const SLACK: f32 = 1.0e-2;

    let right = cone.0 - root;
    let left = cone.1 - root;
    let (right_len, left_len) = (right.length(), left.length());
    // A root sitting on one end of its own interval has no wedge: the ray to that end has
    // no direction, and what it can see is a half plane rather than a slice. There is
    // nothing to clip against, and dividing by that ray's length only manufactures noise.
    if right_len < SLACK || left_len < SLACK {
        return Some(segment);
    }

    // How far each end of the interval sits inside the wedge, as a distance rather than a
    // signed area, so one allowance means the same thing whatever the lengths involved.
    // Inside is left of the ray through the wedge's right hand end and right of the ray
    // through its left hand end, so both of these are positive there.
    let from_right = |point: Vec2| right.perp_dot(point - root) / right_len + SLACK;
    let from_left = |point: Vec2| -left.perp_dot(point - root) / left_len + SLACK;

    let (mut lo, mut hi) = (0.0_f32, 1.0_f32);
    for (at_start, at_end) in [
        (from_right(segment.0), from_right(segment.1)),
        (from_left(segment.0), from_left(segment.1)),
    ] {
        if at_start < 0.0 && at_end < 0.0 {
            // wholly on the wrong side of this ray
            return None;
        }
        // Where the interval crosses the ray, as a fraction along it.
        if at_start < 0.0 {
            lo = lo.max(at_start / (at_start - at_end));
        } else if at_end < 0.0 {
            hi = hi.min(at_start / (at_start - at_end));
        }
    }
    if lo > hi {
        return None;
    }
    let along = segment.1 - segment.0;
    Some((segment.0 + along * lo, segment.0 + along * hi))
}

pub(crate) trait U32Layer {
    fn layer(&self) -> u8;

    fn polygon(&self) -> u32;

    fn from_layer_and_polygon(layer: u8, polygon: u32) -> Self;
}

impl U32Layer for u32 {
    #[inline(always)]
    fn layer(&self) -> u8 {
        (*self >> 24) as u8
    }

    #[inline(always)]
    fn polygon(&self) -> u32 {
        *self & 0b00000000111111111111111111111111
    }

    #[inline(always)]
    fn from_layer_and_polygon(layer: u8, polygon: u32) -> u32 {
        ((layer as u32) << 24) | polygon
    }
}

impl<'m> SearchInstance<'m> {
    pub(crate) fn setup(
        mesh: &'m Mesh,
        from: (Vec2, &[u32]),
        to: (Vec2, &[u32]),
        blocked_layers: HashSet<u8>,
        #[cfg(feature = "stats")] start: Instant,
    ) -> Self {
        let mut search_instance = SearchInstance {
            queue: BinaryHeap::with_capacity(15),
            node_buffer: Vec::with_capacity(10),
            root_history: HashMap::with_capacity(10),
            seen_nodes: hashbrown::HashSet::new(),
            last_f: 0.0,
            stalled_pops: 0,
            stall_limit: mesh
                .layers
                .iter()
                .map(|layer| layer.polygons.len())
                .sum::<usize>()
                .min(STALL_LIMIT) as u32,
            recording: false,
            path_arena: Vec::with_capacity(50),
            from: (from.0, from.1.first().map_or(0, |polygon| polygon.layer())),
            to: to.0,
            polygon_to: to.1.first().copied().unwrap_or(u32::MAX),
            other_polygons_to: to.1.iter().skip(1).copied().collect(),
            mesh,
            blocked_layers,
            #[cfg(feature = "detailed-layers")]
            min_scale: mesh.min_scale(),
            #[cfg(feature = "stats")]
            start,
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
        search_instance.root_history.insert(Root(from.0), 0.0);

        for from_polygon in from.1 {
            // The polygon the search starts in is not reached over an edge, so nothing
            // would put it in the path. Seed the arena with it instead, and every path
            // built from these nodes starts where it actually started.
            let origin = search_instance.path_arena.len() as u32;
            search_instance.path_arena.push(PathArenaNode {
                root: from.0,
                polygon: *from_polygon,
                parent: u32::MAX,
                root_changed: false,
                // Never read: this entry's layer is the one the path starts in, so the
                // reconstruction never sees a layer change on it.
                #[cfg(feature = "detailed-layers")]
                interval: (from.0, from.0),
            });

            let empty_node = SearchNode {
                arena_parent: origin,
                root: from.0,
                interval: (Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)),
                edge: (0, 0),
                polygon_from: *from_polygon,
                polygon_to: *from_polygon,
                previous_polygon_layer: from_polygon.layer(),
                distance_start_to_root: 0.0,
                heuristic: 0.0,
            };

            let from_layer = &mesh.layers[from_polygon.layer() as usize];
            let starting_polygon = &from_layer.polygons[from_polygon.polygon() as usize];

            for [edge0, edge1] in starting_polygon.edges_index() {
                let start = if let Some(v) = from_layer.vertices.get(edge0 as usize) {
                    v
                } else {
                    continue;
                };
                let end = if let Some(v) = from_layer.vertices.get(edge1 as usize) {
                    v
                } else {
                    continue;
                };
                let other_side = start
                    .polygons
                    .iter()
                    .find(|i| **i != u32::MAX && **i != *from_polygon && end.polygons.contains(*i))
                    .unwrap_or(&u32::MAX);

                if search_instance.is_blocked(other_side.layer()) {
                    continue;
                }

                if search_instance.is_goal(*other_side)
                    || (other_side != &u32::MAX
                        && !search_instance.mesh.layers[other_side.layer() as usize]
                            .polygons
                            .get(other_side.polygon() as usize)
                            .unwrap()
                            .is_one_way)
                {
                    search_instance.add_node(
                        from.0,
                        *other_side,
                        (start.coords + from_layer.offset, edge0),
                        (end.coords + from_layer.offset, edge1),
                        &empty_node,
                    );
                }
            }
        }
        search_instance.flush_nodes();
        search_instance
    }

    /// Does reaching this polygon end the search?
    #[inline(always)]
    pub(crate) fn is_goal(&self, polygon: u32) -> bool {
        polygon == self.polygon_to
            || (!self.other_polygons_to.is_empty() && self.other_polygons_to.contains(&polygon))
    }

    pub(crate) fn next(&mut self) -> InstanceStep {
        if let Some(next) = self.pop_node() {
            #[cfg(feature = "verbose")]
            println!("popped off: {} ({})", next, next.polygon_from);
            #[cfg(feature = "stats")]
            {
                self.popped += 1;
            }

            if !self.recording {
                // Every node on a cycle has the same `f`: a node never has a lower `f`
                // than the one it came from, so a run that arrives back where it started
                // cannot have gone up along the way either. A long run of pops that does
                // not raise `f` is what that looks like from here.
                let f = next.distance_start_to_root + next.heuristic;
                if f > self.last_f {
                    self.last_f = f;
                    self.stalled_pops = 0;
                } else {
                    self.stalled_pops += 1;
                    self.recording = self.stalled_pops > self.stall_limit;
                }
            }

            if let Some(o) = self.root_history.get(&Root(next.root)) {
                // TODO: revisit this for layers with different height at the same coordinates
                if o < &next.distance_start_to_root {
                    #[cfg(feature = "verbose")]
                    println!("node is dominated!");
                    #[cfg(feature = "stats")]
                    {
                        self.nodes_pruned_post_pop += 1;
                    }

                    return InstanceStep::Continue;
                }
            }

            if self.recording && !self.is_new(&next) {
                #[cfg(feature = "verbose")]
                println!("node is a duplicate!");
                #[cfg(feature = "stats")]
                {
                    self.nodes_pruned_post_pop += 1;
                }

                return InstanceStep::Continue;
            }

            if self.is_goal(next.polygon_to) {
                #[cfg(feature = "stats")]
                {
                    if self.mesh.scenarios.get() == 0 {
                        eprintln!(
                        "index;micros;successor_calls;generated;pushed;popped;pruned_post_pop;length",
                    );
                    }
                    eprintln!(
                        "{};{};{};{};{};{};{};{}",
                        self.mesh.scenarios.get(),
                        self.start.elapsed().as_secs_f32() * 1_000_000.0,
                        self.successors_called,
                        self.nodes_generated,
                        self.pushed,
                        self.popped,
                        self.nodes_pruned_post_pop,
                        next.distance_start_to_root + next.heuristic,
                    );
                    self.mesh.scenarios.set(self.mesh.scenarios.get() + 1);
                }
                // Reconstruct path and polygons from arena
                let (mut path, path_through_polygons) = self.reconstruct_path(next.arena_parent);

                #[cfg(feature = "detailed-layers")]
                let arena_path_with_layers = self.reconstruct_path_with_layers(next.arena_parent);

                let mut path_with_layers_end = vec![];
                if let Some(turn) = turning_point(next.root, self.to, next.interval) {
                    path.push(turn);
                    path_with_layers_end.push((turn, next.polygon_to.layer()));
                }
                let complete = self.is_goal(next.polygon_to);
                if complete {
                    path.push(self.to);
                    path_with_layers_end.push((self.to, next.polygon_to.layer()));
                }

                #[cfg(feature = "detailed-layers")]
                let path_with_layers = {
                    let mut path_with_layers = vec![];
                    let mut from = self.from.0;
                    for (index, potential_point) in arena_path_with_layers.iter().enumerate() {
                        if potential_point.0 == potential_point.1 {
                            from = potential_point.0;
                            path_with_layers.push((potential_point.0, potential_point.2));
                        } else {
                            // look for next fixed point to find the intersection
                            let to = arena_path_with_layers
                                .iter()
                                .skip(index + 1)
                                .find(|point| point.0 == point.1)
                                .map(|point| point.0)
                                .unwrap_or(path_with_layers_end[0].0);
                            if let Some(intersection) = line_intersect_segment(
                                (from, to),
                                (potential_point.0, potential_point.1),
                            ) {
                                from = intersection;
                                path_with_layers.push((intersection, potential_point.2));
                            }
                        }
                    }
                    path_with_layers.extend(path_with_layers_end);
                    let mut path_with_layers_peekable = path_with_layers.iter().peekable();
                    let mut path_with_layers = vec![];
                    while let Some(p) = path_with_layers_peekable.next() {
                        if let Some(n) = path_with_layers_peekable.peek() {
                            if p.0.distance_squared(n.0) < 1.0e-12 {
                                continue;
                            }
                        }
                        path_with_layers.push(*p);
                    }
                    path_with_layers
                };

                return InstanceStep::Found(Path {
                    #[cfg(not(feature = "detailed-layers"))]
                    // Measured over the path that is actually returned, not as
                    // `distance_start_to_root + heuristic`. The two agree while every
                    // assumption the heuristic makes holds, and stop agreeing when the goal
                    // sits outside the polygon the search ended in, which `search_delta`
                    // allows: the heuristic then measures to a mirrored goal, or misses the
                    // backtrack the reconstruction emits, and the reported length is off by
                    // units in either direction. This is what the `detailed-layers` build
                    // has always done.
                    length: path
                        .iter()
                        .fold((0.0, self.from.0), |(total, previous), point| {
                            (total + previous.distance(*point), *point)
                        })
                        .0,
                    path,
                    #[cfg(feature = "detailed-layers")]
                    length: {
                        let start = (
                            self.from.0,
                            path_through_polygons
                                .first()
                                .map_or(self.from.1, |polygon| polygon.layer()),
                        );
                        let a = path_with_layers.iter().fold((0.0, start), |acc, p| {
                            let scale = self.mesh.layers[acc.1 .1 as usize].scale;
                            let to_point = (acc.1 .0 * scale).distance(p.0 * scale);
                            (acc.0 + to_point, *p)
                        });
                        a.0
                    },
                    #[cfg(feature = "detailed-layers")]
                    path_with_layers: path_with_layers.to_vec(),
                    path_through_polygons,
                });
            }
            self.successors(next);
            return InstanceStep::Continue;
        }
        #[cfg(feature = "stats")]
        eprintln!(
            "{:?} / {:?} / {:?} / {:?}",
            self.successors_called, self.nodes_generated, self.pushed, self.popped
        );
        InstanceStep::NotFound
    }

    /// Reconstruct the path (turning points) and polygon chain from the arena.
    pub(crate) fn reconstruct_path(&self, arena_parent: u32) -> (Vec<Vec2>, Vec<u32>) {
        let mut turning_points = Vec::new();
        let mut polygons = Vec::new();

        // Walk arena chain backwards, collecting into vecs
        let mut chain = Vec::new();
        let mut idx = arena_parent;
        while idx != u32::MAX {
            chain.push(idx);
            idx = self.path_arena[idx as usize].parent;
        }
        chain.reverse();

        for &arena_idx in &chain {
            let entry = &self.path_arena[arena_idx as usize];
            polygons.push(entry.polygon);
            if entry.root_changed {
                turning_points.push(entry.root);
            }
        }

        (turning_points, polygons)
    }

    /// Reconstruct path_with_layers from the arena (only used with detailed-layers feature).
    #[cfg(feature = "detailed-layers")]
    pub(crate) fn reconstruct_path_with_layers(&self, arena_parent: u32) -> Vec<(Vec2, Vec2, u8)> {
        let mut chain = Vec::new();
        let mut idx = arena_parent;
        while idx != u32::MAX {
            chain.push(idx);
            idx = self.path_arena[idx as usize].parent;
        }
        chain.reverse();

        let mut result = Vec::new();
        // The layer the path is travelling in when it reaches an entry is the layer of the
        // polygon the previous entry stepped into. The chain starts on the seeded entry for
        // the polygon the search started in, so that one gives the layer to start from.
        let Some(&origin) = chain.first() else {
            return result;
        };
        let mut previous_layer = self.path_arena[origin as usize].polygon.layer();
        for &arena_idx in &chain {
            let entry = &self.path_arena[arena_idx as usize];
            if entry.root_changed {
                result.push((entry.root, entry.root, previous_layer));
            }
            let layer = entry.polygon.layer();
            if layer != previous_layer {
                result.push((entry.interval.0, entry.interval.1, layer));
            }
            previous_layer = layer;
        }
        result
    }

    /// An intersection that lands on one end of the edge it was computed on, snapped to that
    /// end's exact coordinates.
    ///
    /// The generic path in [`Self::edges_between`] only splits an edge when the intersection falls
    /// strictly inside it, so an interval that reaches a corner carries that corner's exact
    /// coordinates. That has to hold here too: `successors` only lets the search turn at a corner
    /// when the interval end matches the vertex to 1e-10, which a computed intersection misses.
    #[inline(always)]
    fn snap_to_edge_end(intersection: Vec2, start: Vec2, end: Vec2) -> Vec2 {
        const EPSILON: f32 = 1.0e-6;
        if intersection.distance_squared(start) < EPSILON {
            start
        } else if intersection.distance_squared(end) < EPSILON {
            end
        } else {
            intersection
        }
    }

    /// Successors of a search node expanding into a triangle.
    ///
    /// A triangle has only two edges to expand onto, and where the interval splits between them is
    /// decided by two orientation tests, so the generic edge walk in [`Self::edges_between`] can be
    /// skipped entirely: at most three successors, at most two intersections, no iteration.
    ///
    /// Ported from the reference implementation:
    /// <https://bitbucket.org/dharabor/pathfinding/src/624a6abe8777d14d0753e847b0970e74a7913b45/anyangle/polyanya/search/expansion.cpp#lines-220>
    ///
    /// Returns `None` when the triangle can't be handled here, in which case the caller falls back
    /// to the generic path.
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    fn edges_between_triangle(
        &self,
        node: &SearchNode,
        polygon: &Polygon,
        target_layer: &Layer,
        left_vertex_index: usize,
    ) -> Option<SmallVec<[Successor; 10]>> {
        // Naming follows the reference implementation: going counter clockwise from the vertex the
        // interval's left end comes from, the three corners are t3 (left), t1 (right) and t2. We
        // came in through t3-t1, so the successors are on t1-t2 and t2-t3.
        let i3 = left_vertex_index;
        let i1 = (i3 + 1) % 3;
        let i2 = (i3 + 2) % 3;
        let (v1, v2, v3) = (
            polygon.vertices[i1],
            polygon.vertices[i2],
            polygon.vertices[i3],
        );
        if v1.max(v2).max(v3) as usize >= target_layer.vertices.len() {
            return None;
        }
        let t1 = target_layer.vertices[v1 as usize].coords + target_layer.offset;
        let t2 = target_layer.vertices[v2 as usize].coords + target_layer.offset;
        let t3 = target_layer.vertices[v3 as usize].coords + target_layer.offset;

        let root = node.root;
        let (right, left) = node.interval;

        // Turning at an end of the interval is only possible if the interval actually reaches the
        // corner there. Whether that corner is one we're allowed to turn at is checked later, in
        // `successors`, which also knows about blocked layers.
        let reaches_right = lands_on(right, t1, root);
        let reaches_left = lands_on(left, t3, root);

        let mut successors = SmallVec::new();
        match t2.side((root, left)) {
            // t2 is behind the left end of the interval: everything observable is on t1-t2.
            EdgeSide::Left => {
                let li =
                    Self::snap_to_edge_end(line_intersect_segment((root, left), (t1, t2))?, t1, t2);
                let ri = if reaches_right {
                    t1
                } else {
                    Self::snap_to_edge_end(line_intersect_segment((root, right), (t1, t2))?, t1, t2)
                };
                successors.push(Successor {
                    interval: (ri, li),
                    edge: [v1, v2],
                    ty: SuccessorType::Observable,
                });
                if reaches_left {
                    successors.push(Successor {
                        interval: (li, t2),
                        edge: [v1, v2],
                        ty: SuccessorType::LeftNonObservable,
                    });
                    successors.push(Successor {
                        interval: (t2, t3),
                        edge: [v2, v3],
                        ty: SuccessorType::LeftNonObservable,
                    });
                }
            }
            // The left end of the interval points straight at t2: the observable part ends there.
            EdgeSide::Edge => {
                let ri = if reaches_right {
                    t1
                } else {
                    Self::snap_to_edge_end(line_intersect_segment((root, right), (t1, t2))?, t1, t2)
                };
                successors.push(Successor {
                    interval: (ri, t2),
                    edge: [v1, v2],
                    ty: SuccessorType::Observable,
                });
                if reaches_left {
                    successors.push(Successor {
                        interval: (t2, t3),
                        edge: [v2, v3],
                        ty: SuccessorType::LeftNonObservable,
                    });
                }
            }
            // The observable part reaches past t2, onto t2-t3.
            EdgeSide::Right => {
                let li = if reaches_left {
                    t3
                } else {
                    Self::snap_to_edge_end(line_intersect_segment((root, left), (t2, t3))?, t2, t3)
                };
                match t2.side((root, right)) {
                    // The observable part is entirely on t2-t3.
                    EdgeSide::Right => {
                        let ri = Self::snap_to_edge_end(
                            line_intersect_segment((root, right), (t2, t3))?,
                            t2,
                            t3,
                        );
                        if reaches_right {
                            successors.push(Successor {
                                interval: (t1, t2),
                                edge: [v1, v2],
                                ty: SuccessorType::RightNonObservable,
                            });
                            successors.push(Successor {
                                interval: (t2, ri),
                                edge: [v2, v3],
                                ty: SuccessorType::RightNonObservable,
                            });
                        }
                        successors.push(Successor {
                            interval: (ri, li),
                            edge: [v2, v3],
                            ty: SuccessorType::Observable,
                        });
                    }
                    // The right end of the interval points straight at t2.
                    EdgeSide::Edge => {
                        if reaches_right {
                            successors.push(Successor {
                                interval: (t1, t2),
                                edge: [v1, v2],
                                ty: SuccessorType::RightNonObservable,
                            });
                        }
                        successors.push(Successor {
                            interval: (t2, li),
                            edge: [v2, v3],
                            ty: SuccessorType::Observable,
                        });
                    }
                    // The observable part straddles t2, so it spans both edges.
                    EdgeSide::Left => {
                        let ri = if reaches_right {
                            t1
                        } else {
                            Self::snap_to_edge_end(
                                line_intersect_segment((root, right), (t1, t2))?,
                                t1,
                                t2,
                            )
                        };
                        successors.push(Successor {
                            interval: (ri, t2),
                            edge: [v1, v2],
                            ty: SuccessorType::Observable,
                        });
                        successors.push(Successor {
                            interval: (t2, li),
                            edge: [v2, v3],
                            ty: SuccessorType::Observable,
                        });
                    }
                }
            }
        }

        Some(successors)
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    pub(crate) fn edges_between(&self, node: &SearchNode) -> SmallVec<[Successor; 10]> {
        let mut successors = SmallVec::new();

        let target_layer = &self.mesh.layers[node.polygon_to.layer() as usize];

        let polygon = &target_layer.polygons[node.polygon_to.polygon() as usize];

        // if node.interval.0.distance(node.root) < 1.0e-5
        //     || node.interval.1.distance(node.root) < 1.0e-5
        //     || node.root.side(node.interval) == EdgeSide::Edge
        // {
        //     // println!("collinear");
        //     // TODO: possible optimisation
        //     // https://bitbucket.org/dharabor/pathfinding/src/624a6abe8777d14d0753e847b0970e74a7913b45/anyangle/polyanya/search/expansion.cpp#lines-156
        // }

        let left_vertex_index = {
            // Vertex indices are only meaningful within a layer. When the previous polygon is on
            // the same layer, the shared vertex can be found by comparing indices, without
            // touching any coordinates.
            let same_layer_index = (node.previous_polygon_layer == node.polygon_to.layer())
                .then(|| polygon.vertices.iter().position(|v| *v == node.edge.1))
                .flatten();

            same_layer_index.unwrap_or_else(|| {
                let edge = self.mesh.layers[node.previous_polygon_layer as usize].vertices
                    [node.edge.1 as usize]
                    .coords
                    + self.mesh.layers[node.previous_polygon_layer as usize].offset;
                polygon
                    .vertices
                    .iter()
                    .enumerate()
                    .find(|(_, v)| {
                        (target_layer.vertices[**v as usize].coords + target_layer.offset)
                            .distance_squared(edge)
                            < 0.001
                    })
                    .map(|(i, _)| i)
                    .unwrap_or_else(|| {
                        let mut distances = polygon
                            .vertices
                            .iter()
                            .map(|v| {
                                (target_layer.vertices[*v as usize].coords + target_layer.offset)
                                    .distance_squared(edge)
                            })
                            .enumerate()
                            .collect::<Vec<_>>();
                        distances.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                        distances.first().unwrap().0
                    })
            })
        };

        if polygon.vertices.len() == 3 {
            if let Some(successors) =
                self.edges_between_triangle(node, polygon, target_layer, left_vertex_index)
            {
                return successors;
            }
        }

        let right_index = left_vertex_index + 1;
        let left_index = polygon.vertices.len() + right_index - 2;

        let mut ty = SuccessorType::RightNonObservable;
        // Walks the edges starting after the one we came through, wrapping around the polygon at
        // most once.
        for i in right_index..=left_index {
            let [edge0, edge1] = polygon.circular_edge(i);
            if edge0.max(edge1) as usize > target_layer.vertices.len() {
                continue;
            }
            // Bounds are checked just before
            #[allow(unsafe_code)]
            let (start, end) = unsafe {
                (
                    target_layer.vertices.get_unchecked(edge0 as usize),
                    target_layer.vertices.get_unchecked(edge1 as usize),
                )
            };
            let mut start_point = start.coords + target_layer.offset;
            let end_point = end.coords + target_layer.offset;

            #[cfg(debug_assertions)]
            if self.debug {
                println!("| {edge0:?}-{edge1:?} : {start_point:?} / {end_point:?}");
                println!(
                    "|   {:?} - {:?}",
                    start_point.side((node.root, node.interval.0)),
                    start_point.side((node.root, node.interval.1))
                );
                println!(
                    "|   {:?} - {:?}",
                    end_point.side((node.root, node.interval.0)),
                    end_point.side((node.root, node.interval.1))
                );
            }

            match start_point.side((node.root, node.interval.0)) {
                EdgeSide::Right => {
                    if let Some(intersect) = line_intersect_segment(
                        (node.root, node.interval.0),
                        (start_point, end_point),
                    ) {
                        #[cfg(debug_assertions)]
                        if self.debug {
                            println!("|   intersection 0 {intersect:?}");
                            println!(
                                "|     {:?} / {:?}",
                                intersect.distance(start_point),
                                intersect.distance(end_point)
                            );
                        }
                        if intersect.distance_squared(start_point) > 1.0e-6
                            && intersect.distance_squared(end_point) > 1.0e-6
                        {
                            successors.push(Successor {
                                interval: (start_point, intersect),
                                edge: [edge0, edge1],
                                ty,
                            });
                            start_point = intersect;
                        } else {
                            #[cfg(debug_assertions)]
                            if self.debug {
                                println!("|     ignoring intersection");
                            }
                        }
                        if intersect.distance_squared(end_point) > 1.0e-6 {
                            ty = SuccessorType::Observable;
                        }
                    }
                }
                EdgeSide::Left => {
                    if ty == SuccessorType::RightNonObservable {
                        ty = SuccessorType::Observable;
                    }
                }
                EdgeSide::Edge => match end_point.side((node.root, node.interval.0)) {
                    EdgeSide::Edge | EdgeSide::Left => {
                        ty = SuccessorType::Observable;
                    }
                    _ => (),
                },
            }
            let mut end_intersection_p = None;
            let mut found_intersection = false;
            let end_root_int1 = end_point.side((node.root, node.interval.1));

            if end_root_int1 == EdgeSide::Left {
                if let Some(intersect) =
                    line_intersect_segment((node.root, node.interval.1), (start_point, end_point))
                {
                    #[cfg(debug_assertions)]
                    if self.debug {
                        println!("|   intersection 1 {intersect:?}");
                        println!(
                            "|     {:?} / {:?}",
                            intersect.distance(start_point),
                            intersect.distance(end_point)
                        );
                    }

                    if intersect.distance_squared(end_point) > 1.0e-6 {
                        end_intersection_p = Some(intersect);
                    } else {
                        #[cfg(debug_assertions)]
                        if self.debug {
                            println!("|     ignoring intersection");
                        }
                    }
                    found_intersection = true;
                }
            }
            successors.push(Successor {
                interval: (start_point, end_intersection_p.unwrap_or(end_point)),
                edge: [edge0, edge1],
                ty,
            });
            match end_root_int1 {
                EdgeSide::Left => {
                    if found_intersection {
                        ty = SuccessorType::LeftNonObservable;
                    }
                    if let Some(intersect) = end_intersection_p {
                        successors.push(Successor {
                            interval: (intersect, end_point),
                            edge: [edge0, edge1],
                            ty,
                        });
                    }
                }
                EdgeSide::Edge => match end_point.side((node.root, node.interval.0)) {
                    EdgeSide::Edge | EdgeSide::Left => {
                        ty = SuccessorType::LeftNonObservable;
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        successors
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    pub(crate) fn add_node(
        &mut self,
        root: Vec2,
        other_side: u32,
        start: (Vec2, u32),
        end: (Vec2, u32),
        node: &SearchNode,
    ) {
        #[cfg(feature = "stats")]
        {
            self.nodes_generated += 1;
        }

        // Keeping the root means seeing through the parent's wedge, so cut this interval
        // down to the part of it that wedge reaches. The start node has no wedge yet, and a
        // root that moved to a corner starts a wedge of its own, so neither is clipped.
        let (start, end) = if root == node.root && node.interval.0 != node.interval.1 {
            match clip_to_cone(root, node.interval, (start.0, end.0)) {
                Some((clipped_start, clipped_end)) => {
                    ((clipped_start, start.1), (clipped_end, end.1))
                }
                None => return,
            }
        } else {
            (start, end)
        };

        let mut new_f = node.distance_start_to_root;

        if root != node.root {
            #[cfg(not(feature = "detailed-layers"))]
            {
                new_f += node.root.distance(root);
            }
            #[cfg(feature = "detailed-layers")]
            {
                // Both ends scaled by the layer the segment runs through, so this is a
                // length in that layer's space. Scaling only one end, as this used to,
                // is not a distance in any space and is not even offset-invariant.
                new_f += ((root - node.root)
                    * self.mesh.layers[node.polygon_to.layer() as usize].scale)
                    .length();
            }
        }

        let heuristic_to_end: f32;
        #[cfg(not(feature = "detailed-layers"))]
        {
            heuristic_to_end = heuristic(root, self.to, (start.0, end.0));
        }
        #[cfg(feature = "detailed-layers")]
        {
            // Every point in raw coordinates, then scaled by the cheapest a unit of travel
            // can be anywhere on this mesh. No path can beat that rate, so this never
            // overestimates, which is what lets the search stop at the first goal it pops.
            // Mixing scaled interval ends with a raw root and goal, as this used to, is not
            // a bound on anything.
            heuristic_to_end = heuristic(root, self.to, (start.0, end.0)) * self.min_scale;
        }
        if new_f.is_nan() || heuristic_to_end.is_nan() {
            #[cfg(debug_assertions)]
            if self.debug {
                println!("x one of the distance is NaN");
            }

            return;
        }

        // Push arena entry for this edge
        let root_changed = root != node.root;
        let arena_idx = self.path_arena.len() as u32;
        self.path_arena.push(PathArenaNode {
            root,
            polygon: other_side,
            parent: node.arena_parent,
            root_changed,
            #[cfg(feature = "detailed-layers")]
            interval: (start.0, end.0),
        });

        let new_node = SearchNode {
            arena_parent: arena_idx,
            root,
            interval: (start.0, end.0),
            edge: (start.1, end.1),
            polygon_from: node.polygon_to,
            polygon_to: other_side,
            previous_polygon_layer: node.polygon_to.layer(),
            distance_start_to_root: new_f,
            heuristic: heuristic_to_end,
        };

        match self.root_history.entry(Root(root)) {
            Entry::Occupied(mut o) => {
                if o.get() < &new_node.distance_start_to_root {
                    #[cfg(debug_assertions)]
                    if self.debug {
                        println!("x already got a better path");
                    }
                } else {
                    #[cfg(debug_assertions)]
                    if self.debug {
                        println!(
                            "o replaced with {}! ({:?})",
                            new_node.distance_start_to_root, new_node
                        );
                    }
                    o.insert(new_node.distance_start_to_root);
                    self.node_buffer.push(new_node);
                }
            }
            Entry::Vacant(v) => {
                #[cfg(debug_assertions)]
                if self.debug {
                    println!(
                        "o added with {}! ({:?})",
                        new_node.distance_start_to_root, new_node
                    );
                }
                v.insert(new_node.distance_start_to_root);
                self.node_buffer.push(new_node);
            }
        }
    }

    /// The `f` of the cheapest node still queued, or `None` if the queue is empty.
    ///
    /// `f` never overestimates, so nothing still in the queue can produce a path shorter
    /// than this. Once a complete path that short has been found, the search is done.
    #[cfg(feature = "detailed-layers")]
    #[inline(always)]
    pub(crate) fn queued_lower_bound(&self) -> Option<f32> {
        self.queue
            .peek()
            .map(|node| node.distance_start_to_root + node.heuristic)
    }

    /// Does this search block any layer at all?
    #[inline(always)]
    fn has_blocked_layers(&self) -> bool {
        !self.blocked_layers.is_empty()
    }

    /// Is this layer blocked for this search?
    #[inline(always)]
    fn is_blocked(&self, layer: u8) -> bool {
        self.has_blocked_layers() && self.blocked_layers.contains(&layer)
    }

    /// Has this node already been expanded? The key holds everything the expansion reads:
    /// the polygon it goes into, the edge it comes over, and the wedge it looks through.
    /// The cost is deliberately left out, so that a repeat arriving more expensively goes
    /// as well — two nodes with this key have the same heuristic, so the cheapest of them
    /// is the one the queue hands over first.
    ///
    /// Only reached once the search has started going in circles, which is the only time
    /// anything comes back. A funnel that reaches a corner whose polygons form a ring can
    /// walk that ring with the root and the cost pinned, regenerating the same nodes lap
    /// after lap until the iteration limit runs out, and the path that does exist is never
    /// returned. `root_history` cannot stop it: it drops nodes that are strictly worse,
    /// and these are equal. Recording them is what ends the lap, and doing it only once a
    /// search looks stuck keeps it off the paths of every search that does not.
    #[inline(always)]
    fn is_new(&mut self, node: &SearchNode) -> bool {
        self.seen_nodes.insert([
            node.polygon_to,
            node.polygon_from,
            node.edge.0,
            node.edge.1,
            node.root.x.to_bits(),
            node.root.y.to_bits(),
            node.interval.0.x.to_bits(),
            node.interval.0.y.to_bits(),
            node.interval.1.x.to_bits(),
            node.interval.1.y.to_bits(),
        ])
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    pub(crate) fn flush_nodes(&mut self) {
        #[cfg(feature = "stats")]
        {
            self.pushed += self.node_buffer.len();
        }
        #[cfg(feature = "verbose")]
        for new_node in &self.node_buffer {
            println!(
                "        pushing: {} ({}) ({}/{})",
                new_node,
                new_node.interval.1.distance_squared(new_node.interval.0),
                new_node.polygon_to.layer(),
                new_node.polygon_to.polygon(),
            );
        }
        self.queue.extend(self.node_buffer.drain(..));
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    pub(crate) fn pop_node(&mut self) -> Option<SearchNode> {
        self.queue.pop()
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    pub(crate) fn successors(&mut self, mut node: SearchNode) {
        // A node with a single successor is expanded in place instead of going through the queue.
        // Polygons can be laid out so that such a chain walks a cycle, so its length is capped:
        // past the cap the node is left in the buffer and goes back to the queue, where the regular
        // search handles it. Chains this long are vanishingly rare, and cutting one only costs a
        // push and a pop.
        const MAX_CHAINED_EXPANSIONS: u32 = 64;
        let mut chained_expansions = 0;
        // Read once. A search never gains a goal, but `self` is borrowed mutably below, so
        // the compiler has to reload it on every edge otherwise, and this loop is the
        // hottest thing in the crate.
        let has_other_goals = !self.other_polygons_to.is_empty();
        loop {
            #[cfg(feature = "stats")]
            {
                self.successors_called += 1;
            }
            #[cfg(debug_assertions)]
            // select a search node to enable debug more
            if false {
                self.debug = true;
                self.fail_fast = 3;
            }
            for successor in self.edges_between(&node).iter() {
                let [successor_edge_0, successor_edge_1] = successor.edge;
                let target_layer = &self.mesh.layers[node.polygon_to.layer() as usize];
                // we know they exist, it's checked in `edges_between`
                #[allow(unsafe_code)]
                let (start, end) = unsafe {
                    (
                        target_layer
                            .vertices
                            .get_unchecked(successor_edge_0 as usize),
                        target_layer
                            .vertices
                            .get_unchecked(successor_edge_1 as usize),
                    )
                };

                #[cfg(debug_assertions)]
                if self.debug {
                    println!("v {successor:?}");
                }

                let other_side = start
                    .polygons
                    .iter()
                    .find(|i| {
                        **i != u32::MAX && **i != node.polygon_to && end.polygons.contains(*i)
                    })
                    .unwrap_or(&u32::MAX);

                #[cfg(debug_assertions)]
                if self.debug {
                    match other_side {
                        &u32::MAX => println!("| going to u32::MAX"),
                        _ => println!(
                            "| going to {:?} / {:?}",
                            other_side.layer(),
                            other_side.polygon()
                        ),
                    }
                }

                // prune edges that don't have a polygon on the other side: cul de sac pruning
                if other_side == &u32::MAX {
                    #[cfg(debug_assertions)]
                    if self.debug {
                        println!("x cul de sac");
                    }

                    continue;
                }

                if self.is_blocked(other_side.layer()) {
                    #[cfg(debug_assertions)]
                    if self.debug {
                        println!("x blocked layer");
                    }

                    continue;
                }

                // prune edges that only lead to one other polygon, and not the target: dead end pruning
                // `is_one_way` first: it is false for almost every polygon, so the goal
                // test -- which is only there to stop the goal itself being pruned -- is
                // not paid on the edges that do not need it.
                if self.mesh.layers[other_side.layer() as usize].polygons
                    [other_side.polygon() as usize]
                    .is_one_way
                    && !(*other_side == self.polygon_to
                        || (has_other_goals && self.other_polygons_to.contains(other_side)))
                {
                    #[cfg(debug_assertions)]
                    if self.debug {
                        println!("x dead end");
                    }

                    continue;
                }

                if node.polygon_from == *other_side {
                    #[cfg(debug_assertions)]
                    if self.debug {
                        println!("x going back to the same polygon");
                    }

                    continue;
                }

                let root = match successor.ty {
                    SuccessorType::RightNonObservable => {
                        if !lands_on(
                            successor.interval.0,
                            start.coords + target_layer.offset,
                            node.root,
                        ) {
                            #[cfg(debug_assertions)]
                            if self.debug {
                                println!("x non observable on an intersection (right)");
                            }
                            continue;
                        }
                        let vertex = self.mesh.layers[node.previous_polygon_layer as usize]
                            .vertices
                            .get(node.edge.0 as usize)
                            .unwrap();
                        if (vertex.is_corner
                            || (self.has_blocked_layers()
                                && vertex
                                    .polygons
                                    .iter()
                                    .any(|p| *p == u32::MAX || self.is_blocked(p.layer()))))
                            && lands_on(
                                node.interval.0,
                                vertex.coords
                                    + self.mesh.layers[node.previous_polygon_layer as usize].offset,
                                node.root,
                            )
                        {
                            node.interval.0
                        } else {
                            #[cfg(debug_assertions)]
                            if self.debug {
                                println!("x non observable on an non corner");
                            }
                            continue;
                        }
                    }
                    SuccessorType::Observable => node.root,
                    SuccessorType::LeftNonObservable => {
                        if !lands_on(
                            successor.interval.1,
                            end.coords + target_layer.offset,
                            node.root,
                        ) {
                            #[cfg(debug_assertions)]
                            if self.debug {
                                println!("x non observable on an intersection (left)");
                            }
                            continue;
                        }
                        let vertex = self.mesh.layers[node.previous_polygon_layer as usize]
                            .vertices
                            .get(node.edge.1 as usize)
                            .unwrap();
                        if (vertex.is_corner
                            || (self.has_blocked_layers()
                                && vertex
                                    .polygons
                                    .iter()
                                    .any(|p| *p == u32::MAX || self.is_blocked(p.layer()))))
                            && lands_on(
                                node.interval.1,
                                vertex.coords
                                    + self.mesh.layers[node.previous_polygon_layer as usize].offset,
                                node.root,
                            )
                        {
                            node.interval.1
                        } else {
                            #[cfg(debug_assertions)]
                            if self.debug {
                                println!("x non observable on an non corner");
                            }
                            continue;
                        }
                    }
                };

                #[cfg(debug_assertions)]
                if self.debug {
                    println!("| through root {root:?}");
                }

                if successor.interval.0.distance_squared(successor.interval.1) < 1.0e-10 {
                    #[cfg(debug_assertions)]
                    if self.debug {
                        println!("x zero length edge");
                    }

                    continue;
                }

                self.add_node(
                    root,
                    *other_side,
                    (successor.interval.0, successor_edge_0),
                    (successor.interval.1, successor_edge_1),
                    &node,
                );
            }

            if self.node_buffer.len() == 1 && !self.is_goal(self.node_buffer[0].polygon_to) {
                #[cfg(feature = "verbose")]
                for new_node in &self.node_buffer {
                    println!(
                        "        intermediate: {} -> to polygon {}/{}",
                        new_node,
                        new_node.polygon_to.layer(),
                        new_node.polygon_to.polygon()
                    );
                }
                chained_expansions += 1;
                if chained_expansions > MAX_CHAINED_EXPANSIONS {
                    // leave the node in the buffer, it will be pushed to the queue
                    break;
                }
                node = self.node_buffer.drain(..).next().unwrap();
                #[cfg(debug_assertions)]
                {
                    self.fail_fast -= 1;
                    if self.fail_fast == 0 {
                        panic!()
                    }
                }
            } else {
                #[cfg(debug_assertions)]
                {
                    self.fail_fast -= 1;
                    if self.fail_fast == 0 {
                        panic!()
                    }
                }
                break;
            }
        }
        self.flush_nodes();
    }
}
