//! # Spade
//!
//! Delaunay triangulations for the rust ecosystem.
//!
//! # Features
//! * A 2D Delaunay triangulation: [DelaunayTriangulation]
//! * Uses exact geometric predicate evaluation, preventing construction errors due to precision loss.
//! * A 2D constrained Delaunay triangulation: [ConstrainedDelaunayTriangulation]
//! * Supports vertex removal
//! * Serde support with the `serde` feature.
//! * `no_std` support with `default-features = false`
//! * Natural neighbor interpolation: [NaturalNeighbor]
//!
//! # Cargo features
//!
//! These features are disabled by default and need to be enabled in your `Cargo.toml`:
//!  - `serde`: Enable (de)serialization of (constrained) Delaunay triangulations with the
//!    [Serde](https://docs.rs/serde/latest/serde/) crate
//!  - `mint`: Enables rudimentary [mint](https://docs.rs/mint/latest/mint/) interoperability by
//!    implementing the `From` and `Into` conversion traits between `spade::Point2` and
//!    `mint::Point2`. Also implements [HasPosition] for `mint::Point2`.

#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![deny(rustdoc::broken_intra_doc_links)]

extern crate alloc;

mod cdt;
mod delaunay_core;
mod delaunay_triangulation;
mod flood_fill_iterator;
mod intersection_iterator;
mod point;

mod triangulation;

pub use crate::vendored_spade::cdt::{CdtEdge, ConstrainedDelaunayTriangulation};
pub use crate::vendored_spade::delaunay_triangulation::DelaunayTriangulation;
pub use crate::vendored_spade::point::{HasPosition, Point2, SpadeNum};

pub use crate::vendored_spade::delaunay_core::math::{
    mitigate_underflow, validate_coordinate, validate_vertex, InsertionError, PointProjection,
    MAX_ALLOWED_VALUE, MIN_ALLOWED_VALUE,
};

pub use delaunay_core::{
    AngleLimit, HierarchyHintGenerator, HierarchyHintGeneratorWithBranchFactor, HintGenerator,
    LastUsedVertexHintGenerator, RefinementParameters, RefinementResult,
};

pub use crate::vendored_spade::delaunay_core::interpolation::{Barycentric, NaturalNeighbor};
pub use delaunay_core::LineSideInfo;
pub use intersection_iterator::{Intersection, LineIntersectionIterator};
pub use triangulation::{FloatTriangulation, PositionInTriangulation, Triangulation};

pub(crate) use delaunay_core::TriangulationExt;

pub(crate) use delaunay_core::RemovalResult;

