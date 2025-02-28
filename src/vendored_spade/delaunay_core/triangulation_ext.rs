use smallvec::SmallVec;

use super::dcel_operations;
use super::dcel_operations::IsolateVertexResult;
use super::handles::*;
use super::math;

use crate::vendored_spade::HintGenerator;
use crate::vendored_spade::Point2;
use crate::vendored_spade::{HasPosition, InsertionError, PositionInTriangulation, Triangulation};

use std::vec::Vec;

impl<T> TriangulationExt for T where T: Triangulation + ?Sized {}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Hash, Eq, PartialEq)]
pub enum PositionWhenAllVerticesOnLine {
    OnEdge(FixedDirectedEdgeHandle),
    OnVertex(FixedVertexHandle),
    NotOnLine(FixedDirectedEdgeHandle),
    ExtendingLine(FixedVertexHandle),
}

impl PositionWhenAllVerticesOnLine {
    fn to_regular_position_in_triangulation<T: Triangulation>(
        self,
        triangulation: &T,
    ) -> PositionInTriangulation
    where
        T::Vertex: HasPosition,
    {
        use PositionWhenAllVerticesOnLine::*;
        match self {
            OnEdge(edge) => PositionInTriangulation::OnEdge(edge),
            OnVertex(v) => PositionInTriangulation::OnVertex(v),
            NotOnLine(edge) => PositionInTriangulation::OutsideOfConvexHull(edge),
            ExtendingLine(v) => {
                let out_edge = triangulation
                    .vertex(v)
                    .out_edge()
                    .expect("Could not find an out edge. This is a bug in spade.")
                    .fix();
                PositionInTriangulation::OutsideOfConvexHull(out_edge)
            }
        }
    }
}

pub enum InsertionResult {
    NewlyInserted(FixedVertexHandle),
    Updated(FixedVertexHandle),
}

pub struct RemovalResult<V> {
    pub removed_vertex: V,
    pub swapped_in_vertex: Option<FixedVertexHandle>,
}

pub trait TriangulationExt: Triangulation {
    fn insert_with_hint_option(
        &mut self,
        t: Self::Vertex,
        hint: Option<FixedVertexHandle>,
    ) -> Result<FixedVertexHandle, InsertionError> {
        math::validate_vertex(&t)?;
        let position = t.position();
        let result = self.insert_with_hint_option_impl(t, hint);

        self.hint_generator_mut()
            .notify_vertex_inserted(result, position);
        Ok(result)
    }

    fn insert_with_hint_option_impl(
        &mut self,
        t: Self::Vertex,
        hint: Option<FixedVertexHandle>,
    ) -> FixedVertexHandle {
        use PositionInTriangulation::*;

        let insertion_result = match self.num_vertices() {
            0 => return dcel_operations::insert_first_vertex(self.s_mut(), t),
            1 => self.insert_second_vertex(t),
            _ => {
                let pos = t.position();

                if self.all_vertices_on_line() {
                    let location = self.locate_when_all_vertices_on_line(pos);
                    self.insert_when_all_vertices_on_line(location, t)
                } else {
                    let position_in_triangulation = self.locate_with_hint_option_core(pos, hint);
                    match position_in_triangulation {
                        OutsideOfConvexHull(edge) => InsertionResult::NewlyInserted(
                            self.insert_outside_of_convex_hull(edge, t),
                        ),
                        OnFace(face) => {
                            InsertionResult::NewlyInserted(self.insert_into_face(face, t))
                        }
                        OnEdge(edge) => {
                            let (new_handle, split_parts) = self.insert_on_edge(edge, t);

                            if self.is_defined_legal(edge.as_undirected()) {
                                // If the edge is defined as legal the resulting edges must
                                // be redefined as legal
                                self.handle_legal_edge_split(split_parts);
                            }
                            self.legalize_vertex(new_handle);

                            InsertionResult::NewlyInserted(new_handle)
                        }
                        OnVertex(vertex) => {
                            self.s_mut().update_vertex(vertex, t);
                            InsertionResult::Updated(vertex)
                        }
                        NoTriangulation => panic!("Error during vertex lookup. This is a bug."),
                    }
                }
            }
        };

        match insertion_result {
            InsertionResult::NewlyInserted(new_handle) => new_handle,
            InsertionResult::Updated(update_handle) => update_handle,
        }
    }

