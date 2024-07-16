use criterion::{criterion_group, criterion_main, Criterion};
use polyanya::{Mesh, PolyanyaFile};

fn baking(c: &mut Criterion) {
    let mut mesh: Mesh = PolyanyaFile::from_file("meshes/aurora-merged.mesh")
        .try_into()
        .unwrap();
    c.bench_function(&"baking".to_string(), |b| {
        b.iter(|| {
            mesh.unbake();
            mesh.bake();
        })
    });
}

criterion_group!(benches, baking);
criterion_main!(benches);
