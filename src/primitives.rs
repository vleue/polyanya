use std::ops::RangeInclusive;

use smallvec::SmallVec;
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polygon {
    /// List of vertex making this polygon, in counter clockwise order.
    pub vertices: Vec<u32>,
    /// A one way polygon is a polygon that has only one neighbour. This is used to speed up
    /// path finding, but not mandatory.
    pub is_one_way: bool,
}

impl Polygon {
    /// An empty polygon.
    ///
    /// It can be used as a building block, or to use an unreachable spot in the polygon list.
    pub const EMPTY: Polygon = Polygon {
        vertices: vec![],
        is_one_way: false,
    };

    /// Creates a new `Polygon`.
    pub fn new(vertices: Vec<u32>, is_one_way: bool) -> Polygon {
        Polygon {
            vertices,
            is_one_way,
        }
    }

    pub(crate) fn using(nb: usize, data: Vec<isize>) -> Self {
        assert!(data.len() == nb * 2);
        let (vertices, neighbours) = data.split_at(nb);
        let vertices = vertices.iter().copied().map(|v| v as u32).collect();
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
        }
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    pub(crate) fn edges_index(&self) -> SmallVec<[(u32, u32); 10]> {
        let mut edges = SmallVec::with_capacity(self.vertices.len());
        let mut last = self.vertices[0];
        for vertex in self.vertices.iter().skip(1) {
            edges.push((last, *vertex));
            last = *vertex;
        }
        edges.push((last, self.vertices[0]));
        edges
    }

    #[cfg(test)]
    pub(crate) fn double_edges_index(&self) -> SmallVec<[(u32, u32); 20]> {
        use smallvec::smallvec;
        let mut edges = smallvec![(u32::MAX, u32::MAX); self.vertices.len() * 2];
        let mut last = self.vertices[0];
        for (i, vertex) in self.vertices.iter().skip(1).enumerate() {
            edges[i] = (last, *vertex);
            edges[i + self.vertices.len()] = (last, *vertex);
            last = *vertex;
        }
        edges[self.vertices.len() - 1] = (last, self.vertices[0]);
        edges[self.vertices.len() * 2 - 1] = (last, self.vertices[0]);
        edges
    }

    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    pub(crate) fn circular_edges_index(
        &self,
        bounds: RangeInclusive<usize>,
    ) -> SmallVec<[(u32, u32); 10]> {
        let mut edges = SmallVec::with_capacity(self.vertices.len());
        if *bounds.start() < self.vertices.len() {
            let mut last = self.vertices[*bounds.start() % self.vertices.len()];
            for vertex in self.vertices.iter().skip(1 + bounds.start()) {
                edges.push((last, *vertex));
                last = *vertex;
            }
            edges.push((last, self.vertices[0]));
        }
        if *bounds.end() + 1 > self.vertices.len() {
            let start = bounds.start().saturating_sub(self.vertices.len());
            let mut last = self.vertices[0.max(start)];
            for vertex in self.vertices.iter().skip(0.max(start) + 1).take(
                bounds
                    .end()
                    .saturating_sub(self.vertices.len().max(*bounds.start())),
            ) {
                edges.push((last, *vertex));
                last = *vertex;
            }
            edges.push((
                last,
                self.vertices[(*bounds.end() + 1) % self.vertices.len()],
            ));
        }

        edges
    }
}

#[cfg(test)]
mod tests {
    use crate::Polygon;

    #[test]
    fn edges_between() {
        let polygon = Polygon {
            vertices: vec![0, 1, 2, 3],
            is_one_way: false,
        };

        for start in 0..6 {
            for end in (start.max(2) + 1)..8 {
                eprintln!("{} -> {}", start, end);
                assert_eq!(
                    polygon.double_edges_index()[start..=end],
                    polygon.circular_edges_index(start..=end)[..]
                );
            }
        }
    }
}
