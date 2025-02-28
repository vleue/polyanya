use crate::vendored_spade::{
    delaunay_core::dcel_operations::{self},
    HasPosition, Point2,
};

pub use super::handle_defs::*;

use num_traits::Float;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Returns a reference to the single outer face.
///
/// *See also [Triangulation::outer_face()](crate::vendored_spade::Triangulation::outer_face()).*
pub const OUTER_FACE: FixedFaceHandle<PossiblyOuterTag> = dcel_operations::OUTER_FACE_HANDLE;

/// Marker trait for [InnerTag] and [PossiblyOuterTag].
///
/// There should be no need to implement this.
pub trait InnerOuterMarker:
    Clone + Copy + PartialEq + Eq + PartialOrd + Ord + core::fmt::Debug + Default + core::hash::Hash
{
    fn debug_string() -> &'static str;
}

/// Marker type that signifies that a face is an inner faces.
///
/// Used as type parameter for [FixedFaceHandle] and [FaceHandle] to indicate that a face
/// handle cannot possibly reference the outer face.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub struct InnerTag;

/// Marker type that signifies that a face can possibly be the outer faces.
///
/// Used as type parameter for [FixedFaceHandle] and [FaceHandle] to indicate that a face
/// handle can possibly reference the outer face.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub struct PossiblyOuterTag;

impl InnerOuterMarker for InnerTag {
    fn debug_string() -> &'static str {
        "Inner"
    }
}

impl InnerOuterMarker for PossiblyOuterTag {
    fn debug_string() -> &'static str {
        "PossiblyOuter"
    }
}

/// Fixed handle to a vertex.
///
/// *See also the [handles](crate::vendored_spade::handles) module.*
pub type FixedVertexHandle = FixedHandleImpl<VertexTag, InnerTag>;

/// Fixed handle to a directed edge.
///
/// *See also the [handles](crate::vendored_spade::handles) module.*
pub type FixedDirectedEdgeHandle = FixedHandleImpl<DirectedEdgeTag, InnerTag>;

/// Fixed handle to an undirected edge.
///
/// *See also the [handles](crate::vendored_spade::handles) module.*
pub type FixedUndirectedEdgeHandle = FixedHandleImpl<UndirectedEdgeTag, InnerTag>;

/// "Fixed handle to a face.
///
/// The type parameter is either [InnerTag] or [PossiblyOuterTag], depending on the face type.
///
/// "*See also the [handles module](crate::vendored_spade::handles)*
pub type FixedFaceHandle<InnerOuter> = FixedHandleImpl<FaceTag, InnerOuter>;

/// Handle to a directed edge of a triangulation.
///
/// Use this handle to examine the edge's surroundings, e.g. its origin and destination
/// vertices or the adjacent face.
///
/// # Retrieving neighboring edges:
///
/// Use [Self::next], [Self::prev], [Self::rev] to access any adjacent edge:
///
///
/// # Retrieving adjacent faces and vertices
///
/// Use [Self::face], [Self::from] and [Self::to] to access the adjacent face and vertices:
///
///
/// *See also the [handles module](crate::vendored_spade::handles).*
pub type DirectedEdgeHandle<'a, V, DE, UE, F> =
    DynamicHandleImpl<'a, V, DE, UE, F, DirectedEdgeTag, InnerTag>;

/// Handle to an undirected edge of a triangulation.
///
/// Use this handle to examine the edge's surroundings, e.g. its origin and destination
/// vertices or the adjacent face.
///
/// # Method overview
/// An undirected edge handle allows to explore the surroundings of an edge. This diagram
/// shows which methods are available to extract information about the edge's
/// neighboring elements.
///
///
/// *See also the [handles module](crate::vendored_spade::handles)*
pub type UndirectedEdgeHandle<'a, V, DE = (), UE = (), F = ()> =
    DynamicHandleImpl<'a, V, DE, UE, F, UndirectedEdgeTag, InnerTag>;

/// Handle to a vertex of a triangulation.
///
/// Use this handle to retrieve the vertex [position](Self::position) or its
/// [outgoing edges](Self::out_edges).
///
///
/// *See also the [handles module](crate::vendored_spade::handles).*
pub type VertexHandle<'a, V, DE = (), UE = (), F = ()> =
    DynamicHandleImpl<'a, V, DE, UE, F, VertexTag, InnerTag>;

