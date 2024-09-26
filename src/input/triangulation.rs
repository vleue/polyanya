use std::collections::{HashMap, HashSet, VecDeque};

use inflate::Inflate;
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
    prebuilt: Option<(
        GeoPolygon<f32>,
        ConstrainedDelaunayTriangulation<Point2<f64>>,
    )>,
    base_layer: Option<Layer>,
    agent_radius: f32,
    agent_radius_segments: u8,
    agent_radius_simplification: f32,
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
            prebuilt: None,
            base_layer: None,
            agent_radius: 0.0,
            agent_radius_segments: 5,
            agent_radius_simplification: 0.0,
        }
    }

    /// Create a new triangulation from an existing `Mesh`, cloning the specified [`Layer`].
    pub fn from_mesh(mesh: &Mesh, layer: u8) -> Triangulation {
        Self {
            inner: GeoPolygon::new(LineString::new(Vec::new()), vec![]),
            prebuilt: None,
            base_layer: Some(mesh.layers[layer as usize].clone()),
            agent_radius: 0.0,
            agent_radius_segments: 5,
            agent_radius_simplification: 0.0,
        }
    }

    /// Create a new triangulation from an existing `Layer` of a [`Mesh`].
    pub fn from_mesh_layer(layer: Layer) -> Triangulation {
        Self {
            inner: GeoPolygon::new(LineString::new(Vec::new()), vec![]),
            prebuilt: None,
            base_layer: Some(layer),
            agent_radius: 0.0,
            agent_radius_segments: 5,
            agent_radius_simplification: 0.0,
        }
    }

    /// Set the agent radius. THis will be used to offset the edges of the obstacles.
    pub fn set_agent_radius(&mut self, radius: f32) {
        self.agent_radius = radius;
    }

    /// Set the segment counts for the offset when adding rounded corners.
    pub fn set_agent_radius_segments(&mut self, segments: u8) {
        self.agent_radius_segments = segments;
    }

    /// Simplify the inflated obstacles, using a topology-preserving variant of the
    /// [Visvalingam-Whyatt algorithm](https://www.tandfonline.com/doi/abs/10.1179/000870493786962263).
    ///
    /// Epsilon is the minimum area a point should contribute to a polygon.
    pub fn set_agent_radius_simplification(&mut self, simplification: f32) {
        self.agent_radius_simplification = simplification;
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
        let mut cdt = ConstrainedDelaunayTriangulation::<Point2<f64>>::new();
        Triangulation::add_constraint_edges(&mut cdt, self.inner.exterior());

        let exterior = self.inner.exterior().clone();
        let mut inner = std::mem::replace(&mut self.inner, GeoPolygon::new(exterior, vec![]));
        if self.agent_radius > 1.0e-5 {
            inner = inner.inflate_obstacles(
                self.agent_radius,
                self.agent_radius_segments as u32,
                self.agent_radius_simplification,
            );
        };

        inner
            .interiors()
            .iter()
            .for_each(|obstacle| Triangulation::add_constraint_edges(&mut cdt, obstacle));

        if let Some((previous, _)) = self.prebuilt.take() {
            let (_, inners) = previous.into_inner();
            for interior in inners {
                inner.interiors_push(interior);
            }
        }
        self.prebuilt = Some((inner, cdt));
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
        let mut cdt = if self.prebuilt.is_none() {
            let mut cdt = ConstrainedDelaunayTriangulation::<Point2<f64>>::new();
            Triangulation::add_constraint_edges(&mut cdt, self.inner.exterior());
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

        let inner = if self.agent_radius < 1.0e-5 {
            &self.inner
        } else {
            &self.inner.inflate_obstacles(
                self.agent_radius,
                self.agent_radius_segments as u32,
                self.agent_radius_simplification,
            )
        };

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
                        && !inner.interiors().iter().any(|obstacle| {
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

mod inflate {

    use std::f32::consts::TAU;

    use geo::{Coord, EuclideanDistance, Line, LineString, Polygon, SimplifyVwPreserve};
    use i_overlay::{core::solver::Solver, i_float::f32_point::F32Point};

    fn segment_normal(start: &Coord<f32>, end: &Coord<f32>) -> Option<F32Point> {
        let edge_length = end.euclidean_distance(start);
        if edge_length == 0.0 {
            return None;
        }
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let x = -dy / edge_length;
        let y = dx / edge_length;

        Some(F32Point::new(x, y))
    }

    pub trait Inflate {
        fn inflate_obstacles(&self, distance: f32, arc_segments: u32, minimum_surface: f32)
            -> Self;
    }

    impl Inflate for Polygon<f32> {
        fn inflate_obstacles(
            &self,
            distance: f32,
            arc_segments: u32,
            minimum_surface: f32,
        ) -> Polygon<f32> {
            Polygon::new(
                self.exterior().clone(),
                self.interiors()
                    .iter()
                    .map(|ls| inflate(ls, distance, arc_segments))
                    .map(|ls| ls.simplify_vw_preserve(&minimum_surface))
                    .collect(),
            )
        }
    }

    fn inflate(linestring: &LineString<f32>, distance: f32, arc_segments: u32) -> LineString<f32> {
        let mut last;
        let mut lines = linestring.lines();
        let line = lines.next().unwrap();
        let mut inflated_linestring = round_line(&line, distance, arc_segments);

        last = line.end;
        for line in lines {
            let rounded_line = round_line(&line, distance, arc_segments);

            inflated_linestring = union(inflated_linestring, rounded_line);
            last = line.end;
        }
        if !linestring.is_closed() {
            let line = Line::new(last, linestring.0[0]);
            let rounded_line = round_line(&line, distance, arc_segments);
            inflated_linestring = union(inflated_linestring, rounded_line);
        }

        LineString(
            inflated_linestring
                .iter()
                .map(|v| (v.x, v.y).into())
                .collect(),
        )
    }

    fn round_line(line: &Line<f32>, distance: f32, arc_segments: u32) -> Vec<F32Point> {
        let start = F32Point::new(line.start.x, line.start.y);
        let end = F32Point::new(line.end.x, line.end.y);
        let Some(normal) = segment_normal(&line.start, &line.end) else {
            return (0..(arc_segments * 2))
                .map(|i| {
                    let angle = i as f32 * TAU / (arc_segments * 2) as f32;
                    F32Point::new(
                        start.x + angle.cos() * distance,
                        start.y + angle.sin() * distance,
                    )
                })
                .collect();
        };

        let mut vertices = Vec::with_capacity((arc_segments as usize + 2) * 2);

        create_arc(
            &mut vertices,
            &start,
            distance,
            &(start - (normal * distance)),
            &(start + (normal * distance)),
            arc_segments,
            true,
        );
        create_arc(
            &mut vertices,
            &end,
            distance,
            &(end + (normal * distance)),
            &(end - (normal * distance)),
            arc_segments,
            true,
        );

        vertices
    }

    // from https://github.com/w8r/polygon-offset/blob/77382ec02a4a505fe6c2c900fb67345b206961ed/src/offset.js#L139
    fn create_arc(
        vertices: &mut Vec<F32Point>,
        center: &F32Point,
        radius: f32,
        start_vertex: &F32Point,
        end_vertex: &F32Point,
        segment_count: u32,
        outwards: bool,
    ) {
        let start_angle = (start_vertex.y - center.y).atan2(start_vertex.x - center.x);
        let start_angle = if start_angle.is_sign_negative() {
            start_angle + TAU
        } else {
            start_angle
        };

        let end_angle = (end_vertex.y - center.y).atan2(end_vertex.x - center.x);
        let end_angle = if end_angle.is_sign_negative() {
            end_angle + TAU
        } else {
            end_angle
        };

        // odd number please
        let segment_count = if segment_count % 2 == 0 {
            segment_count - 1
        } else {
            segment_count
        };

        let angle = if start_angle > end_angle {
            start_angle - end_angle
        } else {
            start_angle + TAU - end_angle
        };

        let segment_angle = if outwards { -angle } else { TAU - angle } / (segment_count as f32);

        vertices.push(*start_vertex);
        for i in 1..segment_count {
            let angle = start_angle + segment_angle * (i as f32);
            vertices.push(F32Point::new(
                center.x + angle.cos() * radius,
                center.y + angle.sin() * radius,
            ));
        }
        vertices.push(*end_vertex);
    }

    fn union(subj: Vec<F32Point>, clip: Vec<F32Point>) -> Vec<F32Point> {
        let mut overlay = i_overlay::f32::overlay::F32Overlay::new();

        overlay.add_path(subj, i_overlay::core::overlay::ShapeType::Subject);
        overlay.add_path(clip, i_overlay::core::overlay::ShapeType::Clip);

        let graph = overlay.into_graph_with_solver(
            i_overlay::core::fill_rule::FillRule::NonZero,
            Solver {
                strategy: i_overlay::core::solver::Strategy::List,
                precision: i_overlay::core::solver::Precision::Auto,
                multithreading: None,
            },
        );
        let mut shapes = graph.extract_shapes(i_overlay::core::overlay_rule::OverlayRule::Union);

        shapes.swap_remove(0).swap_remove(0)
    }
}
