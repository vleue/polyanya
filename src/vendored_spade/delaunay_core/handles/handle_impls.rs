use super::super::dcel::EdgeEntry;
use super::super::math;
use super::handle_defs::*;
use super::iterators::CircularIterator;
use super::iterators::NextBackFn;
use super::public_handles::*;
use crate::vendored_spade::CdtEdge;
use crate::vendored_spade::{HasPosition, LineSideInfo, Point2};
use core::cmp::Ordering;
use core::fmt::Debug;
use core::hash::{Hash, Hasher};
use num_traits::{Float, One};

// Debug implementations
impl<'a, V, DE, UE, F> Debug for VertexHandle<'a, V, DE, UE, F> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "VertexHandle({:?})", self.handle.index())
    }
}

impl<'a, V, DE, UE, F> Debug for DirectedEdgeHandle<'a, V, DE, UE, F> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "DirectedEdgeHandle - id: {:?} ({:?} -> {:?})",
            self.handle.index(),
            self.from().fix(),
            self.to().fix()
        )
    }
}

impl<'a, V, DE, UE, F> Debug for UndirectedEdgeHandle<'a, V, DE, UE, F> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let [v0, v1] = self.vertices();
        write!(
            f,
            "UndirectedEdgeHandle - id: {:?} ({:?} <-> {:?})",
            self.handle.index(),
            v0.fix(),
            v1.fix(),
        )
    }
}

impl<'a, V, DE, UE, F> Debug for FaceHandle<'a, PossiblyOuterTag, V, DE, UE, F> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Some(inner) = self.as_inner() {
            inner.fmt(f)
        } else {
            write!(f, "OuterFace")
        }
    }
}

impl<'a, V, DE, UE, F> Debug for FaceHandle<'a, InnerTag, V, DE, UE, F> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let [v0, v1, v2] = self.vertices();
        write!(
            f,
            "FaceHandle - id: {:?} ({:?}, {:?}, {:?})",
            self.handle.index(),
            v0.fix().index(),
            v1.fix().index(),
            v2.fix().index(),
        )
    }
}

impl FixedDirectedEdgeHandle {
    #[inline]
    pub(in super::super) fn new_normalized(index: usize) -> Self {
        Self::new(index << 1)
    }

    #[inline]
    pub(in super::super) fn is_normalized(self) -> bool {
        // Use the last bit to store if this edge is normalized
        self.index() & 0x1 == 0x0
    }

    #[inline]
    pub(in super::super) fn normalize_index(self) -> usize {
        self.index() & 0x1
    }

    #[inline]
    pub fn rev(self) -> Self {
        // Flip the last bit
        Self::new(self.index() ^ 0x1)
    }

    #[inline]
    pub fn as_undirected(self) -> FixedUndirectedEdgeHandle {
        FixedHandleImpl::new(self.index() >> 1)
    }
}

impl<'a, V, DE, UE, F, Type: Copy, InnerOuter: InnerOuterMarker> Clone
    for DynamicHandleImpl<'a, V, DE, UE, F, Type, InnerOuter>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, V, DE, UE, F, Type: Copy, InnerOuter: InnerOuterMarker> Copy
    for DynamicHandleImpl<'a, V, DE, UE, F, Type, InnerOuter>
{
}

impl<'a, V, DE, UE, F, Type: PartialEq, InnerOuter: InnerOuterMarker> PartialEq
    for DynamicHandleImpl<'a, V, DE, UE, F, Type, InnerOuter>
{
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl<'a, V, DE, UE, F, Type: Eq, InnerOuter: InnerOuterMarker> Eq
    for DynamicHandleImpl<'a, V, DE, UE, F, Type, InnerOuter>
{
}

impl<'a, V, DE, UE, F, Type: Hash, InnerOuter: InnerOuterMarker> Hash
    for DynamicHandleImpl<'a, V, DE, UE, F, Type, InnerOuter>
{
    fn hash<HA: Hasher>(&self, state: &mut HA) {
        self.handle.hash(state);
    }
}

impl<'a, V, DE, UE, F, Type: Ord, InnerOuter: InnerOuterMarker> Ord
    for DynamicHandleImpl<'a, V, DE, UE, F, Type, InnerOuter>
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.handle.cmp(&other.handle)
    }
}

