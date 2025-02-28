use super::delaunay_core::Dcel;
use crate::vendored_spade::{
    delaunay_core::bulk_load, handles::VertexHandle, HasPosition, HintGenerator, InsertionError,
    LastUsedVertexHintGenerator, NaturalNeighbor, Point2, Triangulation, TriangulationExt,
};

use num_traits::Float;
use std::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[doc(alias = "Voronoi")]
#[doc(alias = "Voronoi diagram")]
#[doc(alias = "Delaunay")]
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub struct DelaunayTriangulation<V, DE = (), UE = (), F = (), L = LastUsedVertexHintGenerator>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    pub(crate) dcel: Dcel<V, DE, UE, F>,
    pub(crate) hint_generator: L,
}

impl<V, DE, UE, F, L> DelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    pub fn nearest_neighbor(
        &self,
        position: Point2<<V as HasPosition>::Scalar>,
    ) -> Option<VertexHandle<V, DE, UE, F>> {
        if self.num_vertices() == 0 {
            return None;
        }

        let hint = self.hint_generator().get_hint(position);
        let hint = self.validate_vertex_handle(hint);

        let vertex = self.walk_to_nearest_neighbor(hint, position);
        self.hint_generator().notify_vertex_lookup(vertex.fix());
        Some(vertex)
    }

    pub fn bulk_load_stable(elements: Vec<V>) -> Result<Self, InsertionError> {
        let mut result: Self = crate::vendored_spade::delaunay_core::bulk_load_stable::<
            _,
            _,
            DelaunayTriangulation<_, _, _, _, _>,
            _,
        >(bulk_load, elements)?;
        *result.hint_generator_mut() = L::initialize_from_triangulation(&result);
        Ok(result)
    }
}

impl<V, DE, UE, F, L> Default for DelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    fn default() -> Self {
        Self {
            dcel: Default::default(),
            hint_generator: Default::default(),
        }
    }
}

impl<V, DE, UE, F, L> DelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    V::Scalar: Float,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    pub fn natural_neighbor(&self) -> NaturalNeighbor<Self> {
        NaturalNeighbor::new(self)
    }
}

impl<V, DE, UE, F, L> Triangulation for DelaunayTriangulation<V, DE, UE, F, L>
where
    V: HasPosition,
    DE: Default,
    UE: Default,
    F: Default,
    L: HintGenerator<<V as HasPosition>::Scalar>,
{
    type Vertex = V;
    type DirectedEdge = DE;
    type UndirectedEdge = UE;
    type Face = F;
    type HintGenerator = L;

    fn s(&self) -> &Dcel<V, DE, UE, F> {
        &self.dcel
    }

    fn s_mut(&mut self) -> &mut Dcel<V, DE, UE, F> {
        &mut self.dcel
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
        assert_eq!(num_constraints, 0);
        Self {
            dcel,
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
        (self.dcel, self.hint_generator, 0)
    }
}