/// Handle to a face of a triangulation.
///
/// Depending on the type parameter, the handle **can refer to the outer face**:
///
/// * `FaceHandle<'a, PossiblyOuterTag, ...>`: The face may refer to the single outer face.
/// * `FaceHandle<'a, InnerTag, ...>`: The face refers to an inner triangle of the triangulation.
///
///
/// `FaceHandle<'a, InnerTag, ...>` implements some additional methods that require it to be an inner
/// face - e.g. [Self::vertices] will return an array containing exactly 3
/// vertices.
///
/// Use [Self::as_inner] to convert from a *possibly outer* face to an *inner*
/// face.
///
/// # Type parameters
/// The type parameters refer to the triangulations vertex, directed edge, undirected edge and
/// face type. See [Triangulation](crate::vendored_spade::Triangulation) for more information.
///
/// *See also the [handles module](crate::vendored_spade::handles) for more general information about handles.*
pub type FaceHandle<'a, InnerOuter, V, DE, UE, F> =
    DynamicHandleImpl<'a, V, DE, UE, F, FaceTag, InnerOuter>;

/// A handle to a directed edge of the Voronoi diagram.
///
/// Several methods are defined to explore adjacent edges, faces and vertices:
///
pub type DirectedVoronoiEdge<'a, V, DE, UE, F> =
    DynamicHandleImpl<'a, V, DE, UE, F, DirectedVoronoiEdgeTag, InnerTag>;

/// A handle to an undirected edge of the Voronoi diagram.
pub type UndirectedVoronoiEdge<'a, V, DE, UE, F> =
    DynamicHandleImpl<'a, V, DE, UE, F, UndirectedVoronoiEdgeTag, InnerTag>;

/// A handle to a face of the voronoi diagram.
pub type VoronoiFace<'a, V, DE, UE, F> =
    DynamicHandleImpl<'a, V, DE, UE, F, VoronoiFaceTag, PossiblyOuterTag>;

/// A handle to a vertex of the voronoi diagram.
///
/// Refer to [DelaunayTriangulation](crate::vendored_spade::DelaunayTriangulation) for an example on how
/// to extract the voronoi diagram from a triangulation.
pub enum VoronoiVertex<'a, V, DE, UE, F> {
    /// Refers to an inner vertex of the voronoi diagram.
    ///
    /// An inner vertex refers to an *inner face* of the original Delaunay triangulation.
    /// Its position is the circumcenter of that face.
    ///
    ///
    /// *Displays several inner voronoi vertices in blue and an arrow towards the inner
    /// face to which they belong. Note that voronoi vertices are not necessarily
    /// located inside the convex hull.*
    Inner(
        /// The inner face handle to which this voronoi vertex refers.
        FaceHandle<'a, InnerTag, V, DE, UE, F>,
    ),

    /// Refers to an outer vertex of the voronoi diagram.
    ///
    /// These vertices don't have a well-defined position as they don't have a dual inner
    /// face in the Delaunay triangulation.
    /// Instead, they are characterized by a dual outer edge (an edge that is part of the
    /// convex hull) of the underlying triangulation.
    ///
    /// Rather than a position, these vertices can be better described by a
    /// *half open line segment*: The voronoi vertex is placed infinitely far away in the
    /// direction the line segment is pointing to.
    /// The line segment's direction is the dual edge's normal (i.e., the dual edge
    /// rotated by 90 degrees).
    ///
    ///
    /// *Displays 3 out of 6 outer Delaunay vertices. For each vertex, its dual outer edge
    /// is shown in blue. The line segment that points toward this vertex is shown in orange.*
    Outer(
        /// The outer directed edge handle dual to this voronoi vertex.
        DirectedVoronoiEdge<'a, V, DE, UE, F>,
    ),
}

impl<'a, V, DE, UE, F> VoronoiVertex<'a, V, DE, UE, F>
where
    V: HasPosition,
    V::Scalar: Float,
{
    /// The position of this voronoi vertex.
    ///
    /// Returns `None` if this vertex is an outer voronoi vertex.
    /// Otherwise, the returned position is the
    /// [circumcenter](crate::vendored_spade::handles::FaceHandle::circumcenter())
    /// of the dual Delaunay face.
    pub fn position(&self) -> Option<Point2<V::Scalar>> {
        match self {
            VoronoiVertex::Inner(face) => Some(face.circumcenter()),
            VoronoiVertex::Outer(_) => None,
        }
    }

    /// Returns the dual delaunay face of this voronoi vertex.
    ///
    /// Returns `None` if this is an outer voronoi vertex.
    pub fn as_delaunay_face(&self) -> Option<FaceHandle<'a, InnerTag, V, DE, UE, F>> {
        match self {
            VoronoiVertex::Inner(face) => Some(*face),
            VoronoiVertex::Outer(_) => None,
        }
    }

    /// Returns all directed voronoi edges going out of this vertex.
    ///
    /// The edges are returned in counterclockwise order. Returns `None` if this is an outer
    /// voronoi vertex.
    pub fn out_edges(&self) -> Option<[DirectedVoronoiEdge<'a, V, DE, UE, F>; 3]> {
        match self {
            VoronoiVertex::Inner(face) => {
                let [e1, e2, e3] = face.adjacent_edges();
                Some([
                    e1.as_voronoi_edge(),
                    e2.as_voronoi_edge(),
                    e3.as_voronoi_edge(),
                ])
            }
            VoronoiVertex::Outer(_) => None,
        }
    }

    /// Returns a voronoi edge going out of this vertex.
    pub fn out_edge(&self) -> DirectedVoronoiEdge<'a, V, DE, UE, F> {
        match self {
            VoronoiVertex::Inner(face) => face.adjacent_edge().as_voronoi_edge(),
            VoronoiVertex::Outer(edge) => *edge,
        }
    }
}

