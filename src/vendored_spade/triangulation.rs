use num_traits::Float;

use crate::vendored_spade::delaunay_core::iterators::HullIterator;
use crate::vendored_spade::delaunay_core::InnerOuterMarker;
use crate::vendored_spade::flood_fill_iterator::CircleMetric;
use crate::vendored_spade::flood_fill_iterator::EdgesInShapeIterator;
use crate::vendored_spade::flood_fill_iterator::FloodFillIterator;
use crate::vendored_spade::flood_fill_iterator::RectangleMetric;
use crate::vendored_spade::flood_fill_iterator::VerticesInShapeIterator;
use crate::vendored_spade::iterators::*;
use crate::vendored_spade::Barycentric;
use crate::vendored_spade::HintGenerator;
use crate::vendored_spade::{delaunay_core::Dcel, handles::*};
use crate::vendored_spade::{HasPosition, InsertionError, Point2, TriangulationExt};

use std::vec::Vec;

/// Describes a position in a triangulation.
///
/// The position is set in relation to the triangulation's vertices, edges and faces.
/// This type is usually the result of calling [Triangulation::locate]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum PositionInTriangulation {
    /// A position lies exactly on an existing vertex. The verticis handle is given.
    OnVertex(FixedVertexHandle),

    /// A position lies exactly on an edge. The edge's handle is given.
    OnEdge(FixedDirectedEdgeHandle),

    /// A position lies in the interior of a face. The face's handle is given.
    OnFace(FixedFaceHandle<InnerTag>),

    /// A position lies outside the convex hull. The given edge handle refers to an edge
    /// of the convex hull which has both the point and the outer face on its left side.
    ///
    /// *Note*: The given edge is *not* necessarily the *closest* edge to a position.
    OutsideOfConvexHull(FixedDirectedEdgeHandle),

    /// The triangulation contains either no vertices or exactly one vertex which has a
    /// different position than the query point.
    NoTriangulation,
}

/// Defines common operations on triangulations.
///
/// These operations are both available for
/// [ConstrainedDelaunayTriangulations](crate::vendored_spade::ConstrainedDelaunayTriangulation) and
/// regular [DelaunayTriangulations](crate::vendored_spade::DelaunayTriangulation).
pub trait Triangulation: Default {
    /// The triangulation's vertex type.
    type Vertex: HasPosition;
    /// The triangulation's edge type. Any new edge is created by using the `Default` trait.
    type DirectedEdge: Default;
    /// The triangulation's undirected edge type. Any new edge is created by using the `Default` trait.
    type UndirectedEdge: Default;
    /// The triangulation's face type. Any new face is created by using the `Default` trait.
    type Face: Default;

    /// The hint generator used by the triangulation. See [HintGenerator] for more information.
    type HintGenerator: HintGenerator<<Self::Vertex as HasPosition>::Scalar>;

    #[doc(hidden)]
    fn s(&self) -> &Dcel<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>;

    #[doc(hidden)]
    fn s_mut(
        &mut self,
    ) -> &mut Dcel<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>;

    #[doc(hidden)]
    fn is_defined_legal(&self, _: FixedUndirectedEdgeHandle) -> bool {
        false
    }

    #[doc(hidden)]
    fn handle_legal_edge_split(&mut self, _: [FixedDirectedEdgeHandle; 2]) {}

    #[doc(hidden)]
    fn hint_generator(&self) -> &Self::HintGenerator;

    #[doc(hidden)]
    fn hint_generator_mut(&mut self) -> &mut Self::HintGenerator;

    #[doc(hidden)]
    fn from_parts(
        dcel: Dcel<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>,
        hint_generator: Self::HintGenerator,
        num_constraints: usize,
    ) -> Self;

    #[doc(hidden)]
    #[allow(clippy::type_complexity)]
    fn into_parts(
        self,
    ) -> (
        Dcel<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>,
        Self::HintGenerator,
        usize,
    );

