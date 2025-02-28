use super::delaunay_core::Dcel;
use crate::vendored_spade::{
    delaunay_core::bulk_load, handles::VertexHandle, HasPosition, HintGenerator, InsertionError,
    LastUsedVertexHintGenerator, NaturalNeighbor, Point2, Triangulation, TriangulationExt,
};

use num_traits::Float;
use std::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A two-dimensional [Delaunay triangulation](https://en.wikipedia.org/wiki/Delaunay_triangulation).
///
/// A Delaunay triangulation  a triangulation that fulfills the *Delaunay Property*: No
/// vertex of the triangulation is contained in the
/// [circumcircle](https://en.wikipedia.org/wiki/Circumscribed_circle) of any triangle.
/// As a consequence, Delaunay triangulations are well suited to support interpolation
/// algorithms and nearest neighbor searches. It is often constructed in order to retrieve its dual
/// graph, the [Voronoi diagram](#voronoi-diagram).
///
///
/// *An example triangulation with a few circumcircles drawn. No circumcircle contains more than three vertices.*
///
/// Most methods on this type require the [Triangulation] trait. Refer to its documentation
/// for more details on how to use `DelaunayTriangulation`.
///
/// # Basic Usage
/// Vertices need to implement the [HasPosition] trait. Spade bundles
/// the [Point2] struct for basic use cases.
///
/// ## Basic example
///  ```
/// use spade::{DelaunayTriangulation, Triangulation, Point2, InsertionError};
///
/// fn main() -> Result<(), InsertionError> {
///
///     let mut triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::new();
///
///     // Insert three vertices that span one triangle (face)
///     triangulation.insert(Point2::new(0.0, 1.0))?;
///     triangulation.insert(Point2::new(1.0, 1.0))?;
///     triangulation.insert(Point2::new(0.5, -1.0))?;
///
///     assert_eq!(triangulation.num_vertices(), 3);
///     assert_eq!(triangulation.num_inner_faces(), 1);
///     assert_eq!(triangulation.num_undirected_edges(), 3);
///     Ok(())
/// }
/// ```
/// ## Right handed and left-handed coordinate systems
/// For simplicity, all method names and their documentation assume that the underlying coordinate system
/// is right-handed (e.g. x-axis points to the right, y-axis points upwards). If a left-handed system
/// (lhs) is used, any term related to orientation needs to be reversed:
///  - "left" becomes "right" (example: the face of a directed edge is on the right side for a lhs)
///  - "counter clock wise" becomes "clockwise" (example: the vertices of a face are returned in clock wise order for a lhs)
///
/// <table>
/// <tr><th>left-handed system</th><th>right-handed system</th></tr>
/// <tr><td>
/// # Extracting geometry information
///
/// Spade uses [handles](crate::vendored_spade::handles) to extract the triangulation's geometry.
/// Handles are usually retrieved by inserting a vertex or by iterating.
///
/// ## Example
/// ```
///  fn main() -> Result<(), spade::InsertionError> {
/// use crate::vendored_spade::spade::{DelaunayTriangulation, Triangulation, Point2};
///
/// let mut triangulation: DelaunayTriangulation<Point2<f64>> = DelaunayTriangulation::new();
///
/// triangulation.insert(Point2::new(0.0, 1.0))?;
/// triangulation.insert(Point2::new(1.0, 1.0))?;
/// triangulation.insert(Point2::new(0.5, -1.0))?;
///
/// for face in triangulation.inner_faces() {
///   // face is a FaceHandle
///   // edges is an array containing 3 directed edge handles
///   let edges = face.adjacent_edges();
///   for edge in &edges {
///     let from = edge.from();
/// let to = edge.to();
///     // from and to are vertex handles
///     println!("found an edge: {:?} -> {:?}", from, to);
///   }
///
///   // vertices is an array containing 3 vertex handles
///   let vertices = face.vertices();
///   for vertex in &vertices {
///     println!("Found vertex with position {:?}", vertex.position());
///   }
/// }
/// # Ok(()) }
/// ```
///
/// # Type parameters
/// The triangulation's vertices, edges and faces can contain custom data.
/// By default, the edge and face types are set to `()`. The vertex type must
/// be specified.
///
///  * `V: HasPosition` The vertex type
///  * `DE: Default` The directed edge type.
///  * `UE: Default` The undirected edge type.
///  * `F: Default` The face type.
///
/// Only vertices can be inserted directly. Faces and edges are create via `Default::default()`.
/// Usually, edge and face data will need to be modified in a separate pass.
///
/// Setting any custom data works by calling [vertex_data_mut](Triangulation::vertex_data_mut),
/// [directed_edge_data_mut](Triangulation::directed_edge_data_mut),
/// [undirected_edge_data_mut](Triangulation::undirected_edge_data_mut) and
/// [face_data_mut](Triangulation::face_data_mut).
///
/// ## Example
/// ```
/// fn main() -> Result<(), spade::InsertionError> {
/// use crate::vendored_spade::spade::{DelaunayTriangulation, Triangulation, Point2};
///
/// // A custom undirected edge type used to cache the length of an edge
/// #[derive(Default)]
/// struct EdgeWithLength { length: f64 }
///
/// // Creates a new triangulation with a custom undirected edge type
/// let mut triangulation: DelaunayTriangulation<Point2<f64>, (), EdgeWithLength>
///                          = DelaunayTriangulation::new();
///
/// triangulation.insert(Point2::new(0.0, 1.0))?;
/// triangulation.insert(Point2::new(1.0, 1.0))?;
/// triangulation.insert(Point2::new(0.5, -1.0))?;
///
/// for edge in triangulation.fixed_undirected_edges() {
///   let positions = triangulation.undirected_edge(edge).positions();
///   let length = positions[0].distance_2(positions[1]).sqrt();
///   // Write length into the edge data
///   triangulation.undirected_edge_data_mut(edge).length = length;
/// }
///
/// for edge in triangulation.undirected_edges() {
///    let length = edge.data().length;
///    assert!(length > 0.0);
/// }
/// # Ok(()) }
/// ```
///
/// # Outer face
/// Every triangulation contains an *outer face* that is adjacent to all edges of the
/// triangulation's convex hull. The outer face is even present for a triangulation without
/// vertices. It is either referenced by calling [Triangulation::outer_face()] or
/// [handles::OUTER_FACE](crate::vendored_spade::handles::OUTER_FACE)
///
///
/// # Voronoi Diagram
///
/// the dual graph of the Delaunay triangulation is the *Voronoi diagram*. The Voronoi diagram
/// subdivides the plane into several *Voronoi cells* (called `VoronoiFace` by Spade for consistency)
/// which contain all points in the plane that share a common nearest neighbor.
///
/// Keep in mind that "faces" and "vertices" are swapped - an (inner) Voronoi *vertex*
/// corresponds to a single Delaunay *face*.
/// The position of an inner voronoi vertex is the *circumcenter* of its dual Delaunay
/// face.
///
///
/// *A Delaunay triangulation (black lines) and its dual graph, the Voronoi diagram (blue lines)*
///
/// ## Extracting the Voronoi Diagram
/// Spade defines various functions to extract information about the Voronoi diagram:
///
/// **Types**
///  * [DirectedVoronoiEdge](crate::vendored_spade::handles::DirectedVoronoiEdge)
///  * [UndirectedVoronoiEdge](crate::vendored_spade::handles::UndirectedVoronoiEdge)
///  * [VoronoiVertex](crate::vendored_spade::handles::VoronoiVertex)
///  * [VoronoiFace](crate::vendored_spade::handles::VoronoiFace)
///
/// **Iterators**
///  * [Triangulation::directed_voronoi_edges()]
///  * [Triangulation::undirected_voronoi_edges()]
///
/// **Conversion**
///  * [DirectedVoronoiEdge::as_undirected()](crate::vendored_spade::handles::DirectedVoronoiEdge::as_undirected())
///  * [UndirectedVoronoiEdge::as_directed()](crate::vendored_spade::handles::UndirectedVoronoiEdge::as_directed())
///  * [DirectedEdgeHandle::as_voronoi_edge()](crate::vendored_spade::handles::DirectedEdgeHandle::as_voronoi_edge())
///  * [DirectedVoronoiEdge::as_delaunay_edge()](crate::vendored_spade::handles::DirectedVoronoiEdge::as_delaunay_edge())
///  * [UndirectedEdgeHandle::as_voronoi_edge()](crate::vendored_spade::handles::UndirectedEdgeHandle::as_voronoi_edge())
///  * [UndirectedVoronoiEdge::as_delaunay_edge()](crate::vendored_spade::handles::UndirectedVoronoiEdge::as_delaunay_edge())
///
/// ## Extracting the Voronoi Diagram (Example)
/// Extracting the geometry of the voronoi diagram can be slightly tricky as some of the voronoi
/// extend into infinity (see the dashed lines in the example above).
///
/// ```
/// use spade::handles::{VoronoiVertex::Inner, VoronoiVertex::Outer};
/// use spade::{DelaunayTriangulation, Point2, Triangulation};
///
/// // Prints out the location of all voronoi edges in a triangulation
/// fn log_voronoi_diagram(triangulation: &DelaunayTriangulation<Point2<f64>>) {
///     for edge in triangulation.undirected_voronoi_edges() {
///         match edge.vertices() {
///             [Inner(from), Inner(to)] => {
///                 // "from" and "to" are inner faces of the Delaunay triangulation
///                 println!(
///                     "Found voronoi edge between {:?} and {:?}",
///                     from.circumcenter(),
///                     to.circumcenter()
///                 );
///             }
///             [Inner(from), Outer(edge)] | [Outer(edge), Inner(from)] => {
///                 // Some lines don't have a finite end and extend into infinity.
///                 println!(
///                     "Found infinite voronoi edge going out of {:?} into the direction {:?}",
///                     from.circumcenter(),
///                     edge.direction_vector()
///                 );
///             }
///             [Outer(_), Outer(_)] => {
///                 // This case only happens if all vertices of the triangulation lie on the
///                 // same line and can probably be ignored.
///             }
///         }
///     }
/// }
/// ```
///
/// # Performance tuning
///
/// Fine-tuning a Delaunay triangulation can be more tricky from time to time. However, some will *nearly always* be
/// the right thing to do:
///
/// - Measure, don't guess. Execution times are hard to predict.
/// - If you plan to perform several random access queries (e.g. looking up the point at an arbitrary position):
///   Consider using `[HierarchyHintGenerator](crate::vendored_spade::HierarchyHintGenerator)
/// - For data sets with uniformly distributed vertices: Use [HierarchyHintGenerator](crate::vendored_spade::HierarchyHintGenerator) if
///   bulk loading is not applicable.
/// - For data sets where vertices are inserted in close local proximity (each vertex is not too far away from the
///   previously inserted vertex): Consider using [LastUsedVertexHintGenerator](LastUsedVertexHintGenerator).
/// - Try to avoid large custom data types for edges, vertices and faces.
/// - Using `f64` and `f32` as scalar type will usually end up roughly having the same run time performance.
/// - Prefer using [bulk_load](Triangulation::bulk_load) over [insert](Triangulation::insert).
/// - The run time of all vertex operations (insertion, removal and lookup) is roughly the same for larger triangulations.
///
/// ## Complexity classes
///
/// This table display the average and amortized cost for inserting a vertex into a triangulation with `n` vertices.
///
/// |                             | Uniformly distributed vertices | Insertion of vertices with local proximity |
/// |-----------------------------|--------------------------------|--------------------------------------------|
/// | LastUsedVertexHintGenerator |        O(sqrt(n)) (worst case) |                  O(1) (best case), fastest |
/// | HierarchyHintGenerator      |       O(log(n)) (Average case) |                           O(1) (best case) |
///
/// # See also
/// Delaunay triangulations are closely related to [constrained Delaunay triangulations](crate::vendored_spade::ConstrainedDelaunayTriangulation)
#[doc(alias = "Voronoi")]
#[doc(alias = "Voronoi diagram")]
#[doc(alias = "Delaunay")]
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub struct DelaunayTriangulation<V, DE = (), UE = (), F = (), L = LastUsedVertexHintGenerator>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    pub(crate) dcel: Dcel<V, DE, UE, F>,
    pub(crate) hint_generator: L,
}

