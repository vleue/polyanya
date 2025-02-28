use std::vec;
use std::vec::Vec;

use num_traits::{zero, Float};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::vendored_spade::cdt::GroupEnd::Existing;
use crate::vendored_spade::delaunay_core::dcel_operations::flip_cw;
use crate::vendored_spade::delaunay_core::{bulk_load_cdt, bulk_load_stable};
use crate::vendored_spade::{
    delaunay_core::Dcel, intersection_iterator::LineIntersectionIterator, SpadeNum,
};
use crate::vendored_spade::{handles::*, intersection_iterator::Intersection};
use crate::vendored_spade::{
    DelaunayTriangulation, HasPosition, HintGenerator, InsertionError, LastUsedVertexHintGenerator,
    Point2, Triangulation, TriangulationExt,
};

/// Undirected edge type of a [ConstrainedDelaunayTriangulation] (CDT).
///
/// CDTs need to store if an undirected edge is a constrained edge. To do so, CDTs don't use
/// the configured undirected edge type directly but wrap it into `CdtEdge<UE>` first.
///
/// This type will only be relevant if the triangulation's undirected edge type is being
/// overwritten.
///
/// # Type parameters
/// UE: The user configurable undirected edge type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub struct CdtEdge<UE>(bool, UE);

impl<UE> CdtEdge<UE> {
    /// Returns `true` if this edge is a constraint edge.
    pub fn is_constraint_edge(&self) -> bool {
        self.0
    }

    fn make_constraint_edge(&mut self) {
        assert!(!self.is_constraint_edge());
        self.0 = true;
    }

    /// Returns the wrapped undirected edge data type.
    pub fn data(&self) -> &UE {
        &self.1
    }

    /// Returns the wrapped undirected edge data type.
    pub fn data_mut(&mut self) -> &mut UE {
        &mut self.1
    }
}

impl<UE: Default> Default for CdtEdge<UE> {
    fn default() -> Self {
        CdtEdge(false, UE::default())
    }
}

impl<UE> AsRef<UE> for CdtEdge<UE> {
    fn as_ref(&self) -> &UE {
        self.data()
    }
}

impl<UE> AsMut<UE> for CdtEdge<UE> {
    fn as_mut(&mut self) -> &mut UE {
        self.data_mut()
    }
}

/// A two-dimensional
/// [constrained Delaunay triangulation](https://en.wikipedia.org/wiki/Constrained_Delaunay_triangulation).
///
/// A constrained Delaunay triangulation (CDT) is a triangulation that
/// can contain _constraint edges_. These edges will always be present
/// in the resulting triangulation.
///
///
/// *Left: A CDT with 4 constraint edges. Right: The same triangulation
/// without constraint edges*
///
///
/// The resulting triangulation
/// does not necessarily fulfill the Delaunay property.
///
/// This implementation currently supports only _weakly intersecting_
/// constraints, thus, constraint edges are allowed to touch at
/// their start or end point but are not allowed to intersect at
/// any interior point.
///
/// The constrained triangulation shares most of the implementation of
/// the usual Delaunay triangulation, refer to `DelaunayTriangulation`
/// for more information about type parameters, iteration, performance
/// and more examples.
///
/// # Example
///
/// ```
/// use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation};
/// # fn try_main() -> Result<(), spade::InsertionError> {
/// let mut cdt = ConstrainedDelaunayTriangulation::<Point2<_>>::new();
/// let v0 = cdt.insert(Point2::new(0f64, 0.0))?;
/// let v1 = cdt.insert(Point2::new(1.0, 0.0))?;
/// cdt.add_constraint(v0, v1);
/// // Alternatively, consider using this shorthand
/// cdt.add_constraint_edge(Point2::new(1.0, 1.0), Point2::new(1.0, 0.0))?;
/// println!("Number of constraints: {}", cdt.num_constraints()); // 2 constraints
/// // Constraints are bidirectional!
/// assert!(cdt.exists_constraint(v1, v0));
/// assert!(cdt.exists_constraint(v0, v1));
/// // Check if a new constraint could be added
/// let from = Point2::new(1.0, -2.0);
/// let to = Point2::new(1.0, 0.0);
/// if !cdt.intersects_constraint(from, to) {
///     // No intersections, the edge can be added
///     cdt.add_constraint_edge(from, to)?;
/// }
/// # Ok(()) }
/// # fn main() { try_main().unwrap() }
/// ```
///
/// # See also
/// Refer to [Triangulation] for most implemented methods on this type.
/// Refer to [DelaunayTriangulation](DelaunayTriangulation) for general
/// information about using Delaunay triangulations.
#[doc(alias = "CDT")]
#[derive(Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub struct ConstrainedDelaunayTriangulation<
    V,
    DE = (),
    UE = (),
    F = (),
    L = LastUsedVertexHintGenerator,