impl<'a, V, DE, UE, F, Type: PartialOrd, InnerOuter: InnerOuterMarker> PartialOrd
    for DynamicHandleImpl<'a, V, DE, UE, F, Type, InnerOuter>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.handle.partial_cmp(&other.handle)
    }
}

impl<'a, V, DE, UE, F, Type: Copy + Default, InnerOuter: InnerOuterMarker>
    DynamicHandleImpl<'a, V, DE, UE, F, Type, InnerOuter>
{
    pub fn fix(&self) -> FixedHandleImpl<Type, InnerOuter> {
        self.handle
    }

    pub fn index(&self) -> usize {
        self.handle.index()
    }
}

impl FixedFaceHandle<PossiblyOuterTag> {
    #[inline]
    pub fn is_outer(&self) -> bool {
        *self == super::super::dcel_operations::OUTER_FACE_HANDLE
    }

    pub fn as_inner(&self) -> Option<FixedFaceHandle<InnerTag>> {
        if self.is_outer() {
            None
        } else {
            Some(self.adjust_inner_outer())
        }
    }
}

impl<'a, V, DE, UE, F> AsRef<DE> for DirectedEdgeHandle<'a, V, DE, UE, F> {
    fn as_ref(&self) -> &DE {
        self.data()
    }
}

impl<'a, V, DE, UE, F> DirectedEdgeHandle<'a, V, DE, UE, F> {
    pub fn vertices(&self) -> [VertexHandle<'a, V, DE, UE, F>; 2] {
        [self.from(), self.to()]
    }

    pub fn from(&self) -> VertexHandle<'a, V, DE, UE, F> {
        let entry = self.dcel.half_edge(self.handle);
        DynamicHandleImpl::new(self.dcel, entry.origin.adjust_inner_outer())
    }

    pub fn to(&self) -> VertexHandle<'a, V, DE, UE, F> {
        self.rev().from()
    }

    #[inline]
    pub fn rev(&self) -> Self {
        DirectedEdgeHandle::new(self.dcel, self.handle.rev())
    }

    pub fn opposite_vertex(&self) -> Option<VertexHandle<'a, V, DE, UE, F>> {
        if self.is_outer_edge() {
            None
        } else {
            Some(self.prev().from())
        }
    }

    pub fn next(&self) -> DirectedEdgeHandle<'a, V, DE, UE, F> {
        let entry = self.dcel.half_edge(self.handle);
        DirectedEdgeHandle::new(self.dcel, entry.next)
    }

    pub fn prev(&self) -> DirectedEdgeHandle<'a, V, DE, UE, F> {
        let entry = self.dcel.half_edge(self.handle);
        DirectedEdgeHandle::new(self.dcel, entry.prev)
    }

    pub fn face(&self) -> FaceHandle<'a, PossiblyOuterTag, V, DE, UE, F> {
        let entry = self.dcel.half_edge(self.handle);
        self.dcel.face(entry.face)
    }

    pub fn cw(&self) -> DirectedEdgeHandle<'a, V, DE, UE, F> {
        self.rev().next()
    }

    pub fn ccw(&self) -> DirectedEdgeHandle<'a, V, DE, UE, F> {
        self.prev().rev()
    }

    pub fn data(&self) -> &'a DE {
        self.entry().get_directed_data(self.handle)
    }

    fn entry(&self) -> &'a EdgeEntry<DE, UE> {
        self.dcel.edge_entry(self.handle.as_undirected())
    }

    #[inline]
    pub fn as_undirected(self) -> UndirectedEdgeHandle<'a, V, DE, UE, F> {
        DynamicHandleImpl::new(self.dcel, self.handle.as_undirected())
    }

    pub fn is_outer_edge(&self) -> bool {
        self.face().is_outer()
    }

    pub fn is_part_of_convex_hull(&self) -> bool {
        self.is_outer_edge() || self.rev().is_outer_edge()
    }

    pub fn as_voronoi_edge(&self) -> DirectedVoronoiEdge<'a, V, DE, UE, F> {
        DirectedVoronoiEdge::new(self.dcel, FixedHandleImpl::new(self.handle.index()))
    }
}

