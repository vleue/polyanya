use std::collections::{HashMap, HashSet, VecDeque};

use geo_offset::Offset;
#[cfg(feature = "tracing")]
use tracing::instrument;

pub use geo::LineString;
use geo::{Contains, Coord, Polygon as GeoPolygon, SimplifyVwPreserve};
use glam::{vec2, Vec2};
use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation as SpadeTriangulation};

use crate::{Layer, Mesh, Polygon, Vertex};

/// An helper to create a [`Mesh`] from a list of edges and obstacle, using a constrained Delaunay triangulation.
#[derive(Clone)]
pub struct Triangulation {
    inner: GeoPolygon<f32>,
    agent_radius: f32,
    prebuilt: Option<(
        GeoPolygon<f32>,
        ConstrainedDelaunayTriangulation<Point2<f64>>,
    )>,
    base_layer: Option<Layer>,
}

impl std::fmt::Debug for Triangulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Triangulation")
            .field("inner", &self.inner)
            .field("prebuilt", &self.prebuilt.is_some())
            .finish()
    }
}

impl Triangulation {
    /// Create a new triangulation from a the list of points on its outer edges.
    pub fn from_outer_edges(edges: &[Vec2]) -> Triangulation {
        Self {
            inner: GeoPolygon::new(
                LineString::from(edges.iter().map(|v| (v.x, v.y)).collect::<Vec<_>>()),
                vec![],
            ),
            agent_radius: 0.0,
            prebuilt: None,
            base_layer: None,
        }
    }

    /// Create a new triangulation from an existing `Mesh`, cloning the specified [`Layer`].
    pub fn from_mesh(mesh: &Mesh, layer: u8) -> Triangulation {
        Self {
            inner: GeoPolygon::new(LineString::new(Vec::new()), vec![]),
            agent_radius: 0.0,
            prebuilt: None,
            base_layer: Some(mesh.layers[layer as usize].clone()),
        }
    }

    /// Create a new triangulation from an existing `Layer` of a [`Mesh`].
    pub fn from_mesh_layer(layer: Layer) -> Triangulation {
        Self {
            inner: GeoPolygon::new(LineString::new(Vec::new()), vec![]),
            agent_radius: 0.0,
            prebuilt: None,
            base_layer: Some(layer),
        }
    }

    pub fn set_agent_radius(&mut self, radius: f32) {
        self.agent_radius = radius;
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
        self.inner.interiors_mut(|interiors| {
            for interior in interiors {
                *interior = interior.simplify_vw_preserve(&epsilon);
            }
        });
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline]
    fn add_constraint_edges(
        cdt: &mut ConstrainedDelaunayTriangulation<Point2<f64>>,
        edges: &LineString<f32>,
    ) {
        if edges.0.is_empty() {
            return;
        }
        for line in edges.lines() {
            let from = line.start;
            let next = line.end;

            let point_a = cdt
                .insert(Point2 {
                    x: from.x as f64,
                    y: from.y as f64,
                })
                .unwrap();
            let point_b = cdt
                .insert(Point2 {
                    x: next.x as f64,
                    y: next.y as f64,
                })
                .unwrap();
            cdt.add_constraint_and_split(point_a, point_b, |v| v);
        }
    }

    /// Prebuild part of the navmesh with the already added obstacles.
    ///
    /// This can be used to cache part of the navmesh generation when some of the obstacles won't change.
    pub fn prebuild(&mut self) {
        if self.base_layer.is_some() {
            return;
        }
        let inner = if self.agent_radius != 0.0 {
            &self
                .inner
                .offset_with_arc_segments(-self.agent_radius, 5)
                .unwrap()
                .0[0]
        } else {
            &self.inner
        };

        let mut cdt = ConstrainedDelaunayTriangulation::<Point2<f64>>::new();
        Triangulation::add_constraint_edges(&mut cdt, inner.exterior());

        inner
            .interiors()
            .iter()
            .for_each(|obstacle| Triangulation::add_constraint_edges(&mut cdt, obstacle));

        let exterior = inner.exterior().clone();
        let mut used = std::mem::replace(&mut self.inner, GeoPolygon::new(exterior, vec![]));
        if let Some((previous, _)) = self.prebuilt.take() {
            let (_, inners) = previous.into_inner();
            for interior in inners {
                used.interiors_push(interior);
            }
        }
        self.prebuilt = Some((used, cdt));
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
        Mesh {
            layers: vec![self.as_layer()],
            ..Default::default()
        }
    }

