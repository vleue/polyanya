use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use glam::{vec2, Vec2};
use polyanya::{Mesh, Triangulation};

const TILE: f32 = 10.0;
const STEP: f32 = 0.5;
const GRID: usize = 4;

const TARGET_LAYER: u8 = 5;

type StitchPoints = Vec<((u8, u8), Vec<Vec2>)>;
type StitchVertices = Vec<((u8, u8), Vec<(usize, usize)>)>;

fn tile_outline(x: f32, y: f32) -> Vec<Vec2> {
    let steps = (TILE / STEP) as usize;
    let mut edges = Vec::with_capacity(steps * 4);
    for i in 0..steps {
        edges.push(vec2(x + i as f32 * STEP, y));
    }
    for i in 0..steps {
        edges.push(vec2(x + TILE, y + i as f32 * STEP));
    }
    for i in 0..steps {
        edges.push(vec2(x + TILE - i as f32 * STEP, y + TILE));
    }
    for i in 0..steps {
        edges.push(vec2(x, y + TILE - i as f32 * STEP));
    }
    edges
}

fn unstitched_mesh() -> Mesh {
    let mut layers = Vec::with_capacity(GRID * GRID);
    for tile_x in 0..GRID {
        for tile_y in 0..GRID {
            let x = tile_x as f32 * TILE;
            let y = tile_y as f32 * TILE;
            let mut triangulation = Triangulation::from_outer_edges(&tile_outline(x, y));
            let offset = ((tile_x * GRID + tile_y) % 3) as f32;
            triangulation.add_obstacle(vec![
                vec2(x + 2.0 + offset, y + 2.0),
                vec2(x + 4.0 + offset, y + 2.0),
                vec2(x + 4.0 + offset, y + 5.0),
                vec2(x + 2.0 + offset, y + 5.0),
            ]);
            triangulation.add_obstacle(vec![
                vec2(x + 6.0, y + 6.0 - offset),
                vec2(x + 8.0, y + 6.0 - offset),
                vec2(x + 7.0, y + 8.0 - offset),
            ]);
            layers.push(triangulation.as_layer());
        }
    }
    Mesh {
        layers,
        ..Default::default()
    }
}

fn stitch_points(mesh: &Mesh) -> StitchPoints {
    mesh.clone().find_stitch_points()
}

fn stitch_vertices(mesh: &Mesh, points: &StitchPoints) -> StitchVertices {
    points
        .iter()
        .map(|((from, to), points)| {
            let vertices = points
                .iter()
                .map(|point| {
                    let find = |layer: u8| {
                        mesh.layers[layer as usize]
                            .vertices
                            .iter()
                            .position(|vertex| vertex.coords == *point)
                            .unwrap()
                    };
                    (find(*from), find(*to))
                })
                .collect();
            ((*from, *to), vertices)
        })
        .collect()
}

fn stitched_mesh(mesh: &Mesh, points: &StitchPoints) -> Mesh {
    let mut mesh = mesh.clone();
    mesh.stitch_at_points(points.clone(), false);
    mesh
}

fn only_target_layer<T: Clone>(stitches: &[((u8, u8), T)]) -> Vec<((u8, u8), T)> {
    stitches
        .iter()
        .filter(|((from, to), _)| *from == TARGET_LAYER || *to == TARGET_LAYER)
        .cloned()
        .collect()
}

fn find_stitch_points(c: &mut Criterion) {
    let mesh = unstitched_mesh();
    let expected = stitch_points(&mesh).len();

    c.bench_function("stitching - find stitch points", |b| {
        b.iter_batched(
            || mesh.clone(),
            |mut mesh| assert_eq!(black_box(mesh.find_stitch_points()).len(), expected),
            BatchSize::SmallInput,
        )
    });
}

fn stitch(c: &mut Criterion) {
    let mesh = unstitched_mesh();
    let points = stitch_points(&mesh);
    let vertices = stitch_vertices(&mesh, &points);

    c.bench_function("stitching - stitch at points", |b| {
        b.iter_batched(
            || (mesh.clone(), points.clone()),
            |(mut mesh, points)| {
                mesh.stitch_at_points(points, false);
                black_box(mesh);
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("stitching - stitch at vertices", |b| {
        b.iter_batched(
            || (mesh.clone(), vertices.clone()),
            |(mut mesh, vertices)| {
                mesh.stitch_at_vertices(vertices, false);
                black_box(mesh);
            },
            BatchSize::SmallInput,
        )
    });
}

fn remove_stitches(c: &mut Criterion) {
    let mesh = unstitched_mesh();
    let stitched = stitched_mesh(&mesh, &stitch_points(&mesh));

    c.bench_function("stitching - remove stitches", |b| {
        b.iter_batched(
            || stitched.clone(),
            |mut mesh| {
                mesh.remove_stitches();
                black_box(mesh);
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("stitching - remove stitches to layer", |b| {
        b.iter_batched(
            || stitched.clone(),
            |mut mesh| {
                mesh.remove_stitches_to_layer(TARGET_LAYER);
                black_box(mesh);
            },
            BatchSize::SmallInput,
        )
    });
}

fn restitch(c: &mut Criterion) {
    let mesh = unstitched_mesh();
    let points = stitch_points(&mesh);
    let vertices = only_target_layer(&stitch_vertices(&mesh, &points));
    let stitched = stitched_mesh(&mesh, &points);
    let points = only_target_layer(&points);

    let mut unstitched_layer = stitched.clone();
    unstitched_layer.remove_stitches_to_layer(TARGET_LAYER);

    c.bench_function("stitching - restitch layer at points", |b| {
        b.iter_batched(
            || (unstitched_layer.clone(), points.clone()),
            |(mut mesh, points)| {
                mesh.restitch_layer_at_points(TARGET_LAYER, points, false);
                black_box(mesh);
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("stitching - restitch layer at vertices", |b| {
        b.iter_batched(
            || (unstitched_layer.clone(), vertices.clone()),
            |(mut mesh, vertices)| {
                mesh.restitch_layer_at_vertices(TARGET_LAYER, vertices, false);
                black_box(mesh);
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    find_stitch_points,
    stitch,
    remove_stitches,
    restitch
);
criterion_main!(benches);
