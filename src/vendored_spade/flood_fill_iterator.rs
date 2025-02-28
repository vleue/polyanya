use hashbrown::HashSet;
use std::{collections::VecDeque, vec::Vec};

use num_traits::{one, zero, Float};
use smallvec::smallvec;
use smallvec::SmallVec;

use crate::vendored_spade::delaunay_core::math;
use crate::vendored_spade::handles::VertexHandle;
use crate::vendored_spade::{
    handles::{
        DirectedEdgeHandle, FixedDirectedEdgeHandle, FixedVertexHandle, UndirectedEdgeHandle,
    },
    HasPosition, Point2, SpadeNum, Triangulation,
};

pub trait DistanceMetric<S>
where
    S: SpadeNum,
{
    fn is_edge_inside(&self, points: [Point2<S>; 2]) -> bool;

    fn is_handle_inside<V, DE, UE, F>(&self, handle: UndirectedEdgeHandle<V, DE, UE, F>) -> bool
    where
        V: HasPosition<Scalar = S>,
    {
        self.is_edge_inside(handle.positions())
    }

    fn distance_to_point(&self, point: Point2<S>) -> S;

    fn is_point_inside(&self, point: Point2<S>) -> bool {
        self.distance_to_point(point) <= zero()
    }
}

/// Defines the shape of circle.
///
/// This is only exported to allow referring to the return type of
/// [crate::vendored_spade::FloatTriangulation::get_edges_in_circle].
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy, Hash)]
pub struct CircleMetric<S: SpadeNum> {
    center: Point2<S>,
    radius_2: S,
}

impl<S: SpadeNum> CircleMetric<S> {
    pub(crate) fn new(center: Point2<S>, radius_2: S) -> Self {
        assert!(radius_2 >= zero());

        Self { center, radius_2 }
    }
}

impl<S> DistanceMetric<S> for CircleMetric<S>
where
    S: SpadeNum + Float,
{
    fn is_edge_inside(&self, points: [Point2<S>; 2]) -> bool {
        let [p0, p1] = points;
        math::distance_2(p0, p1, self.center) <= self.radius_2
    }

    fn distance_to_point(&self, point: Point2<S>) -> S {
        self.center.distance_2(point) - self.radius_2
    }
}

/// Defines the shape of a rectangle.
///
/// This is only exported to allow referring to the return type of
/// [crate::vendored_spade::FloatTriangulation::get_edges_in_rectangle].
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy, Hash)]
pub struct RectangleMetric<S>
where
    S: SpadeNum,
{
    lower: Point2<S>,
    upper: Point2<S>,
}

impl<S> RectangleMetric<S>
where
    S: SpadeNum,
{
    pub(crate) fn new(lower: Point2<S>, upper: Point2<S>) -> Self {
        Self { lower, upper }
    }
}

impl<S> DistanceMetric<S> for RectangleMetric<S>
where
    S: SpadeNum,
    S: Float,
{
    fn is_edge_inside(&self, points: [Point2<S>; 2]) -> bool {
        let [from, to] = points;
        if self.is_point_inside(from) || self.is_point_inside(to) {
            return true;
        }

        if self.lower == self.upper {
            return math::side_query(from, to, self.lower).is_on_line();
        }

        for [v0, v1] in self.edges() {
            let [s0, s1] = get_edge_intersections(v0, v1, from, to);
            if s0.is_infinite() {
                // Edges are colinear
                if !math::side_query(from, to, v0).is_on_line() {
                    // Edges are parallel but not overlapping
                    continue;
                }

                let p1 = math::project_point(from, to, v0);
                let p2 = math::project_point(from, to, v1);

                if p1.is_on_edge() || p2.is_on_edge() {
                    return true;
                }
            } else if (zero()..=one()).contains(&s0) && (zero()..=one()).contains(&s1) {
                return true;
            }
        }
        false
    }

    fn distance_to_point(&self, point: Point2<S>) -> S {
        if self.lower == self.upper {
            return point.distance_2(self.lower);
        }

        if self.is_point_inside(point) {
            zero()
        } else {
            let [d0, d1, d2, d3] = self.edges().map(|[p0, p1]| math::distance_2(p0, p1, point));

            d0.min(d1).min(d2).min(d3)
        }
    }
}

impl<S> RectangleMetric<S>
where
    S: SpadeNum,
{
    fn is_point_inside(&self, point: Point2<S>) -> bool {
        point.all_component_wise(self.lower, |a, b| a >= b)
            && point.all_component_wise(self.upper, |a, b| a <= b)
    }

    fn edges(&self) -> [[Point2<S>; 2]; 4] {
        let lower = self.lower;
        let upper = self.upper;
        let v0 = lower;
        let v1 = Point2::new(lower.x, upper.y);
        let v2 = upper;
        let v3 = Point2::new(upper.x, lower.y);

        [[v0, v1], [v1, v2], [v2, v3], [v3, v0]]
    }
}

