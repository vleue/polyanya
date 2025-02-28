use core::cell::RefCell;

use crate::vendored_spade::{
    delaunay_core::math,
    handles::{FixedDirectedEdgeHandle, FixedVertexHandle},
    DelaunayTriangulation, HasPosition, HintGenerator, Point2, PositionInTriangulation,
    Triangulation,
};
use num_traits::{one, zero, Float};

use std::vec::Vec;

use super::VertexHandle;

#[doc(alias = "Interpolation")]
pub struct NaturalNeighbor<'a, T>
where
    T: Triangulation,
    T::Vertex: HasPosition,
{
    triangulation: &'a T,
    // Various buffers that are used for identifying natural neighbors and their weights. It's significantly faster
    // to clear and re-use these buffers instead of allocating them anew.
    // We also don't run the risk of mutably borrowing them twice (causing a RefCell panic) as the RefCells never leak
    // any of the interpolation methods.
    // Not implementing `Sync` is also fine as the workaround - creating a new `NaturalNeighbor` instance per thread -
    // is very simple.
    inspect_edges_buffer: RefCell<Vec<FixedDirectedEdgeHandle>>,
    natural_neighbor_buffer: RefCell<Vec<FixedDirectedEdgeHandle>>,
    insert_cell_buffer: RefCell<Vec<Point2<<T::Vertex as HasPosition>::Scalar>>>,
    weight_buffer: RefCell<Vec<(FixedVertexHandle, <T::Vertex as HasPosition>::Scalar)>>,
}

#[doc(alias = "Interpolation")]
pub struct Barycentric<'a, T>
where
    T: Triangulation,
{
    triangulation: &'a T,
    weight_buffer: RefCell<Vec<(FixedVertexHandle, <T::Vertex as HasPosition>::Scalar)>>,
}

impl<'a, T> Barycentric<'a, T>
where
    T: Triangulation,
    <T::Vertex as HasPosition>::Scalar: Float,
{
    pub(crate) fn new(triangulation: &'a T) -> Self {
        Self {
            triangulation,
            weight_buffer: Default::default(),
        }
    }

    pub fn get_weights(
        &self,
        position: Point2<<T::Vertex as HasPosition>::Scalar>,
        result: &mut Vec<(FixedVertexHandle, <T::Vertex as HasPosition>::Scalar)>,
    ) {
        result.clear();
        match self.triangulation.locate(position) {
            PositionInTriangulation::OnVertex(vertex) => {
                result.push((vertex, <T::Vertex as HasPosition>::Scalar::from(1.0)))
            }
            PositionInTriangulation::OnEdge(edge) => {
                let [v0, v1] = self.triangulation.directed_edge(edge).vertices();
                let [w0, w1] = two_point_interpolation::<T>(v0, v1, position);
                result.push((v0.fix(), w0));
                result.push((v1.fix(), w1));
            }
            PositionInTriangulation::OnFace(face) => {
                let face = self.triangulation.face(face);
                let [v0, v1, v2] = face.vertices();
                let [c0, c1, c2] = face.barycentric_interpolation(position);
                result.extend([(v0.fix(), c0), (v1.fix(), c1), (v2.fix(), c2)]);
            }
            _ => {}
        }
    }

    pub fn interpolate<I>(
        &self,
        i: I,
        position: Point2<<T::Vertex as HasPosition>::Scalar>,
    ) -> Option<<T::Vertex as HasPosition>::Scalar>
    where
        I: Fn(
            VertexHandle<T::Vertex, T::DirectedEdge, T::UndirectedEdge, T::Face>,
        ) -> <T::Vertex as HasPosition>::Scalar,
    {
        let nns = &mut *self.weight_buffer.borrow_mut();
        self.get_weights(position, nns);
        if nns.is_empty() {
            return None;
        }

        let mut total_sum = zero();
        for (vertex, weight) in nns {
            total_sum = total_sum + i(self.triangulation.vertex(*vertex)) * *weight;
        }
        Some(total_sum)
    }
}