    fn locate_when_all_vertices_on_line(
        &self,
        position: Point2<<Self::Vertex as HasPosition>::Scalar>,
    ) -> PositionWhenAllVerticesOnLine {
        use PositionWhenAllVerticesOnLine::*;

        let edge = self
            .directed_edges()
            .next()
            .expect("Must not be called on empty triangulations");
        let query = edge.side_query(position);
        if query.is_on_left_side() {
            return NotOnLine(edge.fix());
        }
        if query.is_on_right_side() {
            return NotOnLine(edge.fix().rev());
        }

        let mut vertices: Vec<_> = self.vertices().collect();
        vertices.sort_by(|left, right| left.position().partial_cmp(&right.position()).unwrap());

        let index_to_insert =
            match vertices.binary_search_by(|v| v.position().partial_cmp(&position).unwrap()) {
                Ok(index) => return OnVertex(vertices[index].fix()),
                Err(index) => index,
            };

        if index_to_insert == 0 {
            return ExtendingLine(vertices.first().unwrap().fix());
        }
        if index_to_insert == vertices.len() {
            return ExtendingLine(vertices.last().unwrap().fix());
        }

        let v1 = vertices[index_to_insert];
        let v2 = vertices[index_to_insert - 1];

        let edge = self
            .get_edge_from_neighbors(v1.fix(), v2.fix())
            .expect("Expected edge between sorted neighbors. This is a bug in spade.");
        OnEdge(edge.fix())
    }

    fn insert_second_vertex(&mut self, vertex: Self::Vertex) -> InsertionResult {
        assert_eq!(self.num_vertices(), 1);

        let first_vertex = FixedVertexHandle::new(0);
        if self.vertex(first_vertex).position() == vertex.position() {
            self.s_mut().update_vertex(first_vertex, vertex);
            return InsertionResult::Updated(first_vertex);
        }

        let second_vertex = dcel_operations::insert_second_vertex(self.s_mut(), vertex);

        InsertionResult::NewlyInserted(second_vertex)
    }

    fn insert_when_all_vertices_on_line(
        &mut self,
        location: PositionWhenAllVerticesOnLine,
        new_vertex: Self::Vertex,
    ) -> InsertionResult {
        match location {
            PositionWhenAllVerticesOnLine::OnEdge(edge) => {
                let is_constraint_edge = self.is_defined_legal(edge.as_undirected());
                let (new_edges, new_vertex) = dcel_operations::split_edge_when_all_vertices_on_line(
                    self.s_mut(),
                    edge,
                    new_vertex,
                );

                if is_constraint_edge {
                    self.handle_legal_edge_split(new_edges);
                }
                InsertionResult::NewlyInserted(new_vertex)
            }
            PositionWhenAllVerticesOnLine::OnVertex(vertex) => InsertionResult::Updated(vertex),
            PositionWhenAllVerticesOnLine::NotOnLine(edge) => {
                let result = self.insert_outside_of_convex_hull(edge, new_vertex);
                InsertionResult::NewlyInserted(result)
            }
            PositionWhenAllVerticesOnLine::ExtendingLine(end_vertex) => {
                let result = dcel_operations::extend_line(self.s_mut(), end_vertex, new_vertex);
                InsertionResult::NewlyInserted(result)
            }
        }
    }

    fn locate_with_hint_option_core(
        &self,
        point: Point2<<Self::Vertex as HasPosition>::Scalar>,
        hint: Option<FixedVertexHandle>,
    ) -> PositionInTriangulation {
        let start = hint.unwrap_or_else(|| self.hint_generator().get_hint(point));
        self.locate_with_hint_fixed_core(point, start)
    }

