use std::collections::VecDeque;

#[cfg(feature = "tracing")]
use tracing::instrument;

pub use geo::LineString;
use geo::{
    BooleanOps, Contains, Coord, CoordsIter, Intersects, MultiPolygon, Polygon as GeoPolygon,
    SimplifyVwPreserve,
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
    /// Obstacles *MUST NOT* overlap. If some obstacles do overlap, use [`Triangulation::merge_overlapping_obstacles`]
    /// before calling [`Triangulation::as_navmesh`].
    pub fn add_obstacle(&mut self, edges: Vec<Vec2>) {
        self.inner.interiors_push(LineString::from(
            edges.iter().map(|v| (v.x, v.y)).collect::<Vec<_>>(),
        ));
    }

    /// Add obstacles delimited by the list of points on their edges.
    ///
    /// Obstacles *MUST NOT* overlap. If some obstacles do overlap, use [`Triangulation::merge_overlapping_obstacles`]
    /// before calling [`Triangulation::as_navmesh`].
    pub fn add_obstacles(&mut self, obstacles: impl IntoIterator<Item = Vec<Vec2>>) {
        let (exterior, interiors) =
            std::mem::replace(&mut self.inner, GeoPolygon::new(LineString(vec![]), vec![]))
                .into_inner();
        self.inner = GeoPolygon::new(
            exterior,
            interiors
                .into_iter()
                .chain(obstacles.into_iter().map(|edges| {
                    LineString(
                        edges
                            .iter()
                            .map(|v| Coord::from((v.x, v.y)))
                            .collect::<Vec<_>>(),
                    )
                }))
                .collect::<Vec<_>>(),
        );
    }

    /// Merge overlapping obstacles.
    ///
    /// This must be called before converting the triangulation into a [`Mesh`] if there are overlapping obstacles,
    /// otherwise it will fail.
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub fn merge_overlapping_obstacles(&mut self) {
        let (exterior, interiors) =
            std::mem::replace(&mut self.inner, GeoPolygon::new(LineString(vec![]), vec![]))
                .into_inner();

        let mut not_intersecting: Vec<LineString<f32>> = vec![];
        let mut intersecting = vec![];
        for poly in interiors.into_iter() {
            intersecting.clear();
            for (i, other) in not_intersecting.iter().enumerate() {
                if poly.intersects(other) {
                    intersecting.push(i);
                }
            }

            if intersecting.is_empty() {
                not_intersecting.push(poly);
            } else {
                #[cfg(feature = "tracing")]
                let _merging_span = tracing::info_span!("merging polygons").entered();

                intersecting.reverse();
                let mut merged: MultiPolygon<f32> = GeoPolygon::new(poly, vec![]).into();
                for other in intersecting.iter() {
                    merged = merged
                        .union(&GeoPolygon::new(not_intersecting.remove(*other), vec![]).into());
                }
                not_intersecting.push(LineString(
                    merged.exterior_coords_iter().collect::<Vec<_>>(),
                ));
            }
        }

        self.inner = GeoPolygon::new(exterior, not_intersecting);
    }

    /// Simplify the outer edge and obstacles, using a topology-preserving variant of the
    /// [Visvalingam-Whyatt algorithm](https://www.tandfonline.com/doi/abs/10.1179/000870493786962263).
    ///
    /// Epsilon is the minimum area a point should contribute to a polygon.
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub fn simplify(&mut self, epsilon: f32) {
        self.inner = self.inner.simplify_vw_preserve(&epsilon);
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline]
    fn add_constraint_edges(
        cdt: &mut ConstrainedDelaunayTriangulation<Point2<f32>>,
        edges: &LineString<f32>,
    ) -> Option<()> {
        let mut edge_iter = edges.coords().peekable();
        let mut vertex_pairs = Vec::new();
        loop {
            let from = edge_iter.next().unwrap();
            let next = edge_iter.peek();

            let point_a = cdt
                .insert(Point2 {
                    x: from.x,
                    y: from.y,
                })
                .unwrap();
            let point_b = if let Some(next) = next {
                vertex_pairs.push((*from, **next));
                cdt.insert(Point2 {
                    x: next.x,
                    y: next.y,
                })
                .unwrap()
            } else {
                vertex_pairs.push((*from, edges[0]));

                cdt.insert(Point2 {
                    x: edges[0].x,
                    y: edges[0].y,
                })
                .unwrap()
            };
            if cdt.can_add_constraint(point_a, point_b) {
                cdt.add_constraint(point_a, point_b);
            } else {
                return None;
            }
            if next.is_none() {
                break;
            }
        }
        Some(())
    }

    /// Convert the triangulation into a [`Mesh`].
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub fn as_navmesh(&self) -> Option<Mesh> {
        let mut cdt = ConstrainedDelaunayTriangulation::<Point2<f32>>::new();
        Triangulation::add_constraint_edges(&mut cdt, self.inner.exterior())?;

        if self
            .inner
            .interiors()
            .iter()
            .any(|obstacle| Triangulation::add_constraint_edges(&mut cdt, obstacle).is_none())
        {
            return None;
        }

        #[cfg(feature = "tracing")]
        let polygon_span = tracing::info_span!("listing polygons").entered();

        let mut face_to_polygon = HashMap::new();
        let polygons = cdt
            .inner_faces()
            .filter_map(|face| {
                #[cfg(feature = "tracing")]
                let _checking_span = tracing::info_span!("checking polygon").entered();

                let center = face.center();
                let center = Coord::from((center.x, center.y));
                self.inner.contains(&center).then(|| {
                    #[cfg(feature = "tracing")]
                    let _preparing_span = tracing::info_span!("preparing polygon").entered();

                    face_to_polygon.insert(face.index(), face_to_polygon.len() as isize);
                    Polygon::new(
                        face.vertices()
                            .iter()
                            .map(|vertex| vertex.index() as u32)
                            .collect(),
                        // TODO: can this be set to the correct value?
                        false,
                    )
                })
            })
            .collect::<Vec<_>>();

        #[cfg(feature = "tracing")]
        drop(polygon_span);

        #[cfg(feature = "tracing")]
        let vertex_span = tracing::info_span!("listing vertices").entered();

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
                let neighbour_polygons: Vec<_> = if neighbour_polygons.iter().all(|i| *i == -1) {
                    vec![-1]
                } else {
                    while neighbour_polygons[0] == -1 {
                        neighbour_polygons.rotate_left(1);
                    }
                    let mut neighbour_polygons: Vec<_> = neighbour_polygons.into();
                    neighbour_polygons.dedup();
                    neighbour_polygons
                };
                let point = point.position();
                Vertex::new(vec2(point.x, point.y), neighbour_polygons)
            })
            .collect::<Vec<_>>();

        #[cfg(feature = "tracing")]
        drop(vertex_span);

        Some(Mesh::new(vertices, polygons))
    }
}
