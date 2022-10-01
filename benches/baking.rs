use criterion::{criterion_group, criterion_main, Criterion};
use polyanya::Mesh;

fn baking(c: &mut Criterion) {
    let mut mesh = Mesh::from_file("meshes/aurora-merged.mesh".into());

    c.bench_function(&format!("baking"), |b| {
        b.iter(|| {
            mesh.unbake();
            mesh.bake();
        })
    });
}

criterion_group!(benches, baking);
criterion_main!(benches);