impl<'a, V, DE, UE, F, L> NaturalNeighbor<'a, DelaunayTriangulation<V, DE, UE, F, L>>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
    <V as HasPosition>::Scalar: Float,
{
    pub(crate) fn new(triangulation: &'a DelaunayTriangulation<V, DE, UE, F, L>) -> Self {
        Self {
            triangulation,
            inspect_edges_buffer: Default::default(),
            insert_cell_buffer: Default::default(),
            natural_neighbor_buffer: Default::default(),
            weight_buffer: Default::default(),
        }
    }

    pub fn get_weights(
        &self,
        position: Point2<<V as HasPosition>::Scalar>,
        result: &mut Vec<(FixedVertexHandle, <V as HasPosition>::Scalar)>,
    ) {
        let nns = &mut *self.natural_neighbor_buffer.borrow_mut();
        get_natural_neighbor_edges(
            self.triangulation,
            &mut self.inspect_edges_buffer.borrow_mut(),
            position,
            nns,
        );
        self.get_natural_neighbor_weights(position, nns, result);
    }

    pub fn interpolate<I>(
        &self,
        i: I,
        position: Point2<<V as HasPosition>::Scalar>,
    ) -> Option<<V as HasPosition>::Scalar>
    where
        I: Fn(VertexHandle<V, DE, UE, F>) -> <V as HasPosition>::Scalar,
    {
        let nns = &mut *self.weight_buffer.borrow_mut();
        self.get_weights(position, nns);
        if nns.is_empty() {
            return None;
        }

        let mut total_sum = zero();
        for (vertex, weight) in nns {
            total_sum = total_sum + i(self.triangulation.vertex(*vertex)) * *weight;
        }
        Some(total_sum)
    }

    pub fn interpolate_gradient<I, G>(
        &self,
        i: I,
        g: G,
        flatness: <V as HasPosition>::Scalar,
        position: Point2<<V as HasPosition>::Scalar>,
    ) -> Option<<V as HasPosition>::Scalar>
    where
        I: Fn(VertexHandle<V, DE, UE, F>) -> <V as HasPosition>::Scalar,
        G: Fn(VertexHandle<V, DE, UE, F>) -> [<V as HasPosition>::Scalar; 2],
    {
        let nns = &mut *self.weight_buffer.borrow_mut();
        self.get_weights(position, nns);
        if nns.is_empty() {
            return None;
        }

        // Variable names should make more sense after looking into the paper!
        // Roughly speaking, this approach works by blending a smooth c1 approximation into the
        // regular natural neighbor interpolation ("c0 contribution").
        // The c0 / c1 contributions are stored in sum_c0 / sum_c1 and are weighted by alpha and beta
        // respectively.
        let mut sum_c0 = zero();
        let mut sum_c1 = zero();
        let mut sum_c1_weights = zero();
        let mut alpha: <V as HasPosition>::Scalar = zero();
        let mut beta: <V as HasPosition>::Scalar = zero();

        for (handle, weight) in nns {
            let handle = self.triangulation.vertex(*handle);
            let pos_i = handle.position();
            let h_i = i(handle);
            let diff = pos_i.sub(position);
            let r_i2 = diff.length2();
            let r_i = r_i2.powf(flatness);
            let c1_weight_i = *weight / r_i;
            let grad_i = g(handle);
            let zeta_i = h_i + diff.dot(grad_i.into());
            alpha = alpha + c1_weight_i * r_i;
            beta = beta + c1_weight_i * r_i2;
            sum_c1_weights = sum_c1_weights + c1_weight_i;
            sum_c1 = sum_c1 + zeta_i * c1_weight_i;
            sum_c0 = sum_c0 + h_i * *weight;
        }
        alpha = alpha / sum_c1_weights;
        sum_c1 = sum_c1 / sum_c1_weights;
        let result = (alpha * sum_c0 + beta * sum_c1) / (alpha + beta);

        Some(result)
    }

    fn get_natural_neighbor_weights(
        &self,
        position: Point2<<V as HasPosition>::Scalar>,
        nns: &[FixedDirectedEdgeHandle],
        result: &mut Vec<(FixedVertexHandle, <V as HasPosition>::Scalar)>,
    ) {
        result.clear();

        if nns.is_empty() {
            return;
        }

        if nns.len() == 1 {
            let edge = self.triangulation.directed_edge(nns[0]);
            result.push((edge.from().fix(), one()));
            return;
        }

        if nns.len() == 2 {
            let [e0, e1] = [
                self.triangulation.directed_edge(nns[0]),
                self.triangulation.directed_edge(nns[1]),
            ];
            let [v0, v1] = [e0.from(), e1.from()];
            let [w0, w1] =
                two_point_interpolation::<DelaunayTriangulation<V, DE, UE, F>>(v0, v1, position);

            result.push((v0.fix(), w0));
            result.push((v1.fix(), w1));
            return;
        }

        // Get insertion cell vertices. The "insertion cell" refers to the voronoi cell that would be
        // created if "position" would be inserted.
        // These insertion cells happen to lie on the circumcenter of `position` and any two adjacent
        // natural neighbors (e.g. [position, nn[2].position(), nn[3].position()]).
        //
        // `images/natural_neighbor_insertion_cell.svg` depicts the cell as thick orange line.
        let mut insertion_cell = self.insert_cell_buffer.borrow_mut();
        insertion_cell.clear();
        for cur_nn in nns {
            let cur_nn = self.triangulation.directed_edge(*cur_nn);

            let [from, to] = cur_nn.positions();
            insertion_cell.push(math::circumcenter([to, from, position]).0);
        }

        let mut total_area = zero(); // Used to normalize weights at the end

        let mut last_edge = self.triangulation.directed_edge(*nns.last().unwrap());
        let mut last = *insertion_cell.last().unwrap();

        for (stop_edge, first) in core::iter::zip(nns.iter(), &*insertion_cell) {
            // Main loop
            //
            // Refer to images/natural_neighbor_polygon.svg for some visual aid.
            //
            // The outer loops calculates the weight of an individual natural neighbor.
            // To do this, it calculates the intersection area of the insertion cell with the cell of the
            // current natural neighbor.
            // This intersection is a convex polygon with vertices `first, current0, current1 ... last`
            // (ordered ccw, `currentX` refers to the variable in the inner loop)
            //
            // The area of a convex polygon [v0 ... vN] is given by
            // 0.5 * ((v0.x * v1.y + ... vN.x * v0.y) - (v0.y * v1.x + ... vN.y * v0.x))
            //        ⮤       positive_area        ⮥   ⮤         negative_area      ⮥
            //
            // The positive and negative contributions are calculated separately to avoid precision issues.
            // The factor of 0.5 can be omitted as the weights are normalized anyway.

            // `stop_edge` is used to know when to stop the inner loop (= once the polygon is finished)
            let stop_edge = self.triangulation.directed_edge(*stop_edge);
            assert!(!stop_edge.is_outer_edge());

            let mut positive_area = first.x * last.y;
            let mut negative_area = first.y * last.x;

            loop {
                // All other polygon vertices happen lie on the circumcenter of a face adjacent to an
                // out edge of the current natural neighbor.
                //
                // The natural_neighbor_polygon.svg refers to this variable as `c0`, `c1`, and `c2`.
                let current = last_edge.face().as_inner().unwrap().circumcenter();
                positive_area = positive_area + last.x * current.y;
                negative_area = negative_area + last.y * current.x;

                last_edge = last_edge.next().rev();
                last = current;

                if last_edge == stop_edge.rev() {
                    positive_area = positive_area + current.x * first.y;
                    negative_area = negative_area + current.y * first.x;
                    break;
                }
            }

            let polygon_area = positive_area - negative_area;

            total_area = total_area + polygon_area;
            result.push((stop_edge.from().fix(), polygon_area));

            last = *first;
            last_edge = stop_edge;
        }

        for tuple in result {
            tuple.1 = tuple.1 / total_area;
        }
    }
}

