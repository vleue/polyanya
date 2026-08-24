use glam::vec2;
use polyanya::{Layer, Mesh, Triangulation};

/// The minimal reproduction: baking a layer that holds no polygons used to
/// recurse forever in `BVH2d::build` and abort the process with a stack
/// overflow.
#[test]
fn baking_a_layer_without_polygons_returns() {
    let mut layer = Layer::default();
    layer.bake();

    let mesh = Mesh {
        layers: vec![layer],
        ..Default::default()
    };
    assert!(!mesh.point_in_mesh(vec2(0.0, 0.0)));
}

/// How one gets an empty layer without writing one by hand: an obstacle
/// covering the whole outline leaves nothing to triangulate. A chunked mesh
/// meets this whenever a chunk falls entirely inside a river or a city block.
#[test]
fn a_layer_covered_by_an_obstacle_bakes() {
    let outline = [
        vec2(0.0, 0.0),
        vec2(10.0, 0.0),
        vec2(10.0, 10.0),
        vec2(0.0, 10.0),
    ];
    let mut triangulation = Triangulation::from_outer_edges(&outline);
    triangulation.add_obstacle(vec![
        vec2(-1.0, -1.0),
        vec2(-1.0, 11.0),
        vec2(11.0, 11.0),
        vec2(11.0, -1.0),
    ]);

    let mut layer = triangulation.as_layer();
    assert!(layer.polygons.is_empty());
    layer.bake();

    let mesh = Mesh {
        layers: vec![layer],
        ..Default::default()
    };
    assert!(!mesh.point_in_mesh(vec2(5.0, 5.0)));
}
