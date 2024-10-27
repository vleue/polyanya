use smallvec::SmallVec;
#[cfg(feature = "tracing")]
use tracing::instrument;

use std::collections::{BinaryHeap, HashSet};

#[cfg(feature = "stats")]
use std::time::Instant;

use glam::Vec2;
use hashbrown::{hash_map::Entry, HashMap};

#[cfg(feature = "detailed-layers")]
use crate::helpers::EPSILON;
use crate::{
    helpers::{heuristic, line_intersect_segment, turning_point, Vec2Helper},
    Mesh, Path, SearchNode, PRECISION,
};

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
    #[cfg(feature = "detailed-layers")]
    pub(crate) from: (Vec2, u8),
    pub(crate) to: Vec2,
    pub(crate) polygon_to: u32,
    pub(crate) mesh: &'m Mesh,
    pub(crate) blocked_layers: HashSet<u8>,
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
    pub(crate) min_layer_cost: f32,
}

pub(crate) enum InstanceStep {
    Found(Path),
    NotFound,
    Continue,
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
        from: (Vec2, u32),
        to: (Vec2, u32),
        blocked_layers: HashSet<u8>,
        #[cfg(feature = "stats")] start: Instant,
    ) -> Self {
        let starting_polygon =
            &mesh.layers[from.1.layer() as usize].polygons[from.1.polygon() as usize];

        let mut search_instance = SearchInstance {
            queue: BinaryHeap::with_capacity(15),
            node_buffer: Vec::with_capacity(10),
            root_history: HashMap::with_capacity(10),
            #[cfg(feature = "detailed-layers")]
            from: (from.0, from.1.layer()),
            to: to.0,
            polygon_to: to.1,
            mesh,
            blocked_layers,
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
            min_layer_cost: 1.0,
        };
        search_instance.root_history.insert(Root(from.0), 0.0);

        let empty_node = SearchNode {
            path: vec![],
            #[cfg(feature = "detailed-layers")]
            path_with_layers: vec![],
            root: from.0,
            interval: (Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)),
            edge: (0, 0),
            polygon_from: from.1,
            polygon_to: from.1,
            previous_polygon_layer: from.1.layer(),
            distance_start_to_root: 0.0,
            heuristic: 0.0,
        };

        let from_layer = &mesh.layers[from.1.layer() as usize];

        for edge in starting_polygon.edges_index() {
            let start = if let Some(v) = from_layer.vertices.get(edge[0] as usize) {
                v
            } else {
                continue;
            };
            let end = if let Some(v) = from_layer.vertices.get(edge[1] as usize) {
                v
            } else {
                continue;
            };
            let other_side = start
                .polygons
                .iter()
                .filter(|i| **i != u32::MAX && end.polygons.contains(*i))
                .find(|poly| *poly != &from.1)
                .unwrap_or(&u32::MAX);

            if search_instance.blocked_layers.contains(&other_side.layer()) {
                continue;
            }

            if other_side == &to.1
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
                    (start.coords + from_layer.offset, edge[0]),
                    (end.coords + from_layer.offset, edge[1]),
                    &empty_node,
                );
            }
        }
        search_instance.flush_nodes();
        search_instance
    }

    pub(crate) fn next(&mut self) -> InstanceStep {
        if let Some(next) = self.pop_node() {
            #[cfg(feature = "verbose")]
            println!("popped off: {} ({})", next, next.polygon_from);
            #[cfg(feature = "stats")]
            {
                self.popped += 1;
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

            if next.polygon_to == self.polygon_to {
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
                let mut path = next.path;

                let mut path_with_layers_end = vec![];
                if let Some(turn) = turning_point(next.root, self.to, next.interval) {
                    path.push(turn);
                    path_with_layers_end.push((turn, next.polygon_to.layer()));
                }
                let complete = next.polygon_to == self.polygon_to;
                if complete {
                    path.push(self.to);
                    path_with_layers_end.push((self.to, next.polygon_to.layer()));
                }
                #[cfg(feature = "detailed-layers")]
                let path_with_layers = {
                    let mut path_with_layers = vec![];
                    let mut from = self.from.0;
                    for (index, potential_point) in next.path_with_layers.iter().enumerate() {
                        if potential_point.0 == potential_point.1 {
                            from = potential_point.0;
                            path_with_layers.push((potential_point.0, potential_point.2));
                        } else {
                            // look for next fixed point to find the intersection
                            let to = next
                                .path_with_layers
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
                            if p.0.distance_squared(n.0) < EPSILON {
                                continue;
                            }
                        }
                        path_with_layers.push(*p);
                    }
                    path_with_layers
                };

                return InstanceStep::Found(Path {
                    path,
                    #[cfg(not(feature = "detailed-layers"))]
                    length: next.distance_start_to_root + next.heuristic,
                    #[cfg(feature = "detailed-layers")]
                    length: {
                        let a = path_with_layers.iter().fold((0.0, self.from), |acc, p| {
                            let layer = &self.mesh.layers[acc.1 .1 as usize];
                            let scale = layer.scale;
                            let cost = layer.cost;
                            let to_point = (acc.1 .0 * scale).distance(p.0 * scale) * cost;
                            (acc.0 + to_point, *p)
                        });
                        a.0
                    },
                    #[cfg(feature = "detailed-layers")]
                    path_with_layers,
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
        // if polygon.vertices.len() == 3 {
        //     // println!("triangle");
        //     // TODO: possible optimisation
        //     // https://bitbucket.org/dharabor/pathfinding/src/624a6abe8777d14d0753e847b0970e74a7913b45/anyangle/polyanya/search/expansion.cpp#lines-220
        // }

        let right_index = {
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
                    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                    distances.first().unwrap().0
                })
                + 1
        };
        let left_index = polygon.vertices.len() + right_index - 2;

        let mut ty = SuccessorType::RightNonObservable;
        for edge in polygon.circular_edges_index(right_index..=left_index) {
            if edge[0].max(edge[1]) as usize > target_layer.vertices.len() {
                continue;
            }
            // Bounds are checked just before
            #[allow(unsafe_code)]
            let (start, end) = unsafe {
                (
                    target_layer.vertices.get_unchecked(edge[0] as usize),
                    target_layer.vertices.get_unchecked(edge[1] as usize),
                )
            };
            let mut start_point = start.coords + target_layer.offset;
            let end_point = end.coords + target_layer.offset;

            #[cfg(debug_assertions)]
            if self.debug {
                println!("| {edge:?} : {start_point:?} / {end_point:?}");
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
                                edge,
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
                edge,
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
                            edge,
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

        let mut new_f = node.distance_start_to_root;

        let mut path = node.path.clone();
        #[cfg(feature = "detailed-layers")]
        let mut path_with_layers = node.path_with_layers.clone();
        if root != node.root {
            path.push(root);
            #[cfg(feature = "detailed-layers")]
            path_with_layers.push((root, root, node.polygon_to.layer()));
            #[cfg(not(feature = "detailed-layers"))]
            {
                new_f += node.root.distance(root);
            }
            #[cfg(feature = "detailed-layers")]
            {
                let layer = &self.mesh.layers[node.polygon_to.layer() as usize];
                new_f += node
                    .root
                    .distance(root * layer.scale) * layer.cost;
            }
        }
        #[cfg(feature = "detailed-layers")]
        if other_side.layer() != node.polygon_to.layer() {
            path_with_layers.push((start.0, end.0, other_side.layer()));
        }

        let mut heuristic_to_end: f32;
        #[cfg(not(feature = "detailed-layers"))]
        {
            heuristic_to_end = heuristic(root, self.to, (start.0, end.0));
        }
        #[cfg(feature = "detailed-layers")]
        {
            let start_layer = &self.mesh.layers[start.1.layer() as usize];
            let end_layer = &self.mesh.layers[end.1.layer() as usize];
            heuristic_to_end = heuristic(
                root,
                self.to,
                (
                    start.0 * start_layer.scale,
                    end.0 * end_layer.scale,
                ),
            );
        }
        heuristic_to_end *= self.min_layer_cost;
        if new_f.is_nan() || heuristic_to_end.is_nan() {
            #[cfg(debug_assertions)]
            if self.debug {
                println!("x one of the distance is NaN");
            }

            return;
        }

        let new_node = SearchNode {
            path,
            #[cfg(feature = "detailed-layers")]
            path_with_layers,
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
        let mut visited = HashSet::new();
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
                let target_layer = &self.mesh.layers[node.polygon_to.layer() as usize];
                // we know they exist, it's checked in `edges_between`
                #[allow(unsafe_code)]
                let (start, end) = unsafe {
                    (
                        target_layer
                            .vertices
                            .get_unchecked(successor.edge[0] as usize),
                        target_layer
                            .vertices
                            .get_unchecked(successor.edge[1] as usize),
                    )
                };

                #[cfg(debug_assertions)]
                if self.debug {
                    println!("v {successor:?}");
                }

                let other_side = start
                    .polygons
                    .iter()
                    .filter(|i| **i != u32::MAX && end.polygons.contains(*i))
                    .find(|poly| poly != &&node.polygon_to)
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

                if self.blocked_layers.contains(&other_side.layer()) {
                    #[cfg(debug_assertions)]
                    if self.debug {
                        println!("x blocked layer");
                    }

                    continue;
                }

                // prune edges that only lead to one other polygon, and not the target: dead end pruning
                if &self.polygon_to != other_side
                    && self.mesh.layers[other_side.layer() as usize].polygons
                        [other_side.polygon() as usize]
                        .is_one_way
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

                const EPSILON: f32 = 1.0e-10;
                let root = match successor.ty {
                    SuccessorType::RightNonObservable => {
                        if successor
                            .interval
                            .0
                            .distance_squared(start.coords + target_layer.offset)
                            > EPSILON
                        {
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
                            || (!self.blocked_layers.is_empty()
                                && vertex.polygons.iter().any(|p| {
                                    *p == u32::MAX || self.blocked_layers.contains(&p.layer())
                                })))
                            && (vertex.coords
                                + self.mesh.layers[node.previous_polygon_layer as usize].offset)
                                .distance_squared(node.interval.0)
                                < EPSILON
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
                        if (successor.interval.1).distance_squared(end.coords + target_layer.offset)
                            > EPSILON
                        {
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
                            || (!self.blocked_layers.is_empty()
                                && vertex.polygons.iter().any(|p| {
                                    *p == u32::MAX || self.blocked_layers.contains(&p.layer())
                                })))
                            && (vertex.coords
                                + self.mesh.layers[node.previous_polygon_layer as usize].offset)
                                .distance_squared(node.interval.1)
                                < EPSILON
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
                    (successor.interval.0, successor.edge[0]),
                    (successor.interval.1, successor.edge[1]),
                    &node,
                );
            }

            if self.node_buffer.len() == 1 && self.node_buffer[0].polygon_to != self.polygon_to {
                #[cfg(feature = "verbose")]
                for new_node in &self.node_buffer {
                    println!(
                        "        intermediate: {} -> to polygon {}/{}",
                        new_node,
                        new_node.polygon_to.layer(),
                        new_node.polygon_to.polygon()
                    );
                }
                let previous_node = node;
                node = self.node_buffer.drain(..).next().unwrap();
                if node.root == previous_node.root
                    && node.polygon_to == previous_node.polygon_from
                    && node.polygon_from == previous_node.polygon_to
                    && node.interval.0 == previous_node.interval.1
                    && node.interval.1 == previous_node.interval.0
                {
                    // going the exact reverse way as we went into this polygon
                    // TODO: shouldn't happen, identify cases that trigger this
                    break;
                }
                if !visited.insert(node.polygon_to) {
                    // infinite loop, exit now
                    // TODO: shouldn't happen, identify cases that trigger this
                    break;
                }
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
