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
        ((to[0] - from[0]).powi(2) + (to[1] - from[1]).powi(2)).sqrt()
    }
}

impl Mesh {
    pub fn point_in_polygon(&self, point: [f32; 2]) -> usize {
        'polygons: for (i, polygon) in self.polygons.iter().enumerate() {
            let mut side = None;
            for edge in polygon.edges_index() {
                let last = self.vertices.get(edge[0]).unwrap();
                let next = self.vertices.get(edge[1]).unwrap();
                let current_side = (point[1] - last.y) * (next.x - last.x)
                    - (point[0] - last.x) * (next.y - last.y);
                if current_side == 0.0 {
                    return i;
                }
                if side.is_none() {
                    side = Some(current_side.is_sign_positive());
                }
                if side.unwrap() != current_side.is_sign_positive() {
                    continue 'polygons;
                }
            }
            return i;
        }
        return usize::MAX;
    }
}

pub struct SearchNode {
    pub r: [f32; 2],
    pub i: [[f32; 2]; 2],
}

#[cfg(test)]
mod tests {
    use crate::{Mesh, Polygon, Vertex};

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
}
