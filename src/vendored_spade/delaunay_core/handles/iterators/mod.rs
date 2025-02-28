mod circular_iterator;
mod fixed_iterators;
mod hull_iterator;

use super::{handle_defs::*, InnerTag, PossiblyOuterTag};
use fixed_iterators::{DynamicHandleIterator, FixedHandleIterator};

pub use circular_iterator::{CircularIterator, NextBackFn};
pub use hull_iterator::HullIterator;

macro_rules! type_handle_doc {
    ($doc_name:expr, $item_type:expr) => {
        concat!(
            "An iterator over ",
            $doc_name,
            ".\n",
            "\n",
            "This iterator is created by [Triangulation::",
            $item_type,
            "()]",
            "(crate::Triangulation::",
            $item_type,
            ").\n\n",
            "*Note: This iterator specializes `nth` and `nth_back` - those operations run in O(1)*"
        )
    };
}

#[doc = type_handle_doc!("vertices", "vertices")]
pub type VertexIterator<'a, V, DE, UE, F> =
    DynamicHandleIterator<'a, V, DE, UE, F, VertexTag, InnerTag>;

#[doc = type_handle_doc!("fixed vertices", "fixed_vertices")]
pub type FixedVertexIterator = FixedHandleIterator<VertexTag, InnerTag>;

#[doc = type_handle_doc!("directed edges", "directed_edges")]
pub type DirectedEdgeIterator<'a, V, DE, UE, F> =
    DynamicHandleIterator<'a, V, DE, UE, F, DirectedEdgeTag, InnerTag>;

#[doc = type_handle_doc!("fixed directed edges", "fixed_directed_edges")]
pub type FixedDirectedEdgeIterator = FixedHandleIterator<DirectedEdgeTag, InnerTag>;

#[doc = type_handle_doc!("undirected edges", "undirected_edges")]
pub type UndirectedEdgeIterator<'a, V, DE, UE, F> =
    DynamicHandleIterator<'a, V, DE, UE, F, UndirectedEdgeTag, InnerTag>;

#[doc = type_handle_doc!("fixed undirected edges", "fixed_undirected_edges")]
pub type FixedUndirectedEdgeIterator = FixedHandleIterator<UndirectedEdgeTag, InnerTag>;

#[doc = type_handle_doc!("voronoi faces", "voronoi_faces")]
pub type VoronoiFaceIterator<'a, V, DE, UE, F> =
    DynamicHandleIterator<'a, V, DE, UE, F, VoronoiFaceTag, PossiblyOuterTag>;

#[doc = type_handle_doc!("directed voronoi edges", "directed_voronoi_edges")]
pub type DirectedVoronoiEdgeIterator<'a, V, DE, UE, F> =
    DynamicHandleIterator<'a, V, DE, UE, F, DirectedVoronoiEdgeTag, InnerTag>;

#[doc = type_handle_doc!("undirected voronoi edges", "undirected_voronoi_edges")]
pub type UndirectedVoronoiEdgeIterator<'a, V, DE, UE, F> =
    DynamicHandleIterator<'a, V, DE, UE, F, UndirectedVoronoiEdgeTag, InnerTag>;

#[doc = type_handle_doc!("faces", "all_faces")]
pub type FaceIterator<'a, V, DE, UE, F> =
    DynamicHandleIterator<'a, V, DE, UE, F, FaceTag, PossiblyOuterTag>;

#[doc = type_handle_doc!("inner faces", "inner_faces")]
pub type InnerFaceIterator<'a, V, DE, UE, F> =
    DynamicHandleIterator<'a, V, DE, UE, F, FaceTag, InnerTag>;

#[doc = type_handle_doc!("fixed faces", "fixed_all_faces")]
pub type FixedFaceIterator = FixedHandleIterator<FaceTag, PossiblyOuterTag>;

#[doc = type_handle_doc!("fixed inner faces", "fixed_inner_faces")]
pub type FixedInnerFaceIterator = FixedHandleIterator<FaceTag, InnerTag>;
