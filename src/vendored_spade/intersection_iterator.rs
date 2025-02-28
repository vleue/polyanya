use crate::vendored_spade::delaunay_core::math;
use crate::vendored_spade::handles::{DirectedEdgeHandle, FixedVertexHandle, VertexHandle};
use crate::vendored_spade::{HasPosition, Point2, Triangulation, TriangulationExt};

pub struct LineIntersectionIterator<'a, V, DE, UE, F>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
{
    cur_intersection: Option<Intersection<'a, V, DE, UE, F>>,
    line_from: Point2<V::Scalar>,
    line_to: Point2<V::Scalar>,
}

#[allow(clippy::enum_variant_names)]
pub enum Intersection<'a, V, DE, UE, F>
where
    V: HasPosition,
{
    EdgeIntersection(DirectedEdgeHandle<'a, V, DE, UE, F>),

    VertexIntersection(VertexHandle<'a, V, DE, UE, F>),

    EdgeOverlap(DirectedEdgeHandle<'a, V, DE, UE, F>),
}

impl<'a, V, DE, UE, F> core::fmt::Debug for Intersection<'a, V, DE, UE, F>
where
    V: HasPosition,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use self::Intersection::*;
        match self {
            EdgeIntersection(handle) => write!(f, "EdgeIntersection({:?})", handle),
            VertexIntersection(handle) => write!(f, "VertexIntersection({:?})", handle),
            EdgeOverlap(handle) => write!(f, "EdgeOverlap({:?})", handle),
        }
    }
}

impl<'a, V, DE, UE, F> PartialEq for Intersection<'a, V, DE, UE, F>
where
    V: HasPosition,
{
    fn eq(&self, other: &Self) -> bool {
        use self::Intersection::*;
        match (self, other) {
            (&EdgeIntersection(h0), &EdgeIntersection(h1)) => h0 == h1,
            (&VertexIntersection(h0), &VertexIntersection(h1)) => h0 == h1,
            (&EdgeOverlap(h0), &EdgeOverlap(h1)) => h0 == h1,
            _ => false,
        }
    }
}

impl<'a, V, DE, UE, F> Copy for Intersection<'a, V, DE, UE, F> where V: HasPosition {}

impl<'a, V, DE, UE, F> Clone for Intersection<'a, V, DE, UE, F>
where
    V: HasPosition,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, V, DE, UE, F> Intersection<'a, V, DE, UE, F>
where
    V: HasPosition,
{
    pub fn as_edge_intersection(&self) -> Option<DirectedEdgeHandle<'a, V, DE, UE, F>> {
        match self {
            Intersection::EdgeIntersection(ref edge) => Some(*edge),
            _ => None,
        }
    }
}

impl<'a, V, DE, UE, F> LineIntersectionIterator<'a, V, DE, UE, F>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
{
    pub fn new<T>(
        delaunay: &'a T,
        line_from: Point2<V::Scalar>,
        line_to: Point2<V::Scalar>,
    ) -> LineIntersectionIterator<'a, V, DE, UE, F>
    where
        T: Triangulation<Vertex = V, DirectedEdge = DE, UndirectedEdge = UE, Face = F>,
    {
        let first_intersection = Self::get_first_intersection(delaunay, line_from, line_to);
        LineIntersectionIterator {
            cur_intersection: first_intersection,
            line_from,
            line_to,
        }
    }

    pub fn new_from_handles<T>(
        delaunay: &T,
        from: FixedVertexHandle,
        to: FixedVertexHandle,
    ) -> LineIntersectionIterator<V, DE, UE, F>
    where
        T: Triangulation<Vertex = V, DirectedEdge = DE, UndirectedEdge = UE, Face = F>,
    {
        let from = delaunay.vertex(from);
        let line_from = from.position();
        let to = delaunay.vertex(to);
        let line_to = to.position();

        LineIntersectionIterator {
            cur_intersection: Some(Intersection::VertexIntersection(from)),
            line_from,
            line_to,
        }
    }