impl<V, DE, UE, F, L> DelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    /// Returns the nearest neighbor of a given input vertex.
    ///
    /// Returns `None` if the triangulation is empty.
    ///
    /// # Runtime
    /// This method takes `O(sqrt(n))` on average where n is the number of vertices.
    pub fn nearest_neighbor(
        &self,
        position: Point2<<V as HasPosition>::Scalar>,
    ) -> Option<VertexHandle<V, DE, UE, F>> {
        if self.num_vertices() == 0 {
            return None;
        }

        let hint = self.hint_generator().get_hint(position);
        let hint = self.validate_vertex_handle(hint);

        let vertex = self.walk_to_nearest_neighbor(hint, position);
        self.hint_generator().notify_vertex_lookup(vertex.fix());
        Some(vertex)
    }

    /// Creates a new delaunay triangulation with an efficient bulk loading strategy.
    ///
    /// In contrast to [Triangulation::bulk_load], this method will create a triangulation with
    /// vertices returned *in the same order* as the input vertices.
    ///
    /// # Duplicate handling
    ///
    /// If two vertices have the same position, only one of them will be included in the final
    /// triangulation. It is undefined which of them is discarded.
    ///
    /// For example, if the input vertices are [v0, v1, v2, v1] (where v1 is duplicated), the
    /// resulting triangulation will be either
    /// [v0, v1, v2] or [v0, v2, v1]
    ///
    /// Consider checking the triangulation's size after calling this method to ensure that no
    /// duplicates were present.
    ///
    /// # Example
    /// ```
    /// # use spade::InsertionError;
    /// use spade::{DelaunayTriangulation, Point2, Triangulation};
    ///
    /// # fn main() -> Result<(), InsertionError> {
    /// let vertices = vec![
    ///      Point2::new(0.5, 1.0),
    ///      Point2::new(-0.5, 2.0),
    ///      Point2::new(0.2, 3.0),
    ///      Point2::new(0.0, 4.0),
    ///      Point2::new(-0.2, 5.0)
    /// ];
    /// let triangulation = DelaunayTriangulation::<Point2<f64>>::bulk_load_stable(vertices.clone())?;
    /// // This assert will not hold for regular bulk loading!
    /// assert_eq!(triangulation.vertices().map(|v| *v.data()).collect::<Vec<_>>(), vertices);
    ///
    /// // This is how you would check for duplicates:
    /// let duplicates_found = triangulation.num_vertices() < vertices.len();
    /// assert!(!duplicates_found);
    /// # Ok(()) }
    /// ```
    pub fn bulk_load_stable(elements: Vec<V>) -> Result<Self, InsertionError> {
        let mut result: Self = crate::vendored_spade::delaunay_core::bulk_load_stable::<
            _,
            _,
            DelaunayTriangulation<_, _, _, _, _>,
            _,
        >(bulk_load, elements)?;
        *result.hint_generator_mut() = L::initialize_from_triangulation(&result);
        Ok(result)
    }
}

