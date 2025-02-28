use core::sync::atomic::{AtomicUsize, Ordering};

use crate::vendored_spade::{
    DelaunayTriangulation, HasPosition, Point2, SpadeNum, Triangulation, TriangulationExt,
};

use super::FixedVertexHandle;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::vec::Vec;

pub trait HintGenerator<S: SpadeNum>: Default {
    fn get_hint(&self, position: Point2<S>) -> FixedVertexHandle;

    fn notify_vertex_lookup(&self, vertex: FixedVertexHandle);
    fn notify_vertex_inserted(&mut self, vertex: FixedVertexHandle, vertex_position: Point2<S>);
    fn notify_vertex_removed(
        &mut self,
        swapped_in_point: Option<Point2<S>>,
        vertex: FixedVertexHandle,
        vertex_position: Point2<S>,
    );

    fn initialize_from_triangulation<TR, V>(triangulation: &TR) -> Self
    where
        TR: Triangulation<Vertex = V>,
        V: HasPosition<Scalar = S>;
}

#[derive(Default, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub struct LastUsedVertexHintGenerator {
    // Serde does not implement `(De)Serialize` for `AtomicUsize` in no_std environments.
    #[cfg_attr(feature = "serde", serde(skip))]
    index: AtomicUsize,
}

impl Clone for LastUsedVertexHintGenerator {
    fn clone(&self) -> Self {
        Self {
            index: AtomicUsize::new(self.index.load(Ordering::Relaxed)),
        }
    }
}

impl<S: SpadeNum> HintGenerator<S> for LastUsedVertexHintGenerator {
    fn get_hint(&self, _: Point2<S>) -> FixedVertexHandle {
        FixedVertexHandle::new(self.index.load(Ordering::Relaxed))
    }

    fn notify_vertex_lookup(&self, vertex: FixedVertexHandle) {
        self.index.store(vertex.index(), Ordering::Relaxed);
    }

    fn notify_vertex_inserted(&mut self, vertex: FixedVertexHandle, _: Point2<S>) {
        <Self as HintGenerator<S>>::notify_vertex_lookup(self, vertex);
    }

    fn notify_vertex_removed(
        &mut self,
        _swapped_in_point: Option<Point2<S>>,
        vertex: FixedVertexHandle,
        _vertex_position: Point2<S>,
    ) {
        // Use the previous vertex handle as next hint. This should be a good hint if vertices
        // were inserted in local batches.
        let hint = FixedVertexHandle::new(vertex.index().saturating_sub(1));
        <Self as HintGenerator<S>>::notify_vertex_lookup(self, hint);
    }

    fn initialize_from_triangulation<TR, V>(_: &TR) -> Self
    where
        TR: Triangulation,
        V: HasPosition<Scalar = S>,
    {
        Self::default()
    }
}

pub type HierarchyHintGenerator<S> = HierarchyHintGeneratorWithBranchFactor<S, 16>;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
#[doc(hidden)]
pub struct HierarchyHintGeneratorWithBranchFactor<S: SpadeNum, const BRANCH_FACTOR: u32> {
    hierarchy: Vec<DelaunayTriangulation<Point2<S>>>,
    num_elements_of_base_triangulation: usize,
}

impl<S: SpadeNum, const BRANCH_FACTOR: u32> Default
    for HierarchyHintGeneratorWithBranchFactor<S, BRANCH_FACTOR>
{
    fn default() -> Self {
        Self {
            hierarchy: Vec::new(),
            num_elements_of_base_triangulation: 0,
        }
    }
}

impl<S: SpadeNum, const BRANCH_FACTOR: u32> HintGenerator<S>
    for HierarchyHintGeneratorWithBranchFactor<S, BRANCH_FACTOR>
{
    fn get_hint(&self, position: Point2<S>) -> FixedVertexHandle {
        let mut nearest = FixedVertexHandle::new(0);
        for layer in self.hierarchy.iter().rev().skip(1) {
            nearest = layer.walk_to_nearest_neighbor(nearest, position).fix();
            let hint_generator: &LastUsedVertexHintGenerator = layer.hint_generator();
            <LastUsedVertexHintGenerator as HintGenerator<S>>::notify_vertex_lookup(
                hint_generator,
                nearest,
            );
            nearest = FixedVertexHandle::new(nearest.index() * BRANCH_FACTOR as usize);
        }
        nearest
    }

    fn notify_vertex_lookup(&self, _: FixedVertexHandle) {}

    fn notify_vertex_inserted(&mut self, vertex: FixedVertexHandle, vertex_position: Point2<S>) {
        self.num_elements_of_base_triangulation += 1;

        // Find first layer to insert into. Insert into all higher layers.
        let mut index = vertex.index() as u32;

        let mut remainder = 0;
        for triangulation in &mut self.hierarchy {
            remainder = index % BRANCH_FACTOR;
            index /= BRANCH_FACTOR;

            if remainder == 0 {
                triangulation.insert(vertex_position).unwrap();
            } else {
                break;
            }
        }

        if remainder == 0 {
            let mut new_layer = DelaunayTriangulation::new();
            let position_of_vertex_0 = self
                .hierarchy
                .first()
                .map(|layer| layer.vertex(FixedVertexHandle::new(0)).position())
                .unwrap_or(vertex_position);
            new_layer.insert(position_of_vertex_0).unwrap();
            self.hierarchy.push(new_layer);
        }
    }

    fn notify_vertex_removed(
        &mut self,
        mut swapped_in_point: Option<Point2<S>>,
        vertex: FixedVertexHandle,
        _vertex_position: Point2<S>,
    ) {
        let index = vertex.index() as u32;

        let mut current_divisor = BRANCH_FACTOR;
        self.num_elements_of_base_triangulation -= 1;
        let mut last_layer_size = self.num_elements_of_base_triangulation;

        for triangulation in &mut self.hierarchy {
            let remainder = index % current_divisor;
            let index_to_remove = index / current_divisor;
            current_divisor *= BRANCH_FACTOR;

            if remainder == 0 {
                // The current handle is part of this layer and must be removed.
                if let Some(swapped_point) = swapped_in_point.as_ref() {
                    if (triangulation.num_vertices() - 1) * (BRANCH_FACTOR as usize)
                        != last_layer_size
                    {
                        // Only insert a new element if the swapped element is not already present
                        // in the layer
                        triangulation.insert(*swapped_point).unwrap();
                    }
                }
                triangulation.remove(FixedVertexHandle::new(index_to_remove as usize));
            }

            let prev_num_vertices = last_layer_size as u32;
            // Divide by BRANCH_FACTOR and round up
            let max_num_vertices = (prev_num_vertices + BRANCH_FACTOR - 1) / BRANCH_FACTOR;
            if triangulation.num_vertices() as u32 > max_num_vertices {
                // The layer contains too many elements. Remove the last.
                let vertex_to_pop = FixedVertexHandle::new(triangulation.num_vertices() - 1);
                swapped_in_point = None;
                triangulation.remove(vertex_to_pop);
            }

            last_layer_size = triangulation.num_vertices();
        }

        if let [.., ref before_last, _] = self.hierarchy.as_slice() {
            if before_last.num_vertices() == 1 {
                // Last layer has become irrelevant
                self.hierarchy.pop();
            }
        }
    }

    fn initialize_from_triangulation<TR, V>(triangulation: &TR) -> Self
    where
        TR: Triangulation<Vertex = V>,
        V: HasPosition<Scalar = S>,
    {
        let mut result = Self::default();
        for vertex in triangulation.vertices() {
            result.notify_vertex_inserted(vertex.fix(), vertex.position());
        }
        result
    }
}
