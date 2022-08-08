use std::{cmp::Ordering, collections::BinaryHeap};

use helpers::{distance_between, heuristic, on_side};

mod helpers;
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

pub struct Polygon {
    pub vertices: Vec<usize>,
    pub neighbours: Vec<isize>,
}

impl Polygon {
    pub fn new(nb: usize, data: Vec<isize>) -> Self {
        assert!(data.len() == nb * 2);
        let (vertices, neighbours) = data.split_at(nb);
        let vertices = vertices.to_vec().into_iter().map(|v| v as usize).collect();
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

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub polygons: Vec<Polygon>,
}

impl Mesh {
    pub fn path_len(&self, from: [f32; 2], to: [f32; 2]) -> f32 {
        let starting_polygon_index = self.point_in_polygon(from);
        let starting_polygon = self.polygons.get(starting_polygon_index).unwrap();
        let ending_polygon = self.point_in_polygon(to);

        if starting_polygon_index == ending_polygon {
            return distance_between(from, to);
        }

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
            if other_side != isize::MAX {
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
        }

        let mut queue = BinaryHeap::new();
        for node in to_add {
            queue.push(node);
        }

        while let Some(next) = queue.pop() {
            if next.polygon_to == ending_polygon as isize {
                return next.f + next.g;
            }
            let to_add = self.successors(next, to);
            for node in to_add {
                queue.push(node);
            }
        }
        return -1.0;
    }

    fn successors(&self, node: SearchNode, to: [f32; 2]) -> Vec<SearchNode> {
        let into_polygon = self.polygons.get(node.polygon_to as usize).unwrap();

        let mut found_start = false;

        let mut to_add = vec![];
        for edge in into_polygon
            .edges_index()
            .iter()
            .chain(into_polygon.edges_index().iter())
        {
            let start = self.vertices.get(edge[0]).unwrap();
            let end = self.vertices.get(edge[1]).unwrap();

            // break once we reached the interval start
            if found_start
                && on_side(node.i[1], [[start.x, start.y], [end.x, end.y]]) == EdgeSide::Edge
            {
                break;
            }

            // continue until we get to the interval end
            if !found_start
                && on_side(node.i[0], [[start.x, start.y], [end.x, end.y]]) == EdgeSide::Edge
            {
                found_start = true;
            }
            if !found_start {
                continue;
            }

            let start = self.vertices.get(edge[0]).unwrap();
            let end = self.vertices.get(edge[1]).unwrap();
            let mut other_side = isize::MAX;
            // find the polygon at the other side of this edge
            for i in &start.polygons {
                if *i != -1 && *i != node.polygon_to as isize && end.polygons.contains(i) {
                    other_side = *i;
                }
            }
            // prune edges that don't have a polygon on the other side: cul de sac pruning
            if other_side == isize::MAX {
                continue;
            }

            if other_side == node.polygon_from {
                continue;
            }

            to_add.push(SearchNode {
                path: vec![],
                r: node.r,
                i: [[start.x, start.y], [end.x, end.y]],
                polygon_from: node.polygon_to as isize,
                polygon_to: other_side,
                f: node.f,
                g: heuristic(node.r, to, [[start.x, start.y], [end.x, end.y]]),
            })
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
        'polygons: for (i, polygon) in self.polygons.iter().enumerate() {
            let mut side = None;
            for edge in polygon.edges_index() {
                let last = self.vertices.get(edge[0]).unwrap();
                let next = self.vertices.get(edge[1]).unwrap();
                let current_side = on_side(point, [[last.x, last.y], [next.x, next.y]]);
                if current_side == EdgeSide::Edge {
                    return i;
                }
                if side.is_none() {
                    side = Some(current_side);
                }
                if side.unwrap() != current_side {
                    continue 'polygons;
                }
            }
            return i;
        }
        return usize::MAX;
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

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SearchNode {}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.f + self.g).total_cmp(&(other.f + other.g))
    }
}

#[cfg(test)]
mod tests {
    use crate::{helpers::distance_between, Mesh, Polygon, SearchNode, Vertex};

    fn test_mesh() -> Mesh {
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
        let mesh = test_mesh();
        assert_eq!(mesh.point_in_polygon([0.5, 0.5]), 0);
        assert_eq!(mesh.point_in_polygon([1.5, 0.5]), 1);
        assert_eq!(mesh.point_in_polygon([0.5, 1.5]), 3);
        assert_eq!(mesh.point_in_polygon([1.5, 1.5]), usize::MAX);
        assert_eq!(mesh.point_in_polygon([2.5, 1.5]), 4);
    }

    #[test]
    fn successors() {
        let mesh = test_mesh();
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
        let successors = mesh.successors(search_node, to);
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].r, from);
        assert_eq!(successors[0].f, 0.0);
        assert_eq!(successors[0].g, distance_between(from, to));
        assert_eq!(successors[0].polygon_from, 1);
        assert_eq!(successors[0].polygon_to, 2);
        assert_eq!(successors[0].i, [[2.0, 0.0], [2.0, 1.0]]);

        let search_node = SearchNode {
            path: vec![],
            r: to,
            i: [[2.0, 1.0], [2.0, 0.0]],
            polygon_from: mesh.point_in_polygon(to) as isize,
            polygon_to: 1,
            f: 0.0,
            g: distance_between(from, to),
        };
        let successors = mesh.successors(search_node, from);
        assert_eq!(successors.len(), 1);
        assert_eq!(successors[0].r, to);
        assert_eq!(successors[0].f, 0.0);
        assert_eq!(successors[0].g, distance_between(to, from));
        assert_eq!(successors[0].polygon_from, 1);
        assert_eq!(successors[0].polygon_to, 0);
        assert_eq!(successors[0].i, [[1.0, 1.0], [1.0, 0.0]]);
    }
}
