use num_traits::Float;

use crate::vendored_spade::delaunay_core::iterators::HullIterator;
use crate::vendored_spade::delaunay_core::InnerOuterMarker;
use crate::vendored_spade::flood_fill_iterator::CircleMetric;
use crate::vendored_spade::flood_fill_iterator::EdgesInShapeIterator;
use crate::vendored_spade::flood_fill_iterator::FloodFillIterator;
use crate::vendored_spade::flood_fill_iterator::RectangleMetric;
use crate::vendored_spade::flood_fill_iterator::VerticesInShapeIterator;
use crate::vendored_spade::iterators::*;
use crate::vendored_spade::Barycentric;
use crate::vendored_spade::HintGenerator;
use crate::vendored_spade::{delaunay_core::Dcel, handles::*};
use crate::vendored_spade::{HasPosition, InsertionError, Point2, TriangulationExt};

use std::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum PositionInTriangulation {
    OnVertex(FixedVertexHandle),

    OnEdge(FixedDirectedEdgeHandle),

    OnFace(FixedFaceHandle<InnerTag>),

    OutsideOfConvexHull(FixedDirectedEdgeHandle),

    NoTriangulation,
}

pub trait Triangulation: Default {
    type Vertex: HasPosition;

    type DirectedEdge: Default;

    type UndirectedEdge: Default;

    type Face: Default;

    type HintGenerator: HintGenerator<<Self::Vertex as HasPosition>::Scalar>;

    #[doc(hidden)]
    fn s(&self) -> &Dcel<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>;

    #[doc(hidden)]
    fn s_mut(
        &mut self,
    ) -> &mut Dcel<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>;

    #[doc(hidden)]
    fn is_defined_legal(&self, _: FixedUndirectedEdgeHandle) -> bool {
        false
    }

    #[doc(hidden)]
    fn handle_legal_edge_split(&mut self, _: [FixedDirectedEdgeHandle; 2]) {}

    #[doc(hidden)]
    fn hint_generator(&self) -> &Self::HintGenerator;

    #[doc(hidden)]
    fn hint_generator_mut(&mut self) -> &mut Self::HintGenerator;

    #[doc(hidden)]
    fn from_parts(
        dcel: Dcel<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>,
        hint_generator: Self::HintGenerator,
        num_constraints: usize,
    ) -> Self;

    #[doc(hidden)]
    #[allow(clippy::type_complexity)]
    fn into_parts(
        self,
    ) -> (
        Dcel<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>,
        Self::HintGenerator,
        usize,
    );

    // // The third point will generate the first inner face!

    fn new() -> Self {
        Self::default()
    }

    fn with_capacity(num_vertices: usize, num_undirected_edges: usize, num_faces: usize) -> Self {
        let mut result = Self::new();
        result
            .s_mut()
            .reserve_capacity(num_vertices, num_undirected_edges, num_faces);
        result
    }

    fn clear(&mut self) {
        self.s_mut().clear();
        let new_hint_generator = HintGenerator::initialize_from_triangulation(self);
        *self.hint_generator_mut() = new_hint_generator;
    }

    fn bulk_load(elements: Vec<Self::Vertex>) -> Result<Self, InsertionError> {
        let mut result: Self = crate::vendored_spade::delaunay_core::bulk_load(elements)?;
        *result.hint_generator_mut() = Self::HintGenerator::initialize_from_triangulation(&result);
        Ok(result)
    }

    fn vertex(
        &self,
        handle: FixedVertexHandle,
    ) -> VertexHandle<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face> {
        self.s().vertex(handle)
    }

    fn vertex_data_mut(&mut self, handle: FixedVertexHandle) -> &mut Self::Vertex {
        self.s_mut().vertex_data_mut(handle)
    }

    fn face<InnerOuter: InnerOuterMarker>(
        &self,
        handle: FixedFaceHandle<InnerOuter>,
    ) -> FaceHandle<InnerOuter, Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>
    {
        self.s().face(handle)
    }

    fn outer_face(
        &self,
    ) -> FaceHandle<
        PossiblyOuterTag,
        Self::Vertex,
        Self::DirectedEdge,
        Self::UndirectedEdge,
        Self::Face,
    > {
        self.s().outer_face()
    }

    fn directed_edge(
        &self,
        handle: FixedDirectedEdgeHandle,
    ) -> DirectedEdgeHandle<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>
    {
        DirectedEdgeHandle::new(self.s(), handle)
    }

