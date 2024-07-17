use smallvec::SmallVec;
#[cfg(feature = "tracing")]
use tracing::instrument;

use std::collections::BinaryHeap;

#[cfg(feature = "stats")]
use std::time::Instant;

use glam::Vec2;
use hashbrown::{hash_map::Entry, HashMap};

use crate::{
    helpers::{heuristic, line_intersect_segment, turning_point, Vec2Helper},
    Mesh, Path, PolygonInMesh, Root, SearchNode, POLYGON_NOT_FOUND,
};

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
    edge: (u32, u32),
    ty: SuccessorType,
}

pub(crate) struct SearchInstance<'m> {
    pub(crate) queue: BinaryHeap<SearchNode>,
    pub(crate) node_buffer: Vec<SearchNode>,
    pub(crate) root_history: HashMap<Root, f32>,
    pub(crate) from: Vec2,
    pub(crate) to: Vec2,
    pub(crate) polygon_to: PolygonInMesh,
    pub(crate) mesh: &'m Mesh,
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

impl<'m> SearchInstance<'m> {
    pub(crate) fn setup(
        mesh: &'m Mesh,
        from: (Vec2, PolygonInMesh),
        to: (Vec2, PolygonInMesh),
        #[cfg(feature = "stats")] start: Instant,
    ) -> Self {
        let starting_polygon =
            &mesh.layers[from.1.layer as usize].polygons[from.1.polygon as usize];

        let mut search_instance = SearchInstance {
            queue: BinaryHeap::with_capacity(15),
            node_buffer: Vec::with_capacity(10),
            root_history: HashMap::with_capacity(10),
            from: from.0,
            to: to.0,
            polygon_to: to.1,
            mesh,
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

        let empty_node = SearchNode {
            path: vec![],
            root: from.0,
            interval: (Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)),
            edge: (0, 0),
            polygon_from: POLYGON_NOT_FOUND,
            polygon_to: from.1,
            f: 0.0,
            g: 0.0,
        };

        for edge in starting_polygon.edges_index().iter() {
            let start = if let Some(v) = mesh.layers[0].vertices.get(edge.0 as usize) {
                v
            } else {
                continue;
            };
            let end = if let Some(v) = mesh.layers[0].vertices.get(edge.1 as usize) {
                v
            } else {
                continue;
            };
            let other_side = start
                .polygons
                .iter()
                .filter(|i| **i != -1 && end.polygons.contains(*i))
                .map(|i| PolygonInMesh {
                    layer: (*i >> 24) as u8,
                    polygon: ((*i << 8) >> 8) as u32,
                })
                .find(|poly| poly != &from.1)
                .unwrap_or(POLYGON_NOT_FOUND);

            if other_side == to.1
                || (other_side != POLYGON_NOT_FOUND
                    && !search_instance.mesh.layers[other_side.layer as usize]
                        .polygons
                        .get(other_side.polygon as usize)
                        .unwrap()
                        .is_one_way)
            {
                search_instance.add_node(
                    from.0,
                    other_side,
                    (start.coords, edge.0),
                    (end.coords, edge.1),
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
            println!("popped off: {}", next);
            #[cfg(feature = "stats")]
            {
                self.popped += 1;
            }

            if let Some(o) = self.root_history.get(&Root(next.root)) {
                if o < &next.f {
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
                        next.f + next.g,
                    );
                    self.mesh.scenarios.set(self.mesh.scenarios.get() + 1);
                }
                let mut path = next
                    .path
                    .split_first()
                    .map(|(_, p)| p)
                    .unwrap_or(&[])
                    .to_vec();
                if next.root != self.from {
                    path.push(next.root);
                }
                if let Some(turn) = turning_point(next.root, self.to, next.interval) {
                    path.push(turn);
                }
                let complete = next.polygon_to == self.polygon_to;
                if complete {
                    path.push(self.to);
                }
                return InstanceStep::Found(Path {
                    path,
                    length: next.f + next.g,
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

        let polygon = &self.mesh.layers[node.polygon_to.layer as usize].polygons
            [node.polygon_to.polygon as usize];

        // if node.interval.0.distance(node.root) < 1.0e-5
        //     || node.interval.1.distance(node.root) < 1.0e-5
        //     || node.root.side(node.interval) == EdgeSide::Edge
        // {
        //     // println!("collinear");
        //     // TODO: possible optimisation
        // }
        // if polygon.vertices.len() == 3 {
        //     // println!("triangle");
        //     // TODO: possible optimisation
        // }

        let right_index = {
            let mut temp = 0;
            while polygon.vertices[temp] != node.edge.1 {
                temp += 1;
            }
            temp + 1
        };
        let left_index = polygon.vertices.len() + right_index - 2;

        let mut ty = SuccessorType::RightNonObservable;
        for edge in &polygon.circular_edges_index(right_index..=left_index) {
            if edge.0.max(edge.1) as usize > self.mesh.layers[0].vertices.len() {
                continue;
            }
            // Bounds are checked just before
            #[allow(unsafe_code)]
            let (start, end) = unsafe {
                (
                    self.mesh.layers[0].vertices.get_unchecked(edge.0 as usize),
                    self.mesh.layers[0].vertices.get_unchecked(edge.1 as usize),
                )
            };
            let mut start_point = start.coords;
            let end_point = end.coords;

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
                                edge: *edge,
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
                edge: *edge,
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
                            edge: *edge,
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
        other_side: PolygonInMesh,
        start: (Vec2, u32),
        end: (Vec2, u32),
        node: &SearchNode,
    ) {
        #[cfg(feature = "stats")]
        {
            self.nodes_generated += 1;
        }

        let mut path = node.path.clone();
        if root != node.root {
            path.push(node.root);
        }

        let heuristic = heuristic(root, self.to, (start.0, end.0));
        let new_f = node.f + node.root.distance(root);
        if new_f.is_nan() || heuristic.is_nan() {
            #[cfg(debug_assertions)]
            if self.debug {
                println!("x one of the distance is NaN");
            }

            return;
        }

        let new_node = SearchNode {
            path,
            root,
            interval: (start.0, end.0),
            edge: (start.1, end.1),
            polygon_from: node.polygon_to,
            polygon_to: other_side,
            f: new_f,
            g: heuristic,
        };

        match self.root_history.entry(Root(root)) {
            Entry::Occupied(mut o) => {
                if o.get() < &new_node.f {
                    #[cfg(debug_assertions)]
                    if self.debug {
                        println!("x already got a better path");
                    }
                } else {
                    #[cfg(debug_assertions)]
                    if self.debug {
                        println!("o added!");
                    }
                    o.insert(new_node.f);
                    self.node_buffer.push(new_node);
                }
            }
            Entry::Vacant(v) => {
                #[cfg(debug_assertions)]
                if self.debug {
                    println!("o added!");
                }
                v.insert(new_node.f);
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
            println!("        pushing: {}", new_node);
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
                // we know they exist, it's checked in `edges_between`
                #[allow(unsafe_code)]
                let (start, end) = unsafe {
                    (
                        self.mesh.layers[0]
                            .vertices
                            .get_unchecked(successor.edge.0 as usize),
                        self.mesh.layers[0]
                            .vertices
                            .get_unchecked(successor.edge.1 as usize),
                    )
                };

                #[cfg(debug_assertions)]
                if self.debug {
                    println!("v {successor:?}");
                }

                let other_side = start
                    .polygons
                    .iter()
                    // .map(|i| dbg!(i))
                    .filter(|i| **i != -1 && end.polygons.contains(*i))
                    .map(|i| PolygonInMesh {
                        layer: (*i >> 24) as u8,
                        polygon: ((*i << 8) >> 8) as u32,
                    })
                    // .map(|i| dbg!(i))
                    .find(|poly| poly != &node.polygon_to)
                    .unwrap_or(POLYGON_NOT_FOUND);

                #[cfg(debug_assertions)]
                if self.debug {
                    println!("| going to {other_side:?}");
                }

                // prune edges that don't have a polygon on the other side: cul de sac pruning
                if other_side == POLYGON_NOT_FOUND {
                    #[cfg(debug_assertions)]
                    if self.debug {
                        println!("x cul de sac");
                    }

                    continue;
                }

                // prune edges that only lead to one other polygon, and not the target: dead end pruning
                if self.polygon_to != other_side
                    && self.mesh.layers[other_side.layer as usize]
                        .polygons
                        .get(other_side.polygon as usize)
                        .unwrap()
                        .is_one_way
                {
                    #[cfg(debug_assertions)]
                    if self.debug {
                        println!("x dead end");
                    }

                    continue;
                }

                const EPSILON: f32 = 1.0e-10;
                let root = match successor.ty {
                    SuccessorType::RightNonObservable => {
                        if successor.interval.0.distance_squared(start.coords) > EPSILON {
                            #[cfg(debug_assertions)]
                            if self.debug {
                                println!("x non observable on an intersection");
                            }
                            continue;
                        }
                        let vertex = self.mesh.layers[0]
                            .vertices
                            .get(node.edge.0 as usize)
                            .unwrap();
                        if vertex.is_corner
                            && vertex.coords.distance_squared(node.interval.0) < EPSILON
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
                        if successor.interval.1.distance_squared(end.coords) > EPSILON {
                            #[cfg(debug_assertions)]
                            if self.debug {
                                println!("x non observable on an intersection");
                            }
                            continue;
                        }
                        let vertex = self.mesh.layers[0]
                            .vertices
                            .get(node.edge.1 as usize)
                            .unwrap();
                        if vertex.is_corner
                            && vertex.coords.distance_squared(node.interval.1) < EPSILON
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

                self.add_node(
                    root,
                    other_side,
                    (successor.interval.0, successor.edge.0),
                    (successor.interval.1, successor.edge.1),
                    &node,
                )
            }

            if self.node_buffer.len() == 1 && self.node_buffer[0].polygon_to != self.polygon_to {
                #[cfg(feature = "verbose")]
                for new_node in &self.node_buffer {
                    println!("        intermediate: {}", new_node);
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