    fn insert_outside_of_convex_hull(
        &mut self,
        convex_hull_edge: FixedDirectedEdgeHandle,
        new_vertex: Self::Vertex,
    ) -> FixedVertexHandle {
        let position = new_vertex.position();

        assert!(self
            .directed_edge(convex_hull_edge)
            .side_query(position)
            .is_on_left_side());

        let result = dcel_operations::create_new_face_adjacent_to_edge(
            self.s_mut(),
            convex_hull_edge,
            new_vertex,
        );

        let ccw_walk_start = self.directed_edge(convex_hull_edge).prev().rev().fix();
        let cw_walk_start = self.directed_edge(convex_hull_edge).next().rev().fix();

        self.legalize_edge(convex_hull_edge);

        let mut current_edge = ccw_walk_start;
        loop {
            let handle = self.directed_edge(current_edge);
            let prev = handle.prev();
            current_edge = prev.fix();
            if prev.side_query(position).is_on_left_side() {
                let new_edge = dcel_operations::create_single_face_between_edge_and_next(
                    self.s_mut(),
                    current_edge,
                );
                self.legalize_edge(current_edge);
                current_edge = new_edge;
            } else {
                break;
            }
        }

        let mut current_edge = cw_walk_start;
        loop {
            let handle = self.directed_edge(current_edge);
            let next = handle.next();
            let next_fix = next.fix();
            if next.side_query(position).is_on_left_side() {
                let new_edge = dcel_operations::create_single_face_between_edge_and_next(
                    self.s_mut(),
                    current_edge,
                );
                self.legalize_edge(next_fix);
                current_edge = new_edge;
            } else {
                break;
            }
        }

        result
    }

    fn insert_into_face(
        &mut self,
        face: FixedFaceHandle<InnerTag>,
        t: Self::Vertex,
    ) -> FixedVertexHandle {
        let new_handle = dcel_operations::insert_into_triangle(self.s_mut(), t, face);
        self.legalize_vertex(new_handle);
        new_handle
    }

    fn insert_on_edge(
        &mut self,
        edge: FixedDirectedEdgeHandle,
        new_vertex: Self::Vertex,
    ) -> (FixedVertexHandle, [FixedDirectedEdgeHandle; 2]) {
        let edge_handle = self.directed_edge(edge);
        if edge_handle.is_outer_edge() {
            let (new_vertex, [e0, e1]) =
                dcel_operations::split_half_edge(self.s_mut(), edge.rev(), new_vertex);
            (new_vertex, [e1.rev(), e0.rev()])
        } else if edge_handle.rev().is_outer_edge() {
            dcel_operations::split_half_edge(self.s_mut(), edge, new_vertex)
        } else {
            dcel_operations::split_edge(self.s_mut(), edge, new_vertex)
        }
    }

    fn legalize_vertex(&mut self, new_handle: FixedVertexHandle) {
        let edges: SmallVec<[_; 4]> = self
            .vertex(new_handle)
            .out_edges()
            .filter(|e| !e.is_outer_edge())
            .map(|edge| edge.next().fix())
            .collect();

        for edge_to_legalize in edges {
            self.legalize_edge(edge_to_legalize);
        }
    }

    /// The Delaunay property refers to the property that no point lies inside
    /// the circumcircle of the triangulation's triangles. Adding a
    /// new point into the triangulations may violate this property, this method
    /// "repairs" it by strategically flipping edges until the property
    /// holds again. Every flip produces more "illegal" edges that may have to
    /// be flipped. However, since any edge is flipped at most once, this
    /// algorithm is known to terminate.
    ///
    /// The given position is the position of a new point may have
    /// invalidated the Delaunay property, the point must be on the left side
    /// of the given edge.
    ///
    /// "Flipping an edge" refers to switching to the other diagonal in a
    /// four sided polygon.
    ///
    /// Returns `true` if at least one edge was flipped. This will always include the initial edge.
    fn legalize_edge(&mut self, edge: FixedDirectedEdgeHandle) -> bool {
        let mut edges: SmallVec<[FixedDirectedEdgeHandle; 8]> = Default::default();
        edges.push(edge);

        let mut result = false;

        while let Some(e) = edges.pop() {
            if !self.is_defined_legal(e.as_undirected()) {
                let edge = self.directed_edge(e);

                //         v2------ v0
                //          |     / |
                //          |    /  |
                //          |   /<-edge that might be flipped ("edge")
                //          |  /    |
                //          | V     |
                //         v1-------v3
                //
                //
                // If the edge is flipped, the new quad will look like this:
                //
                //      New illegal edge
                //              |
                //              V
                //         v2-------v0
                //          | ^     |
                // New      |  \    |
                // illegal->|   \   |
                // edge     |    \  |
                //          |     \ |
                //         v1-------v3
                let v2 = edge.rev().opposite_position();
                let v3 = edge.opposite_position();

                if let (Some(v2), Some(v3)) = (v2, v3) {
                    let v0 = edge.from().position();
                    let v1 = edge.to().position();
                    debug_assert!(math::is_ordered_ccw(v2, v1, v0));
                    let should_flip = math::contained_in_circumference(v2, v1, v0, v3);
                    result |= should_flip;

                    if should_flip {
                        let e1 = edge.rev().next().fix();
                        let e2 = edge.rev().prev().fix();

                        dcel_operations::flip_cw(self.s_mut(), e.as_undirected());
                        edges.push(e1);
                        edges.push(e2);
                    }
                }
            }
        }
        result
    }