    /// Creates a new triangulation.
    ///
    /// A newly created triangulation contains no vertices, no edges and the single
    /// outer face.
    ///
    /// # Example
    /// ```
    /// use spade::{DelaunayTriangulation, Point2, Triangulation};
    ///
    /// let mut triangulation: DelaunayTriangulation<Point2<f64>> = DelaunayTriangulation::new();
    /// // An empty triangulation has no vertices and one face
    /// assert_eq!(triangulation.num_vertices(), 0);
    /// assert_eq!(triangulation.num_all_faces(), 1);
    /// assert_eq!(triangulation.num_inner_faces(), 0); // This count excludes the outer face
    /// assert_eq!(triangulation.num_directed_edges(), 0);
    ///
    /// triangulation.insert(Point2::new(0.0, 1.0));
    /// assert_eq!(triangulation.num_vertices(), 1);
    /// assert_eq!(triangulation.num_inner_faces(), 0);
    ///
    /// triangulation.insert(Point2::new(1.0, 1.0));
    /// // Two vertices define the first edge
    /// assert_eq!(triangulation.num_undirected_edges(), 1);
    ///
    /// triangulation.insert(Point2::new(1.0, 0.0));
    /// assert_eq!(triangulation.num_vertices(), 3);
    // // The third point will generate the first inner face!
    /// assert_eq!(triangulation.num_inner_faces(), 1);
    /// ```
    fn new() -> Self {
        Self::default()
    }

    /// Creates a new triangulation and pre-allocates some space for vertices, edges and faces
    fn with_capacity(num_vertices: usize, num_undirected_edges: usize, num_faces: usize) -> Self {
        let mut result = Self::new();
        result
            .s_mut()
            .reserve_capacity(num_vertices, num_undirected_edges, num_faces);
        result
    }

    /// Removes all edges, faces and vertices from the triangulation.
    ///
    /// This method does not change the allocated internal capacity.
    fn clear(&mut self) {
        self.s_mut().clear();
        let new_hint_generator = HintGenerator::initialize_from_triangulation(self);
        *self.hint_generator_mut() = new_hint_generator;
    }

    /// Creates a new triangulation populated with some vertices.
    ///
    /// This will usually be more efficient than inserting the elements sequentially by calling
    /// [insert](Triangulation::insert).
    ///
    /// Returns an [InsertionError] if any input coordinate is invalid. This method should never fail
    /// if all vertices were successfully checked with [crate::vendored_spade::validate_vertex].
    ///
    /// # Runtime
    ///
    /// This method has a run time of `O(n)` but will run near linearly in practice.
    /// The runtime can be as bad as `O(n²)` if the inputs are very degenerate, e.g.
    /// if all input vertices lie on the same line.
    ///
    /// # Comparison to incremental insertion
    ///
    /// This graph shows the difference between incremental and bulk loading for a different number of random points - bulk loading becomes
    /// more efficient very quickly.
    fn bulk_load(elements: Vec<Self::Vertex>) -> Result<Self, InsertionError> {
        let mut result: Self = crate::vendored_spade::delaunay_core::bulk_load(elements)?;
        *result.hint_generator_mut() = Self::HintGenerator::initialize_from_triangulation(&result);
        Ok(result)
    }

    /// Converts a fixed vertex handle to a reference vertex handle.
    ///
    /// *See also the [handles](crate::vendored_spade::handles) module for more information.*
    fn vertex(
        &self,
        handle: FixedVertexHandle,
    ) -> VertexHandle<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face> {
        self.s().vertex(handle)
    }

    /// Returns a mutable reference to the associated data of a vertex.
    fn vertex_data_mut(&mut self, handle: FixedVertexHandle) -> &mut Self::Vertex {
        self.s_mut().vertex_data_mut(handle)
    }

    /// Converts a fixed face handle to a reference face handle.
    ///
    /// *See also the [handles](crate::vendored_spade::handles) module for more information.*
    fn face<InnerOuter: InnerOuterMarker>(
        &self,
        handle: FixedFaceHandle<InnerOuter>,
    ) -> FaceHandle<InnerOuter, Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>
    {
        self.s().face(handle)
    }

