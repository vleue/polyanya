use std::{cmp::Ordering, collections::BinaryHeap};

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
        let starting_polygon = self.point_in_polygon(from);
        let starting_polygon = self.polygons.get(starting_polygon).unwrap();

        let mut to_add = vec![];
        for edge in starting_polygon.edges_index() {
            let start = self.vertices.get(edge[0]).unwrap();
            let end = self.vertices.get(edge[1]).unwrap();
            to_add.push(SearchNode {
                path: vec![],
                r: from,
                i: [[start.x, start.y], [end.x, end.y]],
                f: 0.0,
                g: heuristic(from, to, [[start.x, start.y], [end.x, end.y]]),
            })
        }

        let mut queue = BinaryHeap::new();
        for node in to_add {
            queue.push(node);
        }

        distance_between(from, to)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum EdgeSide {
    Left,
    Right,
    Edge,
}

fn on_side(point: [f32; 2], i: [[f32; 2]; 2]) -> EdgeSide {
    let side =
        (point[1] - i[0][1]) * (i[1][0] - i[0][0]) - (point[0] - i[0][0]) * (i[1][1] - i[0][1]);
    match side {
        x if x == 0.0 => EdgeSide::Edge,
        x if x < 0.0 => EdgeSide::Right,
        _ => EdgeSide::Left,
    }
}

// i should be counterclockwise from r
fn heuristic(r: [f32; 2], to: [f32; 2], i: [[f32; 2]; 2]) -> f32 {
    let to = if on_side(r, i) == on_side(to, i) {
        mirror(to, i)
    } else {
        to
    };
    if on_side(to, [r, i[0]]) == EdgeSide::Right {
        distance_between(r, i[0]) + distance_between(i[0], to)
    } else if on_side(to, [r, i[1]]) == EdgeSide::Left {
        distance_between(r, i[1]) + distance_between(i[1], to)
    } else {
        distance_between(r, to)
    }
}

fn mirror(p: [f32; 2], i: [[f32; 2]; 2]) -> [f32; 2] {
    let dx = i[1][0] - i[0][0];
    let dy = i[1][1] - i[0][1];

    let a = (dx * dx - dy * dy) / (dx * dx + dy * dy);
    let b = 2.0 * dx * dy / (dx * dx + dy * dy);

    let x2 = a * (p[0] - i[0][0]) + b * (p[1] - i[0][1]) + i[0][0];
    let y2 = b * (p[0] - i[0][0]) - a * (p[1] - i[0][1]) + i[0][1];

    [x2, y2]
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

#[derive(PartialEq)]
pub struct SearchNode {
    pub path: Vec<[f32; 2]>,
    pub r: [f32; 2],
    pub i: [[f32; 2]; 2],
    pub f: f32,
    pub g: f32,
}

fn distance_between(from: [f32; 2], to: [f32; 2]) -> f32 {
    ((to[0] - from[0]).powi(2) + (to[1] - from[1]).powi(2)).sqrt()
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
    use crate::{heuristic, mirror, on_side, EdgeSide, Mesh, Polygon, Vertex};

    #[test]
    fn point_in_polygon() {
        let mesh = Mesh {
            vertices: vec![
                Vertex::new(0, 0, vec![0]),
                Vertex::new(0, 1, vec![0]),
                Vertex::new(1, 1, vec![0, 1]),
                Vertex::new(1, 0, vec![0, 1]),
                Vertex::new(2, 0, vec![1]),
                Vertex::new(2, 1, vec![1]),
            ],
            polygons: vec![
                Polygon::new(4, vec![0, 1, 2, 3, -1, -1, -1, -1]),
                Polygon::new(4, vec![2, 3, 4, 5, -1, -1, -1, -1]),
            ],
        };
        assert_eq!(mesh.point_in_polygon([0.5, 0.5]), 0);
        assert_eq!(mesh.point_in_polygon([1.5, 0.5]), 1);
        assert_eq!(mesh.point_in_polygon([0.5, 1.5]), usize::MAX);
    }

    #[test]
    fn test_on_side() {
        assert_eq!(
            on_side([0.0, 0.5], [[0.0, 0.0], [1.0, 0.0]]),
            EdgeSide::Left
        );
        assert_eq!(
            on_side([0.0, -0.5], [[0.0, 0.0], [1.0, 0.0]]),
            EdgeSide::Right
        );
        assert_eq!(
            on_side([1.0, 0.0], [[0.0, 0.0], [1.0, 1.0]]),
            EdgeSide::Right
        );
        assert_eq!(
            on_side([0.0, 1.0], [[0.0, 0.0], [1.0, 1.0]]),
            EdgeSide::Left
        );
        assert_eq!(
            on_side([2.0, 2.0], [[0.0, 0.0], [1.0, 1.0]]),
            EdgeSide::Edge
        );
    }

    #[test]
    fn test_mirror() {
        assert_eq!(mirror([1.0, 0.0], [[0.0, 0.0], [0.0, 1.0]]), [-1.0, 0.0]);
        assert_eq!(mirror([-1.0, 0.0], [[0.0, 0.0], [0.0, 1.0]]), [1.0, 0.0]);
    }

    #[test]
    fn test_heuristic() {
        assert_eq!(
            heuristic([0.0, 0.0], [1.0, 1.0], [[1.0, 0.0], [0.0, 1.0]]),
            2.0_f32.sqrt()
        );
        assert_eq!(
            heuristic([0.0, 0.0], [2.0, -1.0], [[1.0, 0.0], [0.0, 1.0]]),
            1.0 + 2.0_f32.sqrt()
        );
        assert_eq!(
            heuristic([0.0, 0.0], [-1.0, 2.0], [[1.0, 0.0], [0.0, 1.0]]),
            1.0 + 2.0_f32.sqrt()
        );
        assert_eq!(
            heuristic([0.0, 0.0], [1.0, -1.0], [[1.0, 0.0], [0.0, 1.0]]),
            2.0
        );
    }
}
