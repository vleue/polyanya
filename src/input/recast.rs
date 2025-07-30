use std::collections::HashSet;

use glam::{Vec2, Vec3Swizzles};
use hashbrown::HashMap;

pub use rerecast::DetailPolygonMesh as RecastPolyMeshDetail;
pub use rerecast::PolygonMesh as RecastPolyMesh;

use crate::{Layer, Mesh, Polygon, U32Layer, Vertex};

trait RecastPolyMeshExt {
    fn areas(&self) -> Vec<u8>;
}

impl RecastPolyMeshExt for RecastPolyMesh {
    fn areas(&self) -> Vec<u8> {
        let mut areas: Vec<u8> = self
            .areas
            .iter()
            .map(|area| area.0)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        areas.sort();
        if let Some(255) = areas.last() {
            areas.pop();
            areas.insert(0, 255);
        }
        areas
    }
}

trait RecastPolyMeshDetailExt {
    fn triangles(&self) -> Vec<[usize; 3]>;
    fn common_vertices(&self) -> HashMap<u32, Vec<u32>>;
}

impl RecastPolyMeshDetailExt for RecastPolyMeshDetail {
    /// Get the list of vertex IDs for each triangle in the mesh.
    fn triangles(&self) -> Vec<[usize; 3]> {
        self.meshes
            .iter()
            .flat_map(|mesh| {
                self.triangles
                    .iter()
                    .skip(mesh.first_triangle_index)
                    .take(mesh.triangle_count)
                    .map(|&(t, _)| {
                        [
                            t.x as usize + mesh.first_vertex_index,
                            t.y as usize + mesh.first_vertex_index,
                            t.z as usize + mesh.first_vertex_index,
                        ]
                    })
            })
            .collect()
    }

    fn common_vertices(&self) -> HashMap<u32, Vec<u32>> {
        self.vertices
            .iter()
            .enumerate()
            .map(|(i, v)| {
                (
                    i as u32,
                    self.vertices
                        .iter()
                        .enumerate()
                        .filter_map(|(i2, v2)| (v == v2).then_some(i2 as u32))
                        .collect(),
                )
            })
            .collect()
    }
}

impl From<RecastPolyMeshDetail> for Mesh {
    fn from(detailed_mesh: RecastPolyMeshDetail) -> Self {
        let common = detailed_mesh.common_vertices();
        let mut layer =
            Layer::new(
                detailed_mesh
                    .vertices
                    .iter()
                    .enumerate()
                    .map(|(i, v)| {
                        Vertex::new(
                            Vec2::new(v.x, v.z),
                            detailed_mesh
                                .triangles()
                                .iter()
                                .enumerate()
                                .filter_map(|(n, p)| {
                                    common.get(&(i as u32)).unwrap().iter().find_map(|ii| {
                                        p.contains(&(*ii as usize)).then_some(n as u32)
                                    })
                                })
                                .collect(),
                        )
                    })
                    .collect(),
                detailed_mesh
                    .triangles()
                    .into_iter()
                    .map(|p| Polygon::new(vec![p[2] as u32, p[1] as u32, p[0] as u32], false))
                    .collect(),
            )
            .unwrap();
        layer.height = detailed_mesh.vertices.iter().map(|v| v.y).collect();

        let mut detailed_navmesh = Mesh {
            layers: vec![layer],
            ..Default::default()
        };
        detailed_navmesh.reorder_neighbors_ccw_and_fix_corners();
        detailed_navmesh.update_is_one_way();
        #[cfg(not(feature = "no-default-baking"))]
        detailed_navmesh.bake();

        detailed_navmesh
    }
}

/// A combined mesh from recast
#[derive(Debug, Clone)]
pub struct RecastFullMesh {
    rasterised: RecastPolyMesh,
    detailed: RecastPolyMeshDetail,
}

