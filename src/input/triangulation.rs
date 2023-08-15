use std::collections::VecDeque;

pub use geo::LineString;
use geo::{
    BooleanOps, Contains, Coord, CoordsIter, Intersects, MultiPolygon, Polygon as GeoPolygon,
};
use glam::{vec2, Vec2};
use hashbrown::HashMap;
use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation as SpadeTriangulation};

use crate::{Mesh, Polygon, Vertex};

/// An helper to create a [`Mesh`] from a list of edges and obstacle, using a constrained Delaunay triangulation.
#[derive(Debug, Clone)]
pub struct Triangulation {
    inner: GeoPolygon<f32>,
}

impl Triangulation {
    /// Create a new triangulation from a the list of points on its outer edges.
    pub fn from_outer_edges(edges: Vec<Vec2>) -> Triangulation {
        Self {
            inner: GeoPolygon::new(
                LineString::from(edges.iter().map(|v| (v.x, v.y)).collect::<Vec<_>>()),
                vec![],
            ),
        }
    }

    /// Add an obstacle delimited by the list of points on its edges.
    ///
    /// Obstacles *MUST NOT* overlap.
    pub fn add_obstacle(&mut self, edges: Vec<Vec2>) {
        self.inner.interiors_push(LineString::from(
            edges.iter().map(|v| (v.x, v.y)).collect::<Vec<_>>(),
        ));
    }

    #[inline]
    fn add_constraint_edges(
        cdt: &mut ConstrainedDelaunayTriangulation<Point2<f32>>,
        edges: &LineString<f32>,
    ) {
        let mut edge_iter = edges.coords().peekable();
        let mut vertex_pairs = Vec::new();
        loop {
            let from = edge_iter.next().unwrap();
            let next = edge_iter.peek();

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
    }
}

impl From<Triangulation> for Mesh {
    fn from(value: Triangulation) -> Self {
        let mut cdt = ConstrainedDelaunayTriangulation::<Point2<f32>>::new();
        Triangulation::add_constraint_edges(&mut cdt, value.inner.exterior());

        value.inner.interiors().iter().for_each(|obstacle| {
            Triangulation::add_constraint_edges(&mut cdt, &obstacle);
        });

        let mut face_to_polygon = HashMap::new();
        let polygons = cdt
            .inner_faces()
            .filter_map(|face| {
                let center = face.center();
                let center = Coord::from((center.x, center.y));
                value.inner.contains(&center).then(|| {
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
