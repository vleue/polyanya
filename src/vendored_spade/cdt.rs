use std::vec;
use std::vec::Vec;

use num_traits::{zero, Float};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::vendored_spade::cdt::GroupEnd::Existing;
use crate::vendored_spade::delaunay_core::dcel_operations::flip_cw;
use crate::vendored_spade::delaunay_core::{bulk_load_cdt, bulk_load_stable};
use crate::vendored_spade::{
    delaunay_core::Dcel, intersection_iterator::LineIntersectionIterator, SpadeNum,
};
use crate::vendored_spade::{handles::*, intersection_iterator::Intersection};
use crate::vendored_spade::{
    DelaunayTriangulation, HasPosition, HintGenerator, InsertionError, LastUsedVertexHintGenerator,
    Point2, Triangulation, TriangulationExt,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub struct CdtEdge<UE>(bool, UE);

impl<UE> CdtEdge<UE> {
    pub fn is_constraint_edge(&self) -> bool {
        self.0
    }

    fn make_constraint_edge(&mut self) {
        assert!(!self.is_constraint_edge());
        self.0 = true;
    }

    pub fn data(&self) -> &UE {
        &self.1
    }

    pub fn data_mut(&mut self) -> &mut UE {
        &mut self.1
    }
}

impl<UE: Default> Default for CdtEdge<UE> {
    fn default() -> Self {
        CdtEdge(false, UE::default())
    }
}

impl<UE> AsRef<UE> for CdtEdge<UE> {
    fn as_ref(&self) -> &UE {
        self.data()
    }
}

impl<UE> AsMut<UE> for CdtEdge<UE> {
    fn as_mut(&mut self) -> &mut UE {
        self.data_mut()
    }
}

#[doc(alias = "CDT")]
#[derive(Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub struct ConstrainedDelaunayTriangulation<
    V,
    DE = (),
    UE = (),
    F = (),
    L = LastUsedVertexHintGenerator,
> where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    dcel: Dcel<V, DE, CdtEdge<UE>, F>,
    num_constraints: usize,
    hint_generator: L,
}

impl<V, DE, UE, F, L> Default for ConstrainedDelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    fn default() -> Self {
        ConstrainedDelaunayTriangulation {
            dcel: Default::default(),
            num_constraints: 0,
            hint_generator: Default::default(),
        }
    }
}

impl<V, DE, UE, F, L> Triangulation for ConstrainedDelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    type Vertex = V;
    type DirectedEdge = DE;
    type UndirectedEdge = CdtEdge<UE>;
    type Face = F;
    type HintGenerator = L;

    fn s(&self) -> &Dcel<V, DE, CdtEdge<UE>, F> {
        &self.dcel
    }

    fn s_mut(&mut self) -> &mut Dcel<V, DE, CdtEdge<UE>, F> {
        &mut self.dcel
    }

    fn is_defined_legal(&self, edge: FixedUndirectedEdgeHandle) -> bool {
        self.is_constraint_edge(edge)
    }

    fn handle_legal_edge_split(&mut self, handles: [FixedDirectedEdgeHandle; 2]) {
        self.num_constraints += 1;
        for handle in handles.iter().map(|e| e.as_undirected()) {
            if !self.is_constraint_edge(handle) {
                self.dcel
                    .undirected_edge_data_mut(handle)
                    .make_constraint_edge();
            }
        }
    }

    fn hint_generator(&self) -> &Self::HintGenerator {
        &self.hint_generator
    }

    fn hint_generator_mut(&mut self) -> &mut Self::HintGenerator {
        &mut self.hint_generator
    }

    fn from_parts(
        dcel: Dcel<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>,
        hint_generator: Self::HintGenerator,
        num_constraints: usize,
    ) -> Self {
        Self {
            dcel,
            num_constraints,
            hint_generator,
        }
    }

    fn into_parts(
        self,
    ) -> (
        Dcel<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>,
        Self::HintGenerator,
        usize,
    ) {
        (self.dcel, self.hint_generator, self.num_constraints)
    }

    fn clear(&mut self) {
        self.num_constraints = 0;
        self.s_mut().clear();
        let new_hint_generator = HintGenerator::initialize_from_triangulation(self);
        *self.hint_generator_mut() = new_hint_generator;
    }
}

