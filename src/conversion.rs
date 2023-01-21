use crate::{Mesh, Polygon, Vertex};
use glam::Vec2;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::iter;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VertexIndices {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl VertexIndices {
    fn from_tuple((a, b, c): (usize, usize, usize)) -> Self {
        Self { a, b, c }
    }
    fn into_array(self) -> [usize; 3] {
        [self.a, self.b, self.c]
    }
    fn contains(self, index: usize) -> bool {
        self.into_array().contains(&index)
    }
}

struct UnrolledTriangle(VertexIndices);

impl UnrolledTriangle {
    fn get_sorted_counterclockwise(&self, vertices: &[Vertex]) -> (usize, usize) {
        let mut other_indices = [self.0.a, self.0.c];
        let own_coords = vertices[self.0.b].coords;
        other_indices.sort_by_key(|index| {
            let dir = vertices[*index].coords - own_coords;
            OrderedFloat(dir.y.atan2(dir.x))
        });
        (other_indices[0], other_indices[1])
    }
    fn get_counterclockwise_neighbor(&self, vertices: &[Vertex]) -> usize {
        self.get_sorted_counterclockwise(vertices).0
    }

    fn get_clockwise_neighbor(&self, vertices: &[Vertex]) -> usize {
        self.get_sorted_counterclockwise(vertices).1
    }
}

fn from_trimesh(vertices: Vec<Vec2>, triangles: Vec<VertexIndices>) -> Mesh {
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
    let polygons: Vec<_> = triangles
        .into_iter()
        .map(|vertex_indices_in_polygon| {
            let is_one_way = vertex_indices_in_polygon
                .into_array()
                .iter()
                .map(|index| &vertices[*index as usize])
                .map(|vertex| &vertex.polygons)
                .flatten()
                .unique()
                .take(3)
                .count()
                // One way means all vertices have at most 2 neighbors: the original polygon and one other
                < 3;
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
        vertex.polygons.sort_by_key(|index| {
            // No -1 present yet, so the unwrap is safe
            let index = usize::try_from(*index).unwrap();
            let polygon = &polygons[index];
            let unrolled_indices = polygon.unroll_triangle_at(vertex_index).unwrap();
            let triangle_center_direction: Vec2 = unrolled_indices
                .0
                .into_array()
                .into_iter()
                .map(|index| &unordered_vertices[index])
                .map(|vertex| vertex.coords)
                .sum();
            let angle_to_positive_x_axis = triangle_center_direction
                .y
                .atan2(triangle_center_direction.x);
            OrderedFloat(angle_to_positive_x_axis)
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
            let last_polygon = &polygons[usize::try_from(last_index).unwrap()];
            let last_counterclockwise_neighbor = last_polygon
                .unroll_triangle_at(vertex_index)
                .unwrap()
                .get_counterclockwise_neighbor(&unordered_vertices);

            let next_polygon = &polygons[usize::try_from(polygon_index).unwrap()];
            let next_clockwise_neighbor = next_polygon
                .unroll_triangle_at(vertex_index)
                .unwrap()
                .get_clockwise_neighbor(&unordered_vertices);
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
    Mesh::new(vertices, polygons)
}

impl Polygon {
    fn do_edges_on_corner_align(&self, other: &Self, vertex: usize) -> Option<bool> {
        let unrolled_self = self.unroll_triangle_at(vertex)?;
        let unrolled_other = other.unroll_triangle_at(vertex)?;
        Some(unrolled_self.0.a == unrolled_other.0.c || unrolled_self.0.c == unrolled_other.0.a)
    }

    fn unroll_triangle_at(&self, vertex_index: usize) -> Option<UnrolledTriangle> {
        self.vertices
            .iter()
            .chain(self.vertices.iter().take(2))
            .tuple_windows()
            .map(|(a, b, c)| (*a as usize, *b as usize, *c as usize))
            .map(VertexIndices::from_tuple)
            .find(|triangle| triangle.b == vertex_index)
            .map(|triangle| UnrolledTriangle(triangle))
    }
}