impl<'a, V, DE, UE, F> VoronoiFace<'a, V, DE, UE, F> {
    /// Converts this face into its dual vertex of the Delaunay Triangulation.
    pub fn as_delaunay_vertex(&self) -> VertexHandle<'a, V, DE, UE, F> {
        VertexHandle::new(self.dcel, FixedVertexHandle::new(self.handle.index()))
    }

    /// Returns an iterator that returns all edges adjacent to this face.
    ///
    /// The edges are returned in clockwise order.
    pub fn adjacent_edges(
        &self,
    ) -> impl DoubleEndedIterator<Item = DirectedVoronoiEdge<'a, V, DE, UE, F>> {
        self.as_delaunay_vertex()
            .out_edges()
            .map(|edge| edge.as_voronoi_edge())
    }
}

impl<'a, V, DE, UE, F> DirectedVoronoiEdge<'a, V, DE, UE, F> {
    /// Returns the voronoi edge's destination.
    pub fn to(&self) -> VoronoiVertex<'a, V, DE, UE, F> {
        self.rev().from()
    }

    /// Returns the voronoi vertex from which this edge originates.
    pub fn from(&self) -> VoronoiVertex<'a, V, DE, UE, F> {
        if let Some(face) = self.as_delaunay_edge().face().as_inner() {
            VoronoiVertex::Inner(face)
        } else {
            VoronoiVertex::Outer(*self)
        }
    }

    /// Returns the Voronoi face to the left of this Voronoi edge
    pub fn face(&self) -> VoronoiFace<'a, V, DE, UE, F> {
        self.as_delaunay_edge().from().as_voronoi_face()
    }

    /// Converts this directed edge handle into an undirected edge handle.
    ///
    /// *See also the [handles](crate::vendored_spade::handles) module for more information.*
    pub fn as_undirected(&self) -> UndirectedVoronoiEdge<'a, V, DE, UE, F> {
        self.as_delaunay_edge().as_undirected().as_voronoi_edge()
    }

    /// Returns the directed dual edge of the underlying Delaunay triangulation.
    ///
    /// The dual edge is always orthogonal to this edge.
    ///
    ///
    /// *Shows various inner voronoi edges (blue) and their dual Delaunay edges (orange)*
    pub fn as_delaunay_edge(&self) -> DirectedEdgeHandle<'a, V, DE, UE, F> {
        DirectedEdgeHandle::new(self.dcel, FixedDirectedEdgeHandle::new(self.handle.index()))
    }

    /// Returns this edge with its direction reversed.
    pub fn rev(&self) -> Self {
        self.as_delaunay_edge().rev().as_voronoi_edge()
    }

    /// Returns the edge that is connected to this edge in counterclockwise order.
    ///
    /// See also [prev](Self::prev)
    pub fn next(&self) -> DirectedVoronoiEdge<'a, V, DE, UE, F> {
        self.as_delaunay_edge().ccw().as_voronoi_edge()
    }

    /// Returns the edge that is connected to this edge in clockwise order.
    ///
    /// See also [next](Self::next)
    pub fn prev(&self) -> DirectedVoronoiEdge<'a, V, DE, UE, F> {
        self.as_delaunay_edge().cw().as_voronoi_edge()
    }
}

impl<'a, V, DE, UE, F> DirectedVoronoiEdge<'a, V, DE, UE, F>
where
    V: HasPosition,
{
    /// Returns a vector that is parallel to the voronoi edge.
    ///
    /// This vector is obtained by rotating the dual Delaunay edge by 90° degree
    /// The returned vector is not necessarily normalized.
    pub fn direction_vector(&self) -> Point2<V::Scalar> {
        let from = self.as_delaunay_edge().from().position();
        let to = self.as_delaunay_edge().to().position();
        let diff = Point2::sub(&to, from);

        Point2::new(-diff.y, diff.x)
    }
}