    fn validate_vertex_handle(&self, handle: FixedVertexHandle) -> FixedVertexHandle {
        if handle.index() < self.num_vertices() {
            handle
        } else {
            FixedVertexHandle::new(0)
        }
    }

    fn walk_to_nearest_neighbor(
        &self,
        start: FixedVertexHandle,
        position: Point2<<Self::Vertex as HasPosition>::Scalar>,
    ) -> VertexHandle<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face> {
        let start_position = self.vertex(start).position();

        if start_position == position {
            return self.vertex(start);
        }

        let mut current_minimal_distance = position.distance_2(start_position);
        let mut current_minimum_vertex = self.vertex(start);

        while let Some((next_minimum_index, next_minimal_distance)) = current_minimum_vertex
            .out_edges()
            .filter_map(|out_edge| {
                let next_candidate = out_edge.to();
                let new_distance = next_candidate.position().distance_2(position);
                if new_distance < current_minimal_distance {
                    Some((next_candidate, new_distance))
                } else {
                    None
                }
            })
            .next()
        {
            current_minimal_distance = next_minimal_distance;
            current_minimum_vertex = next_minimum_index;
        }

        current_minimum_vertex
    }

    /// "Walks" through the triangulation until it finds the target point.
    fn locate_with_hint_fixed_core(
        &self,
        target_position: Point2<<Self::Vertex as HasPosition>::Scalar>,
        start: FixedVertexHandle,
    ) -> PositionInTriangulation {
        if self.num_vertices() < 2 {
            return match self.vertices().next() {
                Some(single_vertex) if single_vertex.position() == target_position => {
                    PositionInTriangulation::OnVertex(single_vertex.fix())
                }
                _ => PositionInTriangulation::NoTriangulation,
            };
        }

        if self.all_vertices_on_line() {
            return self
                .locate_when_all_vertices_on_line(target_position)
                .to_regular_position_in_triangulation(self);
        }

        let start = self.validate_vertex_handle(start);
        let closest_vertex = self.walk_to_nearest_neighbor(start, target_position);

        // From here on spade attempts to find the location by inspecting the neighborhood of
        // the closest vertex and iteratively getting closer to the target position.
        // For regular Delaunay triangulations, the vertex should always be on an adjacent edge or
        // face (or on the closest vertex itself).
        // However, that doesn't hold for CDTs - these can have arbitrarily many faces between the
        // closest vertex and the target position. Thus, a "walk" to cross the remaining distance
        // is necessary. In addition, `walk_to_nearest_neighbor` does not use precise arithmetics -
        // it may fail to always report the real closest vertex. The algorithm below only uses
        // precise queries and should always return the correct answer.
        //
        // It works like this:
        // 1. Choose an arbitrary vertex V (closest_vertex in this case, but can be anything).
        // 2. Rotate around this vertex until we find the angle segment in which the target
        //    position lies:     V
        //                      / \
        //                   e0/   \e1
        //                    /__e2_\
        //                   .\     / .
        //                  .  \   /   .
        //                 .     O  <---.--- "opposite" vertex
        //                .              .
        //                   T      target position T lies between the segment spanned by the
        //                   ^----- outgoing edges e0 and e1
        //
        // 4. If the segment spans a triangle that contains the target position, we're done - the
        //    target lies in the face lined by e0, e1 and e2
        // 3. Otherwise, set V to the _opposite vertex_ (O in the diagram above), Then, go to step 2

        // e0 implicitly defines the rotation vertex V: It is always e0.from(). The segment is
        // adjacent to this edge iff `rotate_ccw == true` (see below)
        let mut e0 = closest_vertex
            .out_edge()
            .expect("No out edge found. This is a bug.");

        let mut e0_query = e0.side_query(target_position);

        // Choose the rotation direction (CW or CCW) to minimize the angle distance to the target
        // position.
        let mut rotate_ccw = e0_query.is_on_left_side_or_on_line();

        let mut loop_counter = self.s().num_directed_edges();
        loop {
            // Prevent accidental infinite loops (easier to debug). Should never happen
            if loop_counter == 0 {
                panic!("Failed to locate position. This is a bug in spade.")
            }
            loop_counter -= 1;

            let [from, to] = e0.vertices();
            if from.position() == target_position {
                self.hint_generator().notify_vertex_lookup(from.fix());
                return PositionInTriangulation::OnVertex(from.fix());
            }
            if to.position() == target_position {
                self.hint_generator().notify_vertex_lookup(to.fix());
                return PositionInTriangulation::OnVertex(to.fix());
            }

            if e0_query.is_on_line() {
                if e0.is_outer_edge() {
                    e0 = e0.rev();
                }

                // Special case: the current_edge is collinear with the target position.
                // This means no segment exists that could be used to advance the rotation vertex.
                // Instead, the algorithm changes to rotate around current_edge.prev().from()
                // Since the triangulation is not degenerate, this vertex cannot have an out edge
                // that is collinear.
                e0 = e0.prev();
                e0_query = e0.side_query(target_position);
                rotate_ccw = e0_query.is_on_left_side_or_on_line();
                continue;
            }

            // Delineates the other side of the segment
            let e1 = if rotate_ccw {
                e0
            } else {
                // Reverse the edge to ensure that the current segment is adjacent.
                e0.rev()
            };

            let Some(face) = e1.face().as_inner() else {
                self.hint_generator().notify_vertex_lookup(e1.from().fix());
                return PositionInTriangulation::OutsideOfConvexHull(e1.fix());
            };

            // rotate around `V = current_edge.from()` (see diagram above)
            // The current segment is spanned by `current_edge` and `rotated`
            let rotated = if rotate_ccw { e0.ccw() } else { e0.cw() };

            let rotated_query = rotated.side_query(target_position);
            if rotated_query.is_on_line() || rotated_query.is_on_left_side() == rotate_ccw {
                // Segment does not contain the target vertex. Continue rotating.
                e0 = rotated;
                e0_query = rotated_query;
                continue;
            }

            // Stores the last edge that is adjacent to the segment triangle
            let e2 = if rotate_ccw { e1.next() } else { e1.prev() };

            let e2_query = e2.side_query(target_position);
            if e2_query.is_on_line() {
                self.hint_generator().notify_vertex_lookup(e2.to().fix());
                return PositionInTriangulation::OnEdge(e2.fix());
            }
            if e2_query.is_on_left_side() {
                self.hint_generator().notify_vertex_lookup(e2.to().fix());
                return PositionInTriangulation::OnFace(face.fix());
            }

            // Check if an opposite vertex ("O" in the diagram above) exists. Otherwise, just
            // rotate around any vertex from e2 instead.
            e0 = e2.rev();
            e0_query = e2_query.reversed();
            if !e0.is_outer_edge() {
                // Opposite vertex exists - use any of its out edges and use that for the next
                // rotation.
                e0 = e0.prev();
                e0_query = e0.side_query(target_position);
            }

            rotate_ccw = e0_query.is_on_left_side_or_on_line();
        }
    }