    /// Returns a reference handle to the single outer face of this triangulation.
    fn outer_face(
        &self,
    ) -> FaceHandle<
        PossiblyOuterTag,
        Self::Vertex,
        Self::DirectedEdge,
        Self::UndirectedEdge,
        Self::Face,
    > {
        self.s().outer_face()
    }

    /// Converts a fixed directed edge handle to a reference directed edge handle.
    ///
    /// *See also the [handles](crate::vendored_spade::handles) module for more information.*
    fn directed_edge(
        &self,
        handle: FixedDirectedEdgeHandle,
    ) -> DirectedEdgeHandle<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>
    {
        DirectedEdgeHandle::new(self.s(), handle)
    }

    /// Converts a fixed undirected edge handle to a reference undirected edge handle.
    ///
    /// *See also the [handles](crate::vendored_spade::handles) module for more information.*
    fn undirected_edge(
        &self,
        handle: FixedUndirectedEdgeHandle,
    ) -> UndirectedEdgeHandle<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>
    {
        UndirectedEdgeHandle::new(self.s(), handle)
    }

    /// Returns a mutable reference ot the associated data of an undirected edge.
    fn undirected_edge_data_mut(
        &mut self,
        handle: FixedUndirectedEdgeHandle,
    ) -> &mut Self::UndirectedEdge {
        self.s_mut().undirected_edge_data_mut(handle)
    }

    /// Returns the number of all faces, including the single outer face, of this
    /// triangulation.
    ///
    /// This is always equal to `triangulation.num_inner_faces() + 1`.
    fn num_all_faces(&self) -> usize {
        self.s().num_faces()
    }

    /// Returns the number of inner faces in this triangulation.
    fn num_inner_faces(&self) -> usize {
        self.s().num_faces() - 1
    }

    /// Returns the number of undirected edges in this triangulation.
    fn num_undirected_edges(&self) -> usize {
        self.s().num_undirected_edges()
    }

    /// Returns the number of directed edges in this triangulation.
    fn num_directed_edges(&self) -> usize {
        self.s().num_directed_edges()
    }

    /// Returns the number of edges of the convex hull.
    ///
    /// *See also [convex_hull](Triangulation::convex_hull)*
    ///
    /// # Complexity
    /// This method does not need to iterate through the convex hull and has a complexity of O(1)
    fn convex_hull_size(&self) -> usize {
        if self.all_vertices_on_line() {
            self.num_directed_edges()
        } else {
            let num_inner_edges = self.num_inner_faces() * 3;
            self.num_directed_edges() - num_inner_edges
        }
    }

    /// An iterator visiting all directed edges.
    ///
    /// The iterator type is [DirectedEdgeHandle].
    fn directed_edges(
        &self,
    ) -> DirectedEdgeIterator<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>
    {
        self.s().directed_edges()
    }

    /// An iterator over all undirected edges.
    ///
    /// The iterator type is [UndirectedEdgeHandle]
    fn undirected_edges(
        &self,
    ) -> UndirectedEdgeIterator<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>
    {
        self.s().undirected_edges()
    }

    /// Returns the number vertices in this triangulation.
    fn num_vertices(&self) -> usize {
        self.s().num_vertices()
    }

    /// An iterator visiting all vertices.
    ///
    /// The iterator type is [VertexHandle]
    fn vertices(
        &self,
    ) -> VertexIterator<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face> {
        self.s().vertices()
    }

    /// An iterator visiting all vertices.
    ///
    /// The iterator type is [FixedVertexHandle]
    fn fixed_vertices(&self) -> FixedVertexIterator {
        self.s().fixed_vertices()
    }

