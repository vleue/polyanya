use super::{CircularIterator, NextBackFn};
use crate::vendored_spade::{
    delaunay_core::Dcel,
    handles::{DirectedEdgeHandle, FixedDirectedEdgeHandle},
};

pub struct HullIterator<'a, V, DE, UE, F> {
    inner_iterator: CircularIterator<'a, V, DE, UE, F, HullNextBackFn>,
}

struct HullNextBackFn;

impl NextBackFn for HullNextBackFn {
    fn next<V, DE, UE, F>(
        edge_handle: DirectedEdgeHandle<V, DE, UE, F>,
    ) -> DirectedEdgeHandle<V, DE, UE, F> {
        edge_handle.next()
    }

    fn next_back<V, DE, UE, F>(
        edge_handle: DirectedEdgeHandle<V, DE, UE, F>,
    ) -> DirectedEdgeHandle<V, DE, UE, F> {
        edge_handle.prev()
    }
}

impl<'a, V, DE, UE, F> HullIterator<'a, V, DE, UE, F> {
    pub(crate) fn new(dcel: &'a Dcel<V, DE, UE, F>) -> Self {
        let outer_face = dcel.outer_face();

        let inner_iterator = if let Some(first_edge) = outer_face.adjacent_edge() {
            CircularIterator::new(first_edge)
        } else {
            CircularIterator::new_empty(DirectedEdgeHandle::new(
                dcel,
                FixedDirectedEdgeHandle::new(0),
            ))
        };

        Self { inner_iterator }
    }
}

// TODO: Implement ExactSizeIterator
impl<'a, V, DE, UE, F> Iterator for HullIterator<'a, V, DE, UE, F> {
    type Item = DirectedEdgeHandle<'a, V, DE, UE, F>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iterator.next()
    }
}

impl<'a, V, DE, UE, F> DoubleEndedIterator for HullIterator<'a, V, DE, UE, F> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner_iterator.next_back()
    }
}