/// Handle types used for traversal and modification of triangulations.
///
/// A handle can either be a "reference handle" or a "fixed handle". Reference handles are
/// used for immutable access to a triangulation. Fixed handles allow mutation but have a
/// more limited API.
///
/// # Reference handles
///
/// Reference handles come in one of four variants:
/// * [FaceHandle](handles::FaceHandle)s refer to a single face (triangle) of the triangulation.
///   They are used get the triangle's adjacent vertices and edges. They also may refer to
///    the single outer face.
/// * [VertexHandle](handles::VertexHandle)s refer to a single vertex of the triangulation. They
///   allow to retrieve the vertex position and its outgoing edges.
/// * [DirectedEdgeHandle](handles::DirectedEdgeHandle)s refer to a single directed edge. They
///   allow to retrieve the edges origin and destination vertex, its adjacent face as well as
///   its previous and next edge.
/// * [UndirectedEdgeHandle](handles::UndirectedEdgeHandle)s refer to an edge without specifying its
///   direction.
///
/// All handles also allow to set and retrieve arbitrary additional data associated with that
/// element. Refer to the type parameters of [Triangulation] for more details.
///
/// # Fixed handles
/// Reference handles all hold an immutable reference to the underlying triangulation.
/// This reference is used to implement a feature rich and intuitive API. However,
/// due to Rust's ownership semantics, modification of a triangulation
/// can not be done using these handles as that would require a mutable reference. This is
/// required in some cases, e.g. whenever traversal of the triangulation is mixed with insertion
/// operations or when attempting to remove vertices.
/// As a solution, Spade relies on _fixed_ handles for mutation: These are created from normal
/// handles by calling `some_handle.fix()` and do not keep a reference to the triangulation.
///
/// Internally, fixed handles are implemented as indices into a `Vec`. **Removing elements from
/// the triangulation can possibly invalidate any fixed handle**. Attempting to resolve an invalid
/// handle may either refer to a different element or panic at run time. It is the callers
/// responsibility to make sure that fixed handles are not used anymore after a removal operation
/// has taken place.
///
/// Fixed handles also come in four variants, depending on which element they refer to:
///  * [FixedVertexHandle](handles::FixedVertexHandle)
///  * [FixedFaceHandle](handles::FixedFaceHandle)
///  * [FixedDirectedEdgeHandle](handles::FixedDirectedEdgeHandle)
///  * [FixedUndirectedEdgeHandle](handles::FixedUndirectedEdgeHandle)
///
/// # Retrieving handles by iteration
///
/// The [Triangulation] trait defines iterators for all handle types:
///
/// |                | Vertices | Directed Edges  | Undirected Edges | Faces |
/// |----------------|----------|-----------------|------------------|-------|
/// | **Reference**  | [vertices()](Triangulation::vertices()) | [directed_edges()](Triangulation::directed_edges) | [undirected_edges()](Triangulation::undirected_edges()) | [inner_faces()](Triangulation::inner_faces())<br>[all_faces()](Triangulation::all_faces()) |
/// | **Fixed**      | [fixed_vertices()](Triangulation::fixed_vertices()) | [fixed_directed_edges()](Triangulation::fixed_directed_edges()) | [fixed_undirected_edges()](Triangulation::fixed_undirected_edges()) | [fixed_inner_faces()](Triangulation::fixed_inner_faces())<br> [fixed_faces()](Triangulation::fixed_inner_faces()) |
///
/// # Creating handles directly
///
/// In some cases it may be desirable to create vertex handles artificially by providing the index
/// manually. This can be done by calling [handles::FixedVertexHandle::from_index](from_index(usize)).
/// Make sure that the provided index is smaller than the number of vertices in the triangulation.
///
/// # Converting between reference and fixed handles
///
/// Converting a reference handle into its fixed counterpart is performed via the
/// `fix()` method (defined on any handle type).
///
/// Converting a fixed handle type back into a reference handle requires calling either
/// [Triangulation::vertex], [Triangulation::face],
/// [Triangulation::directed_edge] or [Triangulation::undirected_edge].
///
/// # Example: Using handles
/// This example implements a nearest neighbor algorithm on a delaunay triangulation. Starting
/// from an arbitrary start vertex, it greedily walks to the closes vertex until arriving at a
/// local minimum. Due to the special properties of Delaunay triangulations, this is also the
/// global nearest neighbor.
///
/// _Note: Spade already implements this method, see [DelaunayTriangulation::nearest_neighbor]_
/// ```
/// use spade::handles::VertexHandle;
/// use spade::{DelaunayTriangulation, Point2, Triangulation};
///
/// fn nearest_neighbor(
///     triangulation: &DelaunayTriangulation<Point2<f64>>,
///     target_point: Point2<f64>,
/// ) -> VertexHandle<Point2<f64>> {
///     let mut current = triangulation.vertices().next().unwrap();
///     let mut best_distance = current.position().distance_2(target_point);
///     loop {
///         let mut closer = None;
///         for neighbor in current.out_edges().map(|edge| edge.to()) {
///             let neighbor_distance = neighbor.position().distance_2(target_point);
///             if neighbor_distance < best_distance {
///                 best_distance = neighbor_distance;
///                 closer = Some(neighbor);
///                 break;
///             }
///         }
///
///         if let Some(closer) = closer {
///             current = closer;
///         } else {
///             return current;
///         }
///     }
/// }
///
/// # fn try_main() -> Result<(), spade::InsertionError> {
///
/// let vertices = vec![
///     Point2::new(0.0, 1.0),
///     Point2::new(1.0, 1.0),
///     Point2::new(0.0, 0.0),
///     Point2::new(1.0, 0.0),
/// ];
/// let triangulation: DelaunayTriangulation<Point2<f64>> = Triangulation::bulk_load(vertices)?;
///
/// // Check that everything works!
/// for vertex in triangulation.vertices() {
///     let nearest_neighbor = nearest_neighbor(&triangulation, vertex.position());
///     assert_eq!(nearest_neighbor, vertex);
/// }
/// # Ok(()) }
/// # fn main() { try_main().unwrap() }
/// ```
pub mod handles {
    pub use crate::vendored_spade::delaunay_core::{
        DirectedEdgeHandle, DirectedVoronoiEdge, FaceHandle, FixedDirectedEdgeHandle,
        FixedFaceHandle, FixedUndirectedEdgeHandle, FixedVertexHandle, InnerTag, PossiblyOuterTag,
        UndirectedEdgeHandle, UndirectedVoronoiEdge, VertexHandle, VoronoiFace, VoronoiVertex,
        OUTER_FACE,
    };
}

/// Iterators over various elements of Delaunay triangulations.
pub mod iterators {
    pub use crate::vendored_spade::delaunay_core::iterators::{
        DirectedEdgeIterator, DirectedVoronoiEdgeIterator, FaceIterator, FixedDirectedEdgeIterator,
        FixedFaceIterator, FixedInnerFaceIterator, FixedUndirectedEdgeIterator,
        FixedVertexIterator, InnerFaceIterator, UndirectedEdgeIterator,
        UndirectedVoronoiEdgeIterator, VertexIterator, VoronoiFaceIterator,
    };
    pub use crate::vendored_spade::flood_fill_iterator::{
        CircleMetric, EdgesInShapeIterator, RectangleMetric, VerticesInShapeIterator,
    };
}

/// Internals that must be published due to technical reasons. This is not the place you are
/// looking for. A change to these items is not considered to be a breaking change.
pub mod internals {
    pub use crate::vendored_spade::delaunay_core::{DynamicHandleImpl, FixedHandleImpl};
}
