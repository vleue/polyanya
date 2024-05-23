use std::io::{self, BufRead, Read, Write};

use glam::Vec2;

use crate::{Mesh, MeshError, Polygon, Vertex};

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

    /// Write a `Mesh` to a file in the format `mesh 2`.
    ///
    /// See <https://github.com/vleue/polyanya/blob/main/meshes/format.txt> for format description.
    pub fn to_file(&self, path: &str) {
        let mut file = std::fs::File::create(path).unwrap();
        let bytes = self.to_bytes();
        file.write_all(&bytes).unwrap();
    }

    /// Write a `Mesh` to bytes in the format `mesh 2`.
    ///
    /// See <https://github.com/vleue/polyanya/blob/main/meshes/format.txt> for format description.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(b"mesh\n");
        bytes.extend_from_slice(b"2\n");
        bytes.extend_from_slice(
            format!("{} {}\n", self.vertices.len(), self.polygons.len()).as_bytes(),
        );
        for vertex in &self.vertices {
            bytes.extend_from_slice(
                format!(
                    "{:.06} {:.06} {} {}\n",
                    vertex.coords.x,
                    vertex.coords.y,
                    vertex.polygons.len(),
                    vertex
                        .polygons
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
                .as_bytes(),
            );
        }
        for polygon in &self.polygons {
            bytes.extend_from_slice(
                format!(
                    "{} {}\n",
                    polygon.vertices.len(),
                    polygon
                        .vertices
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
                .as_bytes(),
            );
        }
        bytes
    }
}

impl TryFrom<PolyanyaFile> for Mesh {
    type Error = MeshError;

    fn try_from(value: PolyanyaFile) -> Result<Self, Self::Error> {
        Mesh::new(value.vertices, value.polygons)
    }
}

impl From<Mesh> for PolyanyaFile {
    fn from(mesh: Mesh) -> Self {
        PolyanyaFile {
            vertices: mesh.vertices,
            polygons: mesh.polygons,
        }
    }
}
