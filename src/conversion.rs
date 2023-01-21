use crate::{Mesh, Polygon, Vertex};
use glam::Vec2;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::iter;

#[derive(Copy, Clone, Debug, PartialEq)]
/// A triangle described by the indices of three vertices passed to `Mesh::from_trimesh`
pub struct VertexIndices {
    /// The index of a vertex
    pub a: usize,
    /// The index of a vertex
    pub b: usize,
    /// The index of a vertex
    pub c: usize,
}

impl From<(usize, usize, usize)> for VertexIndices {
    fn from((a, b, c): (usize, usize, usize)) -> Self {
        Self { a, b, c }
    }
}

impl VertexIndices {
    fn into_array(self) -> [usize; 3] {
        [self.a, self.b, self.c]
    }
    fn contains(self, index: usize) -> bool {
        self.into_array().contains(&index)
    }
}

struct UnrolledTriangle(VertexIndices);

impl UnrolledTriangle {
    fn get_sorted_neighbors(&self, vertices: &[Vertex]) -> (usize, usize) {
        let mut other_indices = [self.0.a, self.0.c];
        let own_coords = vertices[self.0.b].coords;
        other_indices.sort_by(|index_a, index_b| {
            let edge_a = vertices[*index_a].coords - own_coords;
            let edge_b = vertices[*index_b].coords - own_coords;
            if edge_a.perp_dot(edge_b) < 0. {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        (other_indices[0], other_indices[1])
    }
    fn get_counterclockwise_neighbor(&self, vertices: &[Vertex]) -> usize {
        self.get_sorted_neighbors(vertices).0
    }

    fn get_clockwise_neighbor(&self, vertices: &[Vertex]) -> usize {
        self.get_sorted_neighbors(vertices).1
    }
}

impl Mesh {
    /// Convert a mesh composed of triangles to a `Mesh`. Behaves like `Mesh::new`, but does not require
    /// any information on neighbors.
    pub fn from_trimesh(vertices: Vec<Vec2>, triangles: Vec<VertexIndices>) -> Self {
        let mut vertices: Vec<_> = vertices
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
            .collect();
        let triangle_count = triangles.len();
        let polygons: Vec<_> = triangles
            .into_iter()
            .map(|vertex_indices_in_polygon| {
                // It's geometrically impossible for a triangle to have only one neighbor in a trimesh,
                // except for the trivial case
                let is_one_way = triangle_count == 1;
                Polygon::new(
                    vertex_indices_in_polygon
                        .into_array()
                        .map(|index| index as u32)
                        .to_vec(),
                    is_one_way,
                )
            })
            .collect();
        let unordered_vertices = vertices.clone();
        for (vertex_index, vertex) in vertices.iter_mut().enumerate() {
            vertex.polygons.sort_by(|index_a, index_b| {
                let get_counterclockwise_edge = |index: isize| {
                    // No -1 present yet, so the unwrap is safe
                    let index = usize::try_from(index).unwrap();
                    let neighbor_index = polygons[index]
                        .unroll_triangle_at(vertex_index)
                        .unwrap()
                        .get_counterclockwise_neighbor(&unordered_vertices);
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
                let unroll = |index: isize| {
                    let polygon = &polygons[usize::try_from(index).unwrap()];
                    polygon.unroll_triangle_at(vertex_index).unwrap()
                };

                let last_counterclockwise_neighbor =
                    unroll(last_index).get_counterclockwise_neighbor(&unordered_vertices);
                let next_clockwise_neighbor =
                    unroll(polygon_index).get_clockwise_neighbor(&unordered_vertices);

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

impl Polygon {
    fn unroll_triangle_at(&self, vertex_index: usize) -> Option<UnrolledTriangle> {
        self.vertices
            .iter()
            .chain(self.vertices.iter().take(2))
            .tuple_windows()
            .map(|(a, b, c)| (*a as usize, *b as usize, *c as usize))
            .map(VertexIndices::from)
            .find(|triangle| triangle.b == vertex_index)
            .map(UnrolledTriangle)
    }
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
