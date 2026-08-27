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
    RecastFullMesh::new(rasterised, detailed)
        .try_into()
        .unwrap()
}

/// The `aurora-merged` navmesh run through the recast pipeline
fn large_layered_mesh(bands: usize) -> Mesh {
    let mut rasterised: RecastPolyMesh =
        serde_json::from_reader(File::open("meshes/recast/poly_mesh-aurora.json").unwrap())
            .unwrap();
    band_areas(&mut rasterised, bands);
    let detailed: RecastPolyMeshDetail =
        serde_json::from_reader(File::open("meshes/recast/detail_mesh-aurora.json").unwrap())
            .unwrap();
    RecastFullMesh::new(rasterised, detailed)
        .try_into()
        .unwrap()
}

fn band_areas(mesh: &mut RecastPolyMesh, bands: usize) {
    let per_polygon = mesh.max_vertices_per_polygon as usize;
    let centroid = |mesh: &RecastPolyMesh, polygon: usize| {
        let (sum, count) = mesh.polygons[polygon * per_polygon..(polygon + 1) * per_polygon]
            .iter()
            .filter(|vertex| **vertex != u16::MAX)
            .fold((0u32, 0u32), |(sum, count), vertex| {
                (sum + mesh.vertices[*vertex as usize].z as u32, count + 1)
            });
        sum as f32 / count as f32
    };

    let default: Vec<usize> = (0..mesh.areas.len())
        .filter(|polygon| mesh.areas[*polygon].0 == 255)
        .collect();
    let mut centroids: Vec<f32> = default.iter().map(|p| centroid(mesh, *p)).collect();
    centroids.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let edges: Vec<f32> = (1..bands)
        .map(|band| centroids[(band * (centroids.len() - 1)) / bands])
        .collect();

    let mut in_use: Vec<u8> = mesh
        .areas
        .iter()
        .map(|area| area.0)
        .filter(|area| *area != 255)
        .collect();
    in_use.sort_unstable();
    in_use.dedup();
    let next_id = in_use.len() as u8 + 1;

    for polygon in default {
        let z = centroid(mesh, polygon);
        let band = edges.iter().filter(|edge| z > **edge).count();
        mesh.areas[polygon].0 = if band == 0 {
            255
        } else {
            next_id + band as u8 - 1
        };
    }
}

const BANDS: usize = 8;

const CASES: [(Vec2, Vec2, f32); 4] = [
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
];

fn get_path(c: &mut Criterion) {
    let mesh = layered_mesh();
    CASES.iter().for_each(|(from, to, len)| {
        c.bench_function(&format!("layered path {from:?}{SUFFIX}"), |b| {
            b.iter(|| {
                assert_delta!(mesh.path(*from, *to).unwrap(), *len);
            })
        });
    });
}

const LARGE_CASES: [(&str, Vec2, Vec2, f32); 4] = [
    // the length of the mesh, 33 turns
    (
        "longest",
        Vec2::new(993.0, 290.0),
        Vec2::new(34.0, 622.0),
        1150.4978,
    ),
    (
        "crossing",
        Vec2::new(611.0, 658.0),
        Vec2::new(494.0, 282.0),
        628.7788,
    ),
    (
        "midfield",
        Vec2::new(233.0, 323.0),
        Vec2::new(422.0, 650.0),
        623.8622,
    ),
    // Three turns inside one layer
    (
        "short",
        Vec2::new(468.0, 584.0),
        Vec2::new(500.0, 560.0),
        40.0,
    ),
];

fn large_path(c: &mut Criterion) {
    let mesh = large_layered_mesh(BANDS);
    LARGE_CASES.iter().for_each(|(name, from, to, len)| {
        c.bench_function(&format!("large layered path {name}{SUFFIX}"), |b| {
            b.iter(|| {
                assert_delta!(mesh.path(*from, *to).unwrap(), *len);
            })
        });
    });
}

fn no_path(c: &mut Criterion) {
    let mesh = layered_mesh();
    [
        (
            "to unreachable zone",
            Vec2::new(-5.515217, -17.64128),
            Vec2::new(13.808118, -83.25128),
        ),
        (
            "from unreachable zone",
            Vec2::new(13.808118, -83.25128),
            Vec2::new(-5.515217, -17.64128),
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
type ScaleConfig = (&'static str, fn(usize) -> Vec2);

#[cfg(feature = "detailed-layers")]
const CONFIGS: [ScaleConfig; 3] = [
    ("uniform", |_| Vec2::splat(1.5)),
    ("mixed", |i| Vec2::splat(1.0 + (i as f32) * 0.5)),
    ("anisotropic", |_| Vec2::new(1.0, 1.4)),
];

#[cfg(feature = "detailed-layers")]
fn scaled_layers(c: &mut Criterion) {
    let lengths: [[f32; 2]; 3] = [
        [CASES[0].2 * 1.5, CASES[1].2 * 1.5],
        [126.69473, 152.20593],
        [80.38375, 116.50271],
    ];
    for ((name, scale), lengths) in CONFIGS.iter().zip(lengths) {
        let mut mesh = layered_mesh();
        for (i, layer) in mesh.layers.iter_mut().enumerate() {
            layer.scale = scale(i);
        }
        CASES[..2]
            .iter()
            .zip(lengths)
            .for_each(|((from, to, _), len)| {
                c.bench_function(
                    &format!("layered path scaled {name} {from:?}{SUFFIX}"),
                    |b| {
                        b.iter(|| {
                            assert_delta!(mesh.path(*from, *to).unwrap(), len);
                        })
                    },
                );
            });
    }
}

#[cfg(feature = "detailed-layers")]
fn large_scaled_layers(c: &mut Criterion) {
    let lengths: [[f32; 2]; 3] = [
        [LARGE_CASES[1].3 * 1.5, LARGE_CASES[2].3 * 1.5],
        [1897.689, 2019.0597],
        [751.6139, 716.6591],
    ];
    for ((name, scale), lengths) in CONFIGS.iter().zip(lengths) {
        let mut mesh = large_layered_mesh(BANDS);
        for (i, layer) in mesh.layers.iter_mut().enumerate() {
            layer.scale = scale(i);
        }
        LARGE_CASES[1..3]
            .iter()
            .zip(lengths)
            .for_each(|((case, from, to, _), len)| {
                c.bench_function(
                    &format!("large layered path scaled {name} {case}{SUFFIX}"),
                    |b| {
                        b.iter(|| {
                            assert_delta!(mesh.path(*from, *to).unwrap(), len);
                        })
                    },
                );
            });
    }
}

#[cfg(feature = "detailed-layers")]
criterion_group!(
    benches,
    get_path,
    large_path,
    no_path,
    blocked_layers,
    scaled_layers,
    large_scaled_layers
);
#[cfg(not(feature = "detailed-layers"))]
criterion_group!(benches, get_path, large_path, no_path, blocked_layers);
criterion_main!(benches);
