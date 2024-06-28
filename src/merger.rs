use geo::IsConvex;
#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::Mesh;

impl Mesh {
    /// Merge polygons.
    ///
    /// This merge neighbouring polygons when possible, keeping them convex.
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    pub fn merge_polygons(&mut self) -> bool {
        self.unbake();
        let mut area = self
            .polygons
            .iter()
            .enumerate()
            .map(|(i, poly)| (i, poly.area(self)))
            .collect::<Vec<_>>();
        let mut union_polygons = UnionFind::new(self.polygons.len() as i32);

        area.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap().reverse());

        for (poly_index, _) in area.iter() {
            if union_polygons.find(*poly_index as i32) != *poly_index as i32 {
                // already merged
                continue;
            }
            let poly = &self.polygons[*poly_index];
            for edge in poly.edges_index() {
                let start = if let Some(v) = self.vertices.get(edge.0 as usize) {
                    v
                } else {
                    continue;
                };
                let end = if let Some(v) = self.vertices.get(edge.1 as usize) {
                    v
                } else {
                    continue;
                };
                let other_side = *start
                    .polygons
                    .iter()
                    .find(|i| **i != -1 && **i != *poly_index as isize && end.polygons.contains(*i))
                    .unwrap_or(&-1);
                if other_side == -1 {
                    // nothing on the other side
                    continue;
                }
                if union_polygons.find(other_side as i32) != other_side as i32 {
                    // already merged
                    continue;
                }

                let other_vertices = &self.polygons[other_side as usize].vertices;
                let mut joined_vertices_index =
                    Vec::with_capacity(poly.vertices.len() + other_vertices.len() - 2);
                let mut joined_vertices =
                    Vec::with_capacity(poly.vertices.len() + other_vertices.len() - 1);
                for i in poly
                    .vertices
                    .iter()
                    .chain(poly.vertices.iter())
                    .skip_while(|i| **i != edge.1)
                    .take_while(|i| **i != edge.0)
                {
                    joined_vertices_index.push(*i);
                    let c = self.vertices[*i as usize].coords;
                    joined_vertices.push(geo::Coord { x: c.x, y: c.y });
                }
                for i in other_vertices
                    .iter()
                    .chain(other_vertices.iter())
                    .skip_while(|i| **i != edge.0)
                    .take_while(|i| **i != edge.1)
                {
                    joined_vertices_index.push(*i);
                    let c = self.vertices[*i as usize].coords;
                    joined_vertices.push(geo::Coord { x: c.x, y: c.y });
                }
                let mut line = geo::LineString(joined_vertices);
                line.close();
                if line.is_ccw_convex() {
                    union_polygons.union(*poly_index as i32, other_side as i32);
                    self.polygons[*poly_index].vertices = joined_vertices_index;
                    // TODO: correctly set the value for merged polygon
                    self.polygons[*poly_index].is_one_way = false;
                    break;
                }
            }
        }

        let mut new_indexes = vec![-1; self.polygons.len()];
        let mut kept = 0;
        for (i, p) in union_polygons.parent.iter().enumerate() {
            let p = union_polygons.find(*p);
            if new_indexes[p as usize] == -1 {
                new_indexes[p as usize] = kept;
                kept += 1;
            }

            if i as i32 == p {
                let j = new_indexes[p as usize] as usize;
                if i != j {
                    self.polygons.swap(i, j);
                }
            } else {
                new_indexes[i] = new_indexes[p as usize];
            }
        }
        self.polygons.resize_with(kept as usize, || unreachable!());

        for vertex in self.vertices.iter_mut() {
            for p in vertex.polygons.iter_mut() {
                if *p != -1 {
                    *p = new_indexes[*p as usize];
                }
            }
            vertex.polygons.dedup();
        }
        new_indexes.len() != kept as usize
    }
}

#[derive(Debug)]
pub struct UnionFind {
    pub parent: Vec<i32>,
}

impl UnionFind {
    fn new(length: i32) -> Self {
        Self {
            parent: (0..length).collect(),
        }
    }

    fn find_and_update(&mut self, x: i32) -> i32 {
        if x == -1 {
            return -1;
        }
        let x_parent = self.parent[x as usize];
        if x_parent != x {
            self.parent[x as usize] = self.find_and_update(x_parent);
        }
        self.parent[x as usize]
    }

    fn find(&self, x: i32) -> i32 {
        if x == -1 {
            return -1;
        }
        let x_parent = self.parent[x as usize];
        if x_parent != x {
            self.find(x_parent)
        } else {
            self.parent[x as usize]
        }
    }

    fn union(&mut self, x: i32, y: i32) {
        let x = self.find_and_update(x);
        let y = self.find_and_update(y);
        self.parent[y as usize] = x;
    }
}

#[cfg(test)]
mod test {
    use glam::{vec2, Vec2};

    use crate::{Mesh, Polygon, Triangulation, Vertex};