    fn get_first_intersection<T>(
        delaunay: &'a T,
        line_from: Point2<V::Scalar>,
        line_to: Point2<V::Scalar>,
    ) -> Option<Intersection<'a, V, DE, UE, F>>
    where
        T: Triangulation<Vertex = V, DirectedEdge = DE, UndirectedEdge = UE, Face = F>,
    {
        use crate::vendored_spade::PositionInTriangulation::*;

        match delaunay.locate_with_hint_option_core(line_from, None) {
            OutsideOfConvexHull(edge_handle) => {
                let mut edge = delaunay.directed_edge(edge_handle);
                let line_from_query = edge.side_query(line_from);

                loop {
                    if line_from_query.is_on_line() {
                        let dist_from = edge.from().position().distance_2(line_from);
                        let dist_to = edge.to().position().distance_2(line_from);
                        let vertex = if dist_to < dist_from {
                            edge.to()
                        } else {
                            edge.from()
                        };
                        return Some(Intersection::VertexIntersection(vertex));
                    }
                    let line_to_query = edge.side_query(line_to);

                    if line_to_query.is_on_left_side() {
                        return None;
                    }

                    let edge_from = edge.from().position();
                    let edge_to = edge.to().position();
                    let edge_from_query = math::side_query(line_from, line_to, edge_from);
                    let edge_to_query = math::side_query(line_from, line_to, edge_to);

                    match (
                        edge_from_query.is_on_left_side(),
                        edge_to_query.is_on_left_side_or_on_line(),
                    ) {
                        (true, true) => edge = edge.prev(),
                        (false, false) => edge = edge.next(),
                        (false, true) => {
                            if edge_to_query.is_on_line() {
                                return Some(Intersection::VertexIntersection(edge.to()));
                            }
                            if edge_from_query.is_on_line() {
                                return Some(Intersection::VertexIntersection(edge.from()));
                            }
                            return Some(Intersection::EdgeIntersection(edge.rev()));
                        }
                        (true, false) => panic!("Unexpected edge topology. This is a bug."),
                    }
                }
            }
            OnFace(face_handle) => get_first_edge_from_edge_ring(
                delaunay.face(face_handle).adjacent_edges().iter().copied(),
                line_from,
                line_to,
            ),
            OnVertex(vertex_handle) => Some(Intersection::VertexIntersection(
                delaunay.vertex(vertex_handle),
            )),
            OnEdge(edge) => {
                let edge = delaunay.directed_edge(edge);
                let edge_from = edge.from().position();
                let edge_to = edge.to().position();
                let from_query = math::side_query(line_from, line_to, edge_from);
                let to_query = math::side_query(line_from, line_to, edge_to);
                if from_query.is_on_line() && to_query.is_on_line() {
                    let dist_to = edge_to.sub(line_to).length2();
                    let dist_from = edge_from.sub(line_to).length2();
                    if dist_to < dist_from {
                        Some(Intersection::EdgeOverlap(edge))
                    } else {
                        Some(Intersection::EdgeOverlap(edge.rev()))
                    }
                } else {
                    let edge_query = edge.side_query(line_to);
                    if edge_query.is_on_left_side() {
                        Some(Intersection::EdgeIntersection(edge))
                    } else {
                        Some(Intersection::EdgeIntersection(edge.rev()))
                    }
                }
            }
            NoTriangulation => {
                if let Some(next_vertex) = delaunay.vertices().next() {
                    let single_vertex = next_vertex.position();
                    let projection = math::project_point(line_from, line_to, single_vertex);
                    if projection.is_on_edge() {
                        let query = math::side_query(line_from, line_to, single_vertex);
                        if query.is_on_line() {
                            return Some(Intersection::VertexIntersection(next_vertex));
                        }
                    }
                }
                None
            }
        }
    }

