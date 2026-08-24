use std::collections::{HashMap, HashSet};

use inflate::Inflate;
use log::warn;
#[cfg(feature = "tracing")]
use tracing::instrument;

pub use geo::LineString;
use geo::{
    self,
    coordinate_position::{coord_pos_relative_to_ring, CoordPos},
    Coord, SimplifyVwPreserve,
};
use glam::{vec2, Vec2};
use spade::{
    handles::FixedVertexHandle, ConstrainedDelaunayTriangulation, Point2,
    Triangulation as SpadeTriangulation,
};

use crate::{Layer, Mesh, Polygon, Vertex};

#[derive(Clone, Copy, Debug)]
enum AgentRadius {
    None,
    Obstacles(f32, u8, f32),
    Everything(f32, u8, f32),
}

/// An helper to create a [`Mesh`] from a list of edges and obstacle, using a constrained Delaunay triangulation.
#[derive(Clone)]
pub struct Triangulation {
    inner: geo::Polygon<f32>,
    prebuilt: Option<(
        geo::Polygon<f32>,
        ConstrainedDelaunayTriangulation<Point2<f64>>,
    )>,
    base_layer: Option<Layer>,
    agent_radius: AgentRadius,
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
    /// Create a new triangulation from a [`geo::Polygon`].
    ///
    /// The exterior of the polygon will be used as the outer edge of the triangulation,
    /// and inner polygons will be used as obstacles.
    pub fn from_geo_polygon(polygon: geo::Polygon<f32>) -> Triangulation {
        Self {
            inner: polygon,
            prebuilt: None,
            base_layer: None,
            agent_radius: AgentRadius::None,
        }
    }

    /// Create a new triangulation from a the list of points on its outer edges.
    pub fn from_outer_edges(edges: &[Vec2]) -> Triangulation {
        Self {
            inner: geo::Polygon::new(
                LineString::from(edges.iter().map(|v| (v.x, v.y)).collect::<Vec<_>>()),
                vec![],
            ),
            prebuilt: None,
            base_layer: None,
            agent_radius: AgentRadius::None,
        }
    }

    /// Create a new triangulation from an existing `Mesh`, cloning the specified [`Layer`].
    pub fn from_mesh(mesh: &Mesh, layer: u8) -> Triangulation {
        Self::from_mesh_layer(mesh.layers[layer as usize].clone())
    }

    /// Create a new triangulation from an existing `Layer` of a [`Mesh`].
    pub fn from_mesh_layer(layer: Layer) -> Triangulation {
        if !layer.height.is_empty() {
            warn!("Loading a navmesh with height information into a triangulation. All height information will be lost. If the navmesh have overlapping parts the result will be wrong");
        }
        Self {
            inner: geo::Polygon::new(LineString::new(Vec::new()), vec![]),
            prebuilt: None,
            base_layer: Some(layer),
            agent_radius: AgentRadius::None,
        }
    }

    /// Set the agent radius. THis will be used to offset the edges of the obstacles.
    pub fn set_agent_radius(&mut self, radius: f32) {
        self.agent_radius = match self.agent_radius {
            AgentRadius::None => AgentRadius::Obstacles(radius, 5, 0.0),
            AgentRadius::Obstacles(_, segments, simplification) => {
                AgentRadius::Obstacles(radius, segments, simplification)
            }
            AgentRadius::Everything(_, segments, simplification) => {
                AgentRadius::Everything(radius, segments, simplification)
            }
        }
    }

    /// Set the segment counts for the offset when adding rounded corners.
    pub fn set_agent_radius_segments(&mut self, segments: u8) {
        self.agent_radius = match self.agent_radius {
            AgentRadius::None => AgentRadius::Obstacles(0.0, segments, 0.0),
            AgentRadius::Obstacles(radius, _, simplification) => {
                AgentRadius::Obstacles(radius, segments, simplification)
            }
            AgentRadius::Everything(radius, _, simplification) => {
                AgentRadius::Everything(radius, segments, simplification)
            }
        }
    }

    /// Simplify the inflated obstacles, using a topology-preserving variant of the
    /// [Visvalingam-Whyatt algorithm](https://www.tandfonline.com/doi/abs/10.1179/000870493786962263).
    ///
    /// Epsilon is the minimum area a point should contribute to a polygon.
    pub fn set_agent_radius_simplification(&mut self, simplification: f32) {
        self.agent_radius = match self.agent_radius {
            AgentRadius::None => AgentRadius::Obstacles(0.0, 5, simplification),
            AgentRadius::Obstacles(radius, segments, _) => {
                AgentRadius::Obstacles(radius, segments, simplification)
            }
            AgentRadius::Everything(radius, segments, _) => {
                AgentRadius::Everything(radius, segments, simplification)
            }
        }
    }

