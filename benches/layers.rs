use std::{collections::HashSet, fs::File};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glam::Vec2;
use polyanya::{Mesh, RecastFullMesh, RecastPolyMesh, RecastPolyMeshDetail};

#[cfg(not(feature = "detailed-layers"))]
const SUFFIX: &str = "";
#[cfg(feature = "detailed-layers")]
const SUFFIX: &str = " (detailed-layers)";

macro_rules! assert_delta {
    ($x:expr, $y:expr) => {
        let val = $x;
        if (val.length - $y).abs() >= 0.001 {
            assert_eq!(val.length, $y);
        }
        black_box(val);
    };
}

/// A mesh with 5 layers, stitched from the areas of a recast navmesh.
fn layered_mesh() -> Mesh {
    let rasterised: RecastPolyMesh =
        serde_json::from_reader(File::open("meshes/recast/poly_mesh.json").unwrap()).unwrap();
    let detailed: RecastPolyMeshDetail =
        serde_json::from_reader(File::open("meshes/recast/detail_mesh.json").unwrap()).unwrap();
    RecastFullMesh::new(rasterised, detailed).into()
}

fn get_path(c: &mut Criterion) {
    let mesh = layered_mesh();
    [
        // crosses all five layers
        (
            Vec2::new(-1.0152162, -30.441278),
            Vec2::new(46.884785, -0.3412806),
            69.70198,
        ),
        // longest path on this mesh, 61 polygons over four layers
        (
            Vec2::new(18.744785, -52.721283),
            Vec2::new(46.884785, -0.3412806),
            98.30155,
        ),
        (
            Vec2::new(-18.81121, -5.6468863),
            Vec2::new(46.884785, -0.3412806),
            79.285576,
        ),
        // same corridor as the previous one, in the other direction
        (
            Vec2::new(46.884785, -0.3412806),
            Vec2::new(-2.3152168, -27.841278),
            67.36921,
        ),
    ]
    .iter()
    .for_each(|(from, to, len)| {
        c.bench_function(&format!("layered path {from:?}{SUFFIX}"), |b| {
            b.iter(|| {
                assert_delta!(mesh.path(*from, *to).unwrap(), *len);
            })
        });
    });
}

fn no_path(c: &mut Criterion) {
    let mesh = layered_mesh();
    // whole reachable set.
    [
        (
            "to unreachable zone",
            Vec2::new(46.884785, -0.3412806),
            Vec2::new(24.074783, -81.75128),
        ),
        (
            "from unreachable zone",
            Vec2::new(24.074783, -81.75128),
            Vec2::new(46.884785, -0.3412806),
        ),
    ]
    .iter()
    .for_each(|(name, from, to)| {
        c.bench_function(&format!("layered no path {name}{SUFFIX}"), |b| {
            b.iter(|| {
                assert_eq!(black_box(mesh.path(*from, *to)), None);
            })
        });
    });
}

fn blocked_layers(c: &mut Criterion) {
    let mesh = layered_mesh();
    let from = Vec2::new(18.744785, -52.721283);
    let to = Vec2::new(46.884785, -0.3412806);

    let blocked = HashSet::from([1]);
    c.bench_function(&format!("layered path with blocked layer{SUFFIX}"), |b| {
        b.iter(|| {
            assert_delta!(
                mesh.path_on_layers(from, to, blocked.clone()).unwrap(),
                102.75539
            );
        })
    });

    let blocked = HashSet::from([1, 2]);
    c.bench_function(
        &format!("layered no path with blocked layers{SUFFIX}"),
        |b| {
            b.iter(|| {
                assert_eq!(
                    black_box(mesh.path_on_layers(from, to, blocked.clone())),
                    None
                );
            })
        },
    );
}

#[cfg(feature = "detailed-layers")]
fn scaled_layers(c: &mut Criterion) {
    let mut mesh = layered_mesh();
    for (i, layer) in mesh.layers.iter_mut().enumerate() {
        layer.scale = Vec2::splat(1.0 + (i as f32) * 0.5);
    }
    let from = Vec2::new(18.744785, -52.721283);
    let to = Vec2::new(46.884785, -0.3412806);

    c.bench_function(&format!("layered path with scaled layers{SUFFIX}"), |b| {
        b.iter(|| {
            black_box(mesh.path(from, to).unwrap());
        })
    });
}

#[cfg(feature = "detailed-layers")]
criterion_group!(benches, get_path, no_path, blocked_layers, scaled_layers);
#[cfg(not(feature = "detailed-layers"))]
criterion_group!(benches, get_path, no_path, blocked_layers);
criterion_main!(benches);