    /// Convert the triangulation into a [`Layer`].
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub fn as_layer(&self) -> Layer {
        let inner = if self.agent_radius != 0.0 {
            &self
                .inner
                .offset_with_arc_segments(-self.agent_radius, 5)
                .unwrap()
                .0[0]
        } else {
            &self.inner
        };

        let mut cdt = if self.prebuilt.is_none() {
            let mut cdt = ConstrainedDelaunayTriangulation::<Point2<f64>>::new();
            Triangulation::add_constraint_edges(&mut cdt, inner.exterior());
            cdt
        } else {
            self.prebuilt.as_ref().unwrap().1.clone()
        };
        let used = self.prebuilt.as_ref().map(|(used, _)| used);

        if let Some(base_layer) = &self.base_layer {
            let mut added_vertices = HashMap::new();
            let mut added_edges = HashSet::new();
            for polygon in &base_layer.polygons {
                polygon.edges_index().for_each(|[p0, p1]| {
                    if !added_edges.insert((p0, p1)) || !added_edges.insert((p1, p0)) {
                    } else {
                        let p0 = *added_vertices.entry(p0).or_insert_with(|| {
                            let a: Vec2 = base_layer.vertices[p0 as usize].coords;
                            cdt.insert(Point2 {
                                x: a.x as f64,
                                y: a.y as f64,
                            })
                            .unwrap()
                        });
                        let p1 = *added_vertices.entry(p1).or_insert_with(|| {
                            let b = base_layer.vertices[p1 as usize].coords;
                            cdt.insert(Point2 {
                                x: b.x as f64,
                                y: b.y as f64,
                            })
                            .unwrap()
                        });
                        cdt.add_constraint_and_split(p0, p1, |v| v);
                    }
                });
            }
        }

        inner
            .interiors()
            .iter()
            .for_each(|obstacle| Triangulation::add_constraint_edges(&mut cdt, obstacle));

        #[cfg(feature = "tracing")]
        let polygon_span = tracing::info_span!("listing polygons").entered();

        let mut face_to_polygon: Vec<u32> = vec![u32::MAX; cdt.all_faces().len()];
        let mut i = 0;
        let polygons = cdt
            .inner_faces()
            .filter_map(|face| {
                #[cfg(feature = "tracing")]
                let _checking_span = tracing::info_span!("checking polygon").entered();

                let center = face.center();
                let center = Coord::from((center.x as f32, center.y as f32));

                ((used.map(|used| used.contains(&center)).unwrap_or(true)
                    && inner.contains(&center))
                    || (self.base_layer.is_some()
                        && self
                            .base_layer
                            .as_ref()
                            .map(|base_layer| {
                                base_layer
                                    .get_point_location(vec2(center.x, center.y), 0.0)
                                    .is_some()
                            })
                            .unwrap_or(true)
                        && !self.inner.interiors().iter().any(|obstacle| {
                            GeoPolygon::new(obstacle.clone(), vec![]).contains(&center)
                        })))
                .then(|| {
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
                        // look at each neighboring polygons based on the face vertices
                        // if there are only two => it's one way
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
                let neighbour_polygons: Vec<_> =
                    if neighbour_polygons.iter().all(|i| *i == u32::MAX) {
                        vec![u32::MAX]
                    } else {
                        while neighbour_polygons[0] == u32::MAX {
                            neighbour_polygons.rotate_left(1);
                        }
                        let mut neighbour_polygons: Vec<_> = neighbour_polygons.into();
                        neighbour_polygons.dedup();
                        neighbour_polygons
                    };
                let point = point.position();
                Vertex::new(vec2(point.x as f32, point.y as f32), neighbour_polygons)
            })
            .collect::<Vec<_>>();

        #[cfg(feature = "tracing")]
        drop(vertex_span);

        Layer {
            vertices,
            polygons,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangulation() {
        let mut triangulation = Triangulation::from_outer_edges(&[
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(1.0, 1.0),
            vec2(0.0, 1.0),
        ]);
        triangulation.add_obstacle(vec![
            vec2(0.0, 0.25),
            vec2(0.25, 0.25),
            vec2(0.25, 0.0),
            vec2(0.0, 0.0),
        ]);
        triangulation.add_obstacle(vec![
            vec2(1.0, 0.75),
            vec2(0.75, 0.75),
            vec2(0.75, 1.0),
            vec2(1.0, 1.0),
        ]);
        let mesh = triangulation.as_navmesh();
        assert_eq!(
            mesh.layers[0]
                .vertices
                .iter()
                .map(|v| v.coords)
                .collect::<Vec<_>>(),
            vec![
                vec2(0.0, 0.0),
                vec2(1.0, 0.0),
                vec2(1.0, 1.0),
                vec2(0.0, 1.0),
                vec2(0.0, 0.25),
                vec2(0.25, 0.25),
                vec2(0.25, 0.0),
                vec2(1.0, 0.75),
                vec2(0.75, 0.75),
                vec2(0.75, 1.0)
            ]
        );
        assert_eq!(
            mesh.layers[0]
                .polygons
                .iter()
                .map(|v| v.vertices.clone())
                .collect::<Vec<_>>(),
            vec![
                [5, 1, 8],
                [4, 5, 3],
                [5, 6, 1],
                [8, 3, 5],
                [7, 8, 1],
                [8, 9, 3]
            ]
        );
    }

    #[test]
    fn triangulation_prebuilt() {
        let mut triangulation = Triangulation::from_outer_edges(&[
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(1.0, 1.0),
            vec2(0.0, 1.0),
        ]);
        triangulation.add_obstacle(vec![
            vec2(0.0, 0.25),
            vec2(0.25, 0.25),
            vec2(0.25, 0.0),
            vec2(0.0, 0.0),
        ]);
        triangulation.prebuild();
        triangulation.add_obstacle(vec![
            vec2(1.0, 0.75),
            vec2(0.75, 0.75),
            vec2(0.75, 1.0),
            vec2(1.0, 1.0),
        ]);
        let mesh = triangulation.as_navmesh();
        assert_eq!(
            mesh.layers[0]
                .vertices
                .iter()
                .map(|v| v.coords)
                .collect::<Vec<_>>(),
            vec![
                vec2(0.0, 0.0),
                vec2(1.0, 0.0),
                vec2(1.0, 1.0),
                vec2(0.0, 1.0),
                vec2(0.0, 0.25),
                vec2(0.25, 0.25),
                vec2(0.25, 0.0),
                vec2(1.0, 0.75),
                vec2(0.75, 0.75),
                vec2(0.75, 1.0)
            ]
        );
        assert_eq!(
            mesh.layers[0]
                .polygons
                .iter()
                .map(|v| v.vertices.clone())
                .collect::<Vec<_>>(),
            vec![
                [5, 1, 8],
                [4, 5, 3],
                [5, 6, 1],
                [8, 3, 5],
                [7, 8, 1],
                [8, 9, 3]
            ]
        );
    }

    #[test]
    fn triangulation_existing_mesh() {
        let mut base_triangulation = Triangulation::from_outer_edges(&[
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(1.0, 1.0),
            vec2(0.0, 1.0),
        ]);
        base_triangulation.add_obstacle(vec![
            vec2(0.0, 0.25),
            vec2(0.25, 0.25),
            vec2(0.25, 0.0),
            vec2(0.0, 0.0),
        ]);
        let mesh = base_triangulation.as_navmesh();

        let mut triangulation = Triangulation::from_mesh(&mesh, 0);
        triangulation.add_obstacle(vec![
            vec2(1.0, 0.75),
            vec2(0.75, 0.75),
            vec2(0.75, 1.0),
            vec2(1.0, 1.0),
        ]);
        let mesh = triangulation.as_navmesh();
        assert_eq!(
            mesh.layers[0]
                .vertices
                .iter()
                .map(|v| v.coords)
                .collect::<Vec<_>>(),
            vec![
                vec2(1.0, 1.0),
                vec2(0.0, 1.0),
                vec2(0.25, 0.25),
                vec2(1.0, 0.0),
                vec2(0.0, 0.25),
                vec2(0.25, 0.0),
                vec2(1.0, 0.75),
                vec2(0.75, 0.75),
                vec2(0.75, 1.0)
            ]
        );
        assert_eq!(
            mesh.layers[0]
                .polygons
                .iter()
                .map(|v| v.vertices.clone())
                .collect::<Vec<_>>(),
            vec![
                [2, 3, 7],
                [4, 2, 1],
                [3, 2, 5],
                [1, 2, 7],
                [6, 7, 3],
                [7, 8, 1]
            ]
        );
    }
}