> where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    dcel: Dcel<V, DE, CdtEdge<UE>, F>,
    num_constraints: usize,
    hint_generator: L,
}

impl<V, DE, UE, F, L> Default for ConstrainedDelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    fn default() -> Self {
        ConstrainedDelaunayTriangulation {
            dcel: Default::default(),
            num_constraints: 0,
            hint_generator: Default::default(),
        }
    }
}

impl<V, DE, UE, F, L> Triangulation for ConstrainedDelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    type Vertex = V;
    type DirectedEdge = DE;
    type UndirectedEdge = CdtEdge<UE>;
    type Face = F;
    type HintGenerator = L;

    fn s(&self) -> &Dcel<V, DE, CdtEdge<UE>, F> {
        &self.dcel
    }

    fn s_mut(&mut self) -> &mut Dcel<V, DE, CdtEdge<UE>, F> {
        &mut self.dcel
    }

    fn is_defined_legal(&self, edge: FixedUndirectedEdgeHandle) -> bool {
        self.is_constraint_edge(edge)
    }

    fn handle_legal_edge_split(&mut self, handles: [FixedDirectedEdgeHandle; 2]) {
        self.num_constraints += 1;
        for handle in handles.iter().map(|e| e.as_undirected()) {
            if !self.is_constraint_edge(handle) {
                self.dcel
                    .undirected_edge_data_mut(handle)
                    .make_constraint_edge();
            }
        }
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
        Self {
            dcel,
            num_constraints,
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
        (self.dcel, self.hint_generator, self.num_constraints)
    }

    fn clear(&mut self) {
        self.num_constraints = 0;
        self.s_mut().clear();
        let new_hint_generator = HintGenerator::initialize_from_triangulation(self);
        *self.hint_generator_mut() = new_hint_generator;
    }
}

impl<V, DE, UE, F, L> From<DelaunayTriangulation<V, DE, UE, F, L>>
    for ConstrainedDelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    fn from(value: DelaunayTriangulation<V, DE, UE, F, L>) -> Self {
        let dcel = value.dcel;
        let s = dcel.map_undirected_edges(|edge| CdtEdge(false, edge));
        let lookup = value.hint_generator;

        ConstrainedDelaunayTriangulation {
            dcel: s,
            num_constraints: 0,
            hint_generator: lookup,
        }
    }
}