fn get_edge_intersections<S: Float>(
    v0: Point2<S>,
    v1: Point2<S>,
    e0: Point2<S>,
    e1: Point2<S>,
) -> [S; 2] {
    let x4 = v1.x;
    let x3 = v0.x;
    let x2 = e1.x;
    let x1 = e0.x;
    let y4 = v1.y;
    let y3 = v0.y;
    let y2 = e1.y;
    let y1 = e0.y;
    let divisor = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
    let (s0, s1);
    if divisor == zero() {
        s0 = S::infinity();
        s1 = S::infinity();
    } else {
        s0 = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / divisor;
        s1 = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / divisor;
    }
    [s0, s1]
}

/// Implements logic required to flood-fill a convex shape in the triangulation.
/// Flood filling will work by
///  - identifying an initial face that covers the face
///  - expanding the edge hull formed by that face until all of the shape is covered (or the
///    convex hull is reached)
///
/// This type doesn't implement `Iterator` directly - instead, it does the heavy lifting for
/// [VerticesInShapeIterator] and [EdgesInShapeIterator].
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FloodFillIterator<'a, T, M>
where
    T: Triangulation,
    M: DistanceMetric<<T::Vertex as HasPosition>::Scalar>,
{
    t: &'a T,
    edge_loop: VecDeque<FixedDirectedEdgeHandle>,
    pending: Option<FixedDirectedEdgeHandle>,
    already_visited: HashSet<FixedVertexHandle>,
    metric: M,
}

/// An iterator over vertices within a shape (e.g. a rectangle or circle).
///
/// Constructed by calling [crate::vendored_spade::FloatTriangulation::get_vertices_in_rectangle] or
/// [crate::vendored_spade::FloatTriangulation::get_vertices_in_circle]
///
/// The item type is [VertexHandle]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerticesInShapeIterator<'a, T, M>
where
    T: Triangulation,
    M: DistanceMetric<<T::Vertex as HasPosition>::Scalar>,
{
    initial_elements: SmallVec<[FixedVertexHandle; 3]>,
    inner_iter: FloodFillIterator<'a, T, M>,
}

impl<'a, T, M> VerticesInShapeIterator<'a, T, M>
where
    T: Triangulation,
    M: DistanceMetric<<T::Vertex as HasPosition>::Scalar>,
{
    pub(crate) fn new(inner_iter: FloodFillIterator<'a, T, M>) -> Self {
        let initial_elements = if inner_iter.t.num_vertices() == 1 {
            // The flood fill iterator requires at least a single edge to work properly. We'll have
            // to special case triangulations that contain vertices but no edges.
            smallvec![FixedVertexHandle::new(0)]
        } else {
            inner_iter.already_visited.iter().copied().collect()
        };

        Self {
            initial_elements,
            inner_iter,
        }
    }
}

impl<'a, T, M> Iterator for VerticesInShapeIterator<'a, T, M>
where
    T: Triangulation,
    M: DistanceMetric<<T::Vertex as HasPosition>::Scalar>,
{
    type Item = VertexHandle<'a, T::Vertex, T::DirectedEdge, T::UndirectedEdge, T::Face>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.initial_elements.pop() {
            let vertex = self.inner_iter.t.vertex(next);
            if self.inner_iter.metric.is_point_inside(vertex.position()) {
                return Some(vertex);
            }
        }

        while let Some((_, vertex)) = self.inner_iter.next() {
            if let Some(vertex) = vertex {
                if self.inner_iter.metric.is_point_inside(vertex.position()) {
                    return Some(vertex);
                }
            }
        }
        None
    }
}

/// An iterator over edges within a shape (e.g. a rectangle or circle).
///
/// Constructed by calling [crate::vendored_spade::FloatTriangulation::get_edges_in_rectangle] or
/// [crate::vendored_spade::FloatTriangulation::get_edges_in_circle]
///
/// The item type is [UndirectedEdgeHandle]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EdgesInShapeIterator<'a, T, M>
where
    T: Triangulation,
    M: DistanceMetric<<T::Vertex as HasPosition>::Scalar>,
{
    pub(crate) inner_iter: FloodFillIterator<'a, T, M>,
}