fn get_natural_neighbor_edges<T>(
    triangulation: &T,
    inspect_buffer: &mut Vec<FixedDirectedEdgeHandle>,
    position: Point2<<T::Vertex as HasPosition>::Scalar>,
    result: &mut Vec<FixedDirectedEdgeHandle>,
) where
    T: Triangulation,
    <T::Vertex as HasPosition>::Scalar: Float,
{
    inspect_buffer.clear();
    result.clear();
    match triangulation.locate(position) {
        PositionInTriangulation::OnFace(face) => {
            for edge in triangulation
                .face(face)
                .adjacent_edges()
                .into_iter()
                .rev()
                .map(|e| e.rev())
            {
                inspect_flips(triangulation, result, inspect_buffer, edge.fix(), position);
            }
        }
        PositionInTriangulation::OnEdge(edge) => {
            let edge = triangulation.directed_edge(edge);

            if edge.is_part_of_convex_hull() {
                result.extend([edge.fix(), edge.fix().rev()]);
                return;
            }

            for edge in [edge, edge.rev()] {
                inspect_flips(triangulation, result, inspect_buffer, edge.fix(), position);
            }
        }
        PositionInTriangulation::OnVertex(fixed_handle) => {
            let vertex = triangulation.vertex(fixed_handle);
            result.push(
                vertex
                    .out_edge()
                    .map(|e| e.fix())
                    .unwrap_or(FixedDirectedEdgeHandle::new(0)),
            )
        }
        _ => {}
    }
    result.reverse();
}

