use core::convert::TryInto;

use super::super::Dcel;
use super::public_handles::{InnerOuterMarker, PossiblyOuterTag};
use super::FixedVertexHandle;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub trait DelaunayElementType: Sized + Default {
    fn num_elements<V, DE, UE, F>(dcel: &Dcel<V, DE, UE, F>) -> usize;
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub struct FixedHandleImpl<Type, InnerOuter: InnerOuterMarker> {
    index: u32,
    ty: Type,
    inner_outer: InnerOuter,
}

impl<Type, InnerOuter: InnerOuterMarker> core::fmt::Debug for FixedHandleImpl<Type, InnerOuter> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FixedHandle")
            .field("index", &self.index)
            .finish()
    }
}

pub const fn new_fixed_face_handle(index: usize) -> FixedHandleImpl<FaceTag, PossiblyOuterTag> {
    FixedHandleImpl {
        index: index as u32,
        ty: FaceTag,
        inner_outer: PossiblyOuterTag,
    }
}

impl FixedVertexHandle {
    pub fn from_index(index: usize) -> Self {
        // Preferably, `new` would simply be public. However, that would allow to create a handle
        // to the outer face marked with `InnerTag` which is bad. Let's only allow vertices for now.
        Self::new(index)
    }
}

impl<Type: Default, InnerOuter: InnerOuterMarker> FixedHandleImpl<Type, InnerOuter> {
    pub(crate) fn new(index: usize) -> Self {
        Self::new_internal(
            index
                .try_into()
                .expect("Index too big - at most 2^32 elements supported"),
        )
    }

    pub(crate) fn max() -> Self {
        Self::new_internal(u32::MAX)
    }

    pub fn index(&self) -> usize {
        self.index as usize
    }

    pub(crate) fn adjust_inner_outer<TargetInnerOuter: InnerOuterMarker>(
        &self,
    ) -> FixedHandleImpl<Type, TargetInnerOuter> {
        FixedHandleImpl::<_, _>::new_internal(self.index)
    }

    fn new_internal(index: u32) -> Self {
        Self {
            index,
            ty: Type::default(),
            inner_outer: InnerOuter::default(),
        }
    }
}

pub struct DynamicHandleImpl<'a, V, DE, UE, F, Type, InnerOuter: InnerOuterMarker> {
    pub(super) dcel: &'a Dcel<V, DE, UE, F>,
    pub(super) handle: FixedHandleImpl<Type, InnerOuter>,
}

impl<'a, V, DE, UE, F, Type: Default, InnerOuter: InnerOuterMarker>
    DynamicHandleImpl<'a, V, DE, UE, F, Type, InnerOuter>
{
    #[inline]
    pub(crate) fn new(
        dcel: &'a Dcel<V, DE, UE, F>,
        handle: FixedHandleImpl<Type, InnerOuter>,
    ) -> Self {
        Self { dcel, handle }
    }

    pub(in super::super) fn adjust_inner_outer<TargetInnerOuter: InnerOuterMarker>(
        &self,
    ) -> DynamicHandleImpl<'a, V, DE, UE, F, Type, TargetInnerOuter> {
        DynamicHandleImpl::new(self.dcel, self.handle.adjust_inner_outer())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub struct VertexTag;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub struct DirectedEdgeTag;
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct UndirectedEdgeTag;
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct FaceTag;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct DirectedVoronoiEdgeTag;
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct UndirectedVoronoiEdgeTag;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct VoronoiVertexTag;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct VoronoiFaceTag;

impl DelaunayElementType for VertexTag {
    fn num_elements<V, DE, UE, F>(dcel: &Dcel<V, DE, UE, F>) -> usize {
        dcel.num_vertices()
    }
}

impl DelaunayElementType for DirectedEdgeTag {
    fn num_elements<V, DE, UE, F>(dcel: &Dcel<V, DE, UE, F>) -> usize {
        dcel.num_directed_edges()
    }
}

impl DelaunayElementType for UndirectedEdgeTag {
    fn num_elements<V, DE, UE, F>(dcel: &Dcel<V, DE, UE, F>) -> usize {
        dcel.num_undirected_edges()
    }
}

impl DelaunayElementType for FaceTag {
    fn num_elements<V, DE, UE, F>(dcel: &Dcel<V, DE, UE, F>) -> usize {
        dcel.num_faces()
    }
}

impl DelaunayElementType for VoronoiFaceTag {
    fn num_elements<V, DE, UE, F>(dcel: &Dcel<V, DE, UE, F>) -> usize {
        dcel.num_vertices()
    }
}

impl DelaunayElementType for DirectedVoronoiEdgeTag {
    fn num_elements<V, DE, UE, F>(dcel: &Dcel<V, DE, UE, F>) -> usize {
        dcel.num_directed_edges()
    }
}

impl DelaunayElementType for UndirectedVoronoiEdgeTag {
    fn num_elements<V, DE, UE, F>(dcel: &Dcel<V, DE, UE, F>) -> usize {
        dcel.num_undirected_edges()
    }
}
