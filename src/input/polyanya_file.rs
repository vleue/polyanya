use std::io::{self, BufRead, Read};

use glam::Vec2;

use crate::{Mesh, Polygon, Vertex};

/// A mesh read from a Polyanya file in the format `mesh 2`.
///
/// See <https://github.com/vleue/polyanya/blob/main/meshes/format.txt> for format description.
#[derive(Debug)]
pub struct PolyanyaFile {
    /// List of vertex described in the file
    pub vertices: Vec<Vertex>,
    /// List of polygon described in the file
    pub polygons: Vec<Polygon>,
}

impl PolyanyaFile {
    /// Create a `Mesh` from a file in the format `mesh 2`.
    ///
    /// See <https://github.com/vleue/polyanya/blob/main/meshes/format.txt> for format description.
    pub fn from_file(path: &str) -> PolyanyaFile {
        let mut file = std::fs::File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        Self::from_bytes(&buffer)
    }

    /// Create a `Mesh` from bytes in the format `mesh 2`.
    ///
    /// See <https://github.com/vleue/polyanya/blob/main/meshes/format.txt> for format description.
    pub fn from_bytes(bytes: &[u8]) -> PolyanyaFile {
        let mut mesh = PolyanyaFile {
            vertices: vec![],
            polygons: vec![],
        };
        let mut nb_vertices = 0;
        let mut nb_polygons = 0;
        let mut phase = 0;
        for line in io::BufReader::new(bytes).lines() {
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
                    let vertex = Vertex::new(
                        Vec2::new(x, y),
                        values.map(|v| v.parse().unwrap()).collect(),
                    );
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
        mesh
    }
}

impl From<PolyanyaFile> for Mesh {
    fn from(value: PolyanyaFile) -> Self {
        Mesh::new(value.vertices, value.polygons)
    }
}