    fn mesh_u_grid() -> Mesh {
        Mesh {
            vertices: vec![
                Vertex::new(Vec2::new(0., 0.), vec![0, -1]),
                Vertex::new(Vec2::new(1., 0.), vec![0, 1, -1]),
                Vertex::new(Vec2::new(2., 0.), vec![1, 2, -1]),
                Vertex::new(Vec2::new(3., 0.), vec![2, -1]),
                Vertex::new(Vec2::new(0., 1.), vec![3, 0, -1]),
                Vertex::new(Vec2::new(1., 1.), vec![3, 1, 0, -1]),
                Vertex::new(Vec2::new(2., 1.), vec![4, 2, 1, -1]),
                Vertex::new(Vec2::new(3., 1.), vec![4, 2, -1]),
                Vertex::new(Vec2::new(0., 2.), vec![5, 3, -1]),
                Vertex::new(Vec2::new(1., 2.), vec![5, 3, -1]),
                Vertex::new(Vec2::new(2., 2.), vec![6, 4, -1]),
                Vertex::new(Vec2::new(3., 2.), vec![6, 4, -1]),
                Vertex::new(Vec2::new(0., 3.), vec![5, -1]),
                Vertex::new(Vec2::new(1., 3.), vec![5, -1]),
                Vertex::new(Vec2::new(2., 3.), vec![6, -1]),
                Vertex::new(Vec2::new(3., 3.), vec![6, -1]),
            ],
            polygons: vec![
                Polygon::new(vec![0, 1, 5, 4], false),
                Polygon::new(vec![1, 2, 6, 5], false),
                Polygon::new(vec![2, 3, 7, 6], false),
                Polygon::new(vec![4, 5, 9, 8], false),
                Polygon::new(vec![6, 7, 11, 10], false),
                Polygon::new(vec![8, 9, 13, 12], false),
                Polygon::new(vec![10, 11, 15, 14], false),
            ],
            ..Default::default()
        }
    }

    #[test]
    fn merge_u() {
        let mut mesh = mesh_u_grid();
        while mesh.merge_polygons() {}
        mesh.bake();
        assert_eq!(mesh.polygons.len(), 3);
    }

    #[test]
    fn merge_and_path() {
        let mut mesh = mesh_u_grid();
        while mesh.merge_polygons() {}
        mesh.bake();
        assert_eq!(mesh.polygons.len(), 3);
        assert_eq!(
            mesh.polygons[0],
            Polygon::new(vec![5, 4, 0, 1, 2, 6], false)
        );
        assert_eq!(
            mesh.polygons[1],
            Polygon::new(vec![10, 6, 2, 3, 7, 11, 15, 14], false)
        );
        assert_eq!(
            mesh.polygons[2],
            Polygon::new(vec![8, 4, 5, 9, 13, 12], false)
        );
        assert_eq!(
            mesh.vertices[0],
            Vertex::new(Vec2::new(0.0, 0.0), vec![0, -1])
        );
        assert_eq!(
            mesh.vertices[1],
            Vertex::new(Vec2::new(1.0, 0.0), vec![0, -1])
        );
        assert_eq!(
            mesh.vertices[2],
            Vertex::new(Vec2::new(2.0, 0.0), vec![0, 1, -1])
        );
        assert_eq!(
            mesh.vertices[3],
            Vertex::new(Vec2::new(3.0, 0.0), vec![1, -1])
        );
        assert_eq!(
            mesh.vertices[4],
            Vertex::new(Vec2::new(0.0, 1.0), vec![2, 0, -1])
        );
        assert_eq!(
            mesh.vertices[5],
            Vertex::new(Vec2::new(1.0, 1.0), vec![2, 0, -1])
        );
        assert_eq!(
            mesh.vertices[6],
            Vertex::new(Vec2::new(2.0, 1.0), vec![1, 0, -1])
        );
        assert_eq!(
            mesh.vertices[7],
            Vertex::new(Vec2::new(3.0, 1.0), vec![1, -1])
        );
        assert_eq!(
            mesh.vertices[8],
            Vertex::new(Vec2::new(0.0, 2.0), vec![2, -1])
        );
        dbg!(mesh.path(Vec2::new(0.5, 0.5), Vec2::new(2.5, 1.5)));
        dbg!(mesh.path(Vec2::new(0.5, 1.5), Vec2::new(2.5, 1.5)));
    }

    #[test]
    fn merge_and_path_from_triangulation() {
        let mut triangulation = Triangulation::from_outer_edges(&[
            Vec2::new(-5., -5.),
            Vec2::new(5., -5.),
            Vec2::new(5., 5.),
            Vec2::new(-5., 5.),
        ]);
        triangulation.add_obstacle(vec![
            Vec2::new(-1., -1.),
            Vec2::new(-1., 1.),
            Vec2::new(1., 1.),
            Vec2::new(1., -1.),
        ]);
        let mut mesh = triangulation.as_navmesh();
        // println!("{:#?}", mesh);
        while mesh.merge_polygons() {
            // println!("{:#?}", mesh);
        }
        mesh.bake();
        assert_eq!(mesh.polygons.len(), 4);
        dbg!(mesh.path(Vec2::new(0.5, 0.5), Vec2::new(9.5, 9.5)));
    }

    #[test]
    fn merge_and_path_from_triangulation_2() {
        let mut triangulation = Triangulation::from_outer_edges(&[
            Vec2::new(-5., -5.),
            Vec2::new(5., -5.),
            Vec2::new(5., 5.),
            Vec2::new(-5., 5.),
        ]);
        triangulation.add_obstacles(vec![
            vec![
                vec2(3.7, -3.3),
                vec2(3.7, -3.7),
                vec2(3.3, -3.7),
                vec2(3.3, -3.3),
            ],
            vec![
                vec2(4.6, -1.3),
                vec2(4.6, -1.7),
                vec2(4.2, -1.7),
                vec2(4.2, -1.3),
            ],
        ]);
        triangulation.simplify(0.001);
        let mut mesh = triangulation.as_navmesh();

        mesh.unbake();
        // println!("{:#?}", mesh);
        while mesh.merge_polygons() {
            // println!("{:#?}", mesh);
        }
        mesh.bake();
        assert_eq!(mesh.polygons.len(), 6);
        dbg!(mesh.path(Vec2::new(-4.5, 4.0), Vec2::new(-4.0, -4.5)));
    }
}
