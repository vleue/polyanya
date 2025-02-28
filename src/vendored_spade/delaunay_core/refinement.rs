use std::collections::{HashMap, HashSet};

use std::collections::VecDeque;
use std::vec::Vec;

use num_traits::Float;

use crate::vendored_spade::{
    delaunay_core::math, CdtEdge, ConstrainedDelaunayTriangulation, HasPosition, HintGenerator,
    Point2, PositionInTriangulation, SpadeNum, Triangulation,
};

use super::{
    DirectedEdgeHandle, FaceHandle, FixedFaceHandle, FixedUndirectedEdgeHandle, FixedVertexHandle,
    InnerTag, TriangulationExt, UndirectedEdgeHandle,
};

#[derive(Debug, Clone)]
pub struct RefinementResult {
    pub excluded_faces: Vec<FixedFaceHandle<InnerTag>>,

    pub refinement_complete: bool,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct AngleLimit {
    radius_to_shortest_edge_limit: f64,
}

impl AngleLimit {
    pub fn from_deg(degree: f64) -> Self {
        Self::from_rad(degree.to_radians())
    }

    pub fn from_rad(rad: f64) -> Self {
        let sin = rad.sin();
        if sin == 0.0 {
            Self::from_radius_to_shortest_edge_ratio(f64::INFINITY)
        } else {
            Self::from_radius_to_shortest_edge_ratio(0.5 / sin)
        }
    }

    pub fn radius_to_shortest_edge_limit(&self) -> f64 {
        self.radius_to_shortest_edge_limit
    }

    pub fn from_radius_to_shortest_edge_ratio(ratio: f64) -> Self {
        Self {
            radius_to_shortest_edge_limit: ratio,
        }
    }
}

impl std::fmt::Debug for AngleLimit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AngleLimit")
            .field(
                "angle limit (deg)",
                &(0.5 / self.radius_to_shortest_edge_limit)
                    .asin()
                    .to_degrees(),
            )
            .finish()
    }
}

impl Default for AngleLimit {
    fn default() -> Self {
        Self::from_radius_to_shortest_edge_ratio(1.0)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Hash)]
enum RefinementHint {
    Ignore,
    ShouldRefine,
    MustRefine,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RefinementParameters<S: SpadeNum + Float> {
    max_additional_vertices: Option<usize>,

    angle_limit: AngleLimit,
    min_area: Option<S>,
    max_area: Option<S>,
    keep_constraint_edges: bool,
    exclude_outer_faces: bool,
}

impl<S: SpadeNum + Float> Default for RefinementParameters<S> {
    fn default() -> Self {
        Self {
            max_additional_vertices: None,
            angle_limit: AngleLimit::from_radius_to_shortest_edge_ratio(1.0),
            min_area: None,
            max_area: None,
            exclude_outer_faces: false,
            keep_constraint_edges: false,
        }
    }
}

impl<S: SpadeNum + Float> RefinementParameters<S> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_angle_limit(mut self, angle_limit: AngleLimit) -> Self {
        self.angle_limit = angle_limit;
        self
    }

    pub fn with_min_required_area(mut self, min_area: S) -> Self {
        self.min_area = Some(min_area);
        self
    }

    pub fn with_max_allowed_area(mut self, max_area: S) -> Self {
        self.max_area = Some(max_area);
        self
    }

    pub fn with_max_additional_vertices(mut self, max_additional_vertices: usize) -> Self {
        self.max_additional_vertices = Some(max_additional_vertices);
        self
    }

    pub fn keep_constraint_edges(mut self) -> Self {
        self.keep_constraint_edges = true;
        self
    }

    pub fn exclude_outer_faces(mut self, exclude: bool) -> Self {
        self.exclude_outer_faces = exclude;
        self
    }

