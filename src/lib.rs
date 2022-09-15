pub const PRECISION: f32 = 1000.0;

#[cfg(feature = "stats")]
use std::time::Instant;
use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt::{self, Display},
    hash::Hash,
    io,
    io::BufRead,
};

use glam::Vec2;
use hashbrown::{hash_map::Entry, HashMap, HashSet};
use helpers::*;

use indexmap::IndexMap;
#[cfg(feature = "tracing")]
use tracing::instrument;

mod helpers;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum EdgeSide {
    Left,
    Right,
    Edge,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
    pub coords: Vec2,
    pub polygons: Vec<isize>,
    pub is_corner: bool,
}

impl Vertex {
    pub fn new(coords: Vec2, polygons: Vec<isize>) -> Self {
        Self {
            coords,
            is_corner: polygons.contains(&-1),
            polygons,
        }
    }

    pub fn from_coords(x: u32, y: u32, poly: Vec<isize>) -> Self {
        Vertex {
            coords: Vec2::new(x as f32, y as f32),
            is_corner: poly.contains(&-1),
            polygons: poly,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum SuccessorType {
    LeftNonObservable,
    Observable,
    RightNonObservable,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Successor {
    interval: (Vec2, Vec2),
    edge: (usize, usize),
    ty: SuccessorType,
}

#[derive(Debug, PartialEq)]
pub struct Path {
    pub len: f32,
    pub path: Vec<Vec2>,
    pub complete: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    pub vertices: Vec<usize>,
    pub is_one_way: bool,
    aabb: (Vec2, Vec2),
}

impl Polygon {
    pub const EMPTY: Polygon = Polygon {
        vertices: vec![],
        is_one_way: false,
        aabb: (Vec2::ZERO, Vec2::ZERO),
    };

    pub fn new(vertices: Vec<usize>, is_one_way: bool) -> Polygon {
        Polygon {
            vertices,
            is_one_way,
            aabb: (Vec2::ZERO, Vec2::ZERO),
        }
    }

    pub fn using(nb: usize, data: Vec<isize>) -> Self {
        assert!(data.len() == nb * 2);
        let (vertices, neighbours) = data.split_at(nb);
        let vertices = vertices.iter().copied().map(|v| v as usize).collect();
        let neighbours = neighbours.to_vec();
        let mut found_trav = false;
        let mut is_one_way = true;
        for neighbour in &neighbours {
            if *neighbour != -1 {
                if found_trav {
                    is_one_way = false;
                    break;
                } else {
                    found_trav = true;
                }
            }
        }
        Polygon {
            vertices,
            is_one_way,
            aabb: (Vec2::ZERO, Vec2::ZERO),
        }
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    fn edges_index(&self) -> Vec<(usize, usize)> {
        let mut edges = Vec::with_capacity(self.vertices.len() / 2);
        if self.vertices.is_empty() {
            return vec![];
        }
        let mut last = self.vertices[0];
        for vertex in self.vertices.iter().skip(1) {
            edges.push((last, *vertex));
            last = *vertex;
        }
        edges.push((last, self.vertices[0]));
        edges
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    fn double_edges_index(&self) -> Vec<(usize, usize)> {
        let mut edges = Vec::with_capacity(self.vertices.len());
        let mut last = self.vertices[0];
        for vertex in self.vertices.iter().skip(1) {
            edges.push((last, *vertex));
            last = *vertex;
        }
        edges.push((last, self.vertices[0]));
        let mut last = self.vertices[0];
        for vertex in self.vertices.iter().skip(1) {
            edges.push((last, *vertex));
            last = *vertex;
        }
        edges.push((last, self.vertices[0]));
        edges
    }
}

#[derive(Debug, Default, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub polygons: Vec<Polygon>,
    baked_polygons: IndexMap<i32, Vec<usize>>,
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

impl Mesh {
    pub fn bake(&mut self) {
        for polygon in &mut self.polygons {
            polygon.aabb = polygon.vertices.iter().fold(
                (Vec2::new(f32::MAX, f32::MAX), Vec2::ZERO),
                |mut aabb, v| {
                    if let Some(v) = self.vertices.get(*v) {
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
            );
        }

        self.baked_polygons = self
            .vertices
            .iter()
            .map(|v| ((v.coords.x * PRECISION) as i32, vec![]))
            .collect();
        self.baked_polygons.sort_keys();

        for (i, polygon) in self.polygons.iter().enumerate() {
            for (k, polys) in &mut self.baked_polygons {
                if *k < (polygon.aabb.0.x * PRECISION) as i32 {
                    continue;
                }
                polys.push(i);
                if *k > (polygon.aabb.1.x * PRECISION) as i32 {
                    break;
                }
            }
        }
    }

    pub fn new(vertices: Vec<Vertex>, polygons: Vec<Polygon>) -> Mesh {
        let mut mesh = Mesh {
            vertices,
            polygons,
            baked_polygons: IndexMap::default(),
        };
        mesh.bake();
        mesh
    }

    pub fn from_file(path: &str) -> Mesh {
        let file = std::fs::File::open(path).unwrap();
        let mut mesh = Mesh::default();
        let mut nb_vertices = 0;
        let mut nb_polygons = 0;
        let mut phase = 0;
        for line in io::BufReader::new(file).lines() {
            let line: String = line.unwrap();
            if phase == 0 {
                if line == "mesh" || line == "2" {
                    continue;
                } else {
                    (nb_vertices, nb_polygons) = line
                        .split_once(' ')
                        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                        .unwrap();
                    phase = 1;
                    continue;
                }
            }
            if phase == 1 {
                if nb_vertices > 0 {
                    nb_vertices -= 1;
                    let mut values = line.split(' ');
                    let x = values.next().unwrap().parse().unwrap();
                    let y = values.next().unwrap().parse().unwrap();
                    let _ = values.next();
                    let vertex =
                        Vertex::from_coords(x, y, values.map(|v| v.parse().unwrap()).collect());
                    mesh.vertices.push(vertex);
                } else {
                    phase = 2;
                }
            }
            if phase == 2 {
                if nb_polygons > 0 {
                    nb_polygons -= 1;
                    let mut values = line.split(' ');
                    let n = values.next().unwrap().parse().unwrap();
                    let polygon = Polygon::using(n, values.map(|v| v.parse().unwrap()).collect());
                    mesh.polygons.push(polygon)
                } else {
                    panic!("unexpected line");
                }
            }
        }
        mesh.bake();
        mesh
    }
}

struct SearchInstance<'m> {
    queue: BinaryHeap<SearchNode>,
    node_buffer: Vec<SearchNode>,
    root_history: HashMap<Root, f32>,
    to: Vec2,
    polygon_to: isize,
    mesh: &'m Mesh,
    #[cfg(feature = "stats")]
    start: Instant,
    #[cfg(feature = "stats")]
    pushed: usize,
    #[cfg(feature = "stats")]
    popped: usize,
    #[cfg(feature = "stats")]
    successors_called: usize,
    #[cfg(feature = "stats")]
    nodes_generated: usize,
    #[cfg(feature = "stats")]
    nodes_pruned_post_pop: usize,
    #[cfg(debug_assertions)]
    debug: bool,
    #[cfg(debug_assertions)]
    fail_fast: i32,
}

impl Mesh {
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    pub fn path(&self, from: Vec2, to: Vec2) -> Path {
        #[cfg(feature = "stats")]
        let start = Instant::now();

        let starting_polygon_index = self.get_point_location(from);
        // let starting_polygon_index = 12654;
        let starting_polygon = if let Some(polygon) = self.polygons.get(starting_polygon_index) {
            polygon
        } else {
            return Path {
                len: -1.0,
                path: vec![],
                complete: false,
            };
        };
        let ending_polygon = self.get_point_location(to);
        // let ending_polygon = 11397;

        if starting_polygon_index == ending_polygon {
            return Path {
                len: from.distance(to),
                path: vec![to],
                complete: true,
            };
        }

        let mut search_instance = SearchInstance {
            queue: BinaryHeap::with_capacity(15),
            node_buffer: Vec::with_capacity(10),
            root_history: HashMap::with_capacity(10),
            to,
            polygon_to: ending_polygon as isize,
            mesh: self,
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
        search_instance.root_history.insert(Root(from), 0.0);

        let empty_node = SearchNode {
            path: vec![],
            root: from,
            interval: (Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)),
            edge: (0, 0),
            polygon_from: -1,
            polygon_to: starting_polygon_index as isize,
            f: 0.0,
            g: 0.0,
        };

        for edge in starting_polygon.edges_index() {
            let start = if let Some(v) = self.vertices.get(edge.0) {
                v
            } else {
                continue;
            };
            let end = if let Some(v) = self.vertices.get(edge.1) {
                v
            } else {
                continue;
            };

            let mut other_side = isize::MAX;
            for i in &start.polygons {
                if *i != -1 && *i != starting_polygon_index as isize && end.polygons.contains(i) {
                    other_side = *i;
                }
            }

            if other_side != isize::MAX
                && !search_instance
                    .mesh
                    .polygons
                    .get(other_side as usize)
                    .unwrap()
                    .is_one_way
            {
                search_instance.add_node(
                    from,
                    other_side,
                    (start.coords, edge.0),
                    (end.coords, edge.1),
                    &empty_node,
                );
            }
        }
        search_instance.flush_nodes();

        while let Some(next) = search_instance.queue.pop() {
            #[cfg(feature = "verbose")]
            println!("popped off: {}", next);
            #[cfg(feature = "stats")]
            {
                search_instance.popped += 1;
            }

            if let Some(o) = search_instance.root_history.get(&Root(next.root)) {
                if o < &next.f {
                    #[cfg(feature = "verbose")]
                    println!("node is dominated!");
                    #[cfg(feature = "stats")]
                    {
                        search_instance.nodes_pruned_post_pop += 1;
                    }
                    continue;
                }
            }
            //     _ => (),
            // }

            if next.polygon_to == ending_polygon as isize {
                #[cfg(feature = "stats")]
                {
                    eprintln!(
                        "index;micros;successor_calls;generated;pushed;popped;pruned_post_pop;length",
                    );
                    eprintln!(
                        "0;{};{};{};{};{};{};{}",
                        search_instance.start.elapsed().as_secs_f32() * 1_000_000.0,
                        search_instance.successors_called,
                        search_instance.nodes_generated,
                        search_instance.pushed,
                        search_instance.popped,
                        search_instance.nodes_pruned_post_pop,
                        next.f + next.g,
                    );
                }
                let mut path = next
                    .path
                    .split_first()
                    .map(|(_, p)| p)
                    .unwrap_or(&[])
                    .to_vec();
                if next.root != from {
                    path.push(next.root);
                }
                if let Some(turn) = turning_point(next.root, to, next.interval) {
                    path.push(turn);
                }
                let complete = next.polygon_to == ending_polygon as isize;
                if complete {
                    path.push(to);
                }
                return Path {
                    path,
                    len: next.f + next.g,
                    complete,
                };
            }
            search_instance.successors(next);
        }
        #[cfg(feature = "stats")]
        eprintln!(
            "{:?} / {:?} / {:?} / {:?}",
            search_instance.successors_called,
            search_instance.nodes_generated,
            search_instance.pushed,
            search_instance.popped
        );
        Path {
            path: vec![],
            len: -1.0,
            complete: false,
        }
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[cfg(test)]
    fn successors(&self, node: SearchNode, to: Vec2) -> Vec<SearchNode> {
        let mut search_instance = SearchInstance {
            queue: BinaryHeap::new(),
            node_buffer: Vec::new(),
            root_history: HashMap::new(),
            to,
            polygon_to: self.get_point_location(to) as isize,
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
    fn edges_between(&self, node: &SearchNode) -> Vec<Successor> {
        let search_instance = SearchInstance {
            queue: BinaryHeap::new(),
            node_buffer: Vec::new(),
            root_history: HashMap::new(),
            to: Vec2::new(0.0, 0.0),
            polygon_to: self.get_point_location(Vec2::new(0.0, 0.0)) as isize,
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
        search_instance.edges_between(node)
    }
}

impl<'m> SearchInstance<'m> {
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    fn edges_between(&self, node: &SearchNode) -> Vec<Successor> {
        let mut successors = vec![];

        let polygon = self.mesh.polygons.get(node.polygon_to as usize).unwrap();

        if node.interval.0.distance(node.root) < 1.0e-5
            || node.interval.1.distance(node.root) < 1.0e-5
            || node.root.side(node.interval) == EdgeSide::Edge
        {
            // println!("collinear");
            // TODO: possible optimisation
        }
        if polygon.vertices.len() == 3 {
            // println!("triangle");
            // TODO: possible optimisation
        }

        let right_index = {
            let mut temp = 0;
            while polygon.vertices[temp] != node.edge.1 {
                temp += 1;
            }
            temp + 1
        };
        let left_index = polygon.vertices.len() + right_index - 1 - 1;

        let mut ty = SuccessorType::RightNonObservable;
        for edge in &polygon.double_edges_index()[right_index..=left_index] {
            let start = if let Some(vertex) = self.mesh.vertices.get(edge.0) {
                vertex
            } else {
                continue;
            };
            let end = if let Some(vertex) = self.mesh.vertices.get(edge.1) {
                vertex
            } else {
                continue;
            };
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
                        if intersect.distance(start_point) > 1.0e-3
                            && intersect.distance(end_point) > 1.0e-3
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
                        if intersect.distance(end_point) > 1.0e-3 {
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
            if end_point.side((node.root, node.interval.1)) == EdgeSide::Left {
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

                    if intersect.distance(end_point) > 1.0e-3 {
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
            match end_point.side((node.root, node.interval.1)) {
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
    fn add_node(
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
            polygon_from: node.polygon_to as isize,
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
    fn flush_nodes(&mut self) {
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
    fn successors(&mut self, mut node: SearchNode) {
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
            for successor in self.edges_between(&node) {
                let start = self.mesh.vertices.get(successor.edge.0).unwrap();
                let end = self.mesh.vertices.get(successor.edge.1).unwrap();

                #[cfg(debug_assertions)]
                if self.debug {
                    println!("v {:?}", successor);
                }

                let mut other_side = isize::MAX;
                // find the polygon at the other side of this edge
                for i in &start.polygons {
                    if *i != -1 && *i != node.polygon_to as isize && end.polygons.contains(i) {
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

                const EPSILON: f32 = 1.0e-5;
                let root = match successor.ty {
                    SuccessorType::RightNonObservable => {
                        if successor.interval.0.distance(start.coords) > EPSILON {
                            #[cfg(debug_assertions)]
                            if self.debug {
                                println!("x non observable on an intersection");
                            }
                            continue;
                        }
                        let vertex = self.mesh.vertices.get(node.edge.0).unwrap();
                        if vertex.is_corner && vertex.coords.distance(node.interval.0) < EPSILON {
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
                        if successor.interval.1.distance(end.coords) > EPSILON {
                            #[cfg(debug_assertions)]
                            if self.debug {
                                println!("x non observable on an intersection");
                            }
                            continue;
                        }
                        let vertex = self.mesh.vertices.get(node.edge.1).unwrap();
                        if vertex.is_corner && vertex.coords.distance(node.interval.1) < EPSILON {
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

impl Mesh {
    pub fn point_in_mesh(&self, point: Vec2) -> bool {
        self.get_point_location(point) != usize::MAX
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    fn get_point_location(&self, point: Vec2) -> usize {
        let delta = 0.1;
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
        .map(|delta| {
            if self.baked_polygons.is_empty() {
                self.get_point_location_unit(point + *delta)
            } else {
                self.get_point_location_unit_baked(point + *delta)
            }
        })
        .find(|poly| *poly != usize::MAX)
        .unwrap_or(usize::MAX)
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    fn get_point_location_unit(&self, point: Vec2) -> usize {
        for (i, polygon) in self.polygons.iter().enumerate() {
            if self.point_in_polygon(point, polygon) {
                return i;
            }
        }
        usize::MAX
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    fn get_point_location_unit_baked(&self, point: Vec2) -> usize {
        let mut visited = HashSet::new();
        let mut peekable = self.baked_polygons.iter().peekable();
        while let Some(baked) = peekable.next() {
            if let Some((next, _)) = peekable.peek() {
                if **next > (point.x * PRECISION) as i32 {
                    for i in baked.1.iter() {
                        if visited.insert(i) && self.point_in_polygon(point, &self.polygons[*i]) {
                            return *i;
                        }
                    }
                }
            }
            if *baked.0 > (point.x * PRECISION) as i32 {
                break;
            }
        }
        usize::MAX
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    fn point_in_polygon(&self, point: Vec2, polygon: &Polygon) -> bool {
        let mut edged = false;
        for edge in polygon.edges_index() {
            if edge.0.max(edge.1) >= self.vertices.len() {
                return false;
            }
            edged = true;
            let last = self.vertices[edge.0].coords;
            let next = self.vertices[edge.1].coords;

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
    edge: (usize, usize),
    polygon_from: isize,
    polygon_to: isize,
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
    use indexmap::IndexMap;

    use crate::{helpers::*, Mesh, Path, Polygon, SearchNode, Vertex};

    fn mesh_u_grid() -> Mesh {
        Mesh {
            vertices: vec![
                Vertex::from_coords(0, 0, vec![0, -1]),
                Vertex::from_coords(1, 0, vec![0, 1, -1]),
                Vertex::from_coords(2, 0, vec![1, 2, -1]),
                Vertex::from_coords(3, 0, vec![2, -1]),
                Vertex::from_coords(0, 1, vec![3, 0, -1]),
                Vertex::from_coords(1, 1, vec![3, 1, 0, -1]),
                Vertex::from_coords(2, 1, vec![4, 2, 1, -1]),
                Vertex::from_coords(3, 1, vec![4, 2, -1]),
                Vertex::from_coords(0, 2, vec![3, -1]),
                Vertex::from_coords(1, 2, vec![3, -1]),
                Vertex::from_coords(2, 2, vec![4, -1]),
                Vertex::from_coords(3, 2, vec![4, -1]),
            ],
            polygons: vec![
                Polygon::using(4, vec![0, 1, 5, 4, -1, 1, 3, -1]),
                Polygon::using(4, vec![1, 2, 6, 5, -1, 2, -1, 0]),
                Polygon::using(4, vec![2, 3, 7, 6, -1, -1, 4, 1]),
                Polygon::using(4, vec![4, 5, 9, 8, 0, -1, -1, -1]),
                Polygon::using(4, vec![6, 7, 11, 10, 2, -1, -1, -1]),
            ],
            baked_polygons: IndexMap::default(),
        }
    }

    #[test]
    fn point_in_polygon() {
        let mesh = mesh_u_grid();
        assert_eq!(mesh.get_point_location(Vec2::new(0.5, 0.5)), 0);
        assert_eq!(mesh.get_point_location(Vec2::new(1.5, 0.5)), 1);
        assert_eq!(mesh.get_point_location(Vec2::new(0.5, 1.5)), 3);
        assert_eq!(mesh.get_point_location(Vec2::new(1.5, 1.5)), usize::MAX);
        assert_eq!(mesh.get_point_location(Vec2::new(2.5, 1.5)), 4);
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
            polygon_from: mesh.get_point_location(from) as isize,
            polygon_to: 1,
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
        assert_eq!(
            successors[0].interval,
            (Vec2::new(2.0, 0.0), Vec2::new(2.0, 1.0))
        );
        assert_eq!(successors[0].edge, (2, 6));

        assert_eq!(successors[0].path, Vec::<Vec2>::new());

        assert_eq!(
            mesh.path(from, to),
            Path {
                path: vec![to],
                len: from.distance(to),
                complete: true,
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
            polygon_from: mesh.get_point_location(from) as isize,
            polygon_to: 1,
            f: 0.0,
            g: from.distance(to),
        };
        let successors = mesh.successors(search_node, to);
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].f, 0.0);
        assert_eq!(successors[0].g, to.distance(from));
        assert_eq!(successors[0].polygon_from, 1);
        assert_eq!(successors[0].polygon_to, 0);
        assert_eq!(
            successors[0].interval,
            (Vec2::new(1.0, 1.0), Vec2::new(1.0, 0.0))
        );
        assert_eq!(successors[0].edge, (5, 1));
        assert_eq!(successors[0].path, Vec::<Vec2>::new());

        assert_eq!(
            mesh.path(from, to),
            Path {
                path: vec![to],
                len: from.distance(to),
                complete: true,
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
            polygon_from: mesh.get_point_location(from) as isize,
            polygon_to: 0,
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
        assert_eq!(successors[0].polygon_from, 2);
        assert_eq!(successors[0].polygon_to, 4);
        assert_eq!(
            successors[0].interval,
            (Vec2::new(3.0, 1.0), Vec2::new(2.0, 1.0))
        );
        assert_eq!(successors[0].edge, (7, 6));
        assert_eq!(successors[0].path, vec![from, Vec2::new(1.0, 1.0)]);

        assert_eq!(
            mesh.path(from, to),
            Path {
                path: vec![Vec2::new(1.0, 1.0), Vec2::new(2.0, 1.0), to],
                len: from.distance(Vec2::new(1.0, 1.0))
                    + Vec2::new(1.0, 1.0).distance(Vec2::new(2.0, 1.0))
                    + Vec2::new(2.0, 1.0).distance(to),
                complete: true,
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

            polygon_from: 0,
            polygon_to: 1,
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
        assert_eq!(successors[0].polygon_from, 2);
        assert_eq!(successors[0].polygon_to, 4);
        assert_eq!(
            successors[0].interval,
            (Vec2::new(3.0, 1.0), Vec2::new(2.0, 1.0))
        );
        assert_eq!(successors[0].edge, (7, 6));
        assert_eq!(successors[0].path, vec![from, Vec2::new(1.0, 1.0)]);

        assert_eq!(
            mesh.path(from, to),
            Path {
                path: vec![Vec2::new(1.0, 1.0), Vec2::new(2.0, 1.0), to],
                len: from.distance(Vec2::new(1.0, 1.0))
                    + Vec2::new(1.0, 1.0).distance(Vec2::new(2.0, 1.0))
                    + Vec2::new(2.0, 1.0).distance(to),
                complete: true,
            }
        );
    }

    fn mesh_from_paper() -> Mesh {
        Mesh {
            vertices: vec![
                Vertex::from_coords(0, 6, vec![0, -1]),           // 0
                Vertex::from_coords(2, 5, vec![0, -1, 2]),        // 1
                Vertex::from_coords(5, 7, vec![0, 2, -1]),        // 2
                Vertex::from_coords(5, 8, vec![0, -1]),           // 3
                Vertex::from_coords(0, 8, vec![0, -1]),           // 4
                Vertex::from_coords(1, 4, vec![1, -1]),           // 5
                Vertex::from_coords(2, 1, vec![1, -1]),           // 6
                Vertex::from_coords(4, 1, vec![1, -1]),           // 7
                Vertex::from_coords(4, 2, vec![1, -1, 2]),        // 8
                Vertex::from_coords(2, 4, vec![1, 2, -1]),        // 9
                Vertex::from_coords(7, 4, vec![2, -1, 4]),        // 10
                Vertex::from_coords(10, 7, vec![2, 4, 6, -1, 3]), // 11
                Vertex::from_coords(7, 7, vec![2, 3, -1]),        // 12
                Vertex::from_coords(11, 8, vec![3, -1]),          // 13
                Vertex::from_coords(7, 8, vec![3, -1]),           // 14
                Vertex::from_coords(7, 0, vec![5, 4, -1]),        // 15
                Vertex::from_coords(11, 3, vec![4, 5, -1]),       // 16
                Vertex::from_coords(11, 5, vec![4, -1, 6]),       // 17
                Vertex::from_coords(12, 0, vec![5, -1]),          // 18
                Vertex::from_coords(12, 3, vec![5, -1]),          // 19
                Vertex::from_coords(13, 5, vec![6, -1]),          // 20
                Vertex::from_coords(13, 7, vec![6, -1]),          // 21
                Vertex::from_coords(1, 3, vec![1, -1]),           // 22
            ],
            polygons: vec![
                Polygon::using(5, vec![0, 1, 2, 3, 4, -1, -1, 2, -1, -1]),
                Polygon::using(6, vec![5, 22, 6, 7, 8, 9, -1, -1, -1, -1, 2, -1]),
                Polygon::using(7, vec![1, 9, 8, 10, 11, 12, 2, -1, 1, -1, 4, 3, -1, 0]),
                Polygon::using(4, vec![12, 11, 13, 14, 2, -1, -1, -1]),
                Polygon::using(5, vec![10, 15, 16, 17, 11, -1, 5, -1, 6, 2]),
                Polygon::using(4, vec![15, 18, 19, 16, -1, -1, -1, 4]),
                Polygon::using(4, vec![11, 17, 20, 21, 4, -1, -1, -1]),
            ],
            baked_polygons: IndexMap::default(),
        }
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
            polygon_from: mesh.get_point_location(from) as isize,
            polygon_to: 4,
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
        assert_eq!(successors[1].polygon_from, 4);
        assert_eq!(successors[1].polygon_to, 2);
        assert_eq!(
            successors[1].interval,
            (Vec2::new(10.0, 7.0), Vec2::new(9.75, 6.75))
        );
        assert_eq!(successors[1].edge, (11, 10));
        assert_eq!(successors[1].path, vec![from]);

        assert_eq!(successors[0].root, from);
        assert_eq!(successors[0].f, 0.0);
        assert_eq!(successors[0].g, from.distance(to));
        assert_eq!(successors[0].polygon_from, 4);
        assert_eq!(successors[0].polygon_to, 2);
        assert_eq!(
            successors[0].interval,
            (Vec2::new(9.75, 6.75), Vec2::new(7.0, 4.0))
        );
        assert_eq!(successors[0].edge, (11, 10));
        assert_eq!(successors[0].path, Vec::<Vec2>::new());

        assert_eq!(mesh.path(from, to).len, from.distance(to));
        assert_eq!(mesh.path(from, to).path, vec![to]);
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
            polygon_from: mesh.get_point_location(from) as isize,
            polygon_to: 4,
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
        assert_eq!(successors[0].polygon_from, 4);
        assert_eq!(successors[0].polygon_to, 6);
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
        assert_eq!(successors[1].polygon_from, 4);
        assert_eq!(successors[1].polygon_to, 2);
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
        assert_eq!(successors[2].polygon_from, 4);
        assert_eq!(successors[2].polygon_to, 2);
        assert_eq!(
            successors[2].interval,
            (Vec2::new(9.75, 6.75), Vec2::new(7.0, 4.0))
        );
        assert_eq!(successors[2].edge, (11, 10));
        assert_eq!(successors[2].path, Vec::<Vec2>::new());

        assert_delta!(
            mesh.path(from, to).len,
            from.distance(Vec2::new(11.0, 3.0))
                + Vec2::new(11.0, 3.0).distance(Vec2::new(11.0, 5.0))
                + Vec2::new(11.0, 5.0).distance(to)
        );
        assert_eq!(
            mesh.path(from, to).path,
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
            polygon_from: mesh.get_point_location(from) as isize,
            polygon_to: 4,
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
        assert_eq!(successors[1].polygon_from, 4);
        assert_eq!(successors[1].polygon_to, 2);
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
        assert_eq!(successors[0].polygon_from, 4);
        assert_eq!(successors[0].polygon_to, 2);
        assert_eq!(
            successors[0].interval,
            (Vec2::new(9.75, 6.75), Vec2::new(7.0, 4.0))
        );
        assert_eq!(successors[0].edge, (11, 10));
        assert_eq!(successors[0].path, Vec::<Vec2>::new());

        assert_delta!(
            mesh.path(from, to).len,
            from.distance(Vec2::new(7.0, 4.0)) + Vec2::new(7.0, 4.0).distance(to)
        );
        assert_eq!(mesh.path(from, to).path, vec![Vec2::new(7.0, 4.0), to]);
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
            polygon_from: mesh.get_point_location(from) as isize,
            polygon_to: 4,
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
        assert_eq!(successors[1].polygon_from, 4);
        assert_eq!(successors[1].polygon_to, 2);
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
        assert_eq!(successors[0].polygon_from, 4);
        assert_eq!(successors[0].polygon_to, 2);
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
            mesh.path(from, to).len,
            from.distance(Vec2::new(7.0, 4.0))
                + Vec2::new(7.0, 4.0).distance(Vec2::new(4.0, 2.0))
                + Vec2::new(4.0, 2.0).distance(to)
        );

        assert_eq!(
            mesh.path(from, to).path,
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
            polygon_from: mesh.get_point_location(from) as isize,
            polygon_to: 4,
            f: 0.0,
            g: from.distance(to),
        };

        let successors = mesh.edges_between(&search_node);

        for successor in &successors {
            println!("{:?}", successor);
        }

        println!("=========================");

        let search_node = SearchNode {
            path: vec![],
            root: from,
            interval: (Vec2::new(9.75, 6.75), Vec2::new(7.0, 4.0)),
            edge: (11, 10),
            polygon_from: 4,
            polygon_to: 2,
            f: 0.0,
            g: from.distance(to),
        };

        let successors = mesh.edges_between(&search_node);

        for successor in &successors {
            println!("{:?}", successor);
        }

        println!("=========================");

        let search_node = SearchNode {
            path: vec![],
            root: Vec2::new(11.0, 3.0),
            interval: (Vec2::new(10.0, 7.0), Vec2::new(7.0, 4.0)),
            edge: (11, 10),
            polygon_from: 4,
            polygon_to: 2,
            f: 0.0,
            g: from.distance(to),
        };

        let successors = mesh.edges_between(&search_node);

        for successor in &successors {
            println!("{:?}", successor);
        }

        // assert!(false);
    }

    #[test]
    fn edges_between_simple_u() {
        let mesh = mesh_u_grid();

        let search_node = SearchNode {
            path: vec![],
            root: Vec2::new(0.0, 0.0),
            interval: (Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0)),
            edge: (1, 5),
            polygon_from: 0,
            polygon_to: 1,
            f: 0.0,
            g: 1.0,
        };

        let successors = mesh.edges_between(&search_node);

        for successor in &successors {
            println!("{:?}", successor);
        }
        // assert!(false);
    }
}
