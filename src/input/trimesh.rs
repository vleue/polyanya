use crate::{Mesh, MeshError, Polygon, Vertex};
use glam::Vec2;
use num_traits::Zero;
use std::cmp::Ordering;
use std::iter;

trait Triangle {
    fn get_clockwise_neighbor(&self, index: usize) -> usize;
    fn get_counterclockwise_neighbor(&self, index: usize) -> usize;
    fn position(&self, index: usize) -> usize;
}

impl Triangle for [usize; 3] {
    fn get_clockwise_neighbor(&self, index: usize) -> usize {
        let position = self.position(index);
        self[(position + 1) % 3]
    }

    fn get_counterclockwise_neighbor(&self, index: usize) -> usize {
        let position = self.position(index);
        self[(position + 2) % 3]
    }
    fn position(&self, index: usize) -> usize {
        self.iter().position(|i| *i == index).unwrap()
    }
}

impl Triangle for Vec<u32> {
    fn get_clockwise_neighbor(&self, index: usize) -> usize {
        let position = self.position(index);
        self[(position + 1) % self.len()] as usize
    }

    fn get_counterclockwise_neighbor(&self, index: usize) -> usize {
        let position = self.position(index);
        self[(position + (self.len() - 1)) % self.len()] as usize
    }
    fn position(&self, index: usize) -> usize {
        self.iter().position(|i| *i as usize == index).unwrap()
    }
}

/// A triangle mesh, represented by a list of vertex and how they are arranged in triangles
#[derive(Debug)]
pub struct Trimesh {
    /// List of vertex making this trimesh
    pub vertices: Vec<Vec2>,
    /// List of triangles, made of vertex indices in counterclockwise order
    pub triangles: Vec<[usize; 3]>,
}

impl TryFrom<Trimesh> for Mesh {
    type Error = MeshError;