    fn remove_and_notify(&mut self, vertex_to_remove: FixedVertexHandle) -> Self::Vertex {
        let position = self.vertex(vertex_to_remove).position();
        let removal_result = self.remove_core(vertex_to_remove);

        let swapped_in_point = removal_result
            .swapped_in_vertex
            .map(|_| self.vertex(vertex_to_remove).position());

        self.hint_generator_mut().notify_vertex_removed(
            swapped_in_point,
            vertex_to_remove,
            position,
        );

        removal_result.removed_vertex
    }

    fn remove_core(&mut self, vertex_to_remove: FixedVertexHandle) -> RemovalResult<Self::Vertex> {
        if self.num_all_faces() <= 1 {
            return dcel_operations::remove_when_degenerate(self.s_mut(), vertex_to_remove);
        }
        let vertex = self.vertex(vertex_to_remove);

        let mut border_loop = Vec::with_capacity(10);
        let mut convex_hull_edge = None;
        for edge in vertex.out_edges().rev() {
            if edge.is_outer_edge() {
                convex_hull_edge = Some(edge.fix());
                break;
            }
            let border_edge = edge.next();
            border_loop.push(border_edge.fix());
        }

        if let Some(convex_hull_edge) = convex_hull_edge {
            let mut isolation_result = self.isolate_convex_hull_vertex(convex_hull_edge);

            dcel_operations::cleanup_isolated_vertex(self.s_mut(), &mut isolation_result);
            dcel_operations::swap_remove_vertex(self.s_mut(), vertex_to_remove)
        } else {
            let mut isolation_result = dcel_operations::isolate_vertex_and_fill_hole(
                self.s_mut(),
                border_loop,
                vertex_to_remove,
            );
            // Not exactly elegant. IsolateVertexResult should maybe be split into two parts
            let mut new_edges = core::mem::take(&mut isolation_result.new_edges);
            self.legalize_edges_after_removal(&mut new_edges, |edge| {
                !isolation_result.is_new_edge(edge)
            });
            dcel_operations::cleanup_isolated_vertex(self.s_mut(), &mut isolation_result);
            dcel_operations::swap_remove_vertex(self.s_mut(), vertex_to_remove)
        }
    }