impl<V, DE, UE, F, L> Default for DelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    fn default() -> Self {
        Self {
            dcel: Default::default(),
            hint_generator: Default::default(),
        }
    }
}

impl<V, DE, UE, F, L> DelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    V::Scalar: Float,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    /// Allows using natural neighbor interpolation on this triangulation. Refer to the documentation
    /// of [NaturalNeighbor] for more information.
    pub fn natural_neighbor(&self) -> NaturalNeighbor<Self> {
        NaturalNeighbor::new(self)
    }
}

impl<V, DE, UE, F, L> Triangulation for DelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    type Vertex = V;
    type DirectedEdge = DE;
    type UndirectedEdge = UE;
    type Face = F;
    type HintGenerator = L;

    fn s(&self) -> &Dcel<V, DE, UE, F> {
        &self.dcel
    }

    fn s_mut(&mut self) -> &mut Dcel<V, DE, UE, F> {
        &mut self.dcel
    }

    fn hint_generator(&self) -> &Self::HintGenerator {
        &self.hint_generator
    }

    fn hint_generator_mut(&mut self) -> &mut Self::HintGenerator {
        &mut self.hint_generator
    }

    fn from_parts(
        dcel: Dcel<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>,
        hint_generator: Self::HintGenerator,
        num_constraints: usize,
    ) -> Self {
        assert_eq!(num_constraints, 0);
        Self {
            dcel,
            hint_generator,
        }
    }

    fn into_parts(
        self,
    ) -> (
        Dcel<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>,
        Self::HintGenerator,
        usize,
    ) {
        (self.dcel, self.hint_generator, 0)
    }
}