fn inspect_flips<T>(
    triangulation: &T,
    result: &mut Vec<FixedDirectedEdgeHandle>,
    buffer: &mut Vec<FixedDirectedEdgeHandle>,
    edge_to_validate: FixedDirectedEdgeHandle,
    position: Point2<<T::Vertex as HasPosition>::Scalar>,
) where
    T: Triangulation,
{
    buffer.clear();
    buffer.push(edge_to_validate);

    while let Some(edge) = buffer.pop() {
        let edge = triangulation.directed_edge(edge);

        let v2 = edge.opposite_vertex();
        let v1 = edge.from();

        let mut should_flip = false;

        if let Some(v2) = v2 {
            let v0 = edge.to().position();
            let v1 = v1.position();
            let v3 = position;
            debug_assert!(math::is_ordered_ccw(v2.position(), v1, v0));
            should_flip = math::contained_in_circumference(v2.position(), v1, v0, v3);

            if should_flip {
                let e1 = edge.next().fix().rev();
                let e2 = edge.prev().fix().rev();

                buffer.push(e1);
                buffer.push(e2);
            }
        }

        if !should_flip {
            result.push(edge.fix().rev());
        }
    }
}

fn two_point_interpolation<'a, T>(
    v0: VertexHandle<'a, T::Vertex, T::DirectedEdge, T::UndirectedEdge, T::Face>,
    v1: VertexHandle<'a, T::Vertex, T::DirectedEdge, T::UndirectedEdge, T::Face>,
    position: Point2<<T::Vertex as HasPosition>::Scalar>,
) -> [<T::Vertex as HasPosition>::Scalar; 2]
where
    T: Triangulation,
    <T::Vertex as HasPosition>::Scalar: Float,
{
    let projection = math::project_point(v0.position(), v1.position(), position);
    let rel = projection.relative_position();
    let one: <T::Vertex as HasPosition>::Scalar = 1.0.into();
    [one - rel, rel]
}
