//! # Spade
//!
//! Delaunay triangulations for the rust ecosystem.
//!
//! # Features
//! * A 2D Delaunay triangulation: [DelaunayTriangulation]
//! * Uses exact geometric predicate evaluation, preventing construction errors due to precision loss.
//! * A 2D constrained Delaunay triangulation: [ConstrainedDelaunayTriangulation]
//! * Supports vertex removal
//! * Serde support with the `serde` feature.
//! * `no_std` support with `default-features = false`
//! * Natural neighbor interpolation: [NaturalNeighbor]
//!
//! # Cargo features
//!
//! These features are disabled by default and need to be enabled in your `Cargo.toml`:
//!  - `serde`: Enable (de)serialization of (constrained) Delaunay triangulations with the
//!    [Serde](https://docs.rs/serde/latest/serde/) crate
//!  - `mint`: Enables rudimentary [mint](https://docs.rs/mint/latest/mint/) interoperability by
//!    implementing the `From` and `Into` conversion traits between `spade::Point2` and
//!    `mint::Point2`. Also implements [HasPosition] for `mint::Point2`.

#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![deny(rustdoc::broken_intra_doc_links)]

extern crate alloc;

mod cdt;
mod delaunay_core;
mod delaunay_triangulation;
mod flood_fill_iterator;
mod intersection_iterator;
mod point;

mod triangulation;

pub use crate::vendored_spade::cdt::{CdtEdge, ConstrainedDelaunayTriangulation};
pub use crate::vendored_spade::delaunay_triangulation::DelaunayTriangulation;
pub use crate::vendored_spade::point::{HasPosition, Point2, SpadeNum};

pub use crate::vendored_spade::delaunay_core::math::{
    mitigate_underflow, validate_coordinate, validate_vertex, InsertionError, PointProjection,
    MAX_ALLOWED_VALUE, MIN_ALLOWED_VALUE,
};

pub use delaunay_core::{
    AngleLimit, HierarchyHintGenerator, HierarchyHintGeneratorWithBranchFactor, HintGenerator,
    LastUsedVertexHintGenerator, RefinementParameters, RefinementResult,
};

pub use crate::vendored_spade::delaunay_core::interpolation::{Barycentric, NaturalNeighbor};
pub use delaunay_core::LineSideInfo;
pub use intersection_iterator::{Intersection, LineIntersectionIterator};
pub use triangulation::{FloatTriangulation, PositionInTriangulation, Triangulation};

pub(crate) use delaunay_core::TriangulationExt;

pub(crate) use delaunay_core::RemovalResult;

pub mod handles {
    pub use crate::vendored_spade::delaunay_core::{
        DirectedEdgeHandle, DirectedVoronoiEdge, FaceHandle, FixedDirectedEdgeHandle,
        FixedFaceHandle, FixedUndirectedEdgeHandle, FixedVertexHandle, InnerTag, PossiblyOuterTag,
        UndirectedEdgeHandle, UndirectedVoronoiEdge, VertexHandle, VoronoiFace, VoronoiVertex,
        OUTER_FACE,
    };
}

pub mod iterators {
    pub use crate::vendored_spade::delaunay_core::iterators::{
        DirectedEdgeIterator, DirectedVoronoiEdgeIterator, FaceIterator, FixedDirectedEdgeIterator,
        FixedFaceIterator, FixedInnerFaceIterator, FixedUndirectedEdgeIterator,
        FixedVertexIterator, InnerFaceIterator, UndirectedEdgeIterator,
        UndirectedVoronoiEdgeIterator, VertexIterator, VoronoiFaceIterator,
    };
    pub use crate::vendored_spade::flood_fill_iterator::{
        CircleMetric, EdgesInShapeIterator, RectangleMetric, VerticesInShapeIterator,
    };
}

pub mod internals {
    pub use crate::vendored_spade::delaunay_core::{DynamicHandleImpl, FixedHandleImpl};
}
