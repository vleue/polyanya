use crate::{Mesh, Polygon, Vertex};
use glam::Vec2;
use std::cmp::Ordering;
use std::iter;

#[derive(Copy, Clone, Debug, PartialEq)]
/// A triangle described by the indices of three vertices passed to [`Mesh::from_trimesh`] in counterclockwise order
pub struct Triangle(pub [usize; 3]);

impl From<(usize, usize, usize)> for Triangle {
    fn from((a, b, c): (usize, usize, usize)) -> Self {
        Self([a, b, c])
    }
}
impl From<[usize; 3]> for Triangle {
    fn from(vertices: [usize; 3]) -> Self {
        Self(vertices)
    }
}

impl From<&Polygon> for Triangle {
    fn from(value: &Polygon) -> Self {
        let vertices = &value.vertices;
        [vertices[0], vertices[1], vertices[2]]
            .map(|index| index as usize)
            .into()
    }
}

impl Triangle {
    fn contains(self, index: usize) -> bool {
        self.0.contains(&index)
    }

    fn get_clockwise_neighbor(self, index: usize) -> usize {
        let position = self.position(index);
        self.0[(position + 1) % 3]
    }

    fn get_counterclockwise_neighbor(self, index: usize) -> usize {
        let position = self.position(index);
        self.0[(position + 2) % 3]
    }
    fn position(self, index: usize) -> usize {
        self.0.into_iter().position(|i| i == index).unwrap()
    }
}

