use core::marker::PhantomData;

use super::super::DirectedEdgeHandle;

pub trait NextBackFn {
    fn next<V, DE, UE, F>(
        edge_handle: DirectedEdgeHandle<V, DE, UE, F>,
    ) -> DirectedEdgeHandle<V, DE, UE, F>;

    fn next_back<V, DE, UE, F>(
        edge_handle: DirectedEdgeHandle<V, DE, UE, F>,
    ) -> DirectedEdgeHandle<V, DE, UE, F>;
}

pub struct CircularIterator<'a, V, DE, UE, F, NB> {
    pub(super) current_handle: DirectedEdgeHandle<'a, V, DE, UE, F>,
    final_handle: DirectedEdgeHandle<'a, V, DE, UE, F>,
    iteration_finished: bool,
    next_back_fn: PhantomData<NB>,
}

impl<'a, V, DE, UE, F, NB: NextBackFn> CircularIterator<'a, V, DE, UE, F, NB> {
    pub fn new(start_edge: DirectedEdgeHandle<'a, V, DE, UE, F>) -> Self {
        CircularIterator {
            current_handle: start_edge,
            final_handle: start_edge,
            iteration_finished: false,
            next_back_fn: Default::default(),
        }
    }

    pub fn new_empty(some_edge: DirectedEdgeHandle<'a, V, DE, UE, F>) -> Self {
        CircularIterator {
            current_handle: some_edge,
            final_handle: some_edge,
            iteration_finished: true,
            next_back_fn: Default::default(),
        }
    }
}

impl<'a, V, DE, UE, F, NB: NextBackFn> Iterator for CircularIterator<'a, V, DE, UE, F, NB> {
    type Item = DirectedEdgeHandle<'a, V, DE, UE, F>;

    fn next(&mut self) -> Option<DirectedEdgeHandle<'a, V, DE, UE, F>> {
        if self.iteration_finished {
            return None;
        }
        let result = self.current_handle;
        self.current_handle = NB::next(self.current_handle);
        if self.current_handle == self.final_handle {
            self.iteration_finished = true;
        }
        Some(result)
    }
}

impl<'a, V, DE, UE, F, NB: NextBackFn> DoubleEndedIterator
    for CircularIterator<'a, V, DE, UE, F, NB>
{
    fn next_back(&mut self) -> Option<DirectedEdgeHandle<'a, V, DE, UE, F>> {
        if self.iteration_finished {
            return None;
        }
        self.final_handle = NB::next_back(self.final_handle);
        if self.current_handle == self.final_handle {
            self.iteration_finished = true;
        }
        Some(self.final_handle)
    }
}