impl<V, DE, UE, F, L> From<DelaunayTriangulation<V, DE, UE, F, L>>
    for ConstrainedDelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    fn from(value: DelaunayTriangulation<V, DE, UE, F, L>) -> Self {
        let dcel = value.dcel;
        let s = dcel.map_undirected_edges(|edge| CdtEdge(false, edge));
        let lookup = value.hint_generator;

        ConstrainedDelaunayTriangulation {
            dcel: s,
            num_constraints: 0,
            hint_generator: lookup,
        }
    }
}

impl<V, DE, UE, F, L> ConstrainedDelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    pub fn bulk_load_cdt(vertices: Vec<V>, edges: Vec<[usize; 2]>) -> Result<Self, InsertionError> {
        let mut result = bulk_load_cdt(vertices, edges)?;
        *result.hint_generator_mut() = L::initialize_from_triangulation(&result);
        Ok(result)
    }

    pub fn bulk_load_cdt_stable(
        vertices: Vec<V>,
        edges: Vec<[usize; 2]>,
    ) -> Result<Self, InsertionError> {
        let mut result: Self =
            bulk_load_stable(move |vertices| bulk_load_cdt(vertices, edges), vertices)?;
        *result.hint_generator_mut() = L::initialize_from_triangulation(&result);
        Ok(result)
    }

    pub fn remove(&mut self, vertex: FixedVertexHandle) -> V {
        let num_removed_constraints = self
            .dcel
            .vertex(vertex)
            .out_edges()
            .map(|edge| edge.is_constraint_edge())
            .filter(|b| *b)
            .count();
        self.num_constraints -= num_removed_constraints;
        self.remove_and_notify(vertex)
    }

    pub fn num_constraints(&self) -> usize {
        self.num_constraints
    }

    pub fn is_constraint_edge(&self, edge: FixedUndirectedEdgeHandle) -> bool {
        self.dcel.undirected_edge_data(edge).is_constraint_edge()
    }

    pub fn exists_constraint(&self, from: FixedVertexHandle, to: FixedVertexHandle) -> bool {
        self.get_edge_from_neighbors(from, to)
            .map(|e| e.is_constraint_edge())
            .unwrap_or(false)
    }

    pub fn can_add_constraint(&self, from: FixedVertexHandle, to: FixedVertexHandle) -> bool {
        let line_intersection_iterator = LineIntersectionIterator::new_from_handles(self, from, to);
        !self.contains_any_constraint_edge(line_intersection_iterator)
    }

    pub fn intersects_constraint(
        &self,
        line_from: Point2<V::Scalar>,
        line_to: Point2<V::Scalar>,
    ) -> bool {
        let line_intersection_iterator = LineIntersectionIterator::new(self, line_from, line_to);
        self.contains_any_constraint_edge(line_intersection_iterator)
    }

    fn contains_any_constraint_edge(
        &self,
        mut line_intersection_iterator: LineIntersectionIterator<V, DE, CdtEdge<UE>, F>,
    ) -> bool {
        line_intersection_iterator.any(|intersection| match intersection {
            Intersection::EdgeIntersection(edge) => edge.is_constraint_edge(),
            _ => false,
        })
    }

    pub fn add_constraint_edges(
        &mut self,
        vertices: impl IntoIterator<Item = V>,
        closed: bool,
    ) -> Result<(), InsertionError> {
        let mut iter = vertices.into_iter();
        if let Some(first) = iter.next() {
            let first_handle = self.insert(first)?;
            let mut previous_handle = first_handle;
            let mut current_handle = first_handle;
            for current in iter {
                current_handle = self.insert(current)?;
                self.add_constraint(previous_handle, current_handle);
                previous_handle = current_handle;
            }

            if closed && current_handle != first_handle {
                self.add_constraint(current_handle, first_handle);
            }
        }

        Ok(())
    }

    pub fn add_constraint_edge(&mut self, from: V, to: V) -> Result<bool, InsertionError> {
        let from_handle = self.insert(from)?;
        let to_handle = self.insert(to)?;
        Ok(self.add_constraint(from_handle, to_handle))
    }

    pub fn add_constraint(&mut self, from: FixedVertexHandle, to: FixedVertexHandle) -> bool {
        let initial_num_constraints = self.num_constraints();
        self.try_add_constraint_inner(from, to, |_| panic!("Constraint edges must not intersect."));

        self.num_constraints != initial_num_constraints
    }

    fn resolve_conflict_region(
        &mut self,
        conflict_edges: Vec<FixedDirectedEdgeHandle>,
        target_vertex: FixedVertexHandle,
    ) -> Option<FixedDirectedEdgeHandle> {
        let first = conflict_edges.first()?;

        let mut temporary_constraint_edges = Vec::new();

        let first = self.directed_edge(*first);

        if first.to().fix() == target_vertex {
            // Special case: This function is also used to handle constraint edges that fully
            // _overlap_ an existing edge. This makes it easier to return all created edges in their
            // correct order. This will be designated by a "conflict region" consisting of a single
            // edge that ends at the target vertex (which should not happen otherwise).
            return Some(first.fix());
        }

        // These refer to the two edges that go out of the constraint edge origin initially.
        // They are used below but need to be defined declared here to appease the borrow checker.
        let first_border_edge = first.rev().prev().fix();
        let last_border_edge = first.rev().next().fix();

        // Flip all conflict edges in the input order - see function comment.
        for edge in &conflict_edges {
            flip_cw(self.s_mut(), edge.as_undirected());
        }

        // Small optimization: For the legalization, the algorithm doesn't need to look at edges
        // outside the conflict region. They are known to be already legal.
        // To do so, we will make the border edges that encompass the conflict region into temporary
        // constraint edges. The legalization will then skip them. This is undone later,
        let mut make_temporary_edge = |cdt: &mut Self, edge: FixedUndirectedEdgeHandle| {
            // Exclude edges that are already a constraint - those should remain constraint edges
            // and not be undone later!
            if !cdt.undirected_edge(edge).is_constraint_edge() {
                temporary_constraint_edges.push(edge);
                cdt.undirected_edge_data_mut(edge).make_constraint_edge();
            }
        };

        make_temporary_edge(self, first_border_edge.as_undirected());
        make_temporary_edge(self, last_border_edge.as_undirected());

        let mut current = first_border_edge;

        let mut result = None;

        // Loops around all border edges and adds them to the temporary constraint edge list.
        // `first_border_edge` and `last_border_edge` refer to the two border edges that are
        // initially going out of the constraint edge start (the two left most edges in the first
        // ascii drawing of the function comment).
        while current != last_border_edge.rev() {
            let handle = self.directed_edge(current);
            let fixed = handle.fix();
            let next = handle.next().fix().as_undirected();

            current = handle.ccw().fix();
            if target_vertex == handle.to().fix() {
                // This loop also finds the implicitly created constraint edge and makes it an
                // official constraint edge!
                self.make_constraint_edge(fixed.as_undirected());
                result = Some(fixed);
            }
            make_temporary_edge(self, next);
        }

        self.legalize_edges_after_removal(
            &mut conflict_edges
                .into_iter()
                .map(|edge| edge.as_undirected())
                .collect(),
            |_| false,
        );

        // Undo the previously made temporary constraint edges
        for edge in temporary_constraint_edges {
            self.undirected_edge_data_mut(edge).0 = false;
        }

        result
    }

    pub fn get_conflicting_edges_between_points(
        &self,
        from: Point2<<V as HasPosition>::Scalar>,
        to: Point2<<V as HasPosition>::Scalar>,
    ) -> impl Iterator<Item = DirectedEdgeHandle<V, DE, CdtEdge<UE>, F>> {
        LineIntersectionIterator::new(self, from, to)
            .flat_map(|intersection| intersection.as_edge_intersection())
            .filter(|e| e.is_constraint_edge())
    }

    pub fn get_conflicting_edges_between_vertices(
        &self,
        from: FixedVertexHandle,
        to: FixedVertexHandle,
    ) -> impl Iterator<Item = DirectedEdgeHandle<V, DE, CdtEdge<UE>, F>> {
        LineIntersectionIterator::new_from_handles(self, from, to)
            .flat_map(|intersection| intersection.as_edge_intersection())
            .filter(|e| e.is_constraint_edge())
    }

    fn make_constraint_edge(&mut self, edge: FixedUndirectedEdgeHandle) -> bool {
        if !self.is_constraint_edge(edge) {
            self.dcel
                .undirected_edge_data_mut(edge)
                .make_constraint_edge();
            self.num_constraints += 1;
            true
        } else {
            false
        }
    }

    pub fn try_add_constraint(
        &mut self,
        from: FixedVertexHandle,
        to: FixedVertexHandle,
    ) -> Vec<FixedDirectedEdgeHandle> {
        self.try_add_constraint_inner(from, to, |_| ConflictResolution::Cancel)
    }

    fn try_add_constraint_inner<R>(
        &mut self,
        from: FixedVertexHandle,
        to: FixedVertexHandle,
        mut conflict_resolver: R,
    ) -> Vec<FixedDirectedEdgeHandle>
    where
        R: FnMut(DirectedEdgeHandle<V, DE, CdtEdge<UE>, F>) -> ConflictResolution<V>,
    {
        // Constraint edges are added with a two-pass approach:
        // - First, identify potential constraint edge intersections (conflicts). This must be done
        //   beforehand in case that the caller chooses to `ConflictResolution::Cancel` the
        //   operation - no mutation should have happened at this stage.
        let initial_conflict_regions =
            self.get_conflict_resolutions(from, to, &mut conflict_resolver);
        // - Second, apply the conflict resolutions, e.g. by inserting new split points and by
        //   rotating non-constraint edges that intersect the new constraint edge (see function
        //   `resolve_conflict_region`).
        self.resolve_conflict_groups(to, initial_conflict_regions)
    }

    fn get_conflict_resolutions<R>(
        &mut self,
        from: FixedVertexHandle,
        to: FixedVertexHandle,
        conflict_resolver: &mut R,
    ) -> Vec<InitialConflictRegion<V>>
    where
        R: FnMut(DirectedEdgeHandle<V, DE, CdtEdge<UE>, F>) -> ConflictResolution<V>,
    {
        let mut conflict_groups = Vec::new();
        let mut current_group = Vec::new();
        for intersection in LineIntersectionIterator::new_from_handles(self, from, to) {
            match intersection {
                Intersection::EdgeIntersection(edge) => {
                    if edge.is_constraint_edge() {
                        match conflict_resolver(edge) {
                            ConflictResolution::Cancel => {
                                return Vec::new();
                            }
                            ConflictResolution::Split(new_vertex) => {
                                let group_end = GroupEnd::NewVertex(new_vertex, edge.fix());
                                let conflict_edges = core::mem::take(&mut current_group);
                                conflict_groups.push(InitialConflictRegion {
                                    conflict_edges,
                                    group_end,
                                });
                            }
                        }
                    } else {
                        current_group.push(edge.fix());
                    }
                }
                Intersection::VertexIntersection(v) => {
                    let group_end = Existing(v.fix());
                    let conflict_edges = core::mem::take(&mut current_group);
                    conflict_groups.push(InitialConflictRegion {
                        conflict_edges,
                        group_end,
                    });
                }
                Intersection::EdgeOverlap(edge) => {
                    conflict_groups.push(InitialConflictRegion {
                        conflict_edges: vec![edge.fix()],
                        group_end: Existing(edge.to().fix()),
                    });
                }
            }
        }

        conflict_groups
    }

    fn resolve_conflict_groups(
        &mut self,
        final_vertex: FixedVertexHandle,
        conflict_groups: Vec<InitialConflictRegion<V>>,
    ) -> Vec<FixedDirectedEdgeHandle> {
        let mut constraint_edges = Vec::new();
        let mut last_vertex = None;

        for InitialConflictRegion {
            conflict_edges,
            group_end,
        } in conflict_groups
        {
            let mut last_edge = None;
            let target_vertex = match group_end {
                Existing(v) => v,
                GroupEnd::NewVertex(v, conflict_edge) => {
                    let (new_vertex, [e0, e1]) = self.insert_on_edge(conflict_edge, v);
                    let e1_handle = self.directed_edge(e1);
                    // edge_in / edge_out refer to the edge going into / out of the newly split off
                    // vertex.
                    let edge_out = e1_handle.ccw();
                    let edge_in = e1_handle.cw();

                    if Some(edge_in.to().fix()) == last_vertex {
                        constraint_edges.push(edge_in.fix().rev());
                    }

                    if edge_out.to().fix() == final_vertex {
                        // The edge reaches the target vertex - we're done! However, this can
                        // sometimes omit to make the last edge a constraint. This special case
                        // fixes that issue.
                        last_edge = Some(edge_out.fix());
                    }

                    self.handle_legal_edge_split([e0, e1]);
                    new_vertex
                }
            };

            constraint_edges.extend(self.resolve_conflict_region(conflict_edges, target_vertex));
            constraint_edges.extend(last_edge);

            last_vertex = Some(target_vertex);
        }

        for edge in &constraint_edges {
            self.make_constraint_edge(edge.as_undirected());
        }

        constraint_edges
    }
}