    /// Get a vertex by its index
    ///
    /// To get the handle, wrap your local index in a FixedVertexHandle:
    /// `let handle = FixedVertexHandle::from_index(my_index);`
    #[allow(clippy::type_complexity)]
    fn get_vertex(
        &self,
        handle: FixedVertexHandle,
    ) -> Option<VertexHandle<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>>
    {
        self.s().get_vertex(handle)
    }

    /// An iterator visiting all faces.
    ///
    /// The first returned face is the outer face, all other faces will be inner faces.
    /// The iterator type is [FaceHandle<PossiblyOuterTag, ...>](FaceHandle).
    ///
    /// *See also [inner_faces()](Triangulation::inner_faces())*
    fn all_faces(
        &self,
    ) -> FaceIterator<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face> {
        self.s().faces()
    }

    /// An iterator visiting all inner faces.
    //
    /// The iterator type is [FaceHandle<InnerTag, ...>](FaceHandle).
    fn inner_faces(
        &self,
    ) -> InnerFaceIterator<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face> {
        self.s().inner_faces()
    }

    /// An iterator visiting all faces of the Voronoi diagram.
    ///
    /// The iterator type is [VoronoiFace]
    fn voronoi_faces(
        &self,
    ) -> VoronoiFaceIterator<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>
    {
        VoronoiFaceIterator::new(self.s())
    }

    /// An iterator visiting all directed voronoi edges.
    ///
    /// The iterator type is (DirectedVoronoiEdge)[DirectedVoronoiEdge]
    fn directed_voronoi_edges(
        &self,
    ) -> DirectedVoronoiEdgeIterator<
        Self::Vertex,
        Self::DirectedEdge,
        Self::UndirectedEdge,
        Self::Face,
    > {
        DirectedVoronoiEdgeIterator::new(self.s())
    }

    /// An iterator visiting all undirected voronoi edges.
    ///
    /// The iterator type is (UndirectedVoronoiEdge)[UndirectedVoronoiEdge]
    fn undirected_voronoi_edges(
        &self,
    ) -> UndirectedVoronoiEdgeIterator<
        Self::Vertex,
        Self::DirectedEdge,
        Self::UndirectedEdge,
        Self::Face,
    > {
        UndirectedVoronoiEdgeIterator::new(self.s())
    }

    /// Returns information about the location of a point in a triangulation.
    fn locate(
        &self,
        point: Point2<<Self::Vertex as HasPosition>::Scalar>,
    ) -> PositionInTriangulation {
        let hint = self.hint_generator().get_hint(point);
        self.locate_with_hint_option_core(point, Some(hint))
    }

    /// Locates a vertex at a given position.
    ///
    /// Returns `None` if the point could not be found.
    #[allow(clippy::type_complexity)]
    fn locate_vertex(
        &self,
        point: Point2<<Self::Vertex as HasPosition>::Scalar>,
    ) -> Option<VertexHandle<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>>
    {
        match self.locate(point) {
            PositionInTriangulation::OnVertex(vertex) => Some(self.vertex(vertex)),
            _ => None,
        }
    }

    /// Returns an edge between two vertices.
    ///
    /// If the edge does not exist, `None` is returned.
    /// This operation runs in `O(n)` time, where `n` is
    /// the degree of `from`.
    #[allow(clippy::type_complexity)]
    fn get_edge_from_neighbors(
        &self,
        from: FixedVertexHandle,
        to: FixedVertexHandle,
    ) -> Option<
        DirectedEdgeHandle<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>,
    > {
        self.s().get_edge_from_neighbors(from, to)
    }

    /// Returns information about the location of a point in a triangulation.
    ///
    /// Additionally, a hint can be given to speed up computation.
    /// The hint should be a vertex close to the position that
    /// is being looked up.
    ///
    /// *See also [locate](Triangulation::locate), [locate_vertex](Triangulation::locate_vertex)*
    fn locate_with_hint(
        &self,
        point: Point2<<Self::Vertex as HasPosition>::Scalar>,
        hint: FixedVertexHandle,
    ) -> PositionInTriangulation {
        self.locate_with_hint_option_core(point, Some(hint))
    }

