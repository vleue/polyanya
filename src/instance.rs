use smallvec::SmallVec;
#[cfg(feature = "tracing")]
use tracing::instrument;

use std::collections::BinaryHeap;

#[cfg(feature = "stats")]
use std::time::Instant;

use glam::Vec2;
use hashbrown::{hash_map::Entry, HashMap};

use crate::{
    helpers::{heuristic, line_intersect_segment, Vec2Helper},
    Mesh, Root, SearchNode,
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
    edge: (usize, usize),
    ty: SuccessorType,
}

pub(crate) struct SearchInstance<'m> {
    pub(crate) queue: BinaryHeap<SearchNode>,
    pub(crate) node_buffer: Vec<SearchNode>,
    pub(crate) root_history: HashMap<Root, f32>,
    pub(crate) to: Vec2,
    pub(crate) polygon_to: isize,
    pub(crate) mesh: &'m Mesh,
    #[cfg(feature = "stats")]
    pub(crate) start: Instant,
    #[cfg(feature = "stats")]
    pub(crate) pushed: usize,
    #[cfg(feature = "stats")]
    pub(crate) popped: usize,
    #[cfg(feature = "stats")]
    pub(crate) successors_called: usize,
    #[cfg(feature = "stats")]
    pub(crate) nodes_generated: usize,
    #[cfg(feature = "stats")]
    pub(crate) nodes_pruned_post_pop: usize,
    #[cfg(debug_assertions)]
    pub(crate) debug: bool,
    #[cfg(debug_assertions)]
    pub(crate) fail_fast: i32,
}

impl<'m> SearchInstance<'m> {
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    pub(crate) fn edges_between(&self, node: &SearchNode) -> SmallVec<[Successor; 10]> {
        let mut successors = SmallVec::new();

        let polygon = &self.mesh.polygons[node.polygon_to as usize];

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
        for edge in &polygon.double_edges_index()[right_index..=left_index] {
            if edge.0.max(edge.1) > self.mesh.vertices.len() {
                continue;
            }
            // Bounds are checked just before
            #[allow(unsafe_code)]
            let start = unsafe { self.mesh.vertices.get_unchecked(edge.0) };
            #[allow(unsafe_code)]
            let end = unsafe { self.mesh.vertices.get_unchecked(edge.1) };
            let mut start_point = start.coords;
            let end_point = end.coords;

            #[cfg(debug_assertions)]
            if self.debug {
                println!("| {:?} : {:?} / {:?}", edge, start_point, end_point);
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

            let end_root_int1 = end_point.side((node.root, node.interval.1));
            match start_point.side((node.root, node.interval.0)) {
                EdgeSide::Right => {
                    if let Some(intersect) = line_intersect_segment(
                        (node.root, node.interval.0),
                        (start_point, end_point),
                    ) {
                        #[cfg(debug_assertions)]
                        if self.debug {
                            println!("|   intersection 0 {:?}", intersect);
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
            if end_root_int1 == EdgeSide::Left {
                if let Some(intersect) =
                    line_intersect_segment((node.root, node.interval.1), (start_point, end_point))
                {
                    #[cfg(debug_assertions)]
                    if self.debug {
                        println!("|   intersection 1 {:?}", intersect);
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
        other_side: isize,
        start: (Vec2, usize),
        end: (Vec2, usize),
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
                let start = self.mesh.vertices.get(successor.edge.0).unwrap();
                let end = self.mesh.vertices.get(successor.edge.1).unwrap();

                #[cfg(debug_assertions)]
                if self.debug {
                    println!("v {:?}", successor);
                }

                let mut other_side = isize::MAX;
                // find the polygon at the other side of this edge
                for i in &start.polygons {
                    if *i != -1 && *i != node.polygon_to && end.polygons.contains(i) {
                        other_side = *i;
                    }
                }

                #[cfg(debug_assertions)]
                if self.debug {
                    println!("| going to {:?}", other_side);
                }

                // prune edges that don't have a polygon on the other side: cul de sac pruning
                if other_side == isize::MAX {
                    #[cfg(debug_assertions)]
                    if self.debug {
                        println!("x cul de sac");
                    }

                    continue;
                }

                // prune edges that only lead to one other polygon, and not the target: dead end pruning
                if self.polygon_to != other_side
                    && self
                        .mesh
                        .polygons
                        .get(other_side as usize)
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
                        let vertex = self.mesh.vertices.get(node.edge.0).unwrap();
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
                        let vertex = self.mesh.vertices.get(node.edge.1).unwrap();
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
                    println!("| through root {:?}", root);
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
