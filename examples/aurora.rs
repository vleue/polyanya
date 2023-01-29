use glam::Vec2;
use polyanya::{Mesh, PolyanyaFile};

macro_rules! assert_delta {
    ($x:expr, $y:expr) => {
        let val = $x;
        if !((val.length - $y).abs() < 0.001) {
            assert_eq!(val.length, $y);
        }
    };
}

fn main() {
    use tracing_subscriber::layer::SubscriberExt;

    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(tracing_tracy::TracyLayer::new()),
    )
    .expect("set up the subscriber");

    let mesh: Mesh = PolyanyaFile::from_file("meshes/aurora-merged.mesh").into();
    assert_delta!(
        mesh.path(Vec2::new(993.0, 290.0), Vec2::new(34.0, 622.0))
            .unwrap(),
        1123.2226
    );
}
