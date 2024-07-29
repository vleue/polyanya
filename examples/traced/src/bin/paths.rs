use glam::Vec2;
use polyanya::{Mesh, PolyanyaFile};
use tracing_subscriber::layer::SubscriberExt;

fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default()),
    )
    .expect("set up the subscriber");
    let mesh: Mesh = {
        let _execution_span: tracing::span::EnteredSpan =
            tracing::info_span!("loading mesh").entered();
        PolyanyaFile::from_file("meshes/aurora-merged.mesh")
            .try_into()
            .unwrap()
    };
    println!("loaded mesh");

    for _i in 0..1000 {
        let _execution_span: tracing::span::EnteredSpan =
            tracing::info_span!("getting path").entered();
        mesh.path(Vec2::new(233.0, 323.0), Vec2::new(422.0, 650.0))
            .unwrap();
    }
}
