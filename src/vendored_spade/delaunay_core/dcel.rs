use super::handles::handle_defs::FixedHandleImpl;
use super::handles::iterators::*;
use super::handles::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::vec::Vec;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct EdgeData<DE, UE> {
    directed_data: [DE; 2],
    pub undirected_data: UE,
}

impl<DE, UE> EdgeData<DE, UE> {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub(super) struct FaceEntry<F> {
    pub(super) adjacent_edge: Option<FixedDirectedEdgeHandle>,
    pub(super) data: F,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub(super) struct VertexEntry<V> {
    pub(super) data: V,
    pub(super) out_edge: Option<FixedDirectedEdgeHandle>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub(super) struct EdgeEntry<DE, UE> {
    pub entries: [HalfEdgeEntry; 2],
    pub directed_data: [DE; 2],
    pub undirected_data: UE,
}

impl<DE, UE> EdgeEntry<DE, UE>
where
    DE: Default,
    UE: Default,
{
    pub(super) fn new(normalized: HalfEdgeEntry, not_normalized: HalfEdgeEntry) -> Self {
        EdgeEntry {
            entries: [normalized, not_normalized],
            directed_data: Default::default(),
            undirected_data: Default::default(),
        }
    }
}

impl<DE, UE> EdgeEntry<DE, UE> {
    pub fn get_directed_data<InnerOuter: InnerOuterMarker>(
        &self,
        handle: FixedHandleImpl<DirectedEdgeTag, InnerOuter>,
    ) -> &DE {
        &self.directed_data[handle.index() & 0x1]
    }

    pub fn get_directed_data_mut<InnerOuter: InnerOuterMarker>(
        &mut self,
        handle: FixedHandleImpl<DirectedEdgeTag, InnerOuter>,
    ) -> &mut DE {
        &mut self.directed_data[handle.index() & 0x1]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub(super) struct HalfEdgeEntry {
    pub next: FixedDirectedEdgeHandle,
    pub prev: FixedDirectedEdgeHandle,
    pub face: FixedFaceHandle<PossiblyOuterTag>,
    pub origin: FixedVertexHandle,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub struct Dcel<V, DE = (), UE = (), F = ()> {
    pub(super) vertices: Vec<VertexEntry<V>>,
    pub(super) faces: Vec<FaceEntry<F>>,
    pub(super) edges: Vec<EdgeEntry<DE, UE>>,
}

impl<V, DE, UE, F> Default for Dcel<V, DE, UE, F>
where
    DE: Default,
    UE: Default,
    F: Default,
{
    fn default() -> Self {
        super::dcel_operations::new()
    }
}

impl<V, DE, UE, F> Dcel<V, DE, UE, F> {
    pub fn reserve_capacity(
        &mut self,
        num_vertices: usize,
        num_undirected_edges: usize,
        num_faces: usize,
    ) {
        self.vertices.reserve(num_vertices);
        self.edges.reserve(num_undirected_edges);
        self.faces.reserve(num_faces);
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.edges.clear();
        self.faces.truncate(1); // Keep outer face
    }

    pub fn get_vertex(&self, handle: FixedVertexHandle) -> Option<VertexHandle<V, DE, UE, F>> {
        (handle.index() < self.vertices.len()).then(|| self.vertex(handle))
    }

    pub fn map_vertices<M, V2>(self, f: M) -> Dcel<V2, DE, UE, F>
    where
        M: Fn(V) -> V2,
    {
        Dcel {
            vertices: self
                .vertices
                .into_iter()
                .map(|vertex_data| VertexEntry {
                    data: f(vertex_data.data),
                    out_edge: vertex_data.out_edge,
                })
                .collect(),
            faces: self.faces,
            edges: self.edges,
        }
    }

    pub fn map_undirected_edges<M, UE2>(self, f: M) -> Dcel<V, DE, UE2, F>
    where
        M: Fn(UE) -> UE2,
    {
        Dcel {
            vertices: self.vertices,
            edges: self
                .edges
                .into_iter()
                .map(|edge_data| EdgeEntry {
                    entries: edge_data.entries,
                    directed_data: edge_data.directed_data,
                    undirected_data: f(edge_data.undirected_data),
                })
                .collect(),
            faces: self.faces,
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_directed_edges(&self) -> usize {
        self.edges.len() * 2
    }

    pub fn num_undirected_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }

    pub fn vertex(&self, handle: FixedVertexHandle) -> VertexHandle<V, DE, UE, F> {
        DynamicHandleImpl::new(self, handle)
    }

    pub fn vertex_out_edge(&self, handle: FixedVertexHandle) -> Option<FixedDirectedEdgeHandle> {
        self.vertices[handle.index()]
            .out_edge
            .map(|e| e.adjust_inner_outer())
    }

    pub fn directed_edge(
        &self,
        handle: FixedDirectedEdgeHandle,
    ) -> DirectedEdgeHandle<V, DE, UE, F> {
        DirectedEdgeHandle::new(self, handle)
    }

    pub fn undirected_edge(
        &self,
        handle: FixedUndirectedEdgeHandle,
    ) -> UndirectedEdgeHandle<V, DE, UE, F> {
        UndirectedEdgeHandle::new(self, handle)
    }

    pub fn outer_face(&self) -> FaceHandle<PossiblyOuterTag, V, DE, UE, F> {
        let outer_face = super::dcel_operations::OUTER_FACE_HANDLE;
        self.face(outer_face)
    }

    pub(super) fn edge_entry<InnerOuter: InnerOuterMarker>(
        &self,
        handle: FixedHandleImpl<UndirectedEdgeTag, InnerOuter>,
    ) -> &EdgeEntry<DE, UE> {
        &self.edges[handle.index()]
    }

    pub(super) fn edge_entry_mut<InnerOuter: InnerOuterMarker>(
        &mut self,
        handle: FixedHandleImpl<UndirectedEdgeTag, InnerOuter>,
    ) -> &mut EdgeEntry<DE, UE> {
        &mut self.edges[handle.index()]
    }

    pub(super) fn half_edge(&self, handle: FixedDirectedEdgeHandle) -> &HalfEdgeEntry {
        let entry = self.edge_entry(handle.as_undirected());
        &entry.entries[handle.normalize_index()]
    }

    pub(super) fn half_edge_mut(&mut self, handle: FixedDirectedEdgeHandle) -> &mut HalfEdgeEntry {
        let entry = self.edge_entry_mut(handle.as_undirected());
        &mut entry.entries[handle.normalize_index()]
    }

    pub fn directed_edge_data(&self, handle: FixedDirectedEdgeHandle) -> &DE {
        self.edge_entry(handle.as_undirected())
            .get_directed_data(handle)
    }

    pub fn directed_edge_data_mut(&mut self, handle: FixedDirectedEdgeHandle) -> &mut DE {
        self.edge_entry_mut(handle.as_undirected())
            .get_directed_data_mut(handle)
    }

    pub fn undirected_edge_data<InnerOuter: InnerOuterMarker>(
        &self,
        handle: FixedHandleImpl<UndirectedEdgeTag, InnerOuter>,
    ) -> &UE {
        &self.edge_entry(handle).undirected_data
    }

    pub fn undirected_edge_data_mut(&mut self, handle: FixedUndirectedEdgeHandle) -> &mut UE {
        &mut self.edge_entry_mut(handle).undirected_data
    }

    pub fn face<InnerOuter: InnerOuterMarker>(
        &self,
        handle: FixedHandleImpl<FaceTag, InnerOuter>,
    ) -> DynamicHandleImpl<V, DE, UE, F, FaceTag, InnerOuter> {
        DynamicHandleImpl::new(self, handle)
    }

    pub fn face_data<InnerOuter: InnerOuterMarker>(
        &self,
        handle: FixedHandleImpl<FaceTag, InnerOuter>,
    ) -> &F {
        &self.faces[handle.index()].data
    }

    pub fn face_data_mut<InnerOuter: InnerOuterMarker>(
        &mut self,
        handle: FixedHandleImpl<FaceTag, InnerOuter>,
    ) -> &mut F {
        &mut self.faces[handle.index()].data
    }

    pub fn face_adjacent_edge<InnerOuter: InnerOuterMarker>(
        &self,
        handle: FixedHandleImpl<FaceTag, InnerOuter>,
    ) -> Option<FixedHandleImpl<DirectedEdgeTag, InnerTag>> {
        self.faces[handle.index()].adjacent_edge
    }

    pub fn vertex_data<InnerOuter: InnerOuterMarker>(
        &self,
        handle: FixedHandleImpl<VertexTag, InnerOuter>,
    ) -> &V {
        &self.vertices[handle.index()].data
    }

    pub fn vertex_data_mut<InnerOuter: InnerOuterMarker>(
        &mut self,
        handle: FixedHandleImpl<VertexTag, InnerOuter>,
    ) -> &mut V {
        &mut self.vertices[handle.index()].data
    }

    pub fn get_edge_from_neighbors(
        &self,
        from: FixedVertexHandle,
        to: FixedVertexHandle,
    ) -> Option<DirectedEdgeHandle<V, DE, UE, F>> {
        let vertex = self.vertex(from);
        for edge in vertex.out_edges() {
            if edge.to().fix() == to.adjust_inner_outer() {
                return Some(edge.adjust_inner_outer());
            }
        }
        None
    }

    pub fn update_vertex<InnerOuter: InnerOuterMarker>(
        &mut self,
        handle: FixedHandleImpl<VertexTag, InnerOuter>,
        data: V,
    ) {
        self.vertices[handle.index()].data = data;
    }

    pub fn directed_edges(&self) -> DirectedEdgeIterator<V, DE, UE, F> {
        DirectedEdgeIterator::new(self)
    }

    pub fn undirected_edges(&self) -> UndirectedEdgeIterator<V, DE, UE, F> {
        UndirectedEdgeIterator::new(self)
    }

    pub fn vertices(&self) -> VertexIterator<V, DE, UE, F> {
        VertexIterator::new(self)
    }

    pub fn fixed_vertices(&self) -> FixedVertexIterator {
        FixedVertexIterator::new(self.num_vertices())
    }

    pub fn faces(&self) -> FaceIterator<V, DE, UE, F> {
        FaceIterator::new(self)
    }

    pub fn inner_faces(&self) -> InnerFaceIterator<V, DE, UE, F> {
        let mut iterator = InnerFaceIterator::new(self);
        iterator.next(); // Skip the outer face
        iterator
    }

    pub fn fixed_faces(&self) -> FixedFaceIterator {
        FixedFaceIterator::new(self.num_faces())
    }

    pub fn swap_vertices(&mut self, v0: FixedVertexHandle, v1: FixedVertexHandle) {
        let out_0 = self.vertices[v0.index()].out_edge;
        let out_1 = self.vertices[v1.index()].out_edge;
        self.vertices.swap(v0.index(), v1.index());

        if let Some(mut out_0) = out_0 {
            loop {
                let out_entry = self.half_edge_mut(out_0);
                if out_entry.origin == v1 {
                    break;
                }

                out_entry.origin = v1;

                out_0 = out_entry.prev.rev();
            }
        }

        if let Some(mut out_1) = out_1 {
            loop {
                let out_entry = self.half_edge_mut(out_1);
                if out_entry.origin == v0 {
                    break;
                }

                out_entry.origin = v0;

                out_1 = out_entry.prev.rev();
            }
        }
    }

    #[cfg(any(test, fuzzing))]
    pub fn sanity_check(&self) {
        if self.num_vertices() <= 1 {
            assert_eq!(self.num_faces(), 1);
            assert_eq!(self.num_undirected_edges(), 0);
            assert!(
                self.faces[super::dcel_operations::OUTER_FACE_HANDLE.index()]
                    .adjacent_edge
                    .is_none()
            );
            return;
        }

        for (index, face) in self.faces.iter().enumerate() {
            assert_eq!(
                self.directed_edge(face.adjacent_edge.unwrap()).face().fix(),
                FixedFaceHandle::new(index)
            );
        }
        for (index, vertex) in self.vertices.iter().enumerate() {
            assert_eq!(
                self.directed_edge(vertex.out_edge.unwrap()).from().fix(),
                FixedVertexHandle::new(index)
            );
        }

        for handle in 0..self.edges.len() {
            let edge = self.directed_edge(FixedDirectedEdgeHandle::new_normalized(handle));
            assert_eq!(edge, edge.next().prev());
            assert_eq!(edge, edge.prev().next());
            assert_eq!(edge, edge.rev().rev());
            if self.num_faces() > 1 {
                assert_ne!(edge.face(), edge.rev().face());
            }
            if !edge.face().is_outer() {
                assert_eq!(edge, edge.next().next().next());
                assert_eq!(edge, edge.prev().prev().prev());
            }
            assert_ne!(edge, edge.next());
            assert_ne!(edge, edge.prev());

            assert_eq!(edge, edge.cw().ccw());
            assert_eq!(edge, edge.ccw().cw());
            assert_eq!(edge.from(), edge.cw().from());
            assert_eq!(edge.from(), edge.ccw().from());
        }
    }
}