    fn isolate_convex_hull_vertex(
        &mut self,
        convex_hull_out_edge: FixedDirectedEdgeHandle,
    ) -> IsolateVertexResult {
        let mut edges_to_validate = Vec::with_capacity(10);
        let mut convex_edges: Vec<FixedDirectedEdgeHandle> = Vec::with_capacity(10);

        let loop_end = self.directed_edge(convex_hull_out_edge);
        let loop_start = loop_end.ccw().fix();
        let mut current = loop_start;
        let loop_end_next = loop_end.next().fix();
        let loop_end = loop_end.fix();

        loop {
            let current_handle = self.directed_edge(current);
            current = current_handle.ccw().fix();
            let edge = current_handle.next().fix();
            convex_edges.push(edge);

            while let &[.., edge1, edge2] = &*convex_edges {
                let edge1 = self.directed_edge(edge1);
                let edge2 = self.directed_edge(edge2);

                let target_position = edge2.to().position();
                // Check if the new edge would violate the convex hull property by turning left
                // The convex hull must only contain curves turning right
                if edge1.side_query(target_position).is_on_left_side() {
                    // Violation detected. It is resolved by flipping the edge that connects
                    // the most recently added edge (edge2) with the point that was removed
                    let edge_to_flip = edge2.prev().fix().rev();
                    dcel_operations::flip_cw(self.s_mut(), edge_to_flip.as_undirected());
                    convex_edges.pop();
                    convex_edges.pop();
                    convex_edges.push(edge_to_flip);
                    edges_to_validate.push(edge_to_flip.as_undirected());
                } else {
                    break;
                }
            }

            if current == loop_end {
                break;
            }
        }

        convex_edges.push(loop_end_next);
        let result = dcel_operations::disconnect_edge_strip(self.s_mut(), convex_edges);
        self.legalize_edges_after_removal(&mut edges_to_validate, |_| false);
        result
    }

