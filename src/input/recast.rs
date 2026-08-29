use std::collections::HashSet;

use glam::{Vec2, Vec3Swizzles};
use hashbrown::HashMap;
use thiserror::Error;

pub use rerecast::DetailNavmesh as RecastPolyMeshDetail;
pub use rerecast::PolygonNavmesh as RecastPolyMesh;

use crate::{Layer, Mesh, MeshError, Polygon, U32Layer, Vertex};

/// Errors that can happen when importing a mesh from recast.
#[derive(Error, Debug, Copy, Clone, PartialEq)]
pub enum RecastError {
    /// The detail mesh doesn't have exactly one sub mesh per polygon of the polygon mesh, so
    /// there is no way to tell which area each sub mesh belongs to.
    #[error(
        "the detail mesh has {sub_meshes} sub meshes but the polygon mesh has {areas} polygons"
    )]
    MismatchedMeshes {
        /// Number of sub meshes in the detail mesh.
        sub_meshes: usize,
        /// Number of polygons (and so areas) in the polygon mesh.
        areas: usize,
    },
    /// A triangle of the detail mesh refers to a vertex that isn't in it.
    #[error("a triangle refers to vertex {vertex} but the detail mesh has {vertices} vertices")]
    VertexOutOfBounds {
        /// The offending vertex index.
        vertex: usize,
        /// Number of vertices in the detail mesh.
        vertices: usize,
    },
    /// Both the default walkable area (255) and the not-walkable area (0) are in use. They
    /// would both want layer 0, and a not-walkable polygon has no business being in a navmesh
    /// in the first place.
    #[error("area 0 is not walkable and cannot share layer 0 with the default area 255")]
    ConflictingDefaultArea,
    /// The imported mesh is not a valid [`Mesh`].
    #[error(transparent)]
    Mesh(#[from] MeshError),
}

/// Recast's default walkable area, the one every polygon carries until something else is
/// painted over it.
const DEFAULT_WALKABLE: u8 = 255;

