use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glam::{vec2, Vec2};
use polyanya::{Mesh, Triangulation};

const ARENA_OUTER_EDGE: [Vec2; 82] = [
    vec2(1., 3.),
    vec2(2., 3.),
    vec2(2., 2.),
    vec2(3., 2.),
    vec2(3., 1.),
    vec2(15., 1.),
    vec2(15., 3.),
    vec2(18., 3.),
    vec2(18., 2.),
    vec2(19., 2.),
    vec2(19., 1.),
    vec2(20., 1.),
    vec2(20., 2.),
    vec2(23., 2.),
    vec2(23., 1.),
    vec2(26., 1.),
    vec2(26., 3.),
    vec2(29., 3.),
    vec2(29., 2.),
    vec2(30., 2.),
    vec2(30., 1.),
    vec2(31., 1.),
    vec2(31., 3.),
    vec2(34., 3.),
    vec2(34., 2.),
    vec2(35., 2.),
    vec2(35., 1.),
    vec2(47., 1.),
    vec2(47., 3.),
    vec2(48., 3.),
    vec2(48., 15.),
    vec2(47., 15.),
    vec2(47., 19.),
    vec2(48., 19.),
    vec2(48., 31.),
    vec2(47., 31.),
    vec2(47., 35.),
    vec2(48., 35.),
    vec2(48., 47.),
    vec2(47., 47.),
    vec2(47., 48.),
    vec2(35., 48.),
    vec2(35., 47.),
    vec2(31., 47.),
    vec2(31., 48.),
    vec2(30., 48.),
    vec2(30., 47.),
    vec2(29., 47.),
    vec2(29., 46.),
    vec2(26., 46.),
    vec2(26., 48.),
    vec2(24., 48.),
    vec2(24., 47.),
    vec2(23., 47.),
    vec2(23., 46.),
    vec2(20., 46.),
    vec2(20., 48.),
    vec2(19., 48.),
    vec2(19., 47.),
    vec2(15., 47.),
    vec2(15., 48.),
    vec2(3., 48.),
    vec2(3., 47.),
    vec2(1., 47.),
    vec2(1., 35.),
    vec2(2., 35.),
    vec2(2., 34.),
    vec2(3., 34.),
    vec2(3., 31.),
    vec2(1., 31.),
    vec2(1., 30.),
    vec2(3., 30.),
    vec2(3., 27.),
    vec2(2., 27.),
    vec2(2., 26.),
    vec2(1., 26.),
    vec2(1., 23.),
    vec2(2., 23.),
    vec2(2., 18.),
    vec2(3., 18.),
    vec2(3., 15.),
    vec2(1., 15.),
];

const ARENA_OBSTACLES: [[Vec2; 6]; 5] = [
    [
        vec2(15., 15.),
        vec2(19., 15.),
        vec2(19., 18.),
        vec2(18., 18.),
        vec2(18., 19.),
        vec2(15., 19.),
    ],
    [
        vec2(31., 15.),
        vec2(35., 15.),
        vec2(35., 18.),
        vec2(34., 18.),
        vec2(34., 19.),
        vec2(31., 19.),
    ],
    [
        vec2(15., 31.),
        vec2(19., 31.),
        vec2(19., 34.),
        vec2(18., 34.),
        vec2(18., 35.),
        vec2(15., 35.),
    ],
    [
        vec2(31., 31.),
        vec2(35., 31.),
        vec2(35., 34.),
        vec2(34., 34.),
        vec2(34., 35.),
        vec2(31., 35.),
    ],
    [
        vec2(23., 10.),
        vec2(23., 8.),
        vec2(24., 8.),
        vec2(24., 7.),
        vec2(26., 7.),
        vec2(26., 10.),
    ],
];

fn triangulation(c: &mut Criterion) {
    c.bench_function(&"triangulation arena".to_string(), |b| {
        b.iter(|| {
            // Equivalent to the arena mesh
            let mut triangulation = Triangulation::from_outer_edges(ARENA_OUTER_EDGE.to_vec());

            triangulation.add_obstacle(ARENA_OBSTACLES[0].to_vec());
            triangulation.add_obstacle(ARENA_OBSTACLES[1].to_vec());
            triangulation.add_obstacle(ARENA_OBSTACLES[2].to_vec());
            triangulation.add_obstacle(ARENA_OBSTACLES[3].to_vec());
            triangulation.add_obstacle(ARENA_OBSTACLES[4].to_vec());

            let mesh: Mesh = triangulation.into();
            black_box(mesh);
        })
    });
}

