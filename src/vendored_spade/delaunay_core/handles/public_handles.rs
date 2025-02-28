use crate::vendored_spade::{
    delaunay_core::dcel_operations::{self},
    HasPosition, Point2,
};

pub use super::handle_defs::*;

use num_traits::Float;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub const OUTER_FACE: FixedFaceHandle<PossiblyOuterTag> = dcel_operations::OUTER_FACE_HANDLE;

pub trait InnerOuterMarker:
    Clone + Copy + PartialEq + Eq + PartialOrd + Ord + core::fmt::Debug + Default + core::hash::Hash
{
    fn debug_string() -> &'static str;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub struct InnerTag;

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

pub type FixedVertexHandle = FixedHandleImpl<VertexTag, InnerTag>;

pub type FixedDirectedEdgeHandle = FixedHandleImpl<DirectedEdgeTag, InnerTag>;

pub type FixedUndirectedEdgeHandle = FixedHandleImpl<UndirectedEdgeTag, InnerTag>;

pub type FixedFaceHandle<InnerOuter> = FixedHandleImpl<FaceTag, InnerOuter>;

pub type DirectedEdgeHandle<'a, V, DE, UE, F> =
    DynamicHandleImpl<'a, V, DE, UE, F, DirectedEdgeTag, InnerTag>;

pub type UndirectedEdgeHandle<'a, V, DE = (), UE = (), F = ()> =
    DynamicHandleImpl<'a, V, DE, UE, F, UndirectedEdgeTag, InnerTag>;

pub type VertexHandle<'a, V, DE = (), UE = (), F = ()> =
    DynamicHandleImpl<'a, V, DE, UE, F, VertexTag, InnerTag>;

pub type FaceHandle<'a, InnerOuter, V, DE, UE, F> =
    DynamicHandleImpl<'a, V, DE, UE, F, FaceTag, InnerOuter>;

pub type DirectedVoronoiEdge<'a, V, DE, UE, F> =
    DynamicHandleImpl<'a, V, DE, UE, F, DirectedVoronoiEdgeTag, InnerTag>;

pub type UndirectedVoronoiEdge<'a, V, DE, UE, F> =
    DynamicHandleImpl<'a, V, DE, UE, F, UndirectedVoronoiEdgeTag, InnerTag>;

pub type VoronoiFace<'a, V, DE, UE, F> =
    DynamicHandleImpl<'a, V, DE, UE, F, VoronoiFaceTag, PossiblyOuterTag>;

pub enum VoronoiVertex<'a, V, DE, UE, F> {
    Inner(FaceHandle<'a, InnerTag, V, DE, UE, F>),

    Outer(DirectedVoronoiEdge<'a, V, DE, UE, F>),
}

impl<'a, V, DE, UE, F> VoronoiVertex<'a, V, DE, UE, F>
where
    V: HasPosition,
    V::Scalar: Float,
{
    pub fn position(&self) -> Option<Point2<V::Scalar>> {
        match self {
            VoronoiVertex::Inner(face) => Some(face.circumcenter()),
            VoronoiVertex::Outer(_) => None,
        }
    }

    pub fn as_delaunay_face(&self) -> Option<FaceHandle<'a, InnerTag, V, DE, UE, F>> {
        match self {
            VoronoiVertex::Inner(face) => Some(*face),
            VoronoiVertex::Outer(_) => None,
        }
    }

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

    pub fn out_edge(&self) -> DirectedVoronoiEdge<'a, V, DE, UE, F> {
        match self {
            VoronoiVertex::Inner(face) => face.adjacent_edge().as_voronoi_edge(),
            VoronoiVertex::Outer(edge) => *edge,
        }
    }
}

impl<'a, V, DE, UE, F> VoronoiFace<'a, V, DE, UE, F> {
    pub fn as_delaunay_vertex(&self) -> VertexHandle<'a, V, DE, UE, F> {
        VertexHandle::new(self.dcel, FixedVertexHandle::new(self.handle.index()))
    }

    pub fn adjacent_edges(
        &self,
    ) -> impl DoubleEndedIterator<Item = DirectedVoronoiEdge<'a, V, DE, UE, F>> {
        self.as_delaunay_vertex()
            .out_edges()
            .map(|edge| edge.as_voronoi_edge())
    }
}

impl<'a, V, DE, UE, F> DirectedVoronoiEdge<'a, V, DE, UE, F> {
    pub fn to(&self) -> VoronoiVertex<'a, V, DE, UE, F> {
        self.rev().from()
    }

    pub fn from(&self) -> VoronoiVertex<'a, V, DE, UE, F> {
        if let Some(face) = self.as_delaunay_edge().face().as_inner() {
            VoronoiVertex::Inner(face)
        } else {
            VoronoiVertex::Outer(*self)
        }
    }

    pub fn face(&self) -> VoronoiFace<'a, V, DE, UE, F> {
        self.as_delaunay_edge().from().as_voronoi_face()
    }

    pub fn as_undirected(&self) -> UndirectedVoronoiEdge<'a, V, DE, UE, F> {
        self.as_delaunay_edge().as_undirected().as_voronoi_edge()
    }

    pub fn as_delaunay_edge(&self) -> DirectedEdgeHandle<'a, V, DE, UE, F> {
        DirectedEdgeHandle::new(self.dcel, FixedDirectedEdgeHandle::new(self.handle.index()))
    }

    pub fn rev(&self) -> Self {
        self.as_delaunay_edge().rev().as_voronoi_edge()
    }

    pub fn next(&self) -> DirectedVoronoiEdge<'a, V, DE, UE, F> {
        self.as_delaunay_edge().ccw().as_voronoi_edge()
    }

    pub fn prev(&self) -> DirectedVoronoiEdge<'a, V, DE, UE, F> {
        self.as_delaunay_edge().cw().as_voronoi_edge()
    }
}

impl<'a, V, DE, UE, F> DirectedVoronoiEdge<'a, V, DE, UE, F>
where
    V: HasPosition,
{
    pub fn direction_vector(&self) -> Point2<V::Scalar> {
        let from = self.as_delaunay_edge().from().position();
        let to = self.as_delaunay_edge().to().position();
        let diff = Point2::sub(&to, from);

        Point2::new(-diff.y, diff.x)
    }
}