impl Mesh {
    /// Convert a mesh composed of [`Triangle`]s to a [`Mesh`]. Behaves like [`Mesh::new`], but does not require
    /// any information about vertex or polygon neighbors.
    pub fn from_trimesh(vertices: Vec<Vec2>, triangles: Vec<Triangle>) -> Self {
        let mut vertices: Vec<_> = to_vertices(vertices, &triangles);
        let polygons = to_polygons(triangles);
        let unordered_vertices = vertices.clone();

        // Order vertex polygon neighbors counterclockwise
        for (vertex_index, vertex) in vertices.iter_mut().enumerate() {
            vertex.polygons.sort_by(|index_a, index_b| {
                let get_counterclockwise_edge = |index: isize| {
                    // No -1 present yet, so the unwrap is safe
                    let index = usize::try_from(index).unwrap();
                    let neighbor_index = Triangle::from(&polygons[index])
                        .get_counterclockwise_neighbor(vertex_index);
                    let neighbor = &unordered_vertices[neighbor_index];
                    neighbor.coords - vertex.coords
                };
                let edge_a = get_counterclockwise_edge(*index_a);
                let edge_b = get_counterclockwise_edge(*index_b);
                if edge_a.perp_dot(edge_b) > 0. {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

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
                if last_index == -1 {
                    polygons_including_obstacles.push(polygon_index);
                    continue;
                }
                let triangle_at = |index: isize| {
                    let polygon = &polygons[usize::try_from(index).unwrap()];
                    Triangle::from(polygon)
                };

                let last_counterclockwise_neighbor =
                    triangle_at(last_index).get_counterclockwise_neighbor(vertex_index);
                let next_clockwise_neighbor =
                    triangle_at(polygon_index).get_clockwise_neighbor(vertex_index);

                if last_counterclockwise_neighbor != next_clockwise_neighbor {
                    // The edges don't align; there's an obstacle here
                    polygons_including_obstacles.push(-1);
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

fn to_vertices(vertices: Vec<Vec2>, triangles: &[Triangle]) -> Vec<Vertex> {
    vertices
        .into_iter()
        .enumerate()
        .map(|(vertex_index, coords)| {
            let neighbor_indices = triangles
                .iter()
                .enumerate()
                .filter_map(|(polygon_index, vertex_indices_in_polygon)| {
                    vertex_indices_in_polygon
                        .contains(vertex_index)
                        .then_some(polygon_index)
                })
                .map(|index| isize::try_from(index).unwrap())
                .collect();
            Vertex::new(coords, neighbor_indices)
        })
        .collect()
}

fn to_polygons(triangles: Vec<Triangle>) -> Vec<Polygon> {
    let triangle_count = triangles.len();

    triangles
        .into_iter()
        .map(|vertex_indices_in_polygon| {
            // It's geometrically impossible for a triangle to have only one neighbor in a trimesh,
            // except for the trivial case
            let is_one_way = triangle_count == 1;
            Polygon::new(
                vertex_indices_in_polygon
                    .0
                    .map(|index| index as u32)
                    .to_vec(),
                is_one_way,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_from_trimesh_is_same_as_regular() {
        let regular_mesh = Mesh::new(
            vec![
                Vertex::new(Vec2::new(1., 1.), vec![0, 4, -1]), // 0
                Vertex::new(Vec2::new(5., 1.), vec![-1, 1, 3, -1, 0]), // 1
                Vertex::new(Vec2::new(5., 4.), vec![-1, 2, 1]), // 2
                Vertex::new(Vec2::new(1., 4.), vec![-1, 4, -1, 3, 2]), // 3
                Vertex::new(Vec2::new(2., 2.), vec![-1, 4, 0]), // 4
                Vertex::new(Vec2::new(4., 3.), vec![1, 2, 3]),  // 5
            ],
            vec![
                Polygon::new(vec![0, 1, 4], false), // 0
                Polygon::new(vec![1, 2, 5], false), // 1
                Polygon::new(vec![5, 2, 3], false), // 2
                Polygon::new(vec![1, 5, 3], false), // 3
                Polygon::new(vec![0, 4, 3], false), // 4
            ],
        );
        let from_trimesh = Mesh::from_trimesh(
            vec![
                Vec2::new(1., 1.),
                Vec2::new(5., 1.),
                Vec2::new(5., 4.),
                Vec2::new(1., 4.),
                Vec2::new(2., 2.),
                Vec2::new(4., 3.),
            ],
            vec![
                (0, 1, 4).into(),
                (1, 2, 5).into(),
                (5, 2, 3).into(),
                (1, 5, 3).into(),
                (0, 4, 3).into(),
            ],
        );
        assert_eq!(regular_mesh.polygons, from_trimesh.polygons);
        for (index, (expected_vertex, actual_vertex)) in regular_mesh
            .vertices
            .iter()
            .zip(from_trimesh.vertices.iter())
            .enumerate()
        {
            assert_eq!(
                expected_vertex.coords, actual_vertex.coords,
                "\nvertex {index} does not have the expected coords.\nExpected vertices: {0:?}\nGot vertices: {1:?}",
                regular_mesh.vertices, from_trimesh.vertices
            );

            assert_eq!(
                expected_vertex.is_corner, actual_vertex.is_corner,
                "\nvertex {index} does not have the expected value for `is_corner`.\nExpected vertices: {0:?}\nGot vertices: {1:?}",
                regular_mesh.vertices, from_trimesh.vertices
            );
            let adjusted_actual = wrap_to_first(&actual_vertex.polygons, |index| *index != -1).unwrap_or_else(||
                panic!("vertex {index}: Found only surrounded by obstacles.\nExpected vertices: {0:?}\nGot vertices: {1:?}",
                       regular_mesh.vertices, from_trimesh.vertices));

            let adjusted_expectation= wrap_to_first(&expected_vertex.polygons, |polygon| {
                *polygon == adjusted_actual[0]
            })
                .unwrap_or_else(||
                    panic!("vertex {index}: Failed to expected polygons.\nExpected vertices: {0:?}\nGot vertices: {1:?}",
                           regular_mesh.vertices, from_trimesh.vertices));

            assert_eq!(
                adjusted_expectation, adjusted_actual,
                "\nvertex {index} does not have the expected polygons.\nExpected vertices: {0:?}\nGot vertices: {1:?}",
                regular_mesh.vertices, from_trimesh.vertices
            );
        }
    }

    fn wrap_to_first(polygons: &[isize], pred: impl Fn(&isize) -> bool) -> Option<Vec<isize>> {
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