    /// After a vertex removal, the created hole is stitched together by connecting
    /// all vertices to a single vertex at the border (triangle fan).
    /// Since these new edges can violate the Delaunay property, it must be restored.
    ///
    /// The algorithm works like this:
    ///  - Add all new edges to an "invalid" list
    ///  - While the invalid list is not empty: Determine if flipping the top edge is
    ///    required to restore the Delaunay property locally.
    ///  - If the edge was flipped: Determine the flip polygon. A flip refers
    ///    to switching the diagonal in a four sided polygon which defines the
    ///    flip polygon.
    ///  - Add all edges of the flip polygon to the invalid list if they were
    ///    newly created. Otherwise, the edge is part of the border loop surrounding
    ///    the hole created after the vertex removal. These are known to be valid and
    ///    need not be checked
    ///
    /// For more details, refer to
    /// Olivier Devillers. Vertex Removal in Two Dimensional Delaunay Triangulation:
    /// Speed-up by Low Degrees Optimization.
    /// <https://doi.org/10.1016/j.comgeo.2010.10.001>
    ///
    /// Note that the described low degrees optimization is not yet part of this library.
    fn legalize_edges_after_removal<F>(
        &mut self,
        edges_to_validate: &mut Vec<FixedUndirectedEdgeHandle>,
        edge_must_not_be_flipped_predicate: F,
    ) where
        F: Fn(FixedUndirectedEdgeHandle) -> bool,
    {
        while let Some(next_edge) = edges_to_validate.pop() {
            // left----to
            //  |     ^ |
            //  |    /  |
            //  |   /<-edge that might be flipped ("next_edge")
            //  |  /    |
            //  | /     |
            // from----right
            //
            // left, from, right and to define the flip polygon.
            if self.is_defined_legal(next_edge) || edge_must_not_be_flipped_predicate(next_edge) {
                continue;
            }

            let edge = self.directed_edge(next_edge.as_directed());
            let e2 = edge.prev();
            let e4 = edge.rev().prev();

            let from = edge.from().position();
            let to = edge.to().position();
            let left = edge.opposite_position();
            let right = edge.rev().opposite_position();

            let should_flip = match (left, right) {
                (Some(left), Some(right)) => {
                    math::contained_in_circumference(from, to, left, right)
                }
                // Handle special cases when evaluating edges next to the convex hull
                (None, Some(right)) => math::is_ordered_ccw(right, from, to),
                (Some(left), None) => math::is_ordered_ccw(left, to, from),
                (None, None) => {
                    panic!("Unexpected geometry. This is a bug in spade.")
                }
            };

            if should_flip {
                let e1 = edge.next();
                let e3 = edge.rev().next();

                let mut push_if_not_contained = |handle| {
                    if !edges_to_validate.contains(&handle) {
                        edges_to_validate.push(handle);
                    }
                };

                push_if_not_contained(e1.fix().as_undirected());
                push_if_not_contained(e2.fix().as_undirected());
                push_if_not_contained(e3.fix().as_undirected());
                push_if_not_contained(e4.fix().as_undirected());

                let fixed = edge.fix();
                dcel_operations::flip_cw(self.s_mut(), fixed.as_undirected());
            }
        }
    }

    #[cfg(any(test, fuzzing))]
    fn basic_sanity_check(&self, check_convexity: bool) {
        self.s().sanity_check();
        let all_vertices_on_line = self.s().num_faces() <= 1;

        for face in self.s().inner_faces() {
            let triangle = face.vertices();
            // Check that all vertices are stored in ccw orientation
            assert!(math::side_query(
                triangle[0].position(),
                triangle[1].position(),
                triangle[2].position()
            )
            .is_on_left_side(),);
        }

        for edge in self.s().directed_edges() {
            if all_vertices_on_line {
                assert_eq!(edge.face().fix(), dcel_operations::OUTER_FACE_HANDLE)
            } else {
                assert_ne!(edge.face(), edge.rev().face());
            }
            assert_ne!(edge.from(), edge.to());
        }

        if all_vertices_on_line {
            if self.s().num_vertices() > 1 {
                assert_eq!(self.s().num_undirected_edges(), self.s().num_vertices() - 1);
            } else {
                assert_eq!(self.s().num_undirected_edges(), 0);
            }
            assert_eq!(self.s().num_faces(), 1);
        } else {
            let num_inner_edges = self
                .s()
                .directed_edges()
                .filter(|e| !e.face().is_outer())
                .count();

            let num_inner_faces = self.s().num_faces() - 1;
            assert_eq!(num_inner_faces * 3, num_inner_edges);

            if check_convexity {
                for edge in self.convex_hull() {
                    for vert in self.vertices() {
                        assert!(edge
                            .side_query(vert.position())
                            .is_on_right_side_or_on_line(),);
                    }
                }
            }
        }
    }

    #[cfg(any(test, fuzzing))]
    fn sanity_check(&self) {
        self.basic_sanity_check(true);

        for edge in self.undirected_edges() {
            let edge = edge.as_directed();
            let rev = edge.rev();

            if let (Some(edge_opposite), Some(rev_opposite)) =
                (edge.opposite_position(), rev.opposite_position())
            {
                let from = edge.from().position();
                let to = edge.to().position();
                assert!(!math::contained_in_circumference(
                    from,
                    to,
                    edge_opposite,
                    rev_opposite
                ))
            }
        }
    }
}
