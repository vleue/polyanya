use std::ops::RangeInclusive;

use geo::{Area, Coord};
#[cfg(feature = "tracing")]
use tracing::instrument;

use glam::Vec2;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{instance::EdgeSide, layers::Layer, Vec2Helper};

/// A point that lies on an edge of a polygon in the navigation mesh.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vertex {
    /// Coordinates of that point.
    pub coords: Vec2,
    /// Indices of the neighbouring polygons, in a counter clockwise order.
    ///
    /// `u32::MAX` marks a neighbouring spot outside the navigation mesh.
    pub polygons: Vec<u32>,
    /// Is this vertex a corner of one of its polygons?
    pub is_corner: bool,
}

impl Vertex {
    /// Create a new `Vertex`.
    pub fn new(coords: Vec2, polygons: Vec<u32>) -> Self {
        Self {
            coords,
            is_corner: polygons.contains(&u32::MAX),
            polygons,
        }
    }
}

/// A polygon in the navigation mesh.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
        let (vertices, neighbours) = data.split_at(nb);
        let vertices = vertices.iter().copied().map(|v| v as u32).collect();
        let neighbours = neighbours.to_vec();
        let mut found_trav = false;
        // Hack to handle case where there are no neighbours in the file. In
        // this case we want to assume it is not a one way. The correct fix is
        // to update the polyanya file format to include the neighbours.
        let mut is_one_way = !neighbours.is_empty();
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
    pub(crate) fn edges_index(&self) -> impl Iterator<Item = [u32; 2]> + '_ {
        self.vertices
            .windows(2)
            .map(|pair| [pair[0], pair[1]])
            .chain(std::iter::once([
                self.vertices[self.vertices.len() - 1],
                self.vertices[0],
            ]))
    }

    #[cfg(test)]
    pub(crate) fn double_edges_index(&self) -> smallvec::SmallVec<[(u32, u32); 20]> {
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
    ) -> impl Iterator<Item = [u32; 2]> + '_ {
        self.edges_index()
            .chain(self.edges_index())
            .skip(*bounds.start())
            .take(*bounds.end() + 1 - *bounds.start())
    }

    pub(crate) fn area(&self, mesh: &Layer) -> f32 {
        geo::Polygon::new(
            geo::LineString(
                self.vertices
                    .iter()
                    .map(|v| {
                        let c = mesh.vertices[*v as usize].coords;
                        Coord::from((c.x, c.y))
                    })
                    .collect(),
            ),
            vec![],
        )
        .unsigned_area()
    }

    pub(crate) fn contains(&self, mesh: &Layer, point: Vec2) -> bool {
        let closing = vec![
            *self.vertices.last().unwrap(),
            *self.vertices.first().unwrap(),
        ];

        if self
            .vertices
            .windows(2)
            .chain([closing.as_slice()])
            .any(|edge| {
                point.on_segment((
                    mesh.vertices[edge[0] as usize].coords,
                    mesh.vertices[edge[1] as usize].coords,
                ))
            })
        {
            return true;
        }

        if self
            .vertices
            .windows(2)
            .chain([closing.as_slice()])
            .any(|edge| {
                point.side((
                    mesh.vertices[edge[0] as usize].coords,
                    mesh.vertices[edge[1] as usize].coords,
                )) == EdgeSide::Right
            })
        {
            return false;
        }
        return true;
    }

    pub(crate) fn coords(&self, mesh: &Layer) -> Vec<Vec2> {
        self.vertices
            .iter()
            .map(|v| mesh.vertices[*v as usize].coords)
            .collect()
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
                eprintln!("{start} -> {end}");
                assert_eq!(
                    polygon.double_edges_index()[start..=end],
                    polygon
                        .circular_edges_index(start..=end)
                        .map(|[a, b]| (a, b))
                        .collect::<Vec<_>>()
                );
            }
        }
    }
}