impl<'a, V, DE, UE, F> DirectedEdgeHandle<'a, V, DE, UE, F>
where
    V: HasPosition,
{
    pub fn positions(&self) -> [Point2<<V as HasPosition>::Scalar>; 2] {
        [self.from().position(), self.to().position()]
    }

    #[inline]
    pub fn opposite_position(&self) -> Option<Point2<V::Scalar>> {
        self.opposite_vertex().map(|v| v.position())
    }

    pub fn length_2(&self) -> V::Scalar {
        self.as_undirected().length_2()
    }

    #[inline]
    pub fn side_query(&self, query_point: Point2<V::Scalar>) -> LineSideInfo {
        let (p1, p2) = (self.from().position(), self.to().position());
        math::side_query(p1, p2, query_point)
    }

    pub fn project_point(
        &self,
        query_point: Point2<V::Scalar>,
    ) -> math::PointProjection<V::Scalar> {
        let (p1, p2) = (self.from().position(), self.to().position());
        math::project_point(p1, p2, query_point)
    }

    pub(crate) fn intersects_edge_non_collinear(
        &self,
        other_from: Point2<V::Scalar>,
        other_to: Point2<V::Scalar>,
    ) -> bool {
        let other_from_query = self.side_query(other_from);
        let other_to_query = self.side_query(other_to);
        let self_from_query = math::side_query(other_from, other_to, self.from().position());
        let self_to_query = math::side_query(other_from, other_to, self.to().position());

        assert!(
            ![
                &other_from_query,
                &other_to_query,
                &self_from_query,
                &self_to_query
            ]
            .iter()
            .all(|q| q.is_on_line()),
            "intersects_edge_non_collinear: Given edge is collinear."
        );

        other_from_query != other_to_query && self_from_query != self_to_query
    }
}

impl<'a, V, DE, UE, F> DirectedEdgeHandle<'a, V, DE, CdtEdge<UE>, F> {
    pub fn is_constraint_edge(self) -> bool {
        self.as_undirected().is_constraint_edge()
    }
}

impl FixedUndirectedEdgeHandle {
    #[inline]
    pub fn as_directed(&self) -> FixedDirectedEdgeHandle {
        FixedDirectedEdgeHandle::new_normalized(self.index())
    }

    pub fn directed_edges(&self) -> [FixedDirectedEdgeHandle; 2] {
        [self.as_directed(), self.as_directed().rev()]
    }

    #[inline]
    pub(in super::super) fn normalized(&self) -> FixedDirectedEdgeHandle {
        self.as_directed()
    }

    #[inline]
    pub(in super::super) fn not_normalized(&self) -> FixedDirectedEdgeHandle {
        self.as_directed().rev()
    }
}

impl<'a, V, DE, UE, F> UndirectedVoronoiEdge<'a, V, DE, UE, F> {
    pub fn vertices(&self) -> [VoronoiVertex<'a, V, DE, UE, F>; 2] {
        [self.as_directed().from(), self.as_directed().to()]
    }

    pub fn as_directed(&self) -> DirectedVoronoiEdge<'a, V, DE, UE, F> {
        self.as_delaunay_edge().as_directed().as_voronoi_edge()
    }

    pub fn as_delaunay_edge(&self) -> UndirectedEdgeHandle<'a, V, DE, UE, F> {
        UndirectedEdgeHandle::new(
            self.dcel,
            FixedUndirectedEdgeHandle::new(self.handle.index()),
        )
    }
}

impl<'a, V, DE, UE, F> AsRef<UE> for UndirectedEdgeHandle<'a, V, DE, UE, F> {
    fn as_ref(&self) -> &UE {
        self.data()
    }
}

impl<'a, V, DE, UE, F> UndirectedEdgeHandle<'a, V, DE, UE, F> {
    pub fn vertices(&self) -> [VertexHandle<'a, V, DE, UE, F>; 2] {
        [self.as_directed().from(), self.as_directed().to()]
    }

    #[inline]
    pub fn as_directed(&self) -> DirectedEdgeHandle<'a, V, DE, UE, F> {
        DirectedEdgeHandle::new(self.dcel, self.handle.as_directed())
    }

    pub fn as_voronoi_edge(&self) -> UndirectedVoronoiEdge<'a, V, DE, UE, F> {
        UndirectedVoronoiEdge::new(self.dcel, FixedHandleImpl::new(self.handle.index()))
    }

    pub fn data(&self) -> &UE {
        self.dcel.undirected_edge_data(self.handle)
    }

    pub fn is_part_of_convex_hull(&self) -> bool {
        self.as_directed().is_part_of_convex_hull()
    }
}