fn triangulation_bulk(c: &mut Criterion) {
    c.bench_function(&"triangulation arena bulk add obstacles".to_string(), |b| {
        b.iter(|| {
            // Equivalent to the arena mesh
            let mut triangulation = Triangulation::from_outer_edges(ARENA_OUTER_EDGE.to_vec());

            triangulation.add_obstacles(vec![
                ARENA_OBSTACLES[0].to_vec(),
                ARENA_OBSTACLES[1].to_vec(),
                ARENA_OBSTACLES[2].to_vec(),
                ARENA_OBSTACLES[3].to_vec(),
                ARENA_OBSTACLES[4].to_vec(),
            ]);
            let mesh: Mesh = triangulation.into();
            black_box(mesh);
        })
    });
}

fn triangulation_overlapping(c: &mut Criterion) {
    c.bench_function(&"triangulation arena overlapping".to_string(), |b| {
        b.iter(|| {
            // Equivalent to the arena mesh
            let mut triangulation = Triangulation::from_outer_edges(ARENA_OUTER_EDGE.to_vec());

            triangulation.add_obstacle(ARENA_OBSTACLES[0].to_vec());
            triangulation.add_obstacle(ARENA_OBSTACLES[1].to_vec());
            triangulation.add_obstacle(ARENA_OBSTACLES[2].to_vec());
            triangulation.add_obstacle(ARENA_OBSTACLES[3].to_vec());
            triangulation.add_obstacle(ARENA_OBSTACLES[4].to_vec());
            triangulation.merge_overlapping_obstacles();

            let mesh: Mesh = triangulation.into();
            black_box(mesh);
        })
    });
}

fn triangulation_square(c: &mut Criterion) {
    c.bench_function(&"triangulation square".to_string(), |b| {
        b.iter(|| {
            let mut triangulation = Triangulation::from_outer_edges(vec![
                vec2(0.0, 0.0),
                vec2(10.0, 0.0),
                vec2(10.0, 10.0),
                vec2(0.0, 10.0),
            ]);
            triangulation.add_obstacle(vec![
                vec2(2.5, 2.5),
                vec2(2.5, 5.0),
                vec2(5.0, 5.0),
                vec2(5.0, 2.5),
            ]);
            triangulation.add_obstacle(vec![
                vec2(2.5, 5.01),
                vec2(2.5, 7.5),
                vec2(5.01, 7.5),
                vec2(5.01, 5.01),
            ]);
            triangulation.add_obstacle(vec![
                vec2(5.01, 2.5),
                vec2(5.01, 5.0),
                vec2(7.5, 5.0),
                vec2(7.5, 2.5),
            ]);
            triangulation.add_obstacle(vec![
                vec2(5.01, 5.01),
                vec2(5.01, 7.5),
                vec2(7.5, 7.5),
                vec2(7.5, 5.01),
            ]);
            let mesh: Mesh = triangulation.into();
            black_box(mesh);
        })
    });
}

fn triangulation_square_overlapping(c: &mut Criterion) {
    c.bench_function(&"triangulation square overlapping".to_string(), |b| {
        b.iter(|| {
            let mut triangulation = Triangulation::from_outer_edges(vec![
                vec2(0.0, 0.0),
                vec2(10.0, 0.0),
                vec2(10.0, 10.0),
                vec2(0.0, 10.0),
            ]);
            triangulation.add_obstacle(vec![
                vec2(2.5, 2.5),
                vec2(2.5, 6.0),
                vec2(6.0, 6.0),
                vec2(6.0, 2.5),
            ]);
            triangulation.add_obstacle(vec![
                vec2(2.5, 4.0),
                vec2(2.5, 7.5),
                vec2(6.0, 7.5),
                vec2(6.0, 4.0),
            ]);
            triangulation.add_obstacle(vec![
                vec2(4.0, 2.5),
                vec2(4.0, 6.0),
                vec2(7.5, 6.0),
                vec2(7.5, 2.5),
            ]);
            triangulation.add_obstacle(vec![
                vec2(4.0, 4.0),
                vec2(4.0, 7.5),
                vec2(7.5, 7.5),
                vec2(7.5, 4.0),
            ]);
            triangulation.merge_overlapping_obstacles();
            let mesh: Mesh = triangulation.into();
            black_box(mesh);
        })
    });
}

criterion_group!(
    benches,
    triangulation,
    triangulation_bulk,
    triangulation_overlapping,
    triangulation_square,
    triangulation_square_overlapping,
);
criterion_main!(benches);