    /// Changes wether the outer edge should be impacted by the agent radius.
    pub fn agent_radius_on_outer_edge(&mut self, enabled: bool) {
        self.agent_radius = match (self.agent_radius, enabled) {
            (AgentRadius::None, true) => AgentRadius::Everything(0.0, 5, 0.0),
            (AgentRadius::None, false) => AgentRadius::Obstacles(0.0, 5, 0.0),
            (AgentRadius::Obstacles(radius, segments, simplification), true)
            | (AgentRadius::Everything(radius, segments, simplification), true) => {
                AgentRadius::Everything(radius, segments, simplification)
            }
            (AgentRadius::Obstacles(radius, segments, simplification), false)
            | (AgentRadius::Everything(radius, segments, simplification), false) => {
                AgentRadius::Obstacles(radius, segments, simplification)
            }
        };
    }

    /// Add an obstacle delimited by the list of points on its edges.
    pub fn add_obstacle(&mut self, edges: impl IntoIterator<Item = Vec2>) {
        self.inner
            .interiors_push(LineString::from_iter(edges.into_iter().map(|v| (v.x, v.y))));
    }

    /// Add obstacles delimited by the list of points on their edges.
    pub fn add_obstacles(
        &mut self,
        obstacles: impl IntoIterator<Item = impl IntoIterator<Item = Vec2>>,
    ) {
        let (exterior, interiors) = std::mem::replace(
            &mut self.inner,
            geo::Polygon::new(LineString(vec![]), vec![]),
        )
        .into_inner();
        self.inner = geo::Polygon::new(
            exterior,
            interiors
                .into_iter()
                .chain(obstacles.into_iter().map(|edges| {
                    LineString::from_iter(edges.into_iter().map(|v| Coord::from((v.x, v.y))))
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
                *interior = interior.simplify_vw_preserve(epsilon);
            }
        });
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline]
    fn add_constraint_edges(
        cdt: &mut ConstrainedDelaunayTriangulation<Point2<f64>>,
        edges: &LineString<f32>,
    ) {
        if edges.0.len() < 2 {
            return;
        }
        // Each point is inserted once, and reused as the start of the next constraint edge.
        let mut previous: Option<FixedVertexHandle> = None;
        for coord in &edges.0 {
            let vertex = cdt
                .insert(Point2 {
                    x: coord.x as f64,
                    y: coord.y as f64,
                })
                .unwrap();
            if let Some(previous) = previous {
                cdt.add_constraint_and_split(previous, vertex, |v| v);
            }
            previous = Some(vertex);
        }
    }

    /// Prebuild part of the navmesh with the already added obstacles.
    ///
    /// This can be used to cache part of the navmesh generation when some of the obstacles won't change.
    pub fn prebuild(&mut self) {
        if self.base_layer.is_some() {
            return;
        }

        let exterior = self.inner.exterior().clone();
        let mut inner = std::mem::replace(&mut self.inner, geo::Polygon::new(exterior, vec![]));
        match self.agent_radius {
            AgentRadius::Obstacles(radius, segments, simplification) if radius > 1.0e-5 => {
                inner = inner.inflate_obstacles(radius, segments as u32, simplification);
            }
            AgentRadius::Everything(radius, segments, simplification) if radius > 1.0e-5 => {
                inner = inner.inflate(radius, segments as u32, simplification);
            }
            _ => {}
        }

        let mut cdt = ConstrainedDelaunayTriangulation::<Point2<f64>>::new();
        Triangulation::add_constraint_edges(&mut cdt, inner.exterior());

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
        let mut cdt = if let Some((_, cdt)) = &self.prebuilt {
            cdt.clone()
        } else {
            let mut cdt = ConstrainedDelaunayTriangulation::<Point2<f64>>::new();
            match self.agent_radius {
                AgentRadius::Everything(radius, segments, _) if radius > 1.0e-5 => {
                    let deflated = self.inner.inflate(radius, segments as u32, 0.0);
                    Triangulation::add_constraint_edges(&mut cdt, deflated.exterior());
                }
                _ => Triangulation::add_constraint_edges(&mut cdt, self.inner.exterior()),
            };
            cdt
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

        let inner = match self.agent_radius {
            AgentRadius::Everything(radius, segments, simplification) if radius > 1.0e-5 => {
                &self.inner.inflate(radius, segments as u32, simplification)
            }
            AgentRadius::Obstacles(radius, segments, simplification) if radius > 1.0e-5 => &self
                .inner
                .inflate_obstacles(radius, segments as u32, simplification),
            _ => &self.inner,
        };
        // Bounding boxes of the obstacles, so that point-in-polygon tests can skip
        // the obstacles that can't contain the point without walking all their edges.
        let inner_bounds = obstacles_bounds(inner);
        let used_bounds = used.map(obstacles_bounds);

        inner
            .interiors()
            .iter()
            .for_each(|obstacle| Triangulation::add_constraint_edges(&mut cdt, obstacle));

        #[cfg(feature = "tracing")]
        let polygon_span = tracing::info_span!("listing polygons").entered();

        // Use flood-fill via Union-Find to group faces into connected components
        // separated by constraint edges. This way we only need one expensive
        // point-in-polygon test per component instead of per face.
        let num_faces = cdt.all_faces().len();
        let mut component: Vec<usize> = (0..num_faces).collect();
        fn find(component: &mut [usize], mut x: usize) -> usize {
            while component[x] != x {
                component[x] = component[component[x]];
                x = component[x];
            }
            x
        }
        fn union(component: &mut [usize], a: usize, b: usize) {
            let ra = find(component, a);
            let rb = find(component, b);
            if ra != rb {
                component[ra] = rb;
            }
        }

        // Merge faces connected by non-constraint edges
        for face in cdt.inner_faces() {
            let face_idx = face.index();
            for edge in face.adjacent_edges() {
                if !edge.is_constraint_edge() {
                    let neighbor = edge.rev().face();
                    if !neighbor.is_outer() {
                        union(&mut component, face_idx, neighbor.index());
                    }
                }
            }
        }

        // For each component, test one representative face
        let mut component_navigable: Vec<Option<bool>> = vec![None; num_faces];
        for face in cdt.inner_faces() {
            let root = find(&mut component, face.index());
            if component_navigable[root].is_some() {
                continue;
            }
            let center = face.center();
            let center = Coord::from((center.x as f32, center.y as f32));

            let navigable = (used
                .zip(used_bounds.as_deref())
                .map(|(used, bounds)| polygon_contains(used, bounds, center))
                .unwrap_or(true)
                && polygon_contains(inner, &inner_bounds, center))
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
                    && !inner
                        .interiors()
                        .iter()
                        .zip(&inner_bounds)
                        .any(|(obstacle, bounds)| {
                            bounds.contains(center)
                                && coord_pos_relative_to_ring(center, obstacle) == CoordPos::Inside
                        }));
            component_navigable[root] = Some(navigable);
        }

        // Build polygons using the precomputed component results
        let mut face_to_polygon: Vec<u32> = vec![u32::MAX; num_faces];
        let mut i = 0;
        let polygons = cdt
            .inner_faces()
            .filter_map(|face| {
                let root = find(&mut component, face.index());
                (component_navigable[root] == Some(true)).then(|| {
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

        // Scratch buffer reused for every vertex, the final list is copied from it.
        let mut neighbour_polygons = Vec::new();
        let vertices = cdt
            .vertices()
            .map(|point| {
                #[cfg(feature = "tracing")]
                let _preparing_span = tracing::info_span!("preparing vertex").entered();

                neighbour_polygons.clear();
                neighbour_polygons.extend(
                    point
                        .out_edges()
                        .map(|out_edge| face_to_polygon[out_edge.face().index()]),
                );
                let neighbour_polygons: Vec<_> =
                    match neighbour_polygons.iter().position(|i| *i != u32::MAX) {
                        None => vec![u32::MAX],
                        Some(first_polygon) => {
                            // Start the list on a polygon, not on the outside marker
                            neighbour_polygons.rotate_left(first_polygon);
                            neighbour_polygons.dedup();
                            neighbour_polygons.clone()
                        }
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

/// Axis-aligned bounding box of a ring.
#[derive(Clone, Copy, Debug)]
struct Bounds {
    min: Coord<f32>,
    max: Coord<f32>,
}

impl Bounds {
    fn of_ring(ring: &LineString<f32>) -> Self {
        let mut bounds = Bounds {
            min: Coord {
                x: f32::INFINITY,
                y: f32::INFINITY,
            },
            max: Coord {
                x: f32::NEG_INFINITY,
                y: f32::NEG_INFINITY,
            },
        };
        for coord in &ring.0 {
            bounds.min.x = bounds.min.x.min(coord.x);
            bounds.min.y = bounds.min.y.min(coord.y);
            bounds.max.x = bounds.max.x.max(coord.x);
            bounds.max.y = bounds.max.y.max(coord.y);
        }
        bounds
    }

    #[inline]
    fn contains(&self, coord: Coord<f32>) -> bool {
        coord.x >= self.min.x
            && coord.x <= self.max.x
            && coord.y >= self.min.y
            && coord.y <= self.max.y
    }
}

fn obstacles_bounds(polygon: &geo::Polygon<f32>) -> Vec<Bounds> {
    polygon.interiors().iter().map(Bounds::of_ring).collect()
}

/// Same as [`geo::Contains::contains`] for a [`geo::Polygon`] and a [`Coord`], but skipping the
/// obstacles whose bounding box doesn't contain the point.
///
/// `bounds` must be the bounding boxes of the interiors of `polygon`, in the same order.
fn polygon_contains(polygon: &geo::Polygon<f32>, bounds: &[Bounds], coord: Coord<f32>) -> bool {
    if polygon.exterior().0.is_empty() {
        return false;
    }
    match coord_pos_relative_to_ring(coord, polygon.exterior()) {
        CoordPos::Outside | CoordPos::OnBoundary => false,
        CoordPos::Inside => polygon
            .interiors()
            .iter()
            .zip(bounds)
            .filter(|(_, bounds)| bounds.contains(coord))
            .all(|(hole, _)| coord_pos_relative_to_ring(coord, hole) == CoordPos::Outside),
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

    use geo::{
        unary_union, Coord, Distance, Euclidean, Line, LineString, Polygon, SimplifyVwPreserve,
    };

    fn segment_normal(start: &Coord<f32>, end: &Coord<f32>) -> Option<Coord<f32>> {
        let edge_length = Euclidean.distance(*end, *start);
        if edge_length == 0.0 {
            return None;
        }
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let x = -dy / edge_length;
        let y = dx / edge_length;

        Some(Coord { x, y })
    }

    pub trait Inflate {
        fn inflate_obstacles(&self, distance: f32, arc_segments: u32, minimum_surface: f32)
            -> Self;

        fn inflate(&self, distance: f32, arc_segments: u32, minimum_surface: f32) -> Self;
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
                    .map(|ls| {
                        // `simplify_vw_preserve` returns the input unchanged for a non-positive
                        // epsilon, but only after building a spatial index of its edges.
                        if minimum_surface > 0.0 {
                            ls.simplify_vw_preserve(minimum_surface)
                        } else {
                            ls
                        }
                    })
                    .collect(),
            )
        }

        fn inflate(&self, distance: f32, arc_segments: u32, minimum_surface: f32) -> Polygon<f32> {
            let inflated_exterior = inflate_as_polygon(self.exterior(), distance, arc_segments);

            let mut obstacles = self.inflate_obstacles(distance, arc_segments, minimum_surface);

            obstacles.exterior_mut(|exterior| {
                *exterior = inflated_exterior.interiors()[0].clone();
            });
            obstacles
        }
    }

    /// Union of every segment of the ring inflated to a rounded rectangle, as a single polygon.
    ///
    /// All the rounded segments are merged in a single boolean operation instead of one union
    /// per segment on a polygon growing with each step.
    fn inflate_as_polygon(
        linestring: &LineString<f32>,
        distance: f32,
        arc_segments: u32,
    ) -> Polygon<f32> {
        let closing = (!linestring.is_closed())
            .then(|| Line::new(*linestring.0.last().unwrap(), linestring.0[0]));
        let rounded_lines = linestring
            .lines()
            .chain(closing)
            .map(|line| Polygon::new(round_line(&line, distance, arc_segments), vec![]))
            .collect::<Vec<_>>();
        unary_union(&rounded_lines).0.into_iter().next().unwrap()
    }

    fn inflate(linestring: &LineString<f32>, distance: f32, arc_segments: u32) -> LineString<f32> {
        inflate_as_polygon(linestring, distance, arc_segments)
            .into_inner()
            .0
    }

    fn round_line(line: &Line<f32>, distance: f32, arc_segments: u32) -> LineString<f32> {
        let Some(normal) = segment_normal(&line.start, &line.end) else {
            return LineString::from_iter((0..(arc_segments * 2)).map(|i| {
                let angle = i as f32 * TAU / (arc_segments * 2) as f32;
                Coord {
                    x: line.start.x + angle.cos() * distance,
                    y: line.start.y + angle.sin() * distance,
                }
            }));
        };
        let mut vertices = Vec::with_capacity((arc_segments as usize + 2) * 2);

        create_arc(
            &mut vertices,
            &line.start,
            distance,
            &(line.start - (normal * distance)),
            &(line.start + (normal * distance)),
            arc_segments,
            true,
        );
        create_arc(
            &mut vertices,
            &line.end,
            distance,
            &(line.end + (normal * distance)),
            &(line.end - (normal * distance)),
            arc_segments,
            true,
        );

        LineString::new(vertices)
    }

    fn create_arc(
        vertices: &mut Vec<Coord<f32>>,
        center: &Coord<f32>,
        radius: f32,
        start_vertex: &Coord<f32>,
        end_vertex: &Coord<f32>,
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
        let segment_count = if segment_count.is_multiple_of(2) {
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
            vertices.push(Coord {
                x: center.x + angle.cos() * radius,
                y: center.y + angle.sin() * radius,
            });
        }
        vertices.push(*end_vertex);
    }
}