    fn get_refinement_hint<V, DE, UE, F>(
        &self,
        face: FaceHandle<InnerTag, V, DE, UE, F>,
    ) -> RefinementHint
    where
        V: HasPosition<Scalar = S>,
    {
        if let Some(max_area) = self.max_area {
            if face.area() > max_area {
                return RefinementHint::MustRefine;
            }
        }

        if let Some(min_area) = self.min_area {
            if face.area() < min_area {
                return RefinementHint::Ignore;
            }
        }

        let (_, length2) = face.shortest_edge();
        let (_, radius2) = face.circumcircle();

        let ratio2 = radius2 / length2;
        let angle_limit = self.angle_limit.radius_to_shortest_edge_limit;
        if ratio2.into() > angle_limit * angle_limit {
            RefinementHint::ShouldRefine
        } else {
            RefinementHint::Ignore
        }
    }
}

impl<V, DE, UE, F, L> ConstrainedDelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition + From<Point2<<V as HasPosition>::Scalar>>,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
    <V as HasPosition>::Scalar: Float,
{
    #[doc(alias = "Refinement")]
    #[doc(alias = "Delaunay Refinement")]
    pub fn refine(&mut self, parameters: RefinementParameters<V::Scalar>) -> RefinementResult {
        use PositionInTriangulation::*;

        let mut excluded_faces = if parameters.exclude_outer_faces {
            calculate_outer_faces(self)
        } else {
            HashSet::new()
        };

        let mut legalize_edges_buffer = Vec::with_capacity(20);
        let mut forcibly_split_segments_buffer = Vec::with_capacity(5);

        // Maps each steiner point on an input edge onto the two vertices of that
        // input edge. This helps in identifying when two steiner points share a common
        // input angle
        let mut constraint_edge_map = HashMap::new();

        // Stores all edges that should be checked for encroachment
        let mut encroached_segment_candidates =
            VecDeque::with_capacity(self.num_constraints() + self.convex_hull_size());

        encroached_segment_candidates.extend(
            self.undirected_edges()
                .filter(|edge| {
                    if parameters.keep_constraint_edges {
                        edge.is_part_of_convex_hull()
                    } else {
                        Self::is_fixed_edge(*edge)
                    }
                })
                .map(|edge| edge.fix()),
        );

        // Stores all faces that should be checked for their area and angles ("skinniness").
        let mut skinny_triangle_candidates: VecDeque<_> = self.fixed_inner_faces().collect();

        let num_initial_vertices: usize = self.num_vertices();
        let num_additional_vertices = parameters
            .max_additional_vertices
            .unwrap_or(num_initial_vertices * 10);
        let max_allowed_vertices =
            usize::saturating_add(num_initial_vertices, num_additional_vertices);

        let mut refinement_complete = true;

        // Main loop of the algorithm
        //
        // Some terminology:
        //  - "Skinny triangle" refers to any triangle that has a minimum inner angle less than the allowed limit specified
        //    by the refinement parameters. The algorithm will attempt to insert steiner points to increase their minimal
        //    angle.
        //  - An edge is *encroached* by a point if that point lies in the diametral circle of the edge (the smallest circle
        //    fully containing the edge)
        //  - a "fixed" edge is a constraint edge or an edge of the convex hull. These are special as they may not be
        //    flipped - the input geometry must remain the same.
        //  - "input angle" is any angle between two fixed edges. Small input angles cannot be refined away as
        //    the input geometry must be kept intact.
        //  - "excluded faces" may exist if the triangulation's outer faces should not be refined. They are excluded from
        //     the third step in the main loop (see below). We don't simply delete these faces to keep the triangulation's
        //     convexity.
        //
        // Every iteration performs up to three checks:
        //  - First, check if any edges that must be split exists (`forcibly_split_segments_buffer`).
        //  - Second, check if any segment is encroached. If found, resolve the offending encroachment.
        //    Checking segments first makes sure that the algorithm
        //    restores the Delaunay property as quickly as possible.
        //  - Third, search for skinny triangles. Attempt to insert a new vertex at the triangle's circumcenter. If inserting
        //    such a vertex would encroach any fixed edge, add the encroached edge to the forcibly split segments buffer
        //    and revisit the face later.
        //
        // See method `resolve_encroachment` for more details on how step 1 and 2 manage to split edges in order to resolve
        // an encroachment.
        'main_loop: loop {
            if self.num_vertices() >= max_allowed_vertices {
                refinement_complete = false;
                break;
            }

            // Step 1: Check for forcibly split segments.
            if let Some(forcibly_split_segment) = forcibly_split_segments_buffer.pop() {
                self.resolve_encroachment(
                    &mut encroached_segment_candidates,
                    &mut skinny_triangle_candidates,
                    &mut constraint_edge_map,
                    forcibly_split_segment,
                    &mut excluded_faces,
                );
                continue;
            }

            // Step 2: Check for encroached segments.
            if let Some(segment_candidate) = encroached_segment_candidates.pop_front() {
                // Check both adjacent faces of any candidate for encroachment.
                for edge in segment_candidate.directed_edges() {
                    let edge = self.directed_edge(edge);

                    let is_excluded = edge
                        .face()
                        .as_inner()
                        .map(|face| excluded_faces.contains(&face.fix()))
                        .unwrap_or(true);

                    if is_excluded {
                        continue;
                    }

                    if let Some(opposite_position) = edge.opposite_position() {
                        if is_encroaching_edge(
                            edge.from().position(),
                            edge.to().position(),
                            opposite_position,
                        ) {
                            // The edge is encroaching
                            self.resolve_encroachment(
                                &mut encroached_segment_candidates,
                                &mut skinny_triangle_candidates,
                                &mut constraint_edge_map,
                                segment_candidate,
                                &mut excluded_faces,
                            );
                        }
                    }
                }

                continue;
            }

            // Step 3: Take the next skinny triangle candidate
            if let Some(face) = skinny_triangle_candidates.pop_front() {
                if excluded_faces.contains(&face) {
                    continue;
                }

                let face = self.face(face);

                let (shortest_edge, _) = face.shortest_edge();

                let refinement_hint = parameters.get_refinement_hint(face);

                if refinement_hint == RefinementHint::Ignore {
                    // Triangle is fine as is and can be skipped
                    continue;
                }

                if refinement_hint == RefinementHint::ShouldRefine
                    && !Self::is_fixed_edge(shortest_edge.as_undirected())
                {
                    // Check if the shortest edge ends in two input edges that span a small
                    // input angle.
                    //
                    // Such an input angle cannot be maximized as that would require flipping at least one of its edges.
                    //
                    // See Miller, Gary; Pav, Steven; Walkington, Noel (2005). "When and why Delaunay refinement algorithms work".
                    // for more details on this idea.
                    let original_from = constraint_edge_map
                        .get(&shortest_edge.from().fix())
                        .copied();
                    let original_to = constraint_edge_map.get(&shortest_edge.to().fix()).copied();

                    for from_input_vertex in original_from.iter().flatten() {
                        for to_input_vertex in original_to.iter().flatten() {
                            if from_input_vertex == to_input_vertex {
                                // The two edges are input segments and join a common segment.
                                // Don't attempt to subdivide it any further, this is as good as we can get.
                                continue 'main_loop;
                            }
                        }
                    }
                }

                // Continue to resolve the skinny face
                let circumcenter = face.circumcenter();

                let locate_hint = face.vertices()[0].fix();

                assert!(forcibly_split_segments_buffer.is_empty());
                legalize_edges_buffer.clear();

                // "Simulate" inserting the circumcenter by locating the insertion site and identifying which edges
                // would need to be flipped (legalized) by the insertion. If any of these edges is fixed, an
                // encroachment with this edge is found.
                //
                // First step: fill `legalize_edges_buffer` with the initial set of edges that would need to be legalized
                // if the triangle's circumcenter would be inserted.
                match self.locate_with_hint(circumcenter, locate_hint) {
                    OnEdge(edge) => {
                        let edge = self.directed_edge(edge);
                        if parameters.keep_constraint_edges && edge.is_constraint_edge() {
                            continue;
                        }

                        if edge.is_constraint_edge() {
                            // Splitting constraint edges may require updating the `excluded_faces` set.
                            // This is a little cumbersome, we'll re-use the existing implementation of edge
                            // splitting (see function resolve_encroachment).
                            forcibly_split_segments_buffer.push(edge.fix().as_undirected());
                            continue;
                        }

                        for edge in [edge, edge.rev()] {
                            if !edge.is_outer_edge() {
                                legalize_edges_buffer.extend([edge.next().fix(), edge.prev().fix()])
                            }
                        }
                    }
                    OnFace(face_under_circumcenter) => {
                        if excluded_faces.contains(&face_under_circumcenter) {
                            continue;
                        }
                        legalize_edges_buffer.extend(
                            self.face(face_under_circumcenter)
                                .adjacent_edges()
                                .map(|edge| edge.fix()),
                        );
                    }
                    OutsideOfConvexHull(_) => continue,
                    OnVertex(_) => continue,
                    NoTriangulation => unreachable!(),
                };

                let mut is_encroaching = false;

                // Next step: Perform the regular legalization procedure by "simulating" edge flips
                while let Some(edge) = legalize_edges_buffer.pop() {
                    let edge = self.directed_edge(edge);
                    let [from, to] = edge.as_undirected().positions();
                    if Self::is_fixed_edge(edge.as_undirected()) {
                        if is_encroaching_edge(from, to, circumcenter) {
                            // We found an encroaching edge! Makes sure that we won't attempt to
                            // insert the circumcenter.
                            is_encroaching = true;

                            if !parameters.keep_constraint_edges || !edge.is_constraint_edge() {
                                // New circumcenter would encroach a constraint edge. Don't insert the circumcenter
                                // but force splitting the segment
                                forcibly_split_segments_buffer.push(edge.as_undirected().fix());
                            }
                        }
                        continue; // Don't actually flip the edge as it's fixed - continue with any other edge instead.
                    }

                    // edge is not a fixed edge. Check if it needs to be legalized.
                    // We've already checked that this edge is not part of the convex hull - unwrap is safe
                    let opposite = edge.rev().opposite_position().unwrap();
                    let from = edge.from().position();
                    let to = edge.to().position();
                    let should_flip =
                        math::contained_in_circumference(opposite, to, from, circumcenter);

                    if should_flip {
                        let e1 = edge.rev().next().fix();
                        let e2 = edge.rev().prev().fix();

                        legalize_edges_buffer.push(e1);
                        legalize_edges_buffer.push(e2);
                    }
                }

                if !is_encroaching {
                    // The circumcenter doesn't encroach any segment. Continue really inserting it.
                    let new_vertex = self
                        .insert_with_hint(circumcenter.into(), locate_hint)
                        .expect("Failed to insert circumcenter, likely due to loss of precision. Consider refining with fewer additional vertices.");

                    // Add all new and changed faces to the skinny candidate list
                    skinny_triangle_candidates.extend(
                        self.vertex(new_vertex)
                            .out_edges()
                            .flat_map(|edge| edge.face().fix().as_inner()),
                    );
                } else if !forcibly_split_segments_buffer.is_empty() {
                    // Revisit this face later. Since the encroached edge will have been split in the next iteration,
                    // inserting the circumcenter might succeed this time around.
                    skinny_triangle_candidates.push_back(face.fix());
                }
            } else {
                // Done! This branch is reached if no skinny triangle could be identified anymore.
                break;
            }
        }

        RefinementResult {
            excluded_faces: excluded_faces.iter().copied().collect(),
            refinement_complete,
        }
    }

    fn is_fixed_edge(edge: UndirectedEdgeHandle<V, DE, CdtEdge<UE>, F>) -> bool {
        edge.is_constraint_edge() || edge.is_part_of_convex_hull()
    }

    fn resolve_encroachment(
        &mut self,
        encroached_segments_buffer: &mut VecDeque<FixedUndirectedEdgeHandle>,
        encroached_faces_buffer: &mut VecDeque<FixedFaceHandle<InnerTag>>,
        constraint_edge_map: &mut HashMap<FixedVertexHandle, [FixedVertexHandle; 2]>,
        encroached_edge: FixedUndirectedEdgeHandle,
        excluded_faces: &mut HashSet<FixedFaceHandle<InnerTag>>,
    ) {
        // Resolves an encroachment by splitting the encroached edge. Since this reduces the diametral circle, this will
        // eventually get rid of the encroachment completely.
        //
        // There are a few details that make this more complicated:
        //
        // # Runaway encroachment
        // Any input angle less than 45 degrees may lead to a "runaway encroachment". In such a situation, any of the
        // angle's edges will encroach *the other* edge. This goes on forever, subdividing the edges infinitely.
        //
        // To work around this, spade will split edges at their center position only *once*.
        // Any subsegment will not be split at its center position but *rounded towards the nearest power of 2*.
        // With this behavior, neighboring edges will eventually share vertices equally far away from the offending angle's
        // apex vertex. Points and edges in such a configuration cannot encroach each other. Refer to the original paper
        // by Ruppert for more details.
        //
        // # Keeping track of which edges and faces have changed
        // Since `resolve_encroachment` will create new edges and faces, we need to add those to the existing buffers as
        // appropriate. This becomes a little convoluted when supporting all different refinement modes, e.g. excluded faces.

        let segment = self.directed_edge(encroached_edge.as_directed());

        let [v0, v1] = segment.vertices();

        let half = Into::<V::Scalar>::into(0.5f32);

        let v0_constraint_vertex = constraint_edge_map.get(&v0.fix()).copied();
        let v1_constraint_vertex = constraint_edge_map.get(&v1.fix()).copied();

        let (weight0, weight1) = match (v0_constraint_vertex, v1_constraint_vertex) {
            (None, None) => {
                // Split the segment exactly in the middle if it has not been split before.
                (half, half)
            }
            _ => {
                // One point is a steiner point, another point isn't. This will trigger rounding the distance to
                // the nearest power of two to prevent runaway encroachment.

                let half_length = segment.length_2().sqrt() * half;

                let nearest_power_of_two = nearest_power_of_two(half_length);
                let other_vertex_weight = half * nearest_power_of_two / half_length;
                let original_vertex_weight = Into::<V::Scalar>::into(1.0) - other_vertex_weight;

                if v0_constraint_vertex.is_none() {
                    // Orient the weight towards to original vertex. This makes sure that any edge participating in
                    // a runaway encroachment will end up with the same distance to the non-steiner (original) point.
                    (original_vertex_weight, other_vertex_weight)
                } else {
                    (other_vertex_weight, original_vertex_weight)
                }
            }
        };

        let final_position = v0.position().mul(weight0).add(v1.position().mul(weight1));

        if !validate_constructed_vertex(final_position, segment) {
            return;
        }

        let [is_left_side_excluded, is_right_side_excluded] =
            [segment.face(), segment.rev().face()].map(|face| {
                face.as_inner()
                    .map_or(false, |face| excluded_faces.contains(&face.fix()))
            });

        let is_constraint_edge = segment.is_constraint_edge();

        // Perform the actual split!
        let segment = segment.fix();
        let (v0, v1) = (v0.fix(), v1.fix());

        let (new_vertex, [e1, e2]) = self.insert_on_edge(segment, final_position.into());

        let original_vertices = v0_constraint_vertex
            .or(v1_constraint_vertex)
            .unwrap_or([v0, v1]);
        constraint_edge_map.insert(new_vertex, original_vertices);

        if is_constraint_edge {
            // Make sure to update the constraint edges count as required.
            self.handle_legal_edge_split([e1, e2]);
        }

        let (h1, h2) = (self.directed_edge(e1), self.directed_edge(e2));

        if is_left_side_excluded {
            // Any newly added face on the left becomes an excluded face
            excluded_faces.insert(h1.face().fix().as_inner().unwrap());
            excluded_faces.insert(h2.face().fix().as_inner().unwrap());
        }

        if is_right_side_excluded {
            // Any newly added face on the right becomes an excluded face
            excluded_faces.insert(h1.rev().face().fix().as_inner().unwrap());
            excluded_faces.insert(h2.rev().face().fix().as_inner().unwrap());
        }

        self.legalize_vertex(new_vertex);

        // Any of the faces that share an outgoing edge may be changed by the vertex insertion. Make sure that all of them
        // will be revisited.
        encroached_faces_buffer.extend(
            self.vertex(new_vertex)
                .out_edges()
                .flat_map(|edge| edge.face().fix().as_inner()),
        );

        // Neighboring edges may have become encroached. Check if they need to be added to the encroached segment buffer.
        encroached_segments_buffer.extend(
            self.vertex(new_vertex)
                .out_edges()
                .filter(|edge| !edge.is_outer_edge())
                .map(|edge| edge.next().as_undirected())
                .filter(|edge| Self::is_fixed_edge(*edge))
                .map(|edge| edge.fix()),
        );

        // Update encroachment candidates - any of the resulting edges may still be in an encroaching state.
        encroached_segments_buffer.push_back(e1.as_undirected());
        encroached_segments_buffer.push_back(e2.as_undirected());
    }
}

