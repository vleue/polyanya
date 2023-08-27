use std::collections::VecDeque;

use geo_offset::Offset;
#[cfg(feature = "tracing")]
use tracing::instrument;

pub use geo::LineString;
use geo::{
    BooleanOps, Contains, Coord, CoordsIter, Intersects, MultiPolygon, Polygon as GeoPolygon,
    SimplifyVwPreserve,
};
use glam::{vec2, Vec2};
use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation as SpadeTriangulation};

use crate::{Mesh, Polygon, Vertex};

/// An helper to create a [`Mesh`] from a list of edges and obstacle, using a constrained Delaunay triangulation.
#[derive(Debug, Clone)]
pub struct Triangulation {
    inner: GeoPolygon<f32>,
    unit_radius: f32,
}

impl Triangulation {
    /// Create a new triangulation from a the list of points on its outer edges.
    pub fn from_outer_edges(edges: &[Vec2]) -> Triangulation {
        Self {
            inner: GeoPolygon::new(
                LineString::from(edges.iter().map(|v| (v.x, v.y)).collect::<Vec<_>>()),
                vec![],
            ),
            unit_radius: 0.0,
        }
    }

    /// Zut
    pub fn set_unit_radius(&mut self, radius: f32) {
        self.unit_radius = radius;
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
        for poly in interiors.into_iter() {
            let intersecting = not_intersecting
                .iter()
                .enumerate()
                .filter(|(_, other)| poly.intersects(*other))
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            if intersecting.is_empty() {
                not_intersecting.push(poly);
            } else {
                #[cfg(feature = "tracing")]
                let _merging_span = tracing::info_span!("merging polygons").entered();

                let mut merged = MultiPolygon::<f32>(
                    intersecting
                        .iter()
                        .rev()
                        .map(|other| GeoPolygon::new(not_intersecting.remove(*other), vec![]))
                        .collect(),
                );
                merged = merged.union(&GeoPolygon::new(poly, vec![]).into());
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
                cdt.insert(Point2 {
                    x: next.x,
                    y: next.y,
                })
                .unwrap()
            } else {
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
    ///
    /// Meshes generated are not [baked](Mesh::bake), as they are made of triangles and it is recommended to
    /// call [`Mesh::merge_polygons`] on them before baking.
    ///
    /// ```
    /// # use glam::vec2;
    /// # use polyanya::Triangulation;
    /// # let triangulation = Triangulation::from_outer_edges(&[vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(0.0, 1.0)]);
    /// let mut mesh = triangulation.as_navmesh().unwrap();
    ///
    /// // Merge polygons at least once before baking.
    /// mesh.merge_polygons();
    ///
    /// // One call to merge should have reduced the number of polygons, baking will be less expensive.
    /// mesh.bake();
    /// ```
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub fn as_navmesh(&self) -> Option<Mesh> {
        let buffered: GeoPolygon<f32> = if self.unit_radius != 0.0 {
            let radiused = self
                .inner
                .offset_with_arc_segments(-self.unit_radius, 5)
                .unwrap();
            radiused.0[0].simplify_vw_preserve(&(self.unit_radius / 100.0))
        } else {
            self.inner.clone()
        };
        let mut cdt = ConstrainedDelaunayTriangulation::<Point2<f32>>::new();
        Triangulation::add_constraint_edges(
            &mut cdt,
            &LineString(buffered.exterior_coords_iter().collect()),
        )?;

        if buffered.interiors().iter().any(|obstacle| {
            let obstacle = LineString::<f32>(obstacle.0.iter().cloned().collect());
            Triangulation::add_constraint_edges(&mut cdt, &obstacle).is_none()
        }) {
            return None;
        }

        #[cfg(feature = "tracing")]
        let polygon_span = tracing::info_span!("listing polygons").entered();

        let mut face_to_polygon: Vec<isize> = vec![-1; cdt.all_faces().len()];
        let mut i = 0;
        let polygons = cdt
            .inner_faces()
            .filter_map(|face| {
                #[cfg(feature = "tracing")]
                let _checking_span = tracing::info_span!("checking polygon").entered();

                let center = face.center();
                let center = Coord::from((center.x, center.y));
                buffered.contains(&center).then(|| {
                    #[cfg(feature = "tracing")]
                    let _preparing_span = tracing::info_span!("preparing polygon").entered();

                    face_to_polygon[face.index()] = i;
                    i += 1;
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
                #[cfg(feature = "tracing")]
                let _preparing_span = tracing::info_span!("preparing vertex").entered();

                let mut neighbour_polygons = point
                    .out_edges()
                    .map(|out_edge| face_to_polygon[out_edge.face().index()])
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

        Some(Mesh {
            vertices,
            polygons,
            ..Default::default()
        })
    }
}