impl<V, DE, UE, F, L> ConstrainedDelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    /// Efficient bulk loading of a constraint delaunay triangulation, including both vertices and constraint edges.
    ///
    /// The edges are given as pairs of vertex indices.
    ///
    /// Note that the vertex order is not preserved by this function - iterating through all vertices will not result in
    /// the same sequence as the input vertices. Use [ConstrainedDelaunayTriangulation::bulk_load_cdt_stable] for a
    /// slower but order preserving variant.
    ///
    /// Input vertices may have the same position. However, only one vertex for each position will be kept. Edges
    /// that go to a discarded vertex are rerouted and still inserted.
    /// It is arbitrary which duplicated vertex remains.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(), spade::InsertionError> {
    /// use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation};
    /// let mut vertices = vec![
    ///     Point2::new(0.0, 1.0),
    ///     Point2::new(1.0, 2.0),
    ///     Point2::new(3.0, -3.0),
    ///     Point2::new(-1.0, -2.0),
    ///     Point2::new(-4.0, -5.0),
    /// ];
    /// let mut edges = vec![[0, 1], [1, 2], [2, 3], [3, 4]];
    /// let cdt = ConstrainedDelaunayTriangulation::<_>::bulk_load_cdt(vertices.clone(), edges)?;
    ///
    /// assert_eq!(cdt.num_vertices(), 5);
    /// assert_eq!(cdt.num_constraints(), 4);
    /// // The order will usually change
    /// assert_ne!(cdt.vertices().map(|v| v.position()).collect::<Vec<_>>(), vertices);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if any constraint edges overlap. Panics if the edges contain an invalid index (out of range).
    pub fn bulk_load_cdt(vertices: Vec<V>, edges: Vec<[usize; 2]>) -> Result<Self, InsertionError> {
        let mut result = bulk_load_cdt(vertices, edges)?;
        *result.hint_generator_mut() = L::initialize_from_triangulation(&result);
        Ok(result)
    }

    /// Stable bulk load variant that preserves the input vertex order
    ///
    /// The resulting vertex set will be equal to the input vertex set if their positions are all distinct.
    /// See [ConstrainedDelaunayTriangulation::bulk_load_cdt] for additional details like panic behavior and duplicate
    /// handling.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(), spade::InsertionError> {
    /// use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation};
    /// let mut vertices = vec![
    ///     Point2::new(0.0, 1.0),
    ///     Point2::new(1.0, 2.0),
    ///     Point2::new(3.0, -3.0),
    ///     Point2::new(-1.0, -2.0),
    ///     Point2::new(-4.0, -5.0),
    /// ];
    /// let mut edges = vec![[0, 1], [1, 2], [2, 3], [3, 4]];
    /// let cdt = ConstrainedDelaunayTriangulation::<_>::bulk_load_cdt_stable(vertices.clone(), edges)?;
    ///
    /// // The ordered will be preserved:
    /// assert_eq!(cdt.vertices().map(|v| v.position()).collect::<Vec<_>>(), vertices);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// It is fine to include vertex positions multiple times. The resulting order will be the same as if
    /// the duplicates were removed prior to insertion. However, it is unclear *which* duplicates are
    /// removed - e.g. do not assume that always the first duplicated vertex remains.
    ///
    /// ```
    /// # fn main() -> Result<(), spade::InsertionError> {
    /// use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation};
    /// let mut vertices = vec![
    ///     Point2::new(0.0, 1.0),
    ///     Point2::new(1.0, 2.0), // Duplicate
    ///     Point2::new(1.0, 2.0),
    ///     Point2::new(3.0, -3.0),
    ///     Point2::new(3.0, -3.0), // Duplicate
    ///     Point2::new(-4.0, -5.0),
    /// ];
    /// let mut edges = vec![[0, 1], [2, 3], [4, 5]];
    /// let cdt = ConstrainedDelaunayTriangulation::<_>::bulk_load_cdt_stable(vertices.clone(), edges)?;
    ///
    /// // The choice of deduplicated vertices is arbitrary. In this example, dedup[1] and dedup[2] could
    /// // have been swapped
    /// let dedup = [
    ///     Point2::new(0.0, 1.0),
    ///     Point2::new(1.0, 2.0),
    ///     Point2::new(3.0, -3.0),
    ///     Point2::new(-4.0, -5.0),
    /// ];
    /// assert_eq!(cdt.vertices().map(|v| v.position()).collect::<Vec<_>>(), dedup);
    /// # Ok(())
    /// # }
    /// ```
    pub fn bulk_load_cdt_stable(
        vertices: Vec<V>,
        edges: Vec<[usize; 2]>,
    ) -> Result<Self, InsertionError> {
        let mut result: Self =
            bulk_load_stable(move |vertices| bulk_load_cdt(vertices, edges), vertices)?;
        *result.hint_generator_mut() = L::initialize_from_triangulation(&result);
        Ok(result)
    }

    /// Removes a vertex from the triangulation.
    ///
    /// This operation runs in O(n²), where n is the degree of the
    /// removed vertex.
    ///
    /// # Handle invalidation
    /// This method will invalidate all vertex, edge and face handles.
    pub fn remove(&mut self, vertex: FixedVertexHandle) -> V {
        let num_removed_constraints = self
            .dcel
            .vertex(vertex)
            .out_edges()
            .map(|edge| edge.is_constraint_edge())
            .filter(|b| *b)
            .count();
        self.num_constraints -= num_removed_constraints;
        self.remove_and_notify(vertex)
    }

    /// Returns the number of constraint edges.
    pub fn num_constraints(&self) -> usize {
        self.num_constraints
    }

    /// Returns `true` if a given edge is a constraint edge.
    pub fn is_constraint_edge(&self, edge: FixedUndirectedEdgeHandle) -> bool {
        self.dcel.undirected_edge_data(edge).is_constraint_edge()
    }

    /// Checks if two vertices are connected by a constraint edge.
    pub fn exists_constraint(&self, from: FixedVertexHandle, to: FixedVertexHandle) -> bool {
        self.get_edge_from_neighbors(from, to)
            .map(|e| e.is_constraint_edge())
            .unwrap_or(false)
    }

    /// Checks if a constraint edge can be added.
    ///
    /// Returns `false` if the line from `from` to `to` intersects another
    /// constraint edge.
    pub fn can_add_constraint(&self, from: FixedVertexHandle, to: FixedVertexHandle) -> bool {
        let line_intersection_iterator = LineIntersectionIterator::new_from_handles(self, from, to);
        !self.contains_any_constraint_edge(line_intersection_iterator)
    }

    /// Checks if a line intersects a constraint edge.
    ///
    /// Returns `true` if the edge from `from` to `to` intersects a
    /// constraint edge.
    pub fn intersects_constraint(
        &self,
        line_from: Point2<V::Scalar>,
        line_to: Point2<V::Scalar>,
    ) -> bool {
        let line_intersection_iterator = LineIntersectionIterator::new(self, line_from, line_to);
        self.contains_any_constraint_edge(line_intersection_iterator)
    }

    fn contains_any_constraint_edge(
        &self,
        mut line_intersection_iterator: LineIntersectionIterator<V, DE, CdtEdge<UE>, F>,
    ) -> bool {
        line_intersection_iterator.any(|intersection| match intersection {
            Intersection::EdgeIntersection(edge) => edge.is_constraint_edge(),
            _ => false,
        })
    }

    /// Creates a several constraint edges by taking and connecting vertices from an iterator.
    ///
    /// Every two sequential vertices in the input iterator will be connected by a constraint edge.
    /// If `closed` is set to true, the first and last vertex will also be connected.
    ///
    /// # Special cases:
    ///  - Does nothing if input iterator is empty
    ///  - Only inserts the single vertex if the input iterator contains exactly one element
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(), spade::InsertionError> {
    /// use spade::{ConstrainedDelaunayTriangulation, Point2};
    ///
    /// const NUM_VERTICES: usize = 51;
    ///
    /// let mut cdt = ConstrainedDelaunayTriangulation::<_>::default();
    ///
    /// // Iterates through vertices on a circle
    /// let vertices = (0..NUM_VERTICES).map(|i| {
    ///     let angle = std::f64::consts::PI * 2.0 * i as f64 / NUM_VERTICES as f64;
    ///     let (sin, cos) = angle.sin_cos();
    ///     Point2::new(sin, cos)
    /// });
    ///
    /// cdt.add_constraint_edges(vertices, true)?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if any of the generated constraints intersects with any other constraint edge.
    pub fn add_constraint_edges(
        &mut self,
        vertices: impl IntoIterator<Item = V>,
        closed: bool,
    ) -> Result<(), InsertionError> {
        let mut iter = vertices.into_iter();
        if let Some(first) = iter.next() {
            let first_handle = self.insert(first)?;
            let mut previous_handle = first_handle;
            let mut current_handle = first_handle;
            for current in iter {
                current_handle = self.insert(current)?;
                self.add_constraint(previous_handle, current_handle);
                previous_handle = current_handle;
            }

            if closed && current_handle != first_handle {
                self.add_constraint(current_handle, first_handle);
            }
        }

        Ok(())
    }

    /// Insert two points and creates a constraint between them.
    ///
    /// Returns `true` if at least one constraint edge was added.
    ///
    /// # Panics
    ///
    /// Panics if the new constraint edge intersects with an existing
    /// constraint edge. Use [can_add_constraint](Self::can_add_constraint) to check.
    pub fn add_constraint_edge(&mut self, from: V, to: V) -> Result<bool, InsertionError> {
        let from_handle = self.insert(from)?;
        let to_handle = self.insert(to)?;
        Ok(self.add_constraint(from_handle, to_handle))
    }

    /// Adds a constraint edge between to vertices.
    ///
    /// Returns `true` if at least one constraint edge was added.
    /// Note that the given constraint might be split into smaller edges
    /// if a vertex in the triangulation lies exactly on the constraint edge.
    /// Thus, `cdt.exists_constraint(from, to)` is not necessarily `true`
    /// after a call to this function.
    ///
    /// Returns false and does nothing if `from == to`.
    ///
    /// # Panics
    ///
    /// Panics if the new constraint edge intersects an existing
    /// constraint edge. Use [Self::try_add_constraint] or [Self::add_constraint_and_split] to work
    /// around that.
    pub fn add_constraint(&mut self, from: FixedVertexHandle, to: FixedVertexHandle) -> bool {
        let initial_num_constraints = self.num_constraints();
        self.try_add_constraint_inner(from, to, |_| panic!("Constraint edges must not intersect."));

        self.num_constraints != initial_num_constraints
    }

    /// Takes a conflict region (expressed as a list of intersecting edges) rotates edges to create
    /// a new constraint edge. Then, the rotated edges (except the new constraint edge)
    /// are legalized to restore the Delaunay property.
    ///
    /// Usually, this step is described as "delete all conflicting edges, then re-triangulate the
    /// hole". Spade avoids the removal of edges by _rotating_ (flipping) them into place instead.
    /// The final constraint edge is created implicitly.
    /// This works as long as the intersecting edges are ordered "along the constraint edge", i.e.
    /// the intersection points increase in distance from the constraint edge origin.
    ///
    /// # Example
    ///
    /// The input conflict region might look like this (assuming the target constraint edge goes
    /// from v0 to v1):
    ///
    /// ```text
    ///     v__________v
    ///   / |        / |\
    ///  /  |      /   | \
    /// v0  |e0  /e1 e2| v1
    ///  \  |  /       | /
    ///   \ |/         |/
    ///     v_________ v
    /// ```
    ///
    /// `conflict_edges` would be set to `vec![e0, e1, e2]` in this case, `target_vertex` would be
    /// `v1`.
    ///
    /// Now, flipping these edges _in this order_ will implicitly create the desired edge:
    ///
    /// After flipping the result looks like this with all edges going out of `v0`:
    ///
    /// ```text
    ///     v_________v
    ///   /     __---  \
    ///  / __---        \
    /// v0--------------v1
    ///  \ --___        /
    ///   \     --___  /
    ///     v---------v
    ///```
    ///
    /// Now, the new edges can be legalized as usual.
    ///
    /// Returns a handle to the new constraint edge (pointing toward `target_vertex`).
    fn resolve_conflict_region(
        &mut self,
        conflict_edges: Vec<FixedDirectedEdgeHandle>,
        target_vertex: FixedVertexHandle,
    ) -> Option<FixedDirectedEdgeHandle> {
        let first = conflict_edges.first()?;

        let mut temporary_constraint_edges = Vec::new();

        let first = self.directed_edge(*first);

        if first.to().fix() == target_vertex {
            // Special case: This function is also used to handle constraint edges that fully
            // _overlap_ an existing edge. This makes it easier to return all created edges in their
            // correct order. This will be designated by a "conflict region" consisting of a single
            // edge that ends at the target vertex (which should not happen otherwise).
            return Some(first.fix());
        }

        // These refer to the two edges that go out of the constraint edge origin initially.
        // They are used below but need to be defined declared here to appease the borrow checker.
        let first_border_edge = first.rev().prev().fix();
        let last_border_edge = first.rev().next().fix();

        // Flip all conflict edges in the input order - see function comment.
        for edge in &conflict_edges {
            flip_cw(self.s_mut(), edge.as_undirected());
        }

        // Small optimization: For the legalization, the algorithm doesn't need to look at edges
        // outside the conflict region. They are known to be already legal.
        // To do so, we will make the border edges that encompass the conflict region into temporary
        // constraint edges. The legalization will then skip them. This is undone later,
        let mut make_temporary_edge = |cdt: &mut Self, edge: FixedUndirectedEdgeHandle| {
            // Exclude edges that are already a constraint - those should remain constraint edges
            // and not be undone later!
            if !cdt.undirected_edge(edge).is_constraint_edge() {
                temporary_constraint_edges.push(edge);
                cdt.undirected_edge_data_mut(edge).make_constraint_edge();
            }
        };

        make_temporary_edge(self, first_border_edge.as_undirected());
        make_temporary_edge(self, last_border_edge.as_undirected());

        let mut current = first_border_edge;

        let mut result = None;

        // Loops around all border edges and adds them to the temporary constraint edge list.
        // `first_border_edge` and `last_border_edge` refer to the two border edges that are
        // initially going out of the constraint edge start (the two left most edges in the first
        // ascii drawing of the function comment).
        while current != last_border_edge.rev() {
            let handle = self.directed_edge(current);
            let fixed = handle.fix();
            let next = handle.next().fix().as_undirected();

            current = handle.ccw().fix();
            if target_vertex == handle.to().fix() {
                // This loop also finds the implicitly created constraint edge and makes it an
                // official constraint edge!
                self.make_constraint_edge(fixed.as_undirected());
                result = Some(fixed);
            }
            make_temporary_edge(self, next);
        }

        self.legalize_edges_after_removal(
            &mut conflict_edges
                .into_iter()
                .map(|edge| edge.as_undirected())
                .collect(),
            |_| false,
        );

        // Undo the previously made temporary constraint edges
        for edge in temporary_constraint_edges {
            self.undirected_edge_data_mut(edge).0 = false;
        }

        result
    }

    /// Returns all constraint edges that would prevent creating a new constraint between two points.
    ///
    /// # See also
    ///
    /// See also [Self::get_conflicting_edges_between_vertices]
    pub fn get_conflicting_edges_between_points(
        &self,
        from: Point2<<V as HasPosition>::Scalar>,
        to: Point2<<V as HasPosition>::Scalar>,
    ) -> impl Iterator<Item = DirectedEdgeHandle<V, DE, CdtEdge<UE>, F>> {
        LineIntersectionIterator::new(self, from, to)
            .flat_map(|intersection| intersection.as_edge_intersection())
            .filter(|e| e.is_constraint_edge())
    }

    /// Returns all constraint edges that would prevent inserting a new constraint connecting two existing
    /// vertices.
    ///
    /// # See also
    ///
    /// See also [Self::get_conflicting_edges_between_points]
    pub fn get_conflicting_edges_between_vertices(
        &self,
        from: FixedVertexHandle,
        to: FixedVertexHandle,
    ) -> impl Iterator<Item = DirectedEdgeHandle<V, DE, CdtEdge<UE>, F>> {
        LineIntersectionIterator::new_from_handles(self, from, to)
            .flat_map(|intersection| intersection.as_edge_intersection())
            .filter(|e| e.is_constraint_edge())
    }

    fn make_constraint_edge(&mut self, edge: FixedUndirectedEdgeHandle) -> bool {
        if !self.is_constraint_edge(edge) {
            self.dcel
                .undirected_edge_data_mut(edge)
                .make_constraint_edge();
            self.num_constraints += 1;
            true
        } else {
            false
        }
    }

    /// Attempts to add a constraint edge. Leaves the triangulation unchanged if the new edge would
    /// intersect an existing constraint edge.
    ///
    /// Returns all constraint edges that connect `from` and `to`. This includes any constraint
    /// edge that was already present.
    /// Multiple edges are returned if the line from `from` to `to` intersects an existing vertex.
    /// Returns an empty list if the new constraint would intersect any existing constraint or if
    /// `from == to`.
    ///
    /// # Example
    ///
    /// ```
    /// use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation};
    /// # fn try_main() -> Result<(), spade::InsertionError> {
    /// let mut cdt = ConstrainedDelaunayTriangulation::<Point2<_>>::new();
    /// let v0 = cdt.insert(Point2::new(-1.0, 0.0))?;
    /// let v1 = cdt.insert(Point2::new(1.0, 0.0))?;
    /// let v2 = cdt.insert(Point2::new(0.0, 1.0))?;
    /// let v3 = cdt.insert(Point2::new(0.0, -1.0))?;
    /// let first_constraints = cdt.try_add_constraint(v2, v3);
    /// let second_constraints = cdt.try_add_constraint(v0, v1);
    ///
    /// // The first constraint edge can be added as there are no intersecting constraint edges
    /// assert_eq!(first_constraints.len(), 1);
    /// let edge = cdt.directed_edge(first_constraints[0]);
    /// assert_eq!(edge.from().fix(), v2);
    /// assert_eq!(edge.to().fix(), v3);
    ///
    /// // The second edge should not be created as it intersects the first edge.
    /// assert!(second_constraints.is_empty());
    ///
    /// // Consider comparing this to the number of constraints prior to calling
    /// // `try_add_constraint` to check if any new constraint edge was created.
    /// assert_eq!(cdt.num_constraints(), 1);
    /// # Ok(()) }
    /// # fn main() { try_main().unwrap() }
    /// ```
    pub fn try_add_constraint(
        &mut self,
        from: FixedVertexHandle,
        to: FixedVertexHandle,
    ) -> Vec<FixedDirectedEdgeHandle> {
        self.try_add_constraint_inner(from, to, |_| ConflictResolution::Cancel)
    }

    fn try_add_constraint_inner<R>(
        &mut self,
        from: FixedVertexHandle,
        to: FixedVertexHandle,
        mut conflict_resolver: R,
    ) -> Vec<FixedDirectedEdgeHandle>
    where
        R: FnMut(DirectedEdgeHandle<V, DE, CdtEdge<UE>, F>) -> ConflictResolution<V>,
    {
        // Constraint edges are added with a two-pass approach:
        // - First, identify potential constraint edge intersections (conflicts). This must be done
        //   beforehand in case that the caller chooses to `ConflictResolution::Cancel` the
        //   operation - no mutation should have happened at this stage.
        let initial_conflict_regions =
            self.get_conflict_resolutions(from, to, &mut conflict_resolver);
        // - Second, apply the conflict resolutions, e.g. by inserting new split points and by
        //   rotating non-constraint edges that intersect the new constraint edge (see function
        //   `resolve_conflict_region`).
        self.resolve_conflict_groups(to, initial_conflict_regions)
    }

    fn get_conflict_resolutions<R>(
        &mut self,
        from: FixedVertexHandle,
        to: FixedVertexHandle,
        conflict_resolver: &mut R,
    ) -> Vec<InitialConflictRegion<V>>
    where
        R: FnMut(DirectedEdgeHandle<V, DE, CdtEdge<UE>, F>) -> ConflictResolution<V>,
    {
        let mut conflict_groups = Vec::new();
        let mut current_group = Vec::new();
        for intersection in LineIntersectionIterator::new_from_handles(self, from, to) {
            match intersection {
                Intersection::EdgeIntersection(edge) => {
                    if edge.is_constraint_edge() {
                        match conflict_resolver(edge) {
                            ConflictResolution::Cancel => {
                                return Vec::new();
                            }
                            ConflictResolution::Split(new_vertex) => {
                                let group_end = GroupEnd::NewVertex(new_vertex, edge.fix());
                                let conflict_edges = core::mem::take(&mut current_group);
                                conflict_groups.push(InitialConflictRegion {
                                    conflict_edges,
                                    group_end,
                                });
                            }
                        }
                    } else {
                        current_group.push(edge.fix());
                    }
                }
                Intersection::VertexIntersection(v) => {
                    let group_end = Existing(v.fix());
                    let conflict_edges = core::mem::take(&mut current_group);
                    conflict_groups.push(InitialConflictRegion {
                        conflict_edges,
                        group_end,
                    });
                }
                Intersection::EdgeOverlap(edge) => {
                    conflict_groups.push(InitialConflictRegion {
                        conflict_edges: vec![edge.fix()],
                        group_end: Existing(edge.to().fix()),
                    });
                }
            }
        }

        conflict_groups
    }

    fn resolve_conflict_groups(
        &mut self,
        final_vertex: FixedVertexHandle,
        conflict_groups: Vec<InitialConflictRegion<V>>,
    ) -> Vec<FixedDirectedEdgeHandle> {
        let mut constraint_edges = Vec::new();
        let mut last_vertex = None;

        for InitialConflictRegion {
            conflict_edges,
            group_end,
        } in conflict_groups
        {
            let mut last_edge = None;
            let target_vertex = match group_end {
                Existing(v) => v,
                GroupEnd::NewVertex(v, conflict_edge) => {
                    let (new_vertex, [e0, e1]) = self.insert_on_edge(conflict_edge, v);
                    let e1_handle = self.directed_edge(e1);
                    // edge_in / edge_out refer to the edge going into / out of the newly split off
                    // vertex.
                    let edge_out = e1_handle.ccw();
                    let edge_in = e1_handle.cw();

                    if Some(edge_in.to().fix()) == last_vertex {
                        constraint_edges.push(edge_in.fix().rev());
                    }

                    if edge_out.to().fix() == final_vertex {
                        // The edge reaches the target vertex - we're done! However, this can
                        // sometimes omit to make the last edge a constraint. This special case
                        // fixes that issue.
                        last_edge = Some(edge_out.fix());
                    }

                    self.handle_legal_edge_split([e0, e1]);
                    new_vertex
                }
            };

            constraint_edges.extend(self.resolve_conflict_region(conflict_edges, target_vertex));
            constraint_edges.extend(last_edge);

            last_vertex = Some(target_vertex);
        }

        for edge in &constraint_edges {
            self.make_constraint_edge(edge.as_undirected());
        }

        constraint_edges
    }
}

