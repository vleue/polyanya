use criterion::{black_box, criterion_group, criterion_main, Criterion};
use polyanya::Mesh;

macro_rules! assert_delta {
    ($x:expr, $y:expr) => {
        let val = $x;
        if !((val.len - $y).abs() < 0.001) {
            assert_eq!(val.len, $y);
        }
        black_box(val);
    };
}

fn get_path(c: &mut Criterion) {
    let mesh = Mesh::from_file("meshes/aurora-merged.mesh".into());

    [
        ([993.0, 290.0], [34.0, 622.0], 1123.2226),
        ([356.0, 166.0], [661.0, 441.0], 595.041),
        ([827.0, 678.0], [460.0, 383.0], 605.301),
        ([233.0, 323.0], [422.0, 650.0], 598.005),
        ([468.0, 584.0], [280.0, 199.0], 614.314),
        ([512.0, 170.0], [480.0, 595.0], 607.774),
        ([611.0, 658.0], [494.0, 282.0], 604.497),
    ]
    .iter()
    .for_each(|(from, to, len)| {
        c.bench_function(&format!("get path {:?}", from), |b| {
            b.iter(|| {
                assert_delta!(mesh.path(*from, *to), *len);
            })
        });
    });
}

criterion_group!(benches, get_path);
criterion_main!(benches);