fn validate_constructed_vertex<V, DE, UE, F>(
    final_position: Point2<V::Scalar>,
    segment: DirectedEdgeHandle<V, DE, UE, F>,
) -> bool
where
    V: HasPosition,
{
    use math::is_ordered_ccw;
    let [v0, v1] = segment.positions();

    if math::validate_vertex(&final_position).is_err() {
        return false;
    }

    if let Some(v2) = segment.opposite_position() {
        if is_ordered_ccw(v0, v2, final_position) || is_ordered_ccw(v2, v1, final_position) {
            return false;
        }
    }

    if let Some(v3) = segment.rev().opposite_position() {
        if is_ordered_ccw(v3, v0, final_position) || is_ordered_ccw(v1, v3, final_position) {
            return false;
        }
    }

    true
}

fn is_encroaching_edge<S: SpadeNum + Float>(
    edge_from: Point2<S>,
    edge_to: Point2<S>,
    query_point: Point2<S>,
) -> bool {
    let edge_center = edge_from.add(edge_to).mul(0.5f32.into());
    let radius_2 = edge_from.distance_2(edge_to) * 0.25.into();

    query_point.distance_2(edge_center) < radius_2
}

fn nearest_power_of_two<S: Float + SpadeNum>(input: S) -> S {
    input.log2().round().exp2()
}

