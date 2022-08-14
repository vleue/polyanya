use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    fmt::{self, Display},
    hash::Hash,
    io,
    io::BufRead,
};

use helpers::{distance_between, heuristic, on_side, EPSILON};

use crate::helpers::{line_intersect_segment, on_segment, turning_on};

mod helpers;

#[derive(Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub polygons: Vec<isize>,
}

impl Vertex {
    pub fn new(x: u32, y: u32, poly: Vec<isize>) -> Self {
        Vertex {
            x: x as f32,
            y: y as f32,
            polygons: poly,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Path {
    pub len: f32,
    pub path: Vec<[f32; 2]>,
}

#[derive(Debug)]
pub struct Polygon {
    pub vertices: Vec<usize>,
    pub neighbours: Vec<isize>,
}

impl Polygon {
    pub fn new(nb: usize, data: Vec<isize>) -> Self {
        assert!(data.len() == nb * 2);
        let (vertices, neighbours) = data.split_at(nb);
        let vertices = vertices.iter().copied().map(|v| v as usize).collect();
        let neighbours = neighbours.to_vec();
        Polygon {
            vertices,
            neighbours,
        }
    }

    pub fn edges_index(&self) -> Vec<[usize; 2]> {
        let mut edges = vec![];
        let mut last = self.vertices[0];
        for vertex in self.vertices.iter().skip(1) {
            edges.push([last, *vertex]);
            last = *vertex;
        }
        edges.push([last, self.vertices[0]]);
        edges
    }
}

#[derive(Debug, Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub polygons: Vec<Polygon>,
}

struct Root([f32; 2]);
impl PartialEq for Root {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Root {}
impl Hash for Root {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ((self.0[0] * 10000.0) as i32).hash(state);
        ((self.0[1] * 10000.0) as i32).hash(state);
        state.finish();
    }
}

impl Mesh {
    pub fn from_file(path: &str) -> Mesh {
        let file = std::fs::File::open(path).unwrap();
        let mut mesh = Mesh::default();
        let mut nb_vertices = 0;
        let mut nb_polygons = 0;
        let mut phase = 0;
        for line in io::BufReader::new(file).lines() {
            let line: String = line.unwrap();
            if phase == 0 {
                if line == "mesh" {
                    continue;
                } else if line == "2" {
                    continue;
                } else {
                    (nb_vertices, nb_polygons) = line
                        .split_once(" ")
                        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                        .unwrap();
                    phase = 1;
                    continue;
                }
            }
            if phase == 1 {
                if nb_vertices > 0 {
                    nb_vertices -= 1;
                    let mut values = line.split(" ");
                    let x = values.next().unwrap().parse().unwrap();
                    let y = values.next().unwrap().parse().unwrap();
                    let _ = values.next();
                    let vertex = Vertex::new(x, y, values.map(|v| v.parse().unwrap()).collect());
                    mesh.vertices.push(vertex);
                } else {
                    phase = 2;
                }
            }
            if phase == 2 {
                if nb_polygons > 0 {
                    nb_polygons -= 1;
                    let mut values = line.split(" ");
                    let n = values.next().unwrap().parse().unwrap();
                    let polygon = Polygon::new(n, values.map(|v| v.parse().unwrap()).collect());
                    mesh.polygons.push(polygon)
                } else {
                    panic!("unexpected line");
                }
            }
        }
        mesh
    }
}

impl Mesh {
    pub fn path(&self, from: [f32; 2], to: [f32; 2]) -> Path {
        let starting_polygon_index = self.point_in_polygon(from);
        let starting_polygon = self.polygons.get(starting_polygon_index).unwrap();
        let ending_polygon = self.point_in_polygon(to);

        if starting_polygon_index == ending_polygon {
            return Path {
                len: distance_between(from, to),
                path: vec![to],
            };
        }

        let mut root_history = HashMap::new();
        root_history.insert(Root(from), 0.0);

        let mut to_add = vec![];
        for edge in starting_polygon.edges_index() {
            let start = self.vertices.get(edge[0]).unwrap();
            let end = self.vertices.get(edge[1]).unwrap();

            let mut other_side = isize::MAX;
            for i in &start.polygons {
                if *i != -1 && *i != starting_polygon_index as isize && end.polygons.contains(i) {
                    other_side = *i;
                }
            }

            // prune edges that don't have a polygon on the other side: cul de sac pruning
            if other_side == isize::MAX {
                continue;
            }

            to_add.push(SearchNode {
                path: vec![],
                r: from,
                i: [[start.x, start.y], [end.x, end.y]],
                polygon_from: starting_polygon_index as isize,
                polygon_to: other_side,
                f: 0.0,
                g: heuristic(from, to, [[start.x, start.y], [end.x, end.y]]),
            })
        }

        let mut queue = BinaryHeap::new();
        for node in to_add {
            queue.push(node);
        }

        while let Some(next) = queue.pop() {
            println!("popped off: {}", next);
            if next.polygon_to == ending_polygon as isize {
                eprintln!("found path: {:?}", next);
                let mut path = next
                    .path
                    .split_first()
                    .map(|(_, p)| p)
                    .unwrap_or(&[])
                    .to_vec();
                if next.r != from {
                    path.push(next.r);
                }
                if let Some(turn) = turning_on(next.r, to, next.i) {
                    path.push(turn);
                }
                path.push(to);
                return Path {
                    path,
                    len: next.f + next.g,
                };
            }
            let to_add = self.successors(next, to);
            for node in to_add {
                match root_history.entry(Root(node.r)) {
                    Entry::Occupied(mut o) => {
                        if o.get() < &node.f {
                            continue;
                        } else {
                            o.insert(node.f);
                        }
                    }
                    Entry::Vacant(v) => {
                        v.insert(node.f);
                    }
                }
                println!("   pushing: {}", node);
                queue.push(node);
            }
        }
        Path {
            path: vec![],
            len: -1.0,
        }
    }

    fn successors(&self, node: SearchNode, to: [f32; 2]) -> Vec<SearchNode> {
        let to_polygon = self.polygons.get(node.polygon_to as usize).unwrap();

        let mut found_end = false;

        let mut to_add = vec![];
        let mut first_intersect = None;
        let mut second_intersect = None;

        let mut add_node = |root: [f32; 2], other_side: isize, start: [f32; 2], end: [f32; 2]| {
            // prune edges that don't have a polygon on the other side: cul de sac pruning
            if other_side == isize::MAX {
                return;
            }

            let mut path = node.path.clone();
            if root != node.r {
                path.push(node.r);
            }

            let new_node = SearchNode {
                path,
                r: root,
                i: [start, end],
                polygon_from: node.polygon_to as isize,
                polygon_to: other_side,
                f: node.f + distance_between(node.r, root),
                g: heuristic(root, to, [start, end]),
            };

            to_add.push(new_node);
        };

        for edge in to_polygon
            .edges_index()
            .iter()
            .chain(to_polygon.edges_index().iter())
        {
            let mut new_r = None;
            let start = self.vertices.get(edge[0]).unwrap();
            let end = self.vertices.get(edge[1]).unwrap();
            let mut found_end_this_turn = false;

            // continue until we get to the interval end
            if !found_end
                && on_segment(node.i[0], [[start.x, start.y], [end.x, end.y]])
                && node.i[0] != [end.x, end.y]
            {
                found_end = true;
                found_end_this_turn = true;
            }
            if !found_end {
                continue;
            }

            let mut other_side = isize::MAX;
            // find the polygon at the other side of this edge
            for i in &start.polygons {
                if *i != -1 && *i != node.polygon_to as isize && end.polygons.contains(i) {
                    other_side = *i;
                }
            }

            // break once we reached the interval start
            if found_end
                && !found_end_this_turn
                && on_segment(node.i[1], [[start.x, start.y], [end.x, end.y]])
                && node.i[1] != [end.x, end.y]
            {
                break;
            }

            // this goes back to the current polygon, ignore
            if other_side == node.polygon_from {
                continue;
            }

            if (node.i[0] != [start.x, start.y]
                || on_side([end.x, end.y], [node.r, node.i[0]]) == EdgeSide::Left)
                && (node.i[1] != [end.x, end.y]
                    || on_side([start.x, start.y], [node.r, node.i[0]]) == EdgeSide::Right)
            {
                if let Some(intersect1) = line_intersect_segment(
                    [node.r, node.i[0]],
                    [[start.x, start.y], [end.x, end.y]],
                ) {
                    if let Some(intersect2) = line_intersect_segment(
                        [node.r, node.i[1]],
                        [[start.x, start.y], [end.x, end.y]],
                    ) {
                        first_intersect = Some(intersect1);
                        second_intersect = Some(intersect2);
                        if intersect1 != [start.x, start.y] {
                            if let Some(extra_r) = to_polygon
                                .vertices
                                .iter()
                                .flat_map(|v| self.vertices.get(*v))
                                .find(|v| [v.x, v.y] == node.i[0])
                                .and_then(|v| v.polygons.contains(&-1).then(|| [v.x, v.y]))
                            {
                                add_node(extra_r, other_side, [start.x, start.y], intersect1);
                            }
                        }
                        add_node(node.r, other_side, intersect1, intersect2);
                        if intersect2 != [start.x, start.y] {
                            if let Some(extra_r) = to_polygon
                                .vertices
                                .iter()
                                .flat_map(|v| self.vertices.get(*v))
                                .find(|v| [v.x, v.y] == node.i[1])
                                .and_then(|v| v.polygons.contains(&-1).then(|| [v.x, v.y]))
                            {
                                add_node(extra_r, other_side, intersect2, [end.x, end.y]);
                            }
                        }
                        continue;
                    }
                }
            }

            if node.i[0] != [start.x, start.y]
                || on_side([end.x, end.y], [node.r, node.i[0]]) == EdgeSide::Left
            {
                if let Some(intersect) = line_intersect_segment(
                    [node.r, node.i[0]],
                    [[start.x, start.y], [end.x, end.y]],
                ) {
                    if intersect != [end.x, end.y] {
                        // if intersect != [end.x, end.y] && intersect != [start.x, start.y] {
                        first_intersect = Some(intersect);
                        if intersect != [start.x, start.y] {
                            if let Some(extra_r) = to_polygon
                                .vertices
                                .iter()
                                .flat_map(|v| self.vertices.get(*v))
                                .find(|v| [v.x, v.y] == node.i[0])
                                .and_then(|v| v.polygons.contains(&-1).then(|| [v.x, v.y]))
                            {
                                add_node(extra_r, other_side, [start.x, start.y], intersect);
                            }
                        }
                        add_node(node.r, other_side, intersect, [end.x, end.y]);
                        continue;
                    }
                }
            }

            if node.i[1] != [end.x, end.y]
                || on_side([start.x, start.y], [node.r, node.i[0]]) == EdgeSide::Right
            {
                if let Some(intersect) = line_intersect_segment(
                    [node.r, node.i[1]],
                    [[start.x, start.y], [end.x, end.y]],
                ) {
                    if intersect != [start.x, start.y] {
                        second_intersect = Some(intersect);
                        add_node(node.r, other_side, [start.x, start.y], intersect);
                        if intersect != [end.x, end.y] {
                            if let Some(extra_r) = to_polygon
                                .vertices
                                .iter()
                                .flat_map(|v| self.vertices.get(*v))
                                .find(|v| [v.x, v.y] == node.i[1])
                                .and_then(|v| v.polygons.contains(&-1).then(|| [v.x, v.y]))
                            {
                                add_node(extra_r, other_side, intersect, [end.x, end.y]);
                            }
                        }
                        continue;
                    }
                }
            }

            if first_intersect.is_none() && second_intersect.is_none() {
                new_r = Some(node.i[0]);
            }

            if first_intersect.is_some() && second_intersect.is_some() {
                new_r = Some(node.i[1]);
            }

            add_node(
                new_r.unwrap_or(node.r),
                other_side,
                [start.x, start.y],
                [end.x, end.y],
            );
        }

        to_add
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum EdgeSide {
    Left,
    Right,
    Edge,
}

impl Mesh {
    pub fn point_in_polygon(&self, point: [f32; 2]) -> usize {
        [
            [0.0, 0.0],
            [EPSILON, 0.0],
            [EPSILON, EPSILON],
            [0.0, EPSILON],
            [-EPSILON, EPSILON],
            [-EPSILON, 0.0],
            [-EPSILON, -EPSILON],
            [0.0, -EPSILON],
            [EPSILON, -EPSILON],
        ]
        .iter()
        .map(|delta| self.point_in_polygon_unit([point[0] + delta[0], point[1] + delta[1]]))
        .find(|poly| *poly != usize::MAX)
        .unwrap_or(usize::MAX)
    }

    pub fn point_in_polygon_unit(&self, point: [f32; 2]) -> usize {
        'polygons: for (i, polygon) in self.polygons.iter().enumerate() {
            for edge in polygon.edges_index() {
                let last = self.vertices.get(edge[0]).unwrap();
                let next = self.vertices.get(edge[1]).unwrap();
                let current_side = on_side(point, [[last.x, last.y], [next.x, next.y]]);
                if on_segment(point, [[last.x, last.y], [next.x, next.y]]) {
                    return i;
                }
                if current_side != EdgeSide::Left {
                    continue 'polygons;
                }
            }
            return i;
        }
        usize::MAX
    }
}

#[derive(PartialEq, Debug)]
pub struct SearchNode {
    pub path: Vec<[f32; 2]>,
    pub r: [f32; 2],
    pub i: [[f32; 2]; 2],
    pub polygon_from: isize,
    pub polygon_to: isize,
    pub f: f32,
    pub g: f32,
}

impl Display for SearchNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("root=({}, {}); ", self.r[0], self.r[1]))?;
        f.write_str(&format!("left=({}, {}); ", self.i[1][0], self.i[1][1]))?;
        f.write_str(&format!("right=({}, {}); ", self.i[0][0], self.i[0][1]))?;
        f.write_str(&format!("f={}, g={} ", self.g, self.f))?;
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
            Ordering::Equal => Ordering::Equal,
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

    use crate::{
        helpers::{distance_between, mirror},
        Mesh, Path, Polygon, SearchNode, Vertex,
    };

    fn mesh_u_grid() -> Mesh {
        Mesh {
            vertices: vec![
                Vertex::new(0, 0, vec![0, -1]),
                Vertex::new(1, 0, vec![0, 1, -1]),
                Vertex::new(2, 0, vec![1, 2, -1]),
                Vertex::new(3, 0, vec![2, -1]),
                Vertex::new(0, 1, vec![3, 0, -1]),
                Vertex::new(1, 1, vec![3, 1, 0, -1]),
                Vertex::new(2, 1, vec![4, 2, 1, -1]),
                Vertex::new(3, 1, vec![4, 2, -1]),
                Vertex::new(0, 2, vec![3, -1]),
                Vertex::new(1, 2, vec![3, -1]),
                Vertex::new(2, 2, vec![4, -1]),
                Vertex::new(3, 2, vec![4, -1]),
            ],
            polygons: vec![
                Polygon::new(4, vec![0, 1, 5, 4, -1, 1, 3, -1]),
                Polygon::new(4, vec![1, 2, 6, 5, -1, 2, -1, 0]),
                Polygon::new(4, vec![2, 3, 7, 6, -1, -1, 4, 1]),
                Polygon::new(4, vec![4, 5, 9, 8, 0, -1, -1, -1]),
                Polygon::new(4, vec![6, 7, 11, 10, 2, -1, -1, -1]),
            ],
        }
    }

    #[test]
    fn point_in_polygon() {
        let mesh = mesh_u_grid();
        assert_eq!(mesh.point_in_polygon([0.5, 0.5]), 0);
        assert_eq!(mesh.point_in_polygon([1.5, 0.5]), 1);
        assert_eq!(mesh.point_in_polygon([0.5, 1.5]), 3);
        assert_eq!(mesh.point_in_polygon([1.5, 1.5]), usize::MAX);
        assert_eq!(mesh.point_in_polygon([2.5, 1.5]), 4);
    }

    #[test]
    fn successors_straight_line_ahead() {
        let mesh = mesh_u_grid();

        let from = [0.1, 0.1];
        let to = [2.9, 0.9];
        let search_node = SearchNode {
            path: vec![],
            r: from,
            i: [[1.0, 0.0], [1.0, 1.0]],
            polygon_from: mesh.point_in_polygon(from) as isize,
            polygon_to: 1,
            f: 0.0,
            g: distance_between(from, to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].r, from);
        assert_eq!(successors[0].f, 0.0);
        assert_eq!(successors[0].g, distance_between(from, to));
        assert_eq!(successors[0].polygon_from, 1);
        assert_eq!(successors[0].polygon_to, 2);
        assert_eq!(successors[0].i, [[2.0, 0.0], [2.0, 1.0]]);
        assert_eq!(successors[0].path, Vec::<[f32; 2]>::new());

        assert_eq!(
            mesh.path(from, to),
            Path {
                path: vec![to],
                len: distance_between(from, to)
            }
        );
    }

    #[test]
    fn successors_straight_line_reversed() {
        let mesh = mesh_u_grid();

        let to = [0.1, 0.1];
        let from = [2.9, 0.9];
        let search_node = SearchNode {
            path: vec![],
            r: from,
            i: [[2.0, 1.0], [2.0, 0.0]],
            polygon_from: mesh.point_in_polygon(from) as isize,
            polygon_to: 1,
            f: 0.0,
            g: distance_between(from, to),
        };
        let successors = mesh.successors(search_node, to);
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].r, from);
        assert_eq!(successors[0].f, 0.0);
        assert_eq!(successors[0].g, distance_between(to, from));
        assert_eq!(successors[0].polygon_from, 1);
        assert_eq!(successors[0].polygon_to, 0);
        assert_eq!(successors[0].i, [[1.0, 1.0], [1.0, 0.0]]);
        assert_eq!(successors[0].path, Vec::<[f32; 2]>::new());