    fn undirected_edge(
        &self,
        handle: FixedUndirectedEdgeHandle,
    ) -> UndirectedEdgeHandle<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>
    {
        UndirectedEdgeHandle::new(self.s(), handle)
    }

    fn undirected_edge_data_mut(
        &mut self,
        handle: FixedUndirectedEdgeHandle,
    ) -> &mut Self::UndirectedEdge {
        self.s_mut().undirected_edge_data_mut(handle)
    }

    fn num_all_faces(&self) -> usize {
        self.s().num_faces()
    }

    fn num_inner_faces(&self) -> usize {
        self.s().num_faces() - 1
    }

    fn num_undirected_edges(&self) -> usize {
        self.s().num_undirected_edges()
    }

    fn num_directed_edges(&self) -> usize {
        self.s().num_directed_edges()
    }

    fn convex_hull_size(&self) -> usize {
        if self.all_vertices_on_line() {
            self.num_directed_edges()
        } else {
            let num_inner_edges = self.num_inner_faces() * 3;
            self.num_directed_edges() - num_inner_edges
        }
    }

    fn directed_edges(
        &self,
    ) -> DirectedEdgeIterator<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>
    {
        self.s().directed_edges()
    }

    fn undirected_edges(
        &self,
    ) -> UndirectedEdgeIterator<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>
    {
        self.s().undirected_edges()
    }

    fn num_vertices(&self) -> usize {
        self.s().num_vertices()
    }

    fn vertices(
        &self,
    ) -> VertexIterator<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face> {
        self.s().vertices()
    }

    fn fixed_vertices(&self) -> FixedVertexIterator {
        self.s().fixed_vertices()
    }

    #[allow(clippy::type_complexity)]
    fn get_vertex(
        &self,
        handle: FixedVertexHandle,
    ) -> Option<VertexHandle<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>>
    {
        self.s().get_vertex(handle)
    }

    fn all_faces(
        &self,
    ) -> FaceIterator<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face> {
        self.s().faces()
    }

    //

    fn inner_faces(
        &self,
    ) -> InnerFaceIterator<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face> {
        self.s().inner_faces()
    }

    fn voronoi_faces(
        &self,
    ) -> VoronoiFaceIterator<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>
    {
        VoronoiFaceIterator::new(self.s())
    }

    fn directed_voronoi_edges(
        &self,
    ) -> DirectedVoronoiEdgeIterator<
        Self::Vertex,
        Self::DirectedEdge,
        Self::UndirectedEdge,
        Self::Face,
    > {
        DirectedVoronoiEdgeIterator::new(self.s())
    }

    fn undirected_voronoi_edges(
        &self,
    ) -> UndirectedVoronoiEdgeIterator<
        Self::Vertex,
        Self::DirectedEdge,
        Self::UndirectedEdge,
        Self::Face,
    > {
        UndirectedVoronoiEdgeIterator::new(self.s())
    }

    fn locate(
        &self,
        point: Point2<<Self::Vertex as HasPosition>::Scalar>,
    ) -> PositionInTriangulation {
        let hint = self.hint_generator().get_hint(point);
        self.locate_with_hint_option_core(point, Some(hint))
    }

    #[allow(clippy::type_complexity)]
    fn locate_vertex(
        &self,
        point: Point2<<Self::Vertex as HasPosition>::Scalar>,
    ) -> Option<VertexHandle<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>>
    {
        match self.locate(point) {
            PositionInTriangulation::OnVertex(vertex) => Some(self.vertex(vertex)),
            _ => None,
        }
    }

    #[allow(clippy::type_complexity)]
    fn get_edge_from_neighbors(
        &self,
        from: FixedVertexHandle,
        to: FixedVertexHandle,
    ) -> Option<
        DirectedEdgeHandle<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face>,
    > {
        self.s().get_edge_from_neighbors(from, to)
    }

    fn locate_with_hint(
        &self,
        point: Point2<<Self::Vertex as HasPosition>::Scalar>,
        hint: FixedVertexHandle,
    ) -> PositionInTriangulation {
        self.locate_with_hint_option_core(point, Some(hint))
    }

    fn insert_with_hint(
        &mut self,
        t: Self::Vertex,
        hint: FixedVertexHandle,
    ) -> Result<FixedVertexHandle, InsertionError> {
        self.insert_with_hint_option(t, Some(hint))
    }