impl RecastFullMesh {
    /// Create a new `RecastFullMesh` from the combined polygon mesh and detailed mesh from recast.
    pub fn new(rasterised: RecastPolyMesh, detailed: RecastPolyMeshDetail) -> Self {
        Self {
            rasterised,
            detailed,
        }
    }

    fn triangles_with_mesh_info(&self) -> Vec<PolygonWithMeshInfo> {
        self.detailed
            .meshes
            .iter()
            .zip(self.rasterised.areas.iter())
            .flat_map(|(mesh, mesh_area)| {
                self.detailed
                    .triangles
                    .iter()
                    .skip(mesh.first_triangle_index)
                    .take(mesh.triangle_count)
                    .map(|&(t, _)| PolygonWithMeshInfo {
                        vertices: [
                            t.x as usize + mesh.first_vertex_index,
                            t.y as usize + mesh.first_vertex_index,
                            t.z as usize + mesh.first_vertex_index,
                        ],
                        mesh_area: mesh_area.0,
                    })
            })
            .collect()
    }
}

struct PolygonWithMeshInfo {
    vertices: [usize; 3],
    mesh_area: u8,
}

impl From<RecastFullMesh> for Mesh {
    fn from(full: RecastFullMesh) -> Self {
        let common = full.detailed.common_vertices();
        let triangles_with_mesh_info = full.triangles_with_mesh_info();
        let areas = full.rasterised.areas();
        let reindexed_polygons: HashMap<u8, HashMap<usize, u32>> = areas
            .iter()
            .map(|area| {
                (
                    *area,
                    triangles_with_mesh_info
                        .iter()
                        .enumerate()
                        .filter_map(|(original_index, polygon)| {
                            (*area == polygon.mesh_area).then_some(original_index)
                        })
                        .enumerate()
                        .map(|(layer_index, original_index)| {
                            (
                                original_index,
                                U32Layer::from_layer_and_polygon(
                                    if *area == 255 { 0 } else { *area },
                                    layer_index as u32,
                                ),
                            )
                        })
                        .collect::<HashMap<usize, u32>>(),
                )
            })
            .collect();

        let layers = areas
            .iter()
            .map(|area| {
                let mut layer = Layer {
                    vertices: full
                        .detailed
                        .vertices
                        .iter()
                        .enumerate()
                        .map(|(vertex_index, vertex)| {
                            Vertex::new(
                                vertex.xz(),
                                triangles_with_mesh_info
                                    .iter()
                                    .enumerate()
                                    .filter_map(|(polygon_index, polygon)| {
                                        common.get(&(vertex_index as u32)).unwrap().iter().find_map(
                                            |common_vertex_index| {
                                                polygon
                                                    .vertices
                                                    .contains(&(*common_vertex_index as usize))
                                                    .then(|| {
                                                        reindexed_polygons
                                                            .get(&polygon.mesh_area)
                                                            .unwrap()
                                                            .get(&polygon_index)
                                                            .cloned()
                                                    })
                                                    .flatten()
                                            },
                                        )
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                    polygons: triangles_with_mesh_info
                        .iter()
                        .filter(|polygon| polygon.mesh_area == *area)
                        .map(|polygon| {
                            Polygon::new(
                                vec![
                                    polygon.vertices[2] as u32,
                                    polygon.vertices[1] as u32,
                                    polygon.vertices[0] as u32,
                                ],
                                false,
                            )
                        })
                        .collect(),
                    ..Default::default()
                };
                layer.height = full.detailed.vertices.iter().map(|v| v.y).collect();
                // TODO: islands baking doesn't work on stitched layers, don't do it for now
                #[cfg(not(feature = "no-default-baking"))]
                layer.bake_polygon_finder();
                layer
            })
            .collect();

        let mut full_navmesh = Mesh {
            layers,
            ..Default::default()
        };

        if !areas.is_empty() {
            full_navmesh.remove_useless_vertices();
        }
        full_navmesh.reorder_neighbors_ccw_and_fix_corners();
        full_navmesh.update_is_one_way();

        full_navmesh
    }
}
