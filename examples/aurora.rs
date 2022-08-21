use std::time::Instant;

use glam::Vec2;
use polyanya::Mesh;

macro_rules! assert_delta {
    ($x:expr, $y:expr) => {
        let val = $x;
        if !((val.len - $y).abs() < 0.001) {
            assert_eq!(val.len, $y);
        }
    };
}

fn main() {
    use tracing_subscriber::layer::SubscriberExt;

    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(tracing_tracy::TracyLayer::new()),
    )
    .expect("set up the subscriber");

    let mesh = Mesh::from_file("meshes/aurora-merged.mesh".into());

    let now = Instant::now();

    assert_delta!(mesh.path(Vec2::new(993.0, 290.0), Vec2::new(34.0, 622.0)), 1123.2226);

    println!("{}", now.elapsed().as_secs_f32() * 1000.0);
}