    /// Inserts a new vertex into the triangulation.
    ///
    /// A hint can be given to speed up the process.
    /// The hint should be a handle of a vertex close to the new vertex.
    /// in this case the insertion time can be reduced to O(1) on average
    /// if the hint is close. If the hint is randomized, running time will
    /// be O(sqrt(n)) on average with an O(n) worst case.
    ///
    /// *See also [insert](Triangulation::insert)*
    fn insert_with_hint(
        &mut self,
        t: Self::Vertex,
        hint: FixedVertexHandle,
    ) -> Result<FixedVertexHandle, InsertionError> {
        self.insert_with_hint_option(t, Some(hint))
    }

    /// Attempts to remove a vertex from the triangulation.
    ///
    /// Returns the removed vertex data if it could be found.
    ///
    /// # Handle invalidation
    /// This method will invalidate all vertex, edge and face handles
    /// upon successful removal.
    ///
    /// *See also [remove](Triangulation::remove)*
    fn locate_and_remove(
        &mut self,
        point: Point2<<Self::Vertex as HasPosition>::Scalar>,
    ) -> Option<Self::Vertex> {
        match self.locate_with_hint_option_core(point, None) {
            PositionInTriangulation::OnVertex(handle) => Some(self.remove_and_notify(handle)),
            _ => None,
        }
    }

    /// Removes a vertex from the triangulation.
    ///
    /// This operation runs in O(d²), where d is the degree of the
    /// removed vertex (the number of its outgoing edges).
    ///
    /// # Handle invalidation
    /// This method will invalidate all vertex, edge and face handles.
    fn remove(&mut self, vertex: FixedVertexHandle) -> Self::Vertex {
        self.remove_and_notify(vertex)
    }

    /// Inserts a new vertex into the triangulation.
    ///
    /// This operation runs in O(log(n)) on average when using a tree
    /// lookup to back up the triangulation, or in O(sqrt(n)) when using
    /// a walk lookup. n denotes the number of vertices, the given
    /// running times assume that input data is given uniformly randomly
    /// distributed. If the point has already been contained in the
    /// triangulation, the old vertex is overwritten.
    ///
    /// Returns either a handle to the new vertex or an error if the vertex could not be inserted.
    /// The triangulation will remain unchanged if an error ocurred.
    ///
    /// Use [vertex](Triangulation::vertex) to retrieve more information about the inserted vertex.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(), spade::InsertionError> {
    /// use spade::{DelaunayTriangulation, InsertionError, Triangulation, Point2};
    ///
    /// let mut triangulation = DelaunayTriangulation::<_>::default();
    ///
    /// let vertices = vec![Point2::new(0.0, 1.0), Point2::new(4.0, 2.0), Point2::new(3.0, 4.0)];
    /// for vertex in vertices {
    ///   // While not required in this example, it might be a good idea in general to prevent underflow errors like this:
    ///   let corrected_position = spade::mitigate_underflow(vertex);
    ///   triangulation.insert(corrected_position)?;
    /// }
    ///
    /// // Done!
    /// assert_eq!(triangulation.num_inner_faces(), 1);
    /// assert_eq!(triangulation.num_vertices(), 3);
    /// # Ok(()) }
    /// ```
    ///
    /// *See also [insert_with_hint](Triangulation::insert_with_hint), [validate_vertex](crate::vendored_spade::validate_vertex),
    ///  [mitigate_underflow](crate::vendored_spade::mitigate_underflow), [bulk_load](Triangulation::bulk_load)*
    fn insert(&mut self, vertex: Self::Vertex) -> Result<FixedVertexHandle, InsertionError> {
        self.insert_with_hint_option(vertex, None)
    }

    /// An iterator visiting all undirected edges.
    ///
    /// The iterator type is [FixedUndirectedEdgeHandle].
    fn fixed_undirected_edges(&self) -> FixedUndirectedEdgeIterator {
        FixedUndirectedEdgeIterator::new(self.num_undirected_edges())
    }