impl<V, DE, UE, F, L> ConstrainedDelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    V::Scalar: Float,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    /// Adds a constraint to the triangulation. Splits any existing constraint edge that would
    /// intersect the new constraint edge.
    ///
    /// The `vertex_constructor` closure is used to convert the position of the intersection into
    /// a vertex. The returned vertex must have exactly the same position as the argument of the
    /// closure.
    ///
    /// Returns all constraint edges that connect `from` and `to`. This includes any constraint
    /// edge that was already present.
    /// Multiple edges are returned if the line from `from` to `to` intersects any existing vertex
    /// or any existing constraint edge.
    /// Returns an empty list if `from == to`.
    ///
    /// # Image example
    ///
    /// This is an input CDT with 3 constraints:
    ///
    ///
    /// Calling `add_constraint_and_split(v0, v1, ...)` will result in this CDT:
    ///
    ///
    /// # Code example
    ///
    ///```
    /// use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation};
    /// # fn try_main() -> Result<(), spade::InsertionError> {
    /// use spade::handles::FixedVertexHandle;
    /// let mut cdt = ConstrainedDelaunayTriangulation::<Point2<_>>::new();
    /// let v0 = cdt.insert(Point2::new(-1.0, 0.0))?;
    /// let v1 = cdt.insert(Point2::new(1.0, 0.0))?;
    /// let v2 = cdt.insert(Point2::new(0.0, 1.0))?;
    /// let v3 = cdt.insert(Point2::new(0.0, -1.0))?;
    /// cdt.add_constraint(v2, v3);
    ///
    /// // Should create a new split vertex at the origin
    /// let second_constraints = cdt.add_constraint_and_split(v0, v1, |v| v);
    ///
    /// // Expect one additional point introduced by splitting the first constraint edge:
    /// assert_eq!(cdt.num_vertices(), 5);
    ///
    /// let v4 = FixedVertexHandle::from_index(4); // Newly created
    ///
    /// // Expect 4 constraints as the first constraint was split:
    /// assert_eq!(cdt.num_constraints(), 4);
    ///
    /// // The second edge should consist of two edges, v0 -> v4 and v4 -> v1
    /// assert_eq!(second_constraints.len(), 2);
    ///
    /// let [e0, e1] = [second_constraints[0], second_constraints[1]];
    /// let [e0, e1] = [e0, e1].map(|e| cdt.directed_edge(e));
    ///
    /// assert_eq!(e0.from().fix(), v0);
    /// assert_eq!(e0.to().fix(), v4);
    /// assert_eq!(e1.from().fix(), v4);
    /// assert_eq!(e1.to().fix(), v1);
    ///
    /// # Ok(()) }
    /// # fn main() { try_main().unwrap() }
    /// ```
    ///
    /// # Precision warning
    ///
    /// Intersection points may not _exactly_ lie on the line between `from` and `to`, either due to
    /// loss of precision or as the exact value may not be representable with the underlying
    /// floating point number.
    ///
    /// Thus, iterating a `LineIntersectionIterator::new_from_handles(&cdt, from, to)` will often
    /// not return only `Intersection::EdgeOverlap` as would be expected. Instead, use the returned
    /// `Vec` to identify the edges that form the new constraint.
    /// The absolute deviation from the correct position should be minimal, especially when using
    /// `f64` coordinates as storage type.
    pub fn add_constraint_and_split<C>(
        &mut self,
        from: FixedVertexHandle,
        to: FixedVertexHandle,
        vertex_constructor: C,
    ) -> Vec<FixedDirectedEdgeHandle>
    where
        C: Fn(Point2<<V as HasPosition>::Scalar>) -> V,
    {
        let from_pos = self.vertex(from).position();
        let to_pos = self.vertex(to).position();

        self.try_add_constraint_inner(from, to, |edge| {
            let [p0, p1] = edge.positions();
            let line_intersection = line_intersection(p0, p1, from_pos, to_pos);
            let new_vertex = vertex_constructor(line_intersection);
            assert_eq!(new_vertex.position(), line_intersection);
            ConflictResolution::Split(new_vertex)
        })
    }
}

