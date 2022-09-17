use smallvec::{smallvec, SmallVec};
#[cfg(feature = "tracing")]
use tracing::instrument;

use glam::Vec2;

/// A point that lies on an edge of a polygon in the navigation mesh.
#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
    /// Coordinates of that point.
    pub coords: Vec2,
    /// Indices of the neighbouring polygons, in a counter clockwise order.
    ///
    /// `-1` marks a neighbouring spot outside the navigation mesh.
    pub polygons: Vec<isize>,
    pub(crate) is_corner: bool,
}

impl Vertex {
    /// Create a new `Vertex`.
    pub fn new(coords: Vec2, polygons: Vec<isize>) -> Self {
        Self {
            coords,
            is_corner: polygons.contains(&-1),
            polygons,
        }
    }
}

/// A polygon in the navigation mesh.
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    /// List of vertex making this polygon, in counter clockwise order.
    pub vertices: Vec<usize>,
    /// A one way polygon is a polygon that has only one neighbour. This is used to speed up
    /// path finding, but not mandatory.
    pub is_one_way: bool,
    pub(crate) aabb: (Vec2, Vec2),
}

impl Polygon {
    /// An empty polygon.
    ///
    /// It can be used as a building block, or to use an unreachable spot in the polygon list.
    pub const EMPTY: Polygon = Polygon {
        vertices: vec![],
        is_one_way: false,
        aabb: (Vec2::ZERO, Vec2::ZERO),
    };

    /// Creates a new `Polygon`.
    pub fn new(vertices: Vec<usize>, is_one_way: bool) -> Polygon {
        Polygon {
            vertices,
            is_one_way,
            aabb: (Vec2::ZERO, Vec2::ZERO),
        }
    }

    pub(crate) fn using(nb: usize, data: Vec<isize>) -> Self {
        assert!(data.len() == nb * 2);
        let (vertices, neighbours) = data.split_at(nb);
        let vertices = vertices.iter().copied().map(|v| v as usize).collect();
        let neighbours = neighbours.to_vec();
        let mut found_trav = false;
        let mut is_one_way = true;
        for neighbour in &neighbours {
            if *neighbour != -1 {
                if found_trav {
                    is_one_way = false;
                    break;
                } else {
                    found_trav = true;
                }
            }
        }
        Polygon {
            vertices,
            is_one_way,
            aabb: (Vec2::ZERO, Vec2::ZERO),
        }
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    pub(crate) fn edges_index(&self) -> SmallVec<[(usize, usize); 10]> {
        let mut edges = SmallVec::new();
        if self.vertices.is_empty() {
            return smallvec![];
        }
        let mut last = self.vertices[0];
        for vertex in self.vertices.iter().skip(1) {
            edges.push((last, *vertex));
            last = *vertex;
        }
        edges.push((last, self.vertices[0]));
        edges
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    pub(crate) fn double_edges_index(&self) -> SmallVec<[(usize, usize); 20]> {
        let mut edges = SmallVec::new();
        let mut last = self.vertices[0];
        for vertex in self.vertices.iter().skip(1) {
            edges.push((last, *vertex));
            last = *vertex;
        }
        edges.push((last, self.vertices[0]));
        let mut last = self.vertices[0];
        for vertex in self.vertices.iter().skip(1) {
            edges.push((last, *vertex));
            last = *vertex;
        }
        edges.push((last, self.vertices[0]));
        edges
    }
}
