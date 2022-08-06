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
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub polygons: Vec<Polygon>,
}

impl Mesh {
    pub fn path(&self, _from: [f32; 2], _to: [f32; 2]) {
        unimplemented!()
    }

    pub fn path_len(&self, from: [f32; 2], to: [f32; 2]) -> f32 {
        ((to[0] - from[0]).powi(2) + (to[1] - from[1]).powi(2)).sqrt()
    }
}
