use core::marker::PhantomData;

use super::super::handle_defs::{DelaunayElementType, DynamicHandleImpl, FixedHandleImpl};
use super::super::InnerOuterMarker;

use crate::vendored_spade::delaunay_core::Dcel;

pub struct FixedHandleIterator<Type, InnerOuter> {
    range: core::ops::Range<usize>,
    ty: PhantomData<Type>,
    inner_outer: PhantomData<InnerOuter>,
}

impl<Type: Default, InnerOuter: InnerOuterMarker> FixedHandleIterator<Type, InnerOuter> {
    pub(crate) fn new(to: usize) -> Self {
        Self {
            range: 0..to,
            ty: Default::default(),
            inner_outer: Default::default(),
        }
    }
}

impl<Type: Default, InnerOuter: InnerOuterMarker> Iterator
    for FixedHandleIterator<Type, InnerOuter>
{
    type Item = FixedHandleImpl<Type, InnerOuter>;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(FixedHandleImpl::new)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.range.nth(n).map(FixedHandleImpl::new)
    }
}

impl<Type: Default, InnerOuter: InnerOuterMarker> ExactSizeIterator
    for FixedHandleIterator<Type, InnerOuter>
{
}

impl<Type: Default, InnerOuter: InnerOuterMarker> DoubleEndedIterator
    for FixedHandleIterator<Type, InnerOuter>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().map(FixedHandleImpl::new)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.range.nth_back(n).map(FixedHandleImpl::new)
    }
}

pub struct DynamicHandleIterator<'a, V, DE, UE, F, Type, InnerOuter> {
    fixed_iterator: FixedHandleIterator<Type, InnerOuter>,
    dcel: &'a Dcel<V, DE, UE, F>,
    inner_outer: PhantomData<InnerOuter>,
}

impl<'a, V, DE, UE, F, Type, InnerOuter> DynamicHandleIterator<'a, V, DE, UE, F, Type, InnerOuter>
where
    Type: DelaunayElementType + Default,
    InnerOuter: InnerOuterMarker,
{
    pub(crate) fn new(dcel: &'a Dcel<V, DE, UE, F>) -> Self {
        DynamicHandleIterator {
            fixed_iterator: FixedHandleIterator::new(Type::num_elements(dcel)),
            dcel,
            inner_outer: PhantomData,
        }
    }
}

impl<'a, V, DE, UE, F, Type, InnerOuter> Iterator
    for DynamicHandleIterator<'a, V, DE, UE, F, Type, InnerOuter>
where
    Type: DelaunayElementType,
    InnerOuter: InnerOuterMarker,
{
    type Item = DynamicHandleImpl<'a, V, DE, UE, F, Type, InnerOuter>;

    fn next(&mut self) -> Option<Self::Item> {
        self.fixed_iterator
            .next()
            .map(|handle| DynamicHandleImpl::new(self.dcel, handle.adjust_inner_outer()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.fixed_iterator.size_hint()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.fixed_iterator
            .nth(n)
            .map(|handle| DynamicHandleImpl::new(self.dcel, handle.adjust_inner_outer()))
    }
}

impl<'a, V, DE, UE, F, Type, InnerOuter> DoubleEndedIterator
    for DynamicHandleIterator<'a, V, DE, UE, F, Type, InnerOuter>
where
    Type: DelaunayElementType,
    InnerOuter: InnerOuterMarker,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.fixed_iterator
            .next_back()
            .map(|handle| DynamicHandleImpl::new(self.dcel, handle.adjust_inner_outer()))
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.fixed_iterator
            .nth_back(n)
            .map(|handle| DynamicHandleImpl::new(self.dcel, handle.adjust_inner_outer()))
    }
}

impl<'a, V, DE, UE, F, Type, InnerOuter> ExactSizeIterator
    for DynamicHandleIterator<'a, V, DE, UE, F, Type, InnerOuter>
where
    Type: DelaunayElementType,
    InnerOuter: InnerOuterMarker,
{
}
