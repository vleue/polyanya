use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glam::Vec2;
use polyanya::{Mesh, PolyanyaFile};

fn no_path(c: &mut Criterion) {
    let mesh: Mesh = PolyanyaFile::from_file("meshes/v2/aurora-merged.mesh")
        .try_into()
        .unwrap();
    [
        (Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)),
        (Vec2::new(0.0, 0.0), Vec2::new(575.0, 410.0)),
        (Vec2::new(575.0, 410.0), Vec2::new(0.0, 0.0)),
        (Vec2::new(297.0, 438.0), Vec2::new(575.0, 410.0)),
        (Vec2::new(458.0, 47.0), Vec2::new(575.0, 410.0)),
        (Vec2::new(575.0, 410.0), Vec2::new(458.0, 47.0)), // worst case: destination is in a small unreachable zone
    ]
    .iter()
    .for_each(|(from, to)| {
        c.bench_function(&format!("no path {from:?}-{to:?}"), |b| {
            b.iter(|| {
                assert_eq!(black_box(mesh.path(*from, *to)), None);
            })
        });
    });
}

criterion_group!(benches, no_path);
criterion_main!(benches);