    fn locate_and_remove(
        &mut self,
        point: Point2<<Self::Vertex as HasPosition>::Scalar>,
    ) -> Option<Self::Vertex> {
        match self.locate_with_hint_option_core(point, None) {
            PositionInTriangulation::OnVertex(handle) => Some(self.remove_and_notify(handle)),
            _ => None,
        }
    }

    fn remove(&mut self, vertex: FixedVertexHandle) -> Self::Vertex {
        self.remove_and_notify(vertex)
    }

    fn insert(&mut self, vertex: Self::Vertex) -> Result<FixedVertexHandle, InsertionError> {
        self.insert_with_hint_option(vertex, None)
    }

    fn fixed_undirected_edges(&self) -> FixedUndirectedEdgeIterator {
        FixedUndirectedEdgeIterator::new(self.num_undirected_edges())
    }

    fn fixed_directed_edges(&self) -> FixedDirectedEdgeIterator {
        FixedDirectedEdgeIterator::new(self.num_directed_edges())
    }

    fn fixed_all_faces(&self) -> FixedFaceIterator {
        FixedFaceIterator::new(self.num_all_faces())
    }

    fn fixed_inner_faces(&self) -> FixedInnerFaceIterator {
        let mut result = FixedInnerFaceIterator::new(self.num_all_faces());
        result.next();
        result
    }

    fn all_vertices_on_line(&self) -> bool {
        self.num_all_faces() == 1
    }

    fn convex_hull(
        &self,
    ) -> HullIterator<Self::Vertex, Self::DirectedEdge, Self::UndirectedEdge, Self::Face> {
        {
            HullIterator::new(self.s())
        }
    }

    fn face_data_mut<InnerOuter: InnerOuterMarker>(
        &mut self,
        handle: FixedFaceHandle<InnerOuter>,
    ) -> &mut Self::Face {
        self.s_mut().face_data_mut(handle)
    }

    fn directed_edge_data_mut(
        &mut self,
        handle: FixedDirectedEdgeHandle,
    ) -> &mut Self::DirectedEdge {
        self.s_mut().directed_edge_data_mut(handle)
    }
}

pub trait FloatTriangulation: Triangulation
where
    <Self::Vertex as HasPosition>::Scalar: Float,
{
    fn get_edges_in_rectangle(
        &self,
        lower: Point2<<Self::Vertex as HasPosition>::Scalar>,
        upper: Point2<<Self::Vertex as HasPosition>::Scalar>,
    ) -> EdgesInShapeIterator<Self, RectangleMetric<<Self::Vertex as HasPosition>::Scalar>> {
        let distance_metric = RectangleMetric::new(lower, upper);
        let center = lower.add(upper).mul(0.5f32.into());
        EdgesInShapeIterator {
            inner_iter: FloodFillIterator::new(self, distance_metric, center),
        }
    }

    fn get_edges_in_circle(
        &self,
        center: Point2<<Self::Vertex as HasPosition>::Scalar>,
        radius_2: <Self::Vertex as HasPosition>::Scalar,
    ) -> EdgesInShapeIterator<Self, CircleMetric<<Self::Vertex as HasPosition>::Scalar>> {
        let metric = CircleMetric::new(center, radius_2);
        EdgesInShapeIterator {
            inner_iter: FloodFillIterator::new(self, metric, center),
        }
    }

    fn get_vertices_in_rectangle(
        &self,
        lower: Point2<<Self::Vertex as HasPosition>::Scalar>,
        upper: Point2<<Self::Vertex as HasPosition>::Scalar>,
    ) -> VerticesInShapeIterator<Self, RectangleMetric<<Self::Vertex as HasPosition>::Scalar>> {
        let distance_metric = RectangleMetric::new(lower, upper);
        let center = lower.add(upper).mul(0.5f32.into());

        VerticesInShapeIterator::new(FloodFillIterator::new(self, distance_metric, center))
    }

    fn get_vertices_in_circle(
        &self,
        center: Point2<<Self::Vertex as HasPosition>::Scalar>,
        radius_2: <Self::Vertex as HasPosition>::Scalar,
    ) -> VerticesInShapeIterator<Self, CircleMetric<<Self::Vertex as HasPosition>::Scalar>> {
        let distance_metric = CircleMetric::new(center, radius_2);

        VerticesInShapeIterator::new(FloodFillIterator::new(self, distance_metric, center))
    }

    fn barycentric(&self) -> Barycentric<Self> {
        Barycentric::new(self)
    }
}

impl<T> FloatTriangulation for T
where
    T: Triangulation,
    <T::Vertex as HasPosition>::Scalar: Float,
{
}