impl<'a, T, M> Iterator for EdgesInShapeIterator<'a, T, M>
where
    T: Triangulation,
    M: DistanceMetric<<T::Vertex as HasPosition>::Scalar>,
{
    type Item = UndirectedEdgeHandle<'a, T::Vertex, T::DirectedEdge, T::UndirectedEdge, T::Face>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter
            .next()
            .map(|(handle, _)| handle.as_undirected())
    }
}

impl<'a, T, M> FloodFillIterator<'a, T, M>
where
    T: Triangulation,
    M: DistanceMetric<<T::Vertex as HasPosition>::Scalar>,
{
    pub fn new(
        t: &'a T,
        metric: M,
        start_point: Point2<<T::Vertex as HasPosition>::Scalar>,
    ) -> Self {
        let start_edges = Self::get_start_edges(t, &metric, start_point);
        let already_visited = start_edges
            .iter()
            .map(|edge| t.directed_edge(*edge).from().fix())
            .collect();

        Self {
            t,
            edge_loop: start_edges.into(),
            pending: None,
            already_visited,
            metric,
        }
    }

    #[allow(clippy::type_complexity)]
    fn get_start_edges(
        t: &'a T,
        metric: &M,
        start_point: Point2<<T::Vertex as HasPosition>::Scalar>,
    ) -> Vec<FixedDirectedEdgeHandle> {
        use crate::vendored_spade::PositionInTriangulation::*;
        if !metric.is_point_inside(start_point) {
            // Used to indicate an empty metric, e.g. a rectangle metric with upper < lower
            return Vec::new();
        }

        if t.all_vertices_on_line() {
            return t
                .undirected_edges()
                .filter(|edge| metric.is_handle_inside(*edge))
                .flat_map(|edge| [edge.as_directed().fix(), edge.as_directed().rev().fix()])
                .collect();
        }

        let start_face = match t.locate(start_point) {
            OnVertex(point) => t
                .vertex(point)
                .out_edges()
                .flat_map(|edge| edge.face().as_inner())
                .next(),
            OnEdge(edge) => {
                let edge = t.directed_edge(edge);
                edge.face()
                    .as_inner()
                    .or_else(|| edge.rev().face().as_inner())
            }
            OnFace(face) => Some(t.face(face)),
            OutsideOfConvexHull(edge) => {
                // The provided point does not lie on any face of the triangulation.
                // Attempt to find an overlapping edge by iterating over the convex hull and
                // minimizing the edge distance
                let mut current_edge = t.directed_edge(edge);
                let [from_distance, to_distance] = current_edge
                    .positions()
                    .map(|p| metric.distance_to_point(p));
                let walk_forward;
                let mut min_distance;
                if from_distance > to_distance {
                    walk_forward = true;
                    min_distance = to_distance;
                } else {
                    walk_forward = false;
                    min_distance = from_distance;
                }

                loop {
                    if metric.is_handle_inside(current_edge.as_undirected()) {
                        break current_edge.rev().face().as_inner();
                    }

                    let next_distance = if walk_forward {
                        current_edge = current_edge.next();
                        metric.distance_to_point(current_edge.to().position())
                    } else {
                        current_edge = current_edge.prev();
                        metric.distance_to_point(current_edge.from().position())
                    };

                    if next_distance > min_distance {
                        // Advancing the convex hull is increasing the distance to the convex
                        // shape we won't find an intersection.
                        break None;
                    }

                    min_distance = next_distance;
                }
            }
            NoTriangulation => None,
        };

        start_face
            .into_iter()
            .flat_map(|face| face.adjacent_edges().into_iter().rev())
            .map(|edge| edge.fix().rev())
            .collect()
    }
}