    fn get_next(&mut self) -> Option<Intersection<'a, V, DE, UE, F>> {
        use self::Intersection::*;
        match self.cur_intersection {
            Some(EdgeIntersection(cur_edge)) => {
                match trace_direction_out_of_edge(cur_edge, self.line_from, self.line_to) {
                    EdgeOutDirection::ConvexHull => None,
                    EdgeOutDirection::VertexIntersection(vertex) => {
                        Some(VertexIntersection(vertex))
                    }
                    EdgeOutDirection::EdgeIntersection(edge) => Some(EdgeIntersection(edge)),
                    EdgeOutDirection::NoIntersection => None,
                }
            }
            Some(VertexIntersection(vertex)) => {
                if vertex.position() == self.line_to {
                    // Target point was reached - the iteration can finish
                    None
                } else {
                    match trace_direction_out_of_vertex(vertex, self.line_to) {
                        VertexOutDirection::ConvexHull => None,
                        VertexOutDirection::EdgeOverlap(edge) => Some(EdgeOverlap(edge)),
                        VertexOutDirection::EdgeIntersection(edge) => {
                            if edge.side_query(self.line_to).is_on_right_side() {
                                // The target point was skipped over - the iteration can finish
                                None
                            } else {
                                Some(EdgeIntersection(edge))
                            }
                        }
                    }
                }
            }
            Some(EdgeOverlap(edge)) => {
                if self.line_from == self.line_to {
                    None
                } else if math::project_point(self.line_from, self.line_to, edge.to().position())
                    .is_on_edge()
                {
                    Some(VertexIntersection(edge.to()))
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

impl<'a, V, DE, UE, F> Iterator for LineIntersectionIterator<'a, V, DE, UE, F>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
{
    type Item = Intersection<'a, V, DE, UE, F>;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur_intersection;
        self.cur_intersection = self.get_next();
        cur
    }
}

fn get_first_edge_from_edge_ring<'a, I, V, DE, UE, F>(
    ring: I,
    line_from: Point2<V::Scalar>,
    line_to: Point2<V::Scalar>,
) -> Option<Intersection<'a, V, DE, UE, F>>
where
    I: IntoIterator<Item = DirectedEdgeHandle<'a, V, DE, UE, F>>,
    V: HasPosition,
{
    use self::Intersection::*;
    for edge in ring {
        let cur_from = edge.from().position();
        let cur_to = edge.to().position();

        debug_assert!(math::side_query(cur_from, cur_to, line_from).is_on_left_side_or_on_line());
        if math::intersects_edge_non_collinear(line_from, line_to, cur_from, cur_to) {
            if math::side_query(line_from, line_to, cur_from).is_on_line() {
                return Some(VertexIntersection(edge.from()));
            } else if math::side_query(line_from, line_to, cur_to).is_on_line() {
                return Some(VertexIntersection(edge.to()));
            }
            return Some(EdgeIntersection(edge.rev()));
        }
    }
    None
}

pub(super) enum VertexOutDirection<'a, V, DE, UE, F> {
    ConvexHull,
    EdgeOverlap(DirectedEdgeHandle<'a, V, DE, UE, F>),
    EdgeIntersection(DirectedEdgeHandle<'a, V, DE, UE, F>),
}

pub(super) fn trace_direction_out_of_vertex<V, DE, UE, F>(
    vertex: VertexHandle<V, DE, UE, F>,
    line_to: Point2<V::Scalar>,
) -> VertexOutDirection<V, DE, UE, F>
where
    V: HasPosition,
{
    let mut current_edge = match vertex.out_edge() {
        Some(edge) => edge,
        None => return VertexOutDirection::ConvexHull,
    };

    let mut current_query = current_edge.side_query(line_to);

    let iterate_ccw = current_query.is_on_left_side();

    loop {
        if current_query.is_on_line() && !current_edge.project_point(line_to).is_before_edge() {
            return VertexOutDirection::EdgeOverlap(current_edge);
        }

        let next = if iterate_ccw {
            current_edge.ccw()
        } else {
            current_edge.cw()
        };

        let next_query = next.side_query(line_to);
        if next_query.is_on_line() && !next.project_point(line_to).is_before_edge() {
            return VertexOutDirection::EdgeOverlap(next);
        }

        let face_between_current_and_next = if iterate_ccw {
            current_edge.face()
        } else {
            next.face()
        };
        if face_between_current_and_next.is_outer() {
            return VertexOutDirection::ConvexHull;
        }

        if iterate_ccw == next_query.is_on_right_side() {
            let segment_edge = if iterate_ccw {
                current_edge.next()
            } else {
                current_edge.rev().prev()
            };
            return VertexOutDirection::EdgeIntersection(segment_edge.rev());
        }

        current_query = next_query;
        current_edge = next;
    }
}

pub(super) enum EdgeOutDirection<'a, V, DE, UE, F> {
    ConvexHull,
    VertexIntersection(VertexHandle<'a, V, DE, UE, F>),
    EdgeIntersection(DirectedEdgeHandle<'a, V, DE, UE, F>),
    NoIntersection,
}

pub(super) fn trace_direction_out_of_edge<V, DE, UE, F>(
    edge: DirectedEdgeHandle<V, DE, UE, F>,
    line_from: Point2<V::Scalar>,
    line_to: Point2<V::Scalar>,
) -> EdgeOutDirection<V, DE, UE, F>
where
    V: HasPosition,
{
    debug_assert!(
        edge.side_query(line_to).is_on_left_side_or_on_line(),
        "The target must be on the left side of the current edge"
    );

    let e_prev = if edge.is_outer_edge() {
        // Iteration reached an edge of the convex hull, we're done.
        return EdgeOutDirection::ConvexHull;
    } else {
        edge.prev()
    };

    let o_next = edge.next();

    // Find out which edges of the left face intersect the line
    let e_prev_inter = e_prev.intersects_edge_non_collinear(line_from, line_to);
    let o_next_inter = o_next.intersects_edge_non_collinear(line_from, line_to);

    match (e_prev_inter, o_next_inter) {
        (true, false) => EdgeOutDirection::EdgeIntersection(e_prev.rev()),
        (false, true) => EdgeOutDirection::EdgeIntersection(o_next.rev()),
        (true, true) => {
            // Both edges intersect - this means the line is cutting through a common point
            EdgeOutDirection::VertexIntersection(e_prev.from())
        }
        (false, false) => EdgeOutDirection::NoIntersection,
    }
}
