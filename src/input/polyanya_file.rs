use core::panic;
use std::io::{self, BufRead, BufReader, Lines, Read, Write};

use glam::Vec2;
use hashbrown::HashMap;

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
        let mut lines = BufReader::new(bytes).lines();

        // First check the header.
        if let Some(Ok(first)) = lines.next() {
            if first != "mesh" {
                panic!("Invalid polyanya .mesh file");
            } else {
                println!("mesh");
            }
        } else {
            panic!("Invalid polyanya .mesh file");
        }

        let version = if let Some(Ok(version)) = lines.next() {
            if version == "2" {
                2
            } else if version == "3" {
                3
            } else {
                panic!("Invalid polyanya .mesh file");
            }
        } else {
            panic!("Invalid polyanya .mesh file");
        };

        // In the v3 format `nb_polygons` is actually the number of faces.
        let (nb_vertices, nb_polygons) = lines
            .next()
            .unwrap()
            .unwrap()
            .split_once(' ')
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .unwrap();

        let mut mesh = PolyanyaFile {
            vertices: Vec::with_capacity(nb_vertices),
            polygons: Vec::with_capacity(nb_polygons),
        };

        // Parse the remainder of the file depending on what `.mesh` version it is.
        match version {
            2 => parse_v2(&mut mesh, lines, nb_vertices, nb_polygons),
            3 => parse_v3(&mut mesh, lines, nb_vertices, nb_polygons),
            _ => panic!("Invalid polyanya mesh version"),
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
    fn from(mut mesh: Mesh) -> Self {
        let last_layer = mesh.layers.pop().unwrap();
        PolyanyaFile {
            vertices: last_layer.vertices,
            polygons: last_layer.polygons,
        }
    }
}

fn parse_v2(
    mesh: &mut PolyanyaFile,
    lines: Lines<BufReader<&[u8]>>,
    mut nb_vertices: usize,
    mut nb_polygons: usize,
) {
    let mut phase = 1;

    for line in lines {
        let line: String = line.unwrap();
        if phase == 1 {
            if nb_vertices > 0 {
                nb_vertices -= 1;
                let mut values = line.split(' ');
                let x = values.next().unwrap().parse().unwrap();
                let y = values.next().unwrap().parse().unwrap();
                let _ = values.next();
                let vertex = Vertex::new(
                    Vec2::new(x, y),
                    values.map(|v| v.parse().unwrap_or(u32::MAX)).collect(),
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
}

fn parse_v3(
    mesh: &mut PolyanyaFile,
    lines: Lines<BufReader<&[u8]>>,
    mut nb_vertices: usize,
    mut nb_faces: usize,
) {
    let mut phase = 1;

    // These are 1-indexed in the v3 mesh file format.
    let mut vertices = Vec::with_capacity(nb_vertices);
    let mut vertex_polys: HashMap<usize, Vec<u32>> = HashMap::with_capacity(nb_vertices);
    let mut polygon_index = 0;

    for line in lines {
        let line: String = line.unwrap();
        if line.is_empty() {
            continue;
        }
        if phase == 1 {
            if nb_vertices > 0 {
                nb_vertices -= 1;
                let mut values = line.split_whitespace();
                let x = values.next().unwrap().parse().unwrap();
                let y = values.next().unwrap().parse().unwrap();
                vertices.push(Vec2::new(x, y));
            } else {
                phase = 2;
            }
        }
        if phase == 2 {
            if nb_faces > 0 {
                polygon_index += 1;
                nb_faces -= 1;
                let mut values = line.split_whitespace();
                // TODO: What do I use this for???
                let is_traversable = match values.next().unwrap() {
                    "0" => false,
                    "1" => true,
                    _ => panic!("Invalid mesh format..."),
                };

                let num_edges: usize = values.next().unwrap().parse().unwrap();
                let mut vertex_indices = Vec::with_capacity(num_edges);

                // TODO: Short circuit this if the polygon isn't traversible.
                // We still need to add the non-traversible polygons to the vertices.
                let data = values
                    .enumerate()
                    .map(|(i, v)| {
                        
                        let num: isize = v.parse().unwrap();
                        if i < num_edges {
                            // TODO: Account for when it's on the edge of the mesh (the 0 case).
                            // Currently it'll say eg. a corner vert has 2 real faces and ignores the edge of the mesh.
                            // Just checking if there's a 0 in the else and pushing that as u32::max to the ... should work
                            // `num` here is the vertex index
                            let vertex_neighbours =
                                vertex_polys.entry(num as usize).or_insert(Vec::new());
                            vertex_neighbours.push(match is_traversable {
                                true => polygon_index, // Polygon index...
                                false => u32::MAX,
                            });
                            vertex_indices.push(num as usize);
                        } else {
                            // Num here is the polygon/face index.
                            if num <= -1 {
                                return -1;
                            } else if num == 0 {
                                vertex_polys
                                    .entry(vertex_indices[i - num_edges])
                                    .or_insert(Vec::new())
                                    .push(u32::MAX);
                            }
                        }
                        num
                    })
                    .collect();
                if is_traversable {
                    let polygon = Polygon::using(num_edges, data);
                    mesh.polygons.push(polygon)
                }
            } else {
                panic!("unexpected line");
            }
        }
    }

    for (i, coordinates) in vertices.into_iter().enumerate() {
        // Vertices and Polygons/faces are 1-indexed not 0-indexed.
        let vert = Vertex::new(coordinates, vertex_polys.remove(&(i + 1)).unwrap());
        mesh.vertices.push(vert);
    }
}