impl<'a, V, DE, UE, F> UndirectedEdgeHandle<'a, V, DE, UE, F>
where
    V: HasPosition,
{
    pub fn positions(&self) -> [Point2<V::Scalar>; 2] {
        let [v0, v1] = self.vertices();
        [v0.position(), v1.position()]
    }

    pub fn length_2(&self) -> V::Scalar {
        let [p0, p1] = self.positions();
        p0.sub(p1).length2()
    }
}

impl<'a, V, DE, UE, F> UndirectedEdgeHandle<'a, V, DE, UE, F>
where
    V: HasPosition,
    V::Scalar: Float,
{
    pub fn distance_2(&self, query_point: Point2<V::Scalar>) -> V::Scalar {
        let [p1, p2] = self.positions();
        math::distance_2(p1, p2, query_point)
    }

    pub fn nearest_point(&self, query_point: Point2<V::Scalar>) -> Point2<V::Scalar> {
        let [v0, v1] = self.positions();
        math::nearest_point(v0, v1, query_point)
    }

    pub fn center(&self) -> Point2<V::Scalar> {
        let [v0, v1] = self.positions();
        v0.add(v1).mul(0.5.into())
    }
}

impl<'a, V, DE, UE, F> UndirectedEdgeHandle<'a, V, DE, CdtEdge<UE>, F> {
    pub fn is_constraint_edge(self) -> bool {
        self.data().is_constraint_edge()
    }
}

impl<'a, V, DE, UE, InnerOuter, F> AsRef<F> for FaceHandle<'a, InnerOuter, V, DE, UE, F>
where
    InnerOuter: InnerOuterMarker,
{
    fn as_ref(&self) -> &F {
        self.data()
    }
}

impl<'a, V, DE, UE, F> FaceHandle<'a, InnerTag, V, DE, UE, F> {
    pub fn adjacent_edges(&self) -> [DirectedEdgeHandle<'a, V, DE, UE, F>; 3] {
        let e1 = self.adjacent_edge();
        let e0 = e1.prev();
        let e2 = e1.next();
        [e0, e1, e2]
    }

    pub fn adjacent_edge(&self) -> DirectedEdgeHandle<'a, V, DE, UE, F> {
        // unwrap is okay since every inner face has an adjacent edge
        let handle = self.dcel.face_adjacent_edge(self.handle).unwrap();
        DynamicHandleImpl::new(self.dcel, handle)
    }

    pub fn vertices(&self) -> [VertexHandle<'a, V, DE, UE, F>; 3] {
        let [e0, e1, e2] = self.adjacent_edges();
        [e0.from(), e1.from(), e2.from()]
    }
}

impl<'a, V, DE, UE, F> FaceHandle<'a, InnerTag, V, DE, UE, F>
where
    V: HasPosition,
{
    pub fn positions(&self) -> [Point2<V::Scalar>; 3] {
        let [v0, v1, v2] = self.vertices();
        [v0.position(), v1.position(), v2.position()]
    }

    pub fn area(&self) -> V::Scalar {
        math::triangle_area(self.positions())
    }
}

impl<'a, V, DE, UE, F> FaceHandle<'a, InnerTag, V, DE, UE, F>
where
    V: HasPosition,
    V::Scalar: Float,
{
    pub fn distance_2(&self, query_point: Point2<V::Scalar>) -> V::Scalar {
        math::distance_2_triangle(self.positions(), query_point)
    }

    pub fn center(&self) -> Point2<V::Scalar> {
        let [v0, v1, v2] = self.positions();
        let one = V::Scalar::one();
        let three = one + one + one;
        v0.add(v1.add(v2)).mul(one / three)
    }

    pub fn circumcircle(&self) -> (Point2<V::Scalar>, V::Scalar) {
        math::circumcenter(self.positions())
    }

    pub fn circumcenter(&self) -> Point2<V::Scalar> {
        self.circumcircle().0
    }

    pub fn barycentric_interpolation(&self, coordinate: Point2<V::Scalar>) -> [V::Scalar; 3] {
        let [v1, v2, v3] = self.vertices();
        let [v1, v2, v3] = [v1.position(), v2.position(), v3.position()];
        let (x, y) = (coordinate.x, coordinate.y);
        let (x1, x2, x3) = (v1.x, v2.x, v3.x);
        let (y1, y2, y3) = (v1.y, v2.y, v3.y);
        let det = (y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3);
        let lambda1 = ((y2 - y3) * (x - x3) + (x3 - x2) * (y - y3)) / det;
        let lambda2 = ((y3 - y1) * (x - x3) + (x1 - x3) * (y - y3)) / det;
        let lambda3 = V::Scalar::one() - lambda1 - lambda2;
        [lambda1, lambda2, lambda3]
    }

    pub(crate) fn shortest_edge(&self) -> (DirectedEdgeHandle<'a, V, DE, UE, F>, V::Scalar) {
        let [e0, e1, e2] = self.adjacent_edges();
        let [l0, l1, l2] = [e0.length_2(), e1.length_2(), e2.length_2()];
        if l0 < l1 && l0 < l2 {
            (e0, l0)
        } else if l1 < l2 {
            (e1, l1)
        } else {
            (e2, l2)
        }
    }
}