fn calculate_outer_faces<V: HasPosition, DE: Default, UE: Default, F: Default, L>(
    triangulation: &ConstrainedDelaunayTriangulation<V, DE, UE, F, L>,
) -> HashSet<FixedFaceHandle<InnerTag>>
where
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    if triangulation.all_vertices_on_line() {
        return HashSet::new();
    }

    // Determine excluded faces by "peeling of" outer layers and adding them to an outer layer set.
    // This needs to be done repeatedly to also get inner "holes" within the triangulation
    let mut inner_faces = HashSet::new();
    let mut outer_faces = HashSet::new();

    let mut current_todo_list: Vec<_> =
        triangulation.convex_hull().map(|edge| edge.rev()).collect();
    let mut next_todo_list = Vec::new();

    let mut return_outer_faces = true;

    loop {
        // Every iteration of the outer while loop will peel of the outmost layer and pre-populate the
        // next, inner layer.
        while let Some(next_edge) = current_todo_list.pop() {
            let (list, face_set) = if next_edge.is_constraint_edge() {
                // We're crossing a constraint edge - add that face to the *next* todo list
                (&mut next_todo_list, &mut inner_faces)
            } else {
                (&mut current_todo_list, &mut outer_faces)
            };

            if let Some(inner) = next_edge.face().as_inner() {
                if face_set.insert(inner.fix()) {
                    list.push(next_edge.prev().rev());
                    list.push(next_edge.next().rev());
                }
            }
        }

        if next_todo_list.is_empty() {
            break;
        }
        core::mem::swap(&mut inner_faces, &mut outer_faces);
        core::mem::swap(&mut next_todo_list, &mut current_todo_list);

        return_outer_faces = !return_outer_faces;
    }

    if return_outer_faces {
        outer_faces
    } else {
        inner_faces
    }
}
