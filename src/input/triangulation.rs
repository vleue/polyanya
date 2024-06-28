use std::collections::VecDeque;

#[cfg(feature = "tracing")]
use tracing::instrument;

pub use geo::LineString;
use geo::{Contains, Coord, Polygon as GeoPolygon, SimplifyVwPreserve};
use glam::{vec2, Vec2};
use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation as SpadeTriangulation};

use crate::{Mesh, Polygon, Vertex};

/// An helper to create a [`Mesh`] from a list of edges and obstacle, using a constrained Delaunay triangulation.
#[derive(Debug, Clone)]
pub struct Triangulation {
    inner: GeoPolygon<f32>,
}

impl Triangulation {
    /// Create a new triangulation from a the list of points on its outer edges.
    pub fn from_outer_edges(edges: &[Vec2]) -> Triangulation {
        Self {
            inner: GeoPolygon::new(
                LineString::from(edges.iter().map(|v| (v.x, v.y)).collect::<Vec<_>>()),
                vec![],
            ),
        }
    }

    /// Add an obstacle delimited by the list of points on its edges.
    pub fn add_obstacle(&mut self, edges: Vec<Vec2>) {
        self.inner.interiors_push(LineString::from(
            edges.iter().map(|v| (v.x, v.y)).collect::<Vec<_>>(),
        ));
    }

    /// Add obstacles delimited by the list of points on their edges.
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
    ) {
        let mut edge_iter = edges.coords().peekable();
        let mut next_point = None;
        loop {
            let from = edge_iter.next().unwrap();
            let next = edge_iter.peek();

            let point_a = next_point.unwrap_or_else(|| {
                cdt.insert(Point2 {
                    x: from.x,
                    y: from.y,
                })
                .unwrap()
            });
            let point_b = if let Some(next) = next {
                cdt.insert(Point2 {
                    x: next.x,
                    y: next.y,
                })
                .unwrap()
            } else {
                break;
            };
            cdt.add_constraint_and_split(point_a, point_b, |v| v);
            next_point = Some(point_b);
        }
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
    /// let mut mesh = triangulation.as_navmesh();
    ///
    /// // Merge polygons at least once before baking.
    /// mesh.merge_polygons();
    ///
    /// // One call to merge should have reduced the number of polygons, baking will be less expensive.
    /// mesh.bake();
    /// ```
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub fn as_navmesh(&self) -> Mesh {
        let mut cdt = ConstrainedDelaunayTriangulation::<Point2<f32>>::new();
        Triangulation::add_constraint_edges(&mut cdt, self.inner.exterior());

        self.inner
            .interiors()
            .iter()
            .for_each(|obstacle| Triangulation::add_constraint_edges(&mut cdt, obstacle));

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
                self.inner.contains(&center).then(|| {
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

        Mesh {
            vertices,
            polygons,
            ..Default::default()
        }
    }
}
