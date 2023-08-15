use std::collections::VecDeque;

use glam::{vec2, Vec2};
use hashbrown::HashMap;
use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation as SpadeTriangulation};

use crate::{
    helpers::{line_intersect_segment, Vec2Helper},
    Mesh, Polygon, Vertex,
};

/// An helper to create a [`Mesh`] from a list of edges and obstacle, using a constrained Delaunay triangulation.
#[derive(Debug, Clone)]
pub struct Triangulation {
    edges: Vec<Vec2>,
    obstacles: Vec<Vec<Vec2>>,
}

impl Triangulation {
    /// Create a new triangulation from a the list of points on its outer edges.
    pub fn from_outer_edges(edges: Vec<Vec2>) -> Triangulation {
        Self {
            edges,
            obstacles: Default::default(),
        }
    }

    /// Add an obstacle delimited by the list of points on its edges.
    ///
    /// Obstacles *MUST NOT* overlap.
    pub fn add_obstacle(&mut self, edges: Vec<Vec2>) {
        self.obstacles.push(edges);
    }

    #[inline]
    fn add_constraint_edges(
        cdt: &mut ConstrainedDelaunayTriangulation<Point2<f32>>,
        edges: Vec<Vec2>,
    ) -> (Vec<(Vec2, Vec2)>, (Vec2, Vec2)) {
        let mut edge_iter = edges.iter().peekable();
        let mut vertex_pairs = Vec::new();
        let mut aabb_min = edges[0];
        let mut aabb_max = edges[0];
        loop {
            let from = edge_iter.next().unwrap();
            let next = edge_iter.peek();

            aabb_min = aabb_min.min(*from);
            aabb_max = aabb_max.max(*from);

            if let Some(next) = next {
                cdt.add_constraint_edge(
                    Point2 {
                        x: from.x,
                        y: from.y,
                    },
                    Point2 {
                        x: next.x,
                        y: next.y,
                    },
                )
                .unwrap();
                vertex_pairs.push((*from, **next));
            } else {
                cdt.add_constraint_edge(
                    Point2 {
                        x: from.x,
                        y: from.y,
                    },
                    Point2 {
                        x: edges[0].x,
                        y: edges[0].y,
                    },
                )
                .unwrap();
                vertex_pairs.push((*from, edges[0]));
                break;
            }
        }

        (vertex_pairs, (aabb_min, aabb_max))
    }
}

// Check if a point is in a polygon, first by checking its AABB, then by checking the number of times
// a ray from the point to the top of the AABB intersects the polygon.
#[inline]
fn in_polygon(point: Vec2, edges: &[(Vec2, Vec2)], aabb: (Vec2, Vec2)) -> bool {
    if point.x < aabb.0.x || point.x > aabb.1.x || point.y < aabb.0.y || point.y > aabb.1.y {
        return false;
    }
    let mut parallel = 0;
    let intersect = edges
        .iter()
        .filter(|edge| {
            let start = point;
            let far = point + vec2(0.0, aabb.1.y + 1.0);
            if edge.0.on_segment((start, far)) && edge.1.on_segment((start, far)) {
                parallel += 1;
            }
            line_intersect_segment((start, far), **edge)
                .map(|i| i.y > start.y)
                .unwrap_or(false)
        })
        .count();
    (intersect - parallel) % 2 == 1
}

impl From<Triangulation> for Mesh {
    fn from(value: Triangulation) -> Self {
        let mut cdt = ConstrainedDelaunayTriangulation::<Point2<f32>>::new();
        let (outer_edges, aabb_outer) = Triangulation::add_constraint_edges(&mut cdt, value.edges);

        let mut obstacles = vec![];
        for obstacle in value.obstacles {
            obstacles.push(Triangulation::add_constraint_edges(&mut cdt, obstacle));
        }

        let mut face_to_polygon = HashMap::new();
        let polygons = cdt
            .inner_faces()
            .filter_map(|face| {
                let center = face.center();
                let center = vec2(center.x, center.y);
                (in_polygon(center, &outer_edges, aabb_outer)
                    && obstacles
                        .iter()
                        .map(|(edges, aabb)| !in_polygon(center, edges, *aabb))
                        .all(|b| b))
                .then(|| {
                    face_to_polygon.insert(face.index(), face_to_polygon.len() as isize);
                    Polygon::new(
                        face.vertices()
                            .iter()
                            .map(|vertex| vertex.index() as u32)
                            .collect(),
                        // TODO: can this be set to the correct value
                        false,
                    )
                })
            })
            .collect::<Vec<_>>();

        let vertices = cdt
            .vertices()
            .map(|point| {
                let mut neighbour_polygons = point
                    .out_edges()
                    .map(|out_edge| {
                        face_to_polygon
                            .get(&out_edge.face().index())
                            .cloned()
                            .unwrap_or(-1)
                    })
                    .collect::<VecDeque<_>>();
                while neighbour_polygons[0] == -1 {
                    neighbour_polygons.rotate_left(1);
                }
                let mut neighbour_polygons: Vec<_> = neighbour_polygons.into();
                neighbour_polygons.dedup();
                let point = point.position();
                Vertex::new(vec2(point.x, point.y), neighbour_polygons)
            })
            .collect::<Vec<_>>();

        Mesh::new(vertices, polygons)
    }
}
