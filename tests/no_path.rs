use std::sync::OnceLock;
use std::time::Instant;

use glam::{vec2, Vec2};
use polyanya::{Layer, Mesh, PolyanyaFile, Triangulation};

/// Read and bake the aurora mesh once for the whole test binary, then clone it
/// into each mesh that needs it. Baking is per layer and happens before
/// stitching, so a clone of the baked layer is the same thing as baking a
/// clone.
fn aurora_layer() -> &'static Layer {
    static AURORA: OnceLock<Layer> = OnceLock::new();
    AURORA.get_or_init(|| {
        let mut aurora: Mesh = PolyanyaFile::from_file("meshes/v2/aurora-merged.mesh")
            .try_into()
            .unwrap();
        let mut layer = aurora.layers.remove(0);
        layer.bake();
        layer
    })
}

/// A real mesh plus a small square that is not stitched to it, so nothing can
/// reach the square. Island detection is skipped as soon as there is more than
/// one layer, so this is the case where the search really does have to run out
/// of nodes to answer "no path".
fn mesh_with_an_unreachable_island(copies: usize) -> Mesh {
    let mut island = Triangulation::from_outer_edges(&[
        vec2(1000.0, 1000.0),
        vec2(1010.0, 1000.0),
        vec2(1010.0, 1010.0),
        vec2(1000.0, 1010.0),
    ])
    .as_layer();
    island.bake();

    let mut mesh = Mesh::default();
    mesh.layers.push(island);
    for _ in 0..copies {
        mesh.layers.push(aurora_layer().clone());
    }
    // nothing to stitch, but a multi-layer mesh still needs this pass: it is
    // what tags polygon indices with their layer
    mesh.stitch_at_vertices(vec![], false);
    mesh
}

#[test]
fn unreachable_target_is_not_a_path() {
    let mesh = mesh_with_an_unreachable_island(1);
    assert_eq!(
        mesh.path(Vec2::new(1005.0, 1005.0), Vec2::new(233.0, 501.0)),
        None
    );
    assert_eq!(
        mesh.path(Vec2::new(233.0, 501.0), Vec2::new(1005.0, 1005.0)),
        None
    );
}

/// The search starts inside a two-polygon island, so it runs out of nodes after
/// a handful of steps. What it costs to say so must follow from that, not from
/// how many polygons the rest of the mesh happens to have.
///
/// This is a timing assertion, which is not lovely, but the effect is a factor
/// of a hundred and I could not find a way to observe the step count from
/// outside the crate. Happy to drop it or move it to `benches/` instead.
#[test]
fn saying_no_path_does_not_cost_the_whole_mesh() {
    let small = mesh_with_an_unreachable_island(1);
    let large = mesh_with_an_unreachable_island(8);

    let time = |mesh: &Mesh| {
        let (from, to) = (Vec2::new(1005.0, 1005.0), Vec2::new(233.0, 501.0));
        // one call is a few microseconds once this is fixed, too short to time
        let started = Instant::now();
        for _ in 0..100 {
            assert_eq!(mesh.path(from, to), None);
        }
        started.elapsed()
    };

    let small = time(&small);
    let large = time(&large);
    assert!(
        large < small * 3,
        "answering \"no path\" scaled with the size of the mesh the search never \
         entered: {small:?} with one extra layer against {large:?} with eight"
    );
}