impl<'a, T, M> FloodFillIterator<'a, T, M>
where
    T: Triangulation,
    M: DistanceMetric<<T::Vertex as HasPosition>::Scalar>,
{
    /// An iterator over all edges within a convex shape
    ///
    /// To do so, the iterator will, starting from a start edge within the target shape, create a
    /// growing closed edge loop that expands with each iteration. All edges within that loop are also
    /// within the target shape.
    ///
    /// Each iteration step will replace the last edge from the current loop and expand it outwards:
    ///
    /// For a loop with `n` edges, this process looks like this:
    /// Before the iteration:
    ///
    /// ---edge n-1-->O--- edge n--> O ---edge 0-->
    ///
    /// After the iteration:
    ///                      O<--- v_new
    ///                    /   \
    ///        new edge n /     \ new edge n+1
    ///                  /       \
    /// --- edge n-1 -> O - - - - O --- edge 0 --->
    ///                      ^
    ///                      |
    ///   old edge n (gets returned and removed from the loop)
    ///
    /// Where
    /// `new edge n` = (edge n).prev().rev();
    /// `new edge n+1` = (edge n).next().rev();
    ///
    /// This procedure has one caveat: The loop may expand into itself - in the above
    /// depiction, `v_new` may already be part of the loop. This leads to
    /// issues as the loop will contain a "sub loop" that doesn't get cleaned up properly.
    ///
    /// For this reason, a hash set stores all vertices that have already been visited. If a collision
    /// is found, the iterator will skip the edge (by appending it to the front of the loop)
    /// and attempt to replace edge n-1.
    ///
    /// The return type is somewhat unexpected: Since this method is used to iterate both over vertices _and_ edges,
    /// it must both return any new edge and any new vertex. The possible cases are:
    /// - return `None` if the iteration has finished, independent of the element type
    /// - return `Some((edge, None))` if the iteration step produced a new edge but no new vertex
    /// - return `Some((edge, Some(vertex)))` if the iteration produced a new vertex. This always happens if a new edge is
    ///   also available.
    #[allow(clippy::type_complexity)]
    fn next(
        &mut self,
    ) -> Option<(
        DirectedEdgeHandle<'a, T::Vertex, T::DirectedEdge, T::UndirectedEdge, T::Face>,
        Option<VertexHandle<'a, T::Vertex, T::DirectedEdge, T::UndirectedEdge, T::Face>>,
    )> {
        if let Some(pending) = self.pending.take() {
            let pending = self.t.directed_edge(pending);
            if self.metric.is_handle_inside(pending.as_undirected()) {
                return Some((pending, None));
            }
        }

        while let Some(next) = self.edge_loop.pop_front() {
            let next = self.t.directed_edge(next);

            if !self.metric.is_handle_inside(next.as_undirected()) {
                continue;
            }

            // Check if `next` and `next.rev()` are next to each other in the loop.
            // This can commonly happen in two situations:
            // - For degenerate triangulations, the algorithm will simply pre-filter any contained
            //   edge `e` and append `[e, e.rev()]` to the edge loop. The if-statements below make
            //   sure that exactly one edge of these pre-filtered edges gets returned.
            // - Sometimes, the last edges returned in the iteration will all be inside the target
            //   shape and completely surrounded by the edge loop. With each iteration, the edge
            //   loop becomes smaller and smaller until it collapses into a single `[edge, edge.rev()]`
            //   pair. This check makes sure that the iteration stops by clearing the loop.
            if self.edge_loop.front() == Some(&next.rev().fix()) {
                self.edge_loop.pop_front();
                return Some((next, None));
            }

            // Same check as before but for the other neighbor of `next`
            if self.edge_loop.back() == Some(&next.rev().fix()) {
                self.edge_loop.pop_back();
                return Some((next, None));
            }

            if next.is_outer_edge() {
                return Some((next, None));
            }

            // The new edges that this edge may be expanded into
            let new_edge_1 = next.prev().rev();
            let new_edge_2 = next.next().rev();

            let first = self.edge_loop.front().copied();
            if first == Some(next.next().fix()) {
                // Another "special case" happens if the first and last edge in the
                // loop are direct neighbors.
                // In this case, expanding the last edge would create a duplicated edge in the
                // loop. Instead, we may only push one of the two usual successors.
                self.already_visited.remove(&next.to().fix());
                self.pending = self.edge_loop.pop_front();
                self.edge_loop.push_front(new_edge_1.fix());
                return Some((next, None));
            }

            let last = self.edge_loop.back().copied();
            if last == Some(next.prev().fix()) {
                // Same optimization as above but with the edge that comes before next (and not after)
                self.already_visited.remove(&next.from().fix());
                self.pending = self.edge_loop.pop_back();
                self.edge_loop.push_back(next.next().rev().fix());
                return Some((next, None));
            }

            // This line checks that the new vertex is not already part of the loop (see main
            // comment above).
            let new_vertex = new_edge_1.to();
            if !self.already_visited.insert(new_vertex.fix()) {
                let is_e1_inside = self.metric.is_handle_inside(new_edge_1.as_undirected());
                let is_e2_inside = self.metric.is_handle_inside(new_edge_2.as_undirected());

                match (is_e1_inside, is_e2_inside) {
                    (true, true) => {
                        self.edge_loop.push_back(next.fix());
                        continue;
                    }
                    (true, false) => self.edge_loop.push_back(new_edge_1.fix()),
                    (false, true) => self.edge_loop.push_back(new_edge_2.fix()),
                    (false, false) => {}
                }
            } else {
                self.edge_loop.push_back(new_edge_1.fix());
                self.edge_loop.push_back(new_edge_2.fix());
                return Some((next, Some(new_vertex)));
            }

            return Some((next, None));
        }
        None
    }
}
