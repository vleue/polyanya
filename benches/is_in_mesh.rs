use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glam::Vec2;
use polyanya::Mesh;

fn is_in_mesh(c: &mut Criterion) {
    let mesh = Mesh::from_file("meshes/aurora-merged.mesh");

    [
        Vec2::new(575., 410.),
        Vec2::new(728., 148.),
        Vec2::new(131., 669.),
        Vec2::new(135., 360.),
        Vec2::new(308., 147.),
        Vec2::new(22., 432.),
    ]
    .iter()
    .for_each(|from| {
        c.bench_function(&format!("is in mesh {from:?}"), |b| {
            b.iter(|| {
                assert!(black_box(mesh.point_in_mesh(*from)));
            })
        });
    });
}

fn is_not_in_mesh(c: &mut Criterion) {
    let mesh = Mesh::from_file("meshes/aurora-merged.mesh");

    [
        Vec2::new(0., 0.),
        Vec2::new(297., 438.),
        Vec2::new(726., 470.),
        Vec2::new(969., 726.),
        Vec2::new(521., 90.),
    ]
    .iter()
    .for_each(|from| {
        c.bench_function(&format!("is not in mesh {from:?}"), |b| {
            b.iter(|| {
                assert!(black_box(!mesh.point_in_mesh(*from)));
            })
        });
    });
}

criterion_group!(benches, is_in_mesh, is_not_in_mesh);
criterion_main!(benches);
