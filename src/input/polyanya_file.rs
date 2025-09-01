use core::panic;
use std::io::{BufRead, BufReader, Lines, Read, Write};

use glam::Vec2;
use hashbrown::HashMap;

use crate::{Mesh, MeshError, Polygon, Vertex};

/// A mesh read from a Polyanya file in the formats `mesh 2` or `mesh 3`.
///
/// See <https://github.com/vleue/polyanya/blob/main/meshes/v2/format.txt> for v2 format description, or
/// <https://github.com/vleue/polyanya/blob/main/meshes/v3/format.txt>
#[derive(Debug)]
pub struct PolyanyaFile {
    /// List of vertex described in the file
    pub vertices: Vec<Vertex>,
    /// List of polygon described in the file
    pub polygons: Vec<Polygon>,
}

impl PolyanyaFile {
    /// Create a `Mesh` from a file in the formats `mesh 2` or `mesh 3`.
    ///
    /// See <https://github.com/vleue/polyanya/blob/main/meshes/v2/format.txt> for v2 format description, or
    /// <https://github.com/vleue/polyanya/blob/main/meshes/v3/format.txt>
    pub fn from_file(path: &str) -> PolyanyaFile {
        let mut file = std::fs::File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        Self::from_bytes(&buffer)
    }

    /// Create a `Mesh` from bytes in the formats `mesh 2` or `mesh 3`.
    ///
    /// See <https://github.com/vleue/polyanya/blob/main/meshes/v2/format.txt> for v2 format description, or
    /// <https://github.com/vleue/polyanya/blob/main/meshes/v3/format.txt>
    pub fn from_bytes(bytes: &[u8]) -> PolyanyaFile {
        let mut lines = BufReader::new(bytes).lines();

        // First check the header.
        if let Some(Ok(first)) = lines.next() {
            if first != "mesh" {
                panic!("Invalid polyanya .mesh file");
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

/// Create a `PolyanyaFile` from the v2 mesh format
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
                let mut values = line.split_whitespace();
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
                let mut values = line.split_whitespace();
                let n = values.next().unwrap().parse().unwrap();
                let polygon = Polygon::using(n, values.map(|v| v.parse().unwrap()).collect());
                mesh.polygons.push(polygon)
            } else {
                panic!("Failed to parse v2 mesh format, unexpected line.");
            }
        }
    }
}

/// Create a `PolyanyaFile` from the v3 mesh format
fn parse_v3(
    mesh: &mut PolyanyaFile,
    lines: Lines<BufReader<&[u8]>>,
    mut nb_vertices: usize,
    mut nb_polygons: usize,
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
            if nb_polygons > 0 {
                nb_polygons -= 1;
                let mut values = line.split_whitespace();
                let is_traversable = match values.next().unwrap() {
                    "0" => false,
                    "1" => true,
                    _ => panic!("Invalid v3 mesh format."),
                };

                let num_edges: usize = values.next().unwrap().parse().unwrap();
                let mut vertex_indices = Vec::with_capacity(num_edges);

                // We still need to add the non-traversable polygons to the vertices.
                let data = values
                    .enumerate()
                    .map(|(i, v)| {
                        let num: isize = v.parse().unwrap();
                        if i < num_edges {
                            let vertex_neighbours = vertex_polys.entry(num as usize).or_default();
                            vertex_neighbours.push(match is_traversable {
                                true => polygon_index,
                                false => u32::MAX,
                            });
                            vertex_indices.push(num as usize);
                        } else {
                            // Num here is the polygon index.
                            if num <= -1 {
                                // We don't care about the specific impassable polygons, just that they're impassable.
                                return -1;
                            } else if num == 0 {
                                // In the mesh-v3 format 0 means that it's the edge of the map and there are no polygons.
                                vertex_polys
                                    .entry(vertex_indices[i - num_edges])
                                    .or_insert(Vec::new())
                                    .push(u32::MAX);
                            }
                        }
                        // Subtract 1 here since it's 1 indexed, and we're using 0-indexed vecs.
                        num - 1
                    })
                    .collect();
                if is_traversable {
                    // Only increment the polygon index if the polygon is traversable since we don't
                    // care about the non-traversable polygons.
                    polygon_index += 1;
                    let polygon = Polygon::using(num_edges, data);
                    mesh.polygons.push(polygon)
                }
            } else {
                panic!("Failed to parse v3 mesh format, unexpected line.");
            }
        }
    }

    // Add the vertices to the mesh. We do this after parsing since the adjacent polygons for vertices are
    // calculated from the polygons associated with each vertex.
    // TODO: Do we need to de-duplicate adjacent impassable polygons for the vertices?
    for (i, coordinates) in vertices.into_iter().enumerate() {
        let vert = Vertex::new(coordinates, vertex_polys.remove(&(i + 1)).unwrap());
        mesh.vertices.push(vert);
    }
}