    /// An iterator visiting all directed edges.
    ///
    /// The iterator type is [FixedDirectedEdgeHandle].
    fn fixed_directed_edges(&self) -> FixedDirectedEdgeIterator {
        FixedDirectedEdgeIterator::new(self.num_directed_edges())
    }

    /// An iterator visiting all faces.
    ///
    /// The first returned element is the outer face. All other
    /// faces are inner faces.
    ///
    /// The iterator type is [FixedFaceHandle<PossiblyOuterTag, ...>](FixedFaceHandle).
    fn fixed_all_faces(&self) -> FixedFaceIterator {
        FixedFaceIterator::new(self.num_all_faces())
    }

    /// An iterator visiting all inner faces of the triangulation.
    ///
    /// The iterator type is [FixedFaceHandle<InnerTag, ...>](FixedFaceHandle).
    fn fixed_inner_faces(&self) -> FixedInnerFaceIterator {
        let mut result = FixedInnerFaceIterator::new(self.num_all_faces());
        result.next();
        result
    }

    /// Returns `true` if all vertices lie on a single line.
    ///
    /// This is always the case for triangulations with 0, 1 or two vertices.
    fn all_vertices_on_line(&self) -> bool {
        self.num_all_faces() == 1
    }

    /// Returns an iterator over all convex hull edges.
    ///
    /// The edges are returned in clockwise order as seen from any point in the triangulation.
    ///
    ///
    /// *A triangulation with its convex hull being highlighted. `e0` .. `e5` denote the returned
    /// edges in clockwise order.*
    ///
    /// *See also [convex_hull_size](Triangulation::convex_hull_size)*
    fn convex_hull(
        &self,
    ) -> HullIterator<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face> {
        {
            HullIterator::new(self.s())
        }
    }

    /// Returns a mutable reference to the associated data of a face.
    fn face_data_mut<InnerOuter: InnerOuterMarker>(
        &mut self,
        handle: FixedFaceHandle<InnerOuter>,
    ) -> &mut Self::Face {
        self.s_mut().face_data_mut(handle)
    }

    /// Returns a mutable reference to the associated data of a directed edge.
    fn directed_edge_data_mut(
        &mut self,
        handle: FixedDirectedEdgeHandle,
    ) -> &mut Self::DirectedEdge {
        self.s_mut().directed_edge_data_mut(handle)
    }
}