impl<V, DE, UE, F, L> ConstrainedDelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    V::Scalar: Float,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    pub fn add_constraint_and_split<C>(
        &mut self,
        from: FixedVertexHandle,
        to: FixedVertexHandle,
        vertex_constructor: C,
    ) -> Vec<FixedDirectedEdgeHandle>
    where
        C: Fn(Point2<<V as HasPosition>::Scalar>) -> V,
    {
        let from_pos = self.vertex(from).position();
        let to_pos = self.vertex(to).position();

        self.try_add_constraint_inner(from, to, |edge| {
            let [p0, p1] = edge.positions();
            let line_intersection = line_intersection(p0, p1, from_pos, to_pos);
            let new_vertex = vertex_constructor(line_intersection);
            assert_eq!(new_vertex.position(), line_intersection);
            ConflictResolution::Split(new_vertex)
        })
    }
}

enum GroupEnd<V> {
    Existing(FixedVertexHandle),
    NewVertex(V, FixedDirectedEdgeHandle),
}

struct InitialConflictRegion<V> {
    conflict_edges: Vec<FixedDirectedEdgeHandle>,
    group_end: GroupEnd<V>,
}

enum ConflictResolution<V> {
    Cancel,
    Split(V),
}

fn line_intersection<S>(p1: Point2<S>, p2: Point2<S>, p3: Point2<S>, p4: Point2<S>) -> Point2<S>
where
    S: SpadeNum + Float,
{
    let a1 = p2.y - p1.y;
    let b1 = p1.x - p2.x;
    let c1 = a1 * p1.x + b1 * p1.y;

    let a2 = p4.y - p3.y;
    let b2 = p3.x - p4.x;
    let c2 = a2 * p3.x + b2 * p3.y;

    let determinant = a1 * b2 - a2 * b1;

    if determinant == zero() {
        panic!("Lines are parallel");
    }

    let x = (b2 * c1 - b1 * c2) / determinant;
    let y = (a1 * c2 - a2 * c1) / determinant;
    Point2::new(x, y)
}
