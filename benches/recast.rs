use std::fs::File;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glam::{vec3, Vec3Swizzles};
use polyanya::{Mesh, RecastFullMesh, RecastPolyMesh, RecastPolyMeshDetail};

fn prepare_fullmesh(c: &mut Criterion) {
    let rasterised: RecastPolyMesh =
        serde_json::from_reader(File::open("meshes/recast/poly_mesh.json").unwrap()).unwrap();
    let detailed: RecastPolyMeshDetail =
        serde_json::from_reader(File::open("meshes/recast/detail_mesh.json").unwrap()).unwrap();
    let full_mesh = RecastFullMesh::new(rasterised, detailed);

    c.bench_function(&"recast - prepare fullmesh".to_string(), |b| {
        b.iter(|| {
            let full_mesh = full_mesh.clone();
            let mesh: Mesh = full_mesh.into();
            black_box(mesh);
        })
    });
}

fn find_path(c: &mut Criterion) {
    let rasterised: RecastPolyMesh =
        serde_json::from_reader(File::open("meshes/recast/poly_mesh.json").unwrap()).unwrap();
    let detailed: RecastPolyMeshDetail =
        serde_json::from_reader(File::open("meshes/recast/detail_mesh.json").unwrap()).unwrap();
    let full_mesh: polyanya::Mesh = RecastFullMesh::new(rasterised, detailed).into();

    let from = vec3(46.998413, 9.998184, 1.717747);
    let to = vec3(20.703018, 18.651773, -80.770203);

    c.bench_function(&"recast - find path".to_string(), |b| {
        b.iter(|| {
            let path = full_mesh.path(from.xz(), to.xz()).unwrap();
            black_box(path);
        })
    });
}

fn find_path_with_height(c: &mut Criterion) {
    let rasterised: RecastPolyMesh =
        serde_json::from_reader(File::open("meshes/recast/poly_mesh.json").unwrap()).unwrap();
    let detailed: RecastPolyMeshDetail =
        serde_json::from_reader(File::open("meshes/recast/detail_mesh.json").unwrap()).unwrap();
    let full_mesh: polyanya::Mesh = RecastFullMesh::new(rasterised, detailed).into();

    let from = vec3(46.998413, 9.998184, 1.717747);
    let to = vec3(20.703018, 18.651773, -80.770203);

    c.bench_function(&"recast - find path with height".to_string(), |b| {
        b.iter(|| {
            let path = full_mesh.path(from.xz(), to.xz()).unwrap();
            let path_with_height = path.path_with_height(from, to, &full_mesh);
            black_box(path_with_height);
        })
    });
}

fn enrich_path_with_height(c: &mut Criterion) {
    let rasterised: RecastPolyMesh =
        serde_json::from_reader(File::open("meshes/recast/poly_mesh.json").unwrap()).unwrap();
    let detailed: RecastPolyMeshDetail =
        serde_json::from_reader(File::open("meshes/recast/detail_mesh.json").unwrap()).unwrap();
    let full_mesh: polyanya::Mesh = RecastFullMesh::new(rasterised, detailed).into();

    let from = vec3(46.998413, 9.998184, 1.717747);
    let to = vec3(20.703018, 18.651773, -80.770203);
    let path = full_mesh.path(from.xz(), to.xz()).unwrap();

    c.bench_function(&"recast - enrich path with height".to_string(), |b| {
        b.iter(|| {
            let path_with_height = path.path_with_height(from, to, &full_mesh);
            black_box(path_with_height);
        })
    });
}

criterion_group!(
    benches,
    prepare_fullmesh,
    find_path,
    find_path_with_height,
    enrich_path_with_height
);
criterion_main!(benches);