    fn try_from(value: Trimesh) -> Result<Self, Self::Error> {
        println!("=============");
        println!("{:?}", value);
        let mut vertices: Vec<_> = to_vertices(&value);
        let polygons = to_polygons(value.triangles);
        let unordered_vertices = vertices.clone();

        // Order vertex polygon neighbors counterclockwise
        for (vertex_index, vertex) in vertices.iter_mut().enumerate() {
            println!("- vertex {}: {:?}", vertex_index, vertex);
            vertex.polygons.sort_by(|index_a, index_b| {
                let get_counterclockwise_edge = |index: u32| {
                    // No u32::MAX present yet, so the unwrap is safe
                    let index = usize::try_from(index).unwrap();
                    let neighbor_index = polygons[index]
                        .vertices
                        .get_counterclockwise_neighbor(vertex_index);
                    let neighbor = &unordered_vertices[neighbor_index];
                    neighbor.coords - vertex.coords
                };
                let edge_a = get_counterclockwise_edge(*index_a);
                let edge_b = get_counterclockwise_edge(*index_b);
                let det = edge_a.perp_dot(edge_b);
                if det.is_zero() || det < 0. {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });

            if vertex.polygons.is_empty() {
                return Err(MeshError::InvalidMesh);
            }

            // Add obstacles (-1) as vertex neighbors
            let mut polygons_including_obstacles = vec![vertex.polygons[0]];
            for polygon_index in vertex
                .polygons
                .iter()
                .cloned()
                .skip(1)
                .chain(iter::once(polygons_including_obstacles[0]))
            {
                let last_index = *polygons_including_obstacles.last().unwrap();
                if last_index == u32::MAX {
                    polygons_including_obstacles.push(polygon_index);
                    continue;
                }
                let triangle_at = |index: u32| {
                    let polygon = &polygons[usize::try_from(index).unwrap()];
                    &polygon.vertices
                };

                let last_counterclockwise_neighbor =
                    triangle_at(last_index).get_counterclockwise_neighbor(vertex_index);
                let next_clockwise_neighbor =
                    triangle_at(polygon_index).get_clockwise_neighbor(vertex_index);

                if last_counterclockwise_neighbor != next_clockwise_neighbor {
                    // The edges don't align; there's an obstacle here
                    polygons_including_obstacles.push(u32::MAX);
                }
                polygons_including_obstacles.push(polygon_index);
            }
            // The first element is included in the end again
            polygons_including_obstacles.remove(0);
            vertex.polygons = polygons_including_obstacles;
        }
        // Recreate vertices because we now include obstacles, so vertices can now be properly identified as edges
        let vertices: Vec<_> = vertices
            .into_iter()
            .map(|vertex| Vertex::new(vertex.coords, vertex.polygons))
            .collect();
        Self::new(vertices, polygons)
    }
}

fn to_vertices(trimesh: &Trimesh) -> Vec<Vertex> {
    trimesh
        .vertices
        .iter()
        .enumerate()
        .map(|(vertex_index, coords)| {
            let neighbor_indices = trimesh
                .triangles
                .iter()
                .enumerate()
                .filter_map(|(polygon_index, vertex_indices_in_polygon)| {
                    vertex_indices_in_polygon
                        .contains(&vertex_index)
                        .then_some(polygon_index)
                })
                .map(|index| index as u32)
                .collect();
            Vertex::new(*coords, neighbor_indices)
        })
        .collect()
}

fn to_polygons(triangles: Vec<[usize; 3]>) -> Vec<Polygon> {
    let triangle_count = triangles.len();

    triangles
        .into_iter()
        .map(|vertex_indices_in_polygon| {
            // It's geometrically impossible for a triangle to have only one neighbor in a trimesh,
            // except for the trivial case
            let is_one_way = triangle_count == 1;
            Polygon::new(
                vertex_indices_in_polygon.map(|index| index as u32).to_vec(),
                is_one_way,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_from_trimesh_is_same_as_regular() -> Result<(), MeshError> {
        let regular_mesh = Mesh::new(
            vec![
                Vertex::new(Vec2::new(1., 1.), vec![0, 4, u32::MAX]), // 0
                Vertex::new(Vec2::new(5., 1.), vec![u32::MAX, 1, 3, u32::MAX, 0]), // 1
                Vertex::new(Vec2::new(5., 4.), vec![u32::MAX, 2, 1]), // 2
                Vertex::new(Vec2::new(1., 4.), vec![u32::MAX, 4, u32::MAX, 3, 2]), // 3
                Vertex::new(Vec2::new(2., 2.), vec![u32::MAX, 4, 0]), // 4
                Vertex::new(Vec2::new(4., 3.), vec![1, 2, 3]),        // 5
            ],
            vec![
                Polygon::new(vec![0, 1, 4], false), // 0
                Polygon::new(vec![1, 2, 5], false), // 1
                Polygon::new(vec![5, 2, 3], false), // 2
                Polygon::new(vec![1, 5, 3], false), // 3
                Polygon::new(vec![0, 4, 3], false), // 4
            ],
        )?;
        let from_trimesh: Mesh = Trimesh {
            vertices: vec![
                Vec2::new(1., 1.),
                Vec2::new(5., 1.),
                Vec2::new(5., 4.),
                Vec2::new(1., 4.),
                Vec2::new(2., 2.),
                Vec2::new(4., 3.),
            ],
            triangles: vec![[0, 1, 4], [1, 2, 5], [5, 2, 3], [1, 5, 3], [0, 4, 3]],
        }
        .try_into()?;
        assert_eq!(
            regular_mesh.layers[0].polygons,
            from_trimesh.layers[0].polygons
        );
        for (index, (expected_vertex, actual_vertex)) in regular_mesh.layers[0]
            .vertices
            .iter()
            .zip(from_trimesh.layers[0].vertices.iter())
            .enumerate()
        {
            assert_eq!(
                expected_vertex.coords, actual_vertex.coords,
                "\nvertex {index} does not have the expected coords.\nExpected vertices: {0:?}\nGot vertices: {1:?}",
                regular_mesh.layers[0].vertices, from_trimesh.layers[0].vertices
            );

            assert_eq!(
                expected_vertex.is_corner, actual_vertex.is_corner,
                "\nvertex {index} does not have the expected value for `is_corner`.\nExpected vertices: {0:?}\nGot vertices: {1:?}",
                regular_mesh.layers[0].vertices, from_trimesh.layers[0].vertices
            );
            let adjusted_actual = wrap_to_first(&actual_vertex.polygons, |index| *index != u32::MAX).unwrap_or_else(||
                panic!("vertex {index}: Found only surrounded by obstacles.\nExpected vertices: {0:?}\nGot vertices: {1:?}",
                       regular_mesh.layers[0].vertices, from_trimesh.layers[0].vertices));

            let adjusted_expectation= wrap_to_first(&expected_vertex.polygons, |polygon| {
                *polygon == adjusted_actual[0]
            })
                .unwrap_or_else(||
                    panic!("vertex {index}: Failed to expected polygons.\nExpected vertices: {0:?}\nGot vertices: {1:?}",
                           regular_mesh.layers[0].vertices, from_trimesh.layers[0].vertices));

            assert_eq!(
                adjusted_expectation, adjusted_actual,
                "\nvertex {index} does not have the expected polygons.\nExpected vertices: {0:?}\nGot vertices: {1:?}",
                regular_mesh.layers[0].vertices, from_trimesh.layers[0].vertices
            );
        }
        Ok(())
    }

    #[test]
    fn isolated_vertex_fails() {
        let trimesh: Result<Mesh, _> = Trimesh {
            vertices: vec![Vec2::new(1., 1.)],
            triangles: vec![],
        }
        .try_into();
        assert!(matches!(trimesh, Err(MeshError::InvalidMesh)));
    }

    fn wrap_to_first(polygons: &[u32], pred: impl Fn(&u32) -> bool) -> Option<Vec<u32>> {
        let offset = polygons.iter().position(pred)?;
        Some(
            polygons
                .iter()
                .skip(offset)
                .chain(polygons.iter().take(offset))
                .cloned()
                .collect(),
        )
    }
}