/// Implements general functions for triangulations over floating point data types.
///
/// This trait is implemented for any triangulation (constrained and regular Delaunay triangulations)
/// over `f32` and `f64`.
pub trait FloatTriangulation: Triangulation
where
    <Self::Vertex as HasPosition>::Scalar: Float,
{
    /// Returns all edges contained in a rectangle.
    ///
    /// An edge is considered to be contained in the rectangle if at least one point exists
    /// that is both on the edge and inside the rectangle (including its boundary).
    ///
    /// The rectangle is specified by its lower and upper corners. Yields an empty iterator
    /// if `lower.x > upper.x` or `lower.y > upper.y`.
    ///
    ///
    /// *Example: Shows all edges (red) that are returned when iterating over a rectangle (teal)*
    ///
    /// # Memory consumption
    ///
    /// Memory usage is, on average, in O(|convex_hull(E)|) where "E" refers to all edges that
    /// have been returned so far.
    fn get_edges_in_rectangle(
        &self,
        lower: Point2<<Self::Vertex as HasPosition>::Scalar>,
        upper: Point2<<Self::Vertex as HasPosition>::Scalar>,
    ) -> EdgesInShapeIterator<Self, RectangleMetric<<Self::Vertex as HasPosition>::Scalar>> {
        let distance_metric = RectangleMetric::new(lower, upper);
        let center = lower.add(upper).mul(0.5f32.into());
        EdgesInShapeIterator {
            inner_iter: FloodFillIterator::new(self, distance_metric, center),
        }
    }

    /// Returns all edges contained in a circle.
    ///
    /// An edge is considered to be contained in a circle if at least one point exists that is both
    /// on the edge and within the circle (including its boundary).
    ///
    /// `radius_2` refers to the **squared radius** of the circle.
    ///
    ///
    /// *Example: Shows all edges (red) that are returned when iterating over a circle (teal)*
    ///
    /// # Panics
    ///
    /// Panics if `radius_2 < 0.0`
    ///
    /// # Memory consumption
    ///
    /// Memory usage is, on average, in O(|convex_hull(E)|) where "E" refers to all edges that
    /// have been returned so far.
    fn get_edges_in_circle(
        &self,
        center: Point2<<Self::Vertex as HasPosition>::Scalar>,
        radius_2: <Self::Vertex as HasPosition>::Scalar,
    ) -> EdgesInShapeIterator<Self, CircleMetric<<Self::Vertex as HasPosition>::Scalar>> {
        let metric = CircleMetric::new(center, radius_2);
        EdgesInShapeIterator {
            inner_iter: FloodFillIterator::new(self, metric, center),
        }
    }

    /// Returns all vertices in a rectangle.
    ///
    /// Any vertex on the rectangle's boundary or corners is also returned.
    ///
    /// The rectangle is specified by its lower and upper corners. Yields an empty iterator
    /// if `lower.x > upper.x || lower.y > upper.y`.
    ///
    ///
    /// *Example: Shows all vertices (red) that are returned when iterating over a rectangle (teal)*
    ///
    /// # Memory consumption
    ///
    /// Consumed memory is in `O(|convex_hull(V)|)` where `V` refers to all vertices that have been
    /// returned so far.
    fn get_vertices_in_rectangle(
        &self,
        lower: Point2<<Self::Vertex as HasPosition>::Scalar>,
        upper: Point2<<Self::Vertex as HasPosition>::Scalar>,
    ) -> VerticesInShapeIterator<Self, RectangleMetric<<Self::Vertex as HasPosition>::Scalar>> {
        let distance_metric = RectangleMetric::new(lower, upper);
        let center = lower.add(upper).mul(0.5f32.into());

        VerticesInShapeIterator::new(FloodFillIterator::new(self, distance_metric, center))
    }

    /// Returns all vertices in a circle.
    ///
    /// Any vertex on the circle's boundary is also returned.
    ///
    /// `radius_2` refers to the **squared radius** of the circle.
    ///
    ///
    /// *Example: Shows all vertices (red) that are returned when iterating over a circle (teal)*
    ///
    /// # Panics
    ///
    /// Panics if `radius_2 < 0.0`
    ///
    /// # Memory consumption
    ///
    /// Consumed memory is in `O(|convex_hull(V)|)` where `V` refers to all vertices that have been
    /// returned so far.
    fn get_vertices_in_circle(
        &self,
        center: Point2<<Self::Vertex as HasPosition>::Scalar>,
        radius_2: <Self::Vertex as HasPosition>::Scalar,
    ) -> VerticesInShapeIterator<Self, CircleMetric<<Self::Vertex as HasPosition>::Scalar>> {
        let distance_metric = CircleMetric::new(center, radius_2);

        VerticesInShapeIterator::new(FloodFillIterator::new(self, distance_metric, center))
    }

    /// Used for barycentric interpolation on this triangulation. Refer to the documentation of
    /// [Barycentric] and [crate::vendored_spade::NaturalNeighbor] for more information.
    ///
    /// *Note:* In contrast to the other interpolation algorithms, barycentric interpolation also works
    /// for [crate::vendored_spade::ConstrainedDelaunayTriangulation]s.
    fn barycentric(&self) -> Barycentric<Self> {
        Barycentric::new(self)
    }
}

impl<T> FloatTriangulation for T
where
    T: Triangulation,
    <T::Vertex as HasPosition>::Scalar: Float,
{
}