enum GroupEnd<V> {
    Existing(FixedVertexHandle),
    NewVertex(V, FixedDirectedEdgeHandle),
}

/// Represents a conflict region that does not yet fully exist as a vertex may be missing. This can
/// happen if adding a constraint edge should split any intersecting existing edge.
/// This will eventually be turned into a "real" conflict group (described as a list of edges) by
/// inserting the missing vertex.
struct InitialConflictRegion<V> {
    conflict_edges: Vec<FixedDirectedEdgeHandle>,
    group_end: GroupEnd<V>,
}

enum ConflictResolution<V> {
    Cancel,
    Split(V),
}

fn line_intersection<S>(p1: Point2<S>, p2: Point2<S>, p3: Point2<S>, p4: Point2<S>) -> Point2<S>
where
    S: SpadeNum + Float,
{
    let a1 = p2.y - p1.y;
    let b1 = p1.x - p2.x;
    let c1 = a1 * p1.x + b1 * p1.y;

    let a2 = p4.y - p3.y;
    let b2 = p3.x - p4.x;
    let c2 = a2 * p3.x + b2 * p3.y;

    let determinant = a1 * b2 - a2 * b1;

    if determinant == zero() {
        panic!("Lines are parallel");
    }

    let x = (b2 * c1 - b1 * c2) / determinant;
    let y = (a1 * c2 - a2 * c1) / determinant;
    Point2::new(x, y)
}