/// The layer a recast area becomes on import.
///
/// Area ids mean something to whoever painted them, so they are kept as the layer index: an
/// area tagged `20` is layer 20. The one exception is [`DEFAULT_WALKABLE`], which nobody
/// chose -- recast puts it on everything by default -- and which would otherwise push every
/// imported mesh to the full 256 layers.
const fn layer_of_area(area: u8) -> u8 {
    if area == DEFAULT_WALKABLE {
        0
    } else {
        area
    }
}

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
        areas.sort_unstable();
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
                    .skip(mesh.base_triangle_index as usize)
                    .take(mesh.triangle_count as usize)
                    .map(|[a, b, c]| {
                        [
                            *a as usize + mesh.base_vertex_index as usize,
                            *b as usize + mesh.base_vertex_index as usize,
                            *c as usize + mesh.base_vertex_index as usize,
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

impl TryFrom<RecastPolyMeshDetail> for Mesh {
    type Error = RecastError;

    fn try_from(detailed_mesh: RecastPolyMeshDetail) -> Result<Self, Self::Error> {
        let common = detailed_mesh.common_vertices();
        let triangles = detailed_mesh.triangles();
        check_vertices(
            triangles.iter().flatten().copied(),
            detailed_mesh.vertices.len(),
        )?;
        let mut layer =
            Layer::new(
                detailed_mesh
                    .vertices
                    .iter()
                    .enumerate()
                    .map(|(i, v)| {
                        Vertex::new(
                            Vec2::new(v.x, v.z),
                            triangles
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
                triangles
                    .into_iter()
                    .map(|[p0, p1, p2]| Polygon::new(vec![p2 as u32, p1 as u32, p0 as u32], false))
                    .collect(),
            )?;
        layer.height = detailed_mesh.vertices.iter().map(|v| v.y).collect();

        let mut detailed_navmesh = Mesh {
            layers: vec![layer],
            ..Default::default()
        };
        detailed_navmesh.reorder_neighbors_ccw_and_fix_corners();
        detailed_navmesh.update_is_one_way();
        #[cfg(not(feature = "no-default-baking"))]
        detailed_navmesh.bake();

        Ok(detailed_navmesh)
    }
}

/// A triangle indexes into the detail mesh vertices, and both the polygon and the vertex list
/// come from outside the crate: an out of range index would be a panic several steps later.
fn check_vertices(
    triangles: impl IntoIterator<Item = usize>,
    vertices: usize,
) -> Result<(), RecastError> {
    for vertex in triangles {
        if vertex >= vertices {
            return Err(RecastError::VertexOutOfBounds { vertex, vertices });
        }
    }
    Ok(())
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

    /// Which [`Mesh`] layer each recast area becomes on import.
    ///
    /// An area keeps its id as its layer index: an area tagged `20` is layer 20, so per-area
    /// data can be looked up by the number it was painted with. Recast's default walkable area
    /// `255` is the exception and becomes layer 0, because nobody picked it and it would
    /// otherwise push every imported mesh to the full 256 layers.
    ///
    /// The layers of area ids that nothing is tagged with are present but empty. They cost a
    /// few nanoseconds each per query, so numbering areas `1, 2, 3` is still cheaper than
    /// numbering them `50, 150, 250`.
    ///
    /// This is the mapping to use to attach per-area data — a cost, a name — to the imported
    /// layers. It only depends on the areas in use, so it can be read before converting.
    pub fn area_to_layer(&self) -> std::collections::HashMap<u8, u8> {
        self.rasterised
            .areas()
            .into_iter()
            .map(|area| (area, layer_of_area(area)))
            .collect()
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
                    .skip(mesh.base_triangle_index as usize)
                    .take(mesh.triangle_count as usize)
                    .map(|[a, b, c]| PolygonWithMeshInfo {
                        vertices: [
                            *a as usize + mesh.base_vertex_index as usize,
                            *b as usize + mesh.base_vertex_index as usize,
                            *c as usize + mesh.base_vertex_index as usize,
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

impl TryFrom<RecastFullMesh> for Mesh {
    type Error = RecastError;

    /// Import a recast navmesh, with one [`Layer`] per recast area.
    ///
    /// Use [`RecastFullMesh::area_to_layer`] to know which layer each area ended up in.
    fn try_from(full: RecastFullMesh) -> Result<Self, Self::Error> {
        if full.detailed.meshes.len() != full.rasterised.areas.len() {
            return Err(RecastError::MismatchedMeshes {
                sub_meshes: full.detailed.meshes.len(),
                areas: full.rasterised.areas.len(),
            });
        }

        let common = full.detailed.common_vertices();
        let triangles_with_mesh_info = full.triangles_with_mesh_info();
        let num_vertices = full.detailed.vertices.len();
        check_vertices(
            triangles_with_mesh_info
                .iter()
                .flat_map(|polygon| polygon.vertices),
            num_vertices,
        )?;

        let areas = full.rasterised.areas();
        if areas.contains(&0) && areas.contains(&DEFAULT_WALKABLE) {
            return Err(RecastError::ConflictingDefaultArea);
        }
        // An area keeps its id as its layer index, so `layers` is as long as the largest id in
        // use and the ids nothing was tagged with are empty layers. Indexing it by anything
        // else -- the position of the area in this list, say -- would silently renumber the
        // layers whenever an area is added or removed.
        let area_of_layer: Vec<Option<u8>> = {
            let count = areas
                .iter()
                .map(|area| layer_of_area(*area) as usize + 1)
                .max()
                .unwrap_or(0);
            let mut area_of_layer = vec![None; count];
            for area in &areas {
                area_of_layer[layer_of_area(*area) as usize] = Some(*area);
            }
            area_of_layer
        };
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
                        .map(|(polygon_index, original_index)| {
                            (
                                original_index,
                                U32Layer::from_layer_and_polygon(
                                    layer_of_area(*area),
                                    polygon_index as u32,
                                ),
                            )
                        })
                        .collect::<HashMap<usize, u32>>(),
                )
            })
            .collect();

        // A polygon index shares its `u32` with the layer index, so a layer can't hold more
        // polygons than `Layer::new` would accept either.
        for polygons in reindexed_polygons.values() {
            if polygons.len() > (2_i32.pow(24) - 1) as usize {
                return Err(MeshError::TooManyPolygons.into());
            }
        }

        // Precompute: for each vertex index, which polygon indices contain it
        let mut vertex_to_polygons: Vec<Vec<usize>> = vec![vec![]; num_vertices];
        for (polygon_index, polygon) in triangles_with_mesh_info.iter().enumerate() {
            for &v in &polygon.vertices {
                vertex_to_polygons[v].push(polygon_index);
            }
        }

        // Expand using common (coincident) vertices
        let vertex_to_all_polygons: Vec<Vec<usize>> = (0..num_vertices)
            .map(|vertex_index| {
                let mut all_polygons = Vec::new();
                if let Some(common_vertices) = common.get(&(vertex_index as u32)) {
                    for cv in common_vertices {
                        all_polygons.extend(&vertex_to_polygons[*cv as usize]);
                    }
                }
                all_polygons.sort_unstable();
                all_polygons.dedup();
                all_polygons
            })
            .collect();

        let layers = area_of_layer
            .iter()
            .map(|area| {
                // An area id nothing was tagged with: the layer has to exist to keep the ids
                // that follow it on their own index, but it holds nothing.
                let Some(area) = area else {
                    return Layer::default();
                };
                let mut layer = Layer {
                    vertices: full
                        .detailed
                        .vertices
                        .iter()
                        .enumerate()
                        .map(|(vertex_index, vertex)| {
                            Vertex::new(
                                vertex.xz(),
                                vertex_to_all_polygons[vertex_index]
                                    .iter()
                                    .filter_map(|&polygon_index| {
                                        let polygon = &triangles_with_mesh_info[polygon_index];
                                        reindexed_polygons
                                            .get(&polygon.mesh_area)
                                            .and_then(|m| m.get(&polygon_index).cloned())
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

        Ok(full_navmesh)
    }
}
