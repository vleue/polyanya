use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glam::Vec2;
use polyanya::Mesh;

macro_rules! assert_delta {
    ($x:expr, $y:expr) => {
        let val = $x;
        if !((val.length - $y).abs() < 0.001) {
            assert_eq!(val.length, $y);
        }
        black_box(val);
    };
}

fn get_path(c: &mut Criterion) {
    let mesh = Mesh::from_file("meshes/aurora-merged.mesh".into());

    [
        (Vec2::new(993.0, 290.0), Vec2::new(34.0, 622.0), 1123.2226),
        (Vec2::new(356.0, 166.0), Vec2::new(661.0, 441.0), 595.041),
        (Vec2::new(827.0, 678.0), Vec2::new(460.0, 383.0), 605.301),
        (Vec2::new(233.0, 323.0), Vec2::new(422.0, 650.0), 598.005),
        (Vec2::new(468.0, 584.0), Vec2::new(280.0, 199.0), 614.314),
        (Vec2::new(512.0, 170.0), Vec2::new(480.0, 595.0), 607.774),
        (Vec2::new(611.0, 658.0), Vec2::new(494.0, 282.0), 604.497),
    ]
    .iter()
    .for_each(|(from, to, len)| {
        c.bench_function(&format!("get path {:?}", from), |b| {
            b.iter(|| {
                assert_delta!(mesh.path(*from, *to).unwrap(), *len);
            })
        });
    });
}

criterion_group!(benches, get_path);
criterion_main!(benches);