        assert_eq!(
            mesh.path(from, to),
            Path {
                path: vec![to],
                len: distance_between(from, to)
            }
        );
    }

    #[test]
    fn successors_corner_first_step() {
        let mesh = mesh_u_grid();

        let from = [0.1, 1.9];
        let to = [2.1, 1.9];
        let search_node = SearchNode {
            path: vec![],
            r: from,
            i: [[0.0, 1.0], [1.0, 1.0]],
            polygon_from: mesh.point_in_polygon(from) as isize,
            polygon_to: 0,
            f: 0.0,
            g: distance_between(from, to),
        };
        let successors = mesh.successors(search_node, to);
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].r, from);
        assert_eq!(successors[0].f, 0.0);
        assert_eq!(
            successors[0].g,
            distance_between(from, [1.0, 1.0]) + distance_between([1.0, 1.0], to)
        );
        assert_eq!(successors[0].polygon_from, 0);
        assert_eq!(successors[0].polygon_to, 1);
        assert_eq!(successors[0].i, [[1.0, 0.0], [1.0, 1.0]]);
        assert_eq!(successors[0].path, Vec::<[f32; 2]>::new());

        assert_eq!(
            mesh.path(from, to),
            Path {
                path: vec![[1.0, 1.0], [2.0, 1.0], to],
                len: distance_between(from, [1.0, 1.0])
                    + distance_between([1.0, 1.0], [2.0, 1.0])
                    + distance_between([2.0, 1.0], to)
            }
        );
    }

    #[test]
    fn successors_corner_observable_second_step() {
        let mesh = mesh_u_grid();

        let from = [0.1, 1.9];
        let to = [2.1, 1.9];
        let search_node = SearchNode {
            path: vec![],
            r: from,
            i: [[1.0, 0.0], [1.0, 1.0]],
            polygon_from: 0,
            polygon_to: 1,
            f: 0.0,
            g: distance_between(from, to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].r, [1.0, 1.0]);
        assert_eq!(successors[0].f, distance_between(from, [1.0, 1.0]));
        assert_eq!(
            successors[0].g,
            distance_between([1.0, 1.0], [2.0, 1.0]) + distance_between([2.0, 1.0], to)
        );
        assert_eq!(successors[0].polygon_from, 1);
        assert_eq!(successors[0].polygon_to, 2);
        assert_eq!(successors[0].i, [[2.0, 0.0], [2.0, 1.0]]);
        assert_eq!(successors[0].path, vec![from]);

        assert_eq!(
            mesh.path(from, to),
            Path {
                path: vec![[1.0, 1.0], [2.0, 1.0], to],
                len: distance_between(from, [1.0, 1.0])
                    + distance_between([1.0, 1.0], [2.0, 1.0])
                    + distance_between([2.0, 1.0], to)
            }
        );
    }

    fn mesh_from_paper() -> Mesh {
        Mesh {
            vertices: vec![
                Vertex::new(0, 6, vec![0, -1]),           // 0
                Vertex::new(2, 5, vec![0, -1, 2]),        // 1
                Vertex::new(5, 7, vec![0, 2, -1]),        // 2
                Vertex::new(5, 8, vec![0, -1]),           // 3
                Vertex::new(0, 8, vec![0, -1]),           // 4
                Vertex::new(1, 4, vec![1, -1]),           // 5
                Vertex::new(2, 1, vec![1, -1]),           // 6
                Vertex::new(4, 1, vec![1, -1]),           // 7
                Vertex::new(4, 2, vec![1, -1, 2]),        // 8
                Vertex::new(2, 4, vec![1, 2, -1]),        // 9
                Vertex::new(7, 4, vec![2, -1, 4]),        // 10
                Vertex::new(10, 7, vec![2, 4, 6, -1, 3]), // 11
                Vertex::new(7, 7, vec![2, 3, -1]),        // 12
                Vertex::new(11, 8, vec![3, -1]),          // 13
                Vertex::new(7, 8, vec![3, -1]),           // 14
                Vertex::new(7, 0, vec![5, 4, -1]),        // 15
                Vertex::new(11, 3, vec![4, 5, -1]),       // 16
                Vertex::new(11, 5, vec![4, -1, 6]),       // 17
                Vertex::new(12, 0, vec![5, -1]),          // 18
                Vertex::new(12, 3, vec![5, -1]),          // 19
                Vertex::new(13, 5, vec![6, -1]),          // 20
                Vertex::new(13, 7, vec![6, -1]),          // 21
                Vertex::new(1, 3, vec![1, -1]),           // 22
            ],
            polygons: vec![
                Polygon::new(5, vec![0, 1, 2, 3, 4, -1, -1, 2, -1, -1]),
                Polygon::new(6, vec![5, 22, 6, 7, 8, 9, -1, -1, -1, -1, 2, -1]),
                Polygon::new(7, vec![1, 9, 8, 10, 11, 12, 2, -1, 1, -1, 4, 3, -1, 0]),
                Polygon::new(4, vec![12, 11, 13, 14, 2, -1, -1, -1]),
                Polygon::new(5, vec![10, 15, 16, 17, 11, -1, 5, -1, 6, 2]),
                Polygon::new(4, vec![15, 18, 19, 16, -1, -1, -1, 4]),
                Polygon::new(4, vec![11, 17, 20, 21, 4, -1, -1, -1]),
            ],
        }
    }

    #[test]
    fn paper_straight() {
        let mesh = mesh_from_paper();

        let from = [12.0, 0.0];
        let to = [7.0, 6.9];
        let search_node = SearchNode {
            path: vec![],
            r: from,
            i: [[11.0, 3.0], [7.0, 0.0]],
            polygon_from: mesh.point_in_polygon(from) as isize,
            polygon_to: 4,
            f: 0.0,
            g: distance_between(from, to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 3);

        assert_eq!(successors[0].r, [11.0, 3.0]);
        assert_eq!(successors[0].f, distance_between(from, [11.0, 3.0]));
        assert_eq!(
            successors[0].g,
            distance_between([11.0, 3.0], [11.0, 5.0])
                + distance_between([11.0, 5.0], mirror(to, [[11.0, 5.0], [10.0, 7.0]]))
        );
        assert_eq!(successors[0].polygon_from, 4);
        assert_eq!(successors[0].polygon_to, 6);
        assert_eq!(successors[0].i, [[11.0, 5.0], [10.0, 7.0]]);
        assert_eq!(successors[0].path, vec![from]);

        assert_eq!(successors[1].r, [11.0, 3.0]);
        assert_eq!(successors[1].f, distance_between(from, [11.0, 3.0]));
        assert_eq!(
            successors[1].g,
            distance_between([11.0, 3.0], [9.75, 6.75]) + distance_between([9.75, 6.75], to)
        );
        assert_eq!(successors[1].polygon_from, 4);
        assert_eq!(successors[1].polygon_to, 2);
        assert_eq!(successors[1].i, [[10.0, 7.0], [9.75, 6.75]]);
        assert_eq!(successors[1].path, vec![from]);

        assert_eq!(successors[2].r, from);
        assert_eq!(successors[2].f, 0.0);
        assert_eq!(successors[2].g, distance_between(from, to));
        assert_eq!(successors[2].polygon_from, 4);
        assert_eq!(successors[2].polygon_to, 2);
        assert_eq!(successors[2].i, [[9.75, 6.75], [7.0, 4.0]]);
        assert_eq!(successors[2].path, Vec::<[f32; 2]>::new());

        assert_eq!(mesh.path(from, to).len, distance_between(from, to));
        assert_eq!(mesh.path(from, to).path, vec![to]);
    }

    #[test]
    fn paper_corner_right() {
        let mesh = mesh_from_paper();

        let from = [12.0, 0.0];
        let to = [13.0, 6.0];
        let search_node = SearchNode {
            path: vec![],
            r: from,
            i: [[11.0, 3.0], [7.0, 0.0]],
            polygon_from: mesh.point_in_polygon(from) as isize,
            polygon_to: 4,
            f: 0.0,
            g: distance_between(from, to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 3);

        assert_eq!(successors[0].r, [11.0, 3.0]);
        assert_eq!(successors[0].f, distance_between(from, [11.0, 3.0]));
        assert_eq!(
            successors[0].g,
            distance_between([11.0, 3.0], [11.0, 5.0]) + distance_between([11.0, 5.0], to)
        );
        assert_eq!(successors[0].polygon_from, 4);
        assert_eq!(successors[0].polygon_to, 6);
        assert_eq!(successors[0].i, [[11.0, 5.0], [10.0, 7.0]]);
        assert_eq!(successors[0].path, vec![from]);

        assert_eq!(successors[1].r, [11.0, 3.0]);
        assert_eq!(successors[1].f, distance_between(from, [11.0, 3.0]));
        assert_eq!(
            successors[1].g,
            distance_between([11.0, 3.0], mirror(to, [[10.0, 7.0], [9.75, 6.75]]))
        );
        assert_eq!(successors[1].polygon_from, 4);
        assert_eq!(successors[1].polygon_to, 2);
        assert_eq!(successors[1].i, [[10.0, 7.0], [9.75, 6.75]]);
        assert_eq!(successors[1].path, vec![from]);

        assert_eq!(successors[2].r, from);
        assert_eq!(successors[2].f, 0.0);
        assert_eq!(
            successors[2].g,
            distance_between(from, [9.75, 6.75])
                + distance_between([9.75, 6.75], mirror(to, [[9.75, 6.75], [7.0, 4.0]]))
        );
        assert_eq!(successors[2].polygon_from, 4);
        assert_eq!(successors[2].polygon_to, 2);
        assert_eq!(successors[2].i, [[9.75, 6.75], [7.0, 4.0]]);
        assert_eq!(successors[2].path, Vec::<[f32; 2]>::new());

        assert_delta!(
            mesh.path(from, to).len,
            distance_between(from, [11.0, 3.0])
                + distance_between([11.0, 3.0], [11.0, 5.0])
                + distance_between([11.0, 5.0], to)
        );
        assert_eq!(mesh.path(from, to).path, vec![[11.0, 3.0], [11.0, 5.0], to]);
    }

    #[test]
    fn paper_corner_left() {
        let mesh = mesh_from_paper();

        let from = [12.0, 0.0];
        let to = [5.0, 3.0];
        let search_node = SearchNode {
            path: vec![],
            r: from,
            i: [[11.0, 3.0], [7.0, 0.0]],
            polygon_from: mesh.point_in_polygon(from) as isize,
            polygon_to: 4,
            f: 0.0,
            g: distance_between(from, to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 3);

        assert_eq!(successors[0].r, [11.0, 3.0]);
        assert_eq!(successors[0].f, distance_between(from, [11.0, 3.0]));
        assert_eq!(
            successors[0].g,
            distance_between([11.0, 3.0], [11.0, 5.0])
                + distance_between([11.0, 5.0], mirror(to, [[11.0, 5.0], [10.0, 7.0]]))
        );
        assert_eq!(successors[0].polygon_from, 4);
        assert_eq!(successors[0].polygon_to, 6);
        assert_eq!(successors[0].i, [[11.0, 5.0], [10.0, 7.0]]);
        assert_eq!(successors[0].path, vec![from]);

        assert_eq!(successors[1].r, [11.0, 3.0]);
        assert_eq!(successors[1].f, distance_between(from, [11.0, 3.0]));
        assert_eq!(
            successors[1].g,
            distance_between([11.0, 3.0], [9.75, 6.75]) + distance_between([9.75, 6.75], to)
        );
        assert_eq!(successors[1].polygon_from, 4);
        assert_eq!(successors[1].polygon_to, 2);
        assert_eq!(successors[1].i, [[10.0, 7.0], [9.75, 6.75]]);
        assert_eq!(successors[1].path, vec![from]);

        assert_eq!(successors[2].r, from);
        assert_eq!(successors[2].f, 0.0);
        assert_eq!(
            successors[2].g,
            distance_between(from, [7.0, 4.0]) + distance_between([7.0, 4.0], to)
        );
        assert_eq!(successors[2].polygon_from, 4);
        assert_eq!(successors[2].polygon_to, 2);
        assert_eq!(successors[2].i, [[9.75, 6.75], [7.0, 4.0]]);
        assert_eq!(successors[2].path, Vec::<[f32; 2]>::new());

        assert_delta!(
            mesh.path(from, to).len,
            distance_between(from, [7.0, 4.0]) + distance_between([7.0, 4.0], to)
        );
        assert_eq!(mesh.path(from, to).path, vec![[7.0, 4.0], to]);
    }

    #[test]
    fn paper_corner_left_twice() {
        let mesh = mesh_from_paper();

        let from = [12.0, 0.0];
        let to = [3.0, 1.0];
        let search_node = SearchNode {
            path: vec![],
            r: from,
            i: [[11.0, 3.0], [7.0, 0.0]],
            polygon_from: mesh.point_in_polygon(from) as isize,
            polygon_to: 4,
            f: 0.0,
            g: distance_between(from, to),
        };
        let successors = dbg!(mesh.successors(search_node, to));
        assert_eq!(successors.len(), 3);

        assert_eq!(successors[0].r, [11.0, 3.0]);
        assert_eq!(successors[0].f, distance_between(from, [11.0, 3.0]));
        assert_eq!(
            successors[0].g,
            distance_between([11.0, 3.0], [11.0, 5.0])
                + distance_between([11.0, 5.0], mirror(to, [[11.0, 5.0], [10.0, 7.0]]))
        );
        assert_eq!(successors[0].polygon_from, 4);
        assert_eq!(successors[0].polygon_to, 6);
        assert_eq!(successors[0].i, [[11.0, 5.0], [10.0, 7.0]]);
        assert_eq!(successors[0].path, vec![from]);

        assert_eq!(successors[1].r, [11.0, 3.0]);
        assert_eq!(successors[1].f, distance_between(from, [11.0, 3.0]));
        assert_eq!(
            successors[1].g,
            distance_between([11.0, 3.0], [9.75, 6.75]) + distance_between([9.75, 6.75], to)
        );
        assert_eq!(successors[1].polygon_from, 4);
        assert_eq!(successors[1].polygon_to, 2);
        assert_eq!(successors[1].i, [[10.0, 7.0], [9.75, 6.75]]);
        assert_eq!(successors[1].path, vec![from]);

        assert_eq!(successors[2].r, from);
        assert_eq!(successors[2].f, 0.0);
        assert_eq!(
            successors[2].g,
            distance_between(from, [7.0, 4.0]) + distance_between([7.0, 4.0], to)
        );
        assert_eq!(successors[2].polygon_from, 4);
        assert_eq!(successors[2].polygon_to, 2);
        assert_eq!(successors[2].i, [[9.75, 6.75], [7.0, 4.0]]);
        assert_eq!(successors[2].path, Vec::<[f32; 2]>::new());

        let successor = successors.into_iter().nth(2).unwrap();
        let successors = dbg!(mesh.successors(successor, to));
        dbg!(&successors[2]);
        assert_eq!(successors.len(), 4);

        assert_delta!(
            mesh.path(from, to).len,
            distance_between(from, [7.0, 4.0])
                + distance_between([7.0, 4.0], [4.0, 2.0])
                + distance_between([4.0, 2.0], to)
        );

        assert_eq!(mesh.path(from, to).path, vec![[7.0, 4.0], [4.0, 2.0], to]);
    }
}
