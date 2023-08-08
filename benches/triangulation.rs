use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glam::vec2;
use polyanya::{Mesh, Triangulation};

fn triangulation(c: &mut Criterion) {
    c.bench_function(&"triangulation".to_string(), |b| {
        b.iter(|| {
            // Equivalent to the arena mesh
            let mut triangulation = Triangulation::from_outer_edges(vec![
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
            ]);

            triangulation.add_obstacle(vec![
                vec2(15., 15.),
                vec2(19., 15.),
                vec2(19., 18.),
                vec2(18., 18.),
                vec2(18., 19.),
                vec2(15., 19.),
            ]);
            triangulation.add_obstacle(vec![
                vec2(31., 15.),
                vec2(35., 15.),
                vec2(35., 18.),
                vec2(34., 18.),
                vec2(34., 19.),
                vec2(31., 19.),
            ]);
            triangulation.add_obstacle(vec![
                vec2(15., 31.),
                vec2(19., 31.),
                vec2(19., 34.),
                vec2(18., 34.),
                vec2(18., 35.),
                vec2(15., 35.),
            ]);
            triangulation.add_obstacle(vec![
                vec2(31., 31.),
                vec2(35., 31.),
                vec2(35., 34.),
                vec2(34., 34.),
                vec2(34., 35.),
                vec2(31., 35.),
            ]);
            triangulation.add_obstacle(vec![
                vec2(23., 10.),
                vec2(23., 8.),
                vec2(24., 8.),
                vec2(24., 7.),
                vec2(26., 7.),
                vec2(26., 10.),
            ]);
            let mesh: Mesh = triangulation.into();
            black_box(mesh);
        })
    });
}

criterion_group!(benches, triangulation);
criterion_main!(benches);