impl<'a, V, DE, UE, F> AsRef<V> for VertexHandle<'a, V, DE, UE, F> {
    fn as_ref(&self) -> &V {
        self.data()
    }
}

impl<'a, V, DE, UE, F> VertexHandle<'a, V, DE, UE, F>
where
    V: HasPosition,
{
    pub fn position(&self) -> Point2<V::Scalar> {
        self.dcel.vertex_data(self.handle).position()
    }
}

pub struct CCWEdgesNextBackFn;

impl NextBackFn for CCWEdgesNextBackFn {
    fn next<V, DE, UE, F>(
        edge_handle: DirectedEdgeHandle<V, DE, UE, F>,
    ) -> DirectedEdgeHandle<V, DE, UE, F> {
        edge_handle.ccw()
    }

    fn next_back<V, DE, UE, F>(
        edge_handle: DirectedEdgeHandle<V, DE, UE, F>,
    ) -> DirectedEdgeHandle<V, DE, UE, F> {
        edge_handle.cw()
    }
}

impl<'a, V, DE, UE, F> VertexHandle<'a, V, DE, UE, F> {
    pub fn out_edges(&self) -> CircularIterator<'a, V, DE, UE, F, CCWEdgesNextBackFn> {
        if let Some(edge) = self.out_edge() {
            CircularIterator::new(edge)
        } else {
            CircularIterator::new_empty(DirectedEdgeHandle::new(
                self.dcel,
                FixedDirectedEdgeHandle::new(0),
            ))
        }
    }

    pub fn out_edge(&self) -> Option<DirectedEdgeHandle<'a, V, DE, UE, F>> {
        self.dcel
            .vertex_out_edge(self.handle)
            .map(|handle| DirectedEdgeHandle::new(self.dcel, handle))
    }

    pub fn data(&self) -> &V {
        self.dcel.vertex_data(self.handle)
    }

    pub fn as_voronoi_face(&self) -> VoronoiFace<'a, V, DE, UE, F> {
        VoronoiFace::new(self.dcel, FixedHandleImpl::new(self.handle.index()))
    }
}

impl<'a, V, DE, UE, F> DirectedEdgeHandle<'a, V, DE, UE, F>
where
    V: HasPosition,
    V::Scalar: Float,
{
    pub fn distance_2(&self, query_point: Point2<V::Scalar>) -> V::Scalar {
        self.as_undirected().distance_2(query_point)
    }

    pub fn nearest_point(&self, query_point: Point2<V::Scalar>) -> Point2<V::Scalar> {
        self.as_undirected().nearest_point(query_point)
    }

    pub fn center(&self) -> Point2<V::Scalar> {
        self.as_undirected().center()
    }
}

impl<'a, V, DE, UE, F, InnerOuter: InnerOuterMarker> FaceHandle<'a, InnerOuter, V, DE, UE, F> {
    pub fn data(&self) -> &F {
        self.dcel.face_data(self.handle)
    }
}

impl<'a, V, DE, UE, F> FaceHandle<'a, PossiblyOuterTag, V, DE, UE, F> {
    #[inline]
    pub fn is_outer(&self) -> bool {
        self.handle.is_outer()
    }

    pub fn as_inner(&self) -> Option<FaceHandle<'a, InnerTag, V, DE, UE, F>> {
        if self.is_outer() {
            None
        } else {
            Some(FaceHandle::new(self.dcel, self.handle.adjust_inner_outer()))
        }
    }

    pub fn adjacent_edge(&self) -> Option<DirectedEdgeHandle<'a, V, DE, UE, F>> {
        self.dcel
            .face_adjacent_edge(self.handle)
            .map(|handle| DirectedEdgeHandle::new(self.dcel, handle))
    }
}
