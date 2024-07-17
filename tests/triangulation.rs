use glam::vec2;
use polyanya::{Mesh, Triangulation};

#[test]
fn is_in_mesh() {
    let mut triangulation = Triangulation::from_outer_edges(&[
        vec2(0.0, 0.0),
        vec2(10.0, 0.0),
        vec2(10.0, 10.0),
        vec2(0.0, 10.0),
    ]);
    triangulation.add_obstacle(vec![
        vec2(2.5, 2.5),
        vec2(2.5, 7.5),
        vec2(7.5, 7.5),
        vec2(7.5, 2.5),
    ]);
    let mesh: Mesh = triangulation.as_navmesh();
    for i in 0..10 {
        for j in 0..10 {
            if i > 2 && i < 8 && j > 2 && j < 8 {
                assert!(!mesh.point_in_mesh(vec2(i as f32, j as f32)));
            } else {
                assert!(mesh.point_in_mesh(vec2(i as f32, j as f32)));
            }
        }
    }
}

#[test]
fn is_in_mesh_4_obstacles() {
    let mut triangulation = Triangulation::from_outer_edges(&[
        vec2(0.0, 0.0),
        vec2(10.0, 0.0),
        vec2(10.0, 10.0),
        vec2(0.0, 10.0),
    ]);
    triangulation.add_obstacle(vec![
        vec2(2.5, 2.5),
        vec2(2.5, 5.0),
        vec2(5.0, 5.0),
        vec2(5.0, 2.5),
    ]);
    triangulation.add_obstacle(vec![
        vec2(2.5, 5.0),
        vec2(2.5, 7.5),
        vec2(5.0, 7.5),
        vec2(5.0, 5.0),
    ]);
    triangulation.add_obstacle(vec![
        vec2(5.0, 2.5),
        vec2(5.0, 5.0),
        vec2(7.5, 5.0),
        vec2(7.5, 2.5),
    ]);
    triangulation.add_obstacle(vec![
        vec2(5.0, 5.0),
        vec2(5.0, 7.5),
        vec2(7.5, 7.5),
        vec2(7.5, 5.0),
    ]);
    triangulation.simplify(0.5);
    let mesh: Mesh = triangulation.as_navmesh();

    dbg!(mesh.layers[0].polygons.len());
    for i in 0..10 {
        for j in 0..10 {
            if i > 2 && i < 8 && j > 2 && j < 8 {
                assert!(!mesh.point_in_mesh(vec2(i as f32, j as f32)));
            } else {
                assert!(mesh.point_in_mesh(vec2(i as f32, j as f32)));
            }
        }
    }
}

#[test]
fn is_in_mesh_overlapping() {
    let mut triangulation = Triangulation::from_outer_edges(&[
        vec2(0.0, 0.0),
        vec2(10.0, 0.0),
        vec2(10.0, 10.0),
        vec2(0.0, 10.0),
    ]);
    triangulation.add_obstacle(vec![
        vec2(2.5, 2.5),
        vec2(2.5, 6.0),
        vec2(6.0, 6.0),
        vec2(6.0, 2.5),
    ]);
    triangulation.add_obstacle(vec![
        vec2(2.5, 4.0),
        vec2(2.5, 7.5),
        vec2(6.0, 7.5),
        vec2(6.0, 4.0),
    ]);
    triangulation.add_obstacle(vec![
        vec2(4.0, 2.5),
        vec2(4.0, 6.0),
        vec2(7.5, 6.0),
        vec2(7.5, 2.5),
    ]);
    triangulation.add_obstacle(vec![
        vec2(4.0, 4.0),
        vec2(4.0, 7.5),
        vec2(7.5, 7.5),
        vec2(7.5, 4.0),
    ]);
    let mesh: Mesh = triangulation.as_navmesh();
    for i in 0..10 {
        for j in 0..10 {
            if i > 2 && i < 8 && j > 2 && j < 8 {
                assert!(!mesh.point_in_mesh(vec2(i as f32, j as f32)));
            } else {
                assert!(mesh.point_in_mesh(vec2(i as f32, j as f32)));
            }
        }
    }
}

#[test]
fn is_in_mesh_overlapping_merged() {
    let mut triangulation = Triangulation::from_outer_edges(&[
        vec2(0.0, 0.0),
        vec2(10.0, 0.0),
        vec2(10.0, 10.0),
        vec2(0.0, 10.0),
    ]);
    triangulation.add_obstacle(vec![
        vec2(2.5, 2.5),
        vec2(2.5, 6.0),
        vec2(6.0, 6.0),
        vec2(6.0, 2.5),
    ]);
    triangulation.add_obstacle(vec![
        vec2(2.5, 4.0),
        vec2(2.5, 7.5),
        vec2(6.0, 7.5),
        vec2(6.0, 4.0),
    ]);
    triangulation.add_obstacle(vec![
        vec2(4.0, 2.5),
        vec2(4.0, 6.0),
        vec2(7.5, 6.0),
        vec2(7.5, 2.5),
    ]);
    triangulation.add_obstacle(vec![
        vec2(4.0, 4.0),
        vec2(4.0, 7.5),
        vec2(7.5, 7.5),
        vec2(7.5, 4.0),
    ]);
    let mesh: Mesh = triangulation.as_navmesh();
    for i in 0..10 {
        for j in 0..10 {
            if i > 2 && i < 8 && j > 2 && j < 8 {
                assert!(!mesh.point_in_mesh(vec2(i as f32, j as f32)));
            } else {
                assert!(mesh.point_in_mesh(vec2(i as f32, j as f32)));
            }
        }
    }
}

#[test]
fn is_in_mesh_simplified() {
    let mut triangulation = Triangulation::from_outer_edges(&[
        vec2(0.0, 0.0),
        vec2(10.0, 0.0),
        vec2(10.0, 10.0),
        vec2(0.0, 10.0),
    ]);
    // adding a circle obstacle in the middle
    let nb_points = 100;
    let radius = 2.5;
    triangulation.add_obstacle(
        (0..nb_points)
            .map(|i| {
                let angle = i as f32 * std::f32::consts::TAU / nb_points as f32;
                let (x, y) = angle.sin_cos();
                vec2(x, y) * radius + vec2(5.0, 5.0)
            })
            .collect(),
    );
    let polygons_before = triangulation.as_navmesh().layers[0].polygons.clone();
    triangulation.simplify(0.1);
    let mesh: Mesh = triangulation.as_navmesh();
    assert!(dbg!(polygons_before.len()) > dbg!(mesh.layers[0].polygons.len()));
    for i in 0..20 {
        for j in 0..20 {
            let point = vec2(i as f32 / 2.0, j as f32 / 2.0);
            if point.distance(vec2(5.0, 5.0)) < radius {
                assert!(!mesh.point_in_mesh(point));
            } else {
                assert!(mesh.point_in_mesh(point));
            }
        }
    }
}

#[test]
fn is_in_mesh_overlapping_simplified() {
    let mut triangulation = Triangulation::from_outer_edges(&[
        vec2(0.0, 0.0),
        vec2(10.0, 0.0),
        vec2(10.0, 10.0),
        vec2(0.0, 10.0),
    ]);
    // adding a circle obstacle in the middle
    let nb_points = 1000;
    let radius = 2.5;
    triangulation.add_obstacle(
        (0..nb_points)
            .map(|i| {
                let angle = i as f32 * std::f32::consts::TAU / nb_points as f32;
                let (x, y) = angle.sin_cos();
                vec2(x, y) * radius + vec2(5.0, 5.0)
            })
            .collect(),
    );
    triangulation.add_obstacle(vec![
        vec2(2.5, 2.5),
        vec2(2.5, 5.0),
        vec2(5.0, 5.0),
        vec2(5.0, 2.5),
    ]);

    let mesh_before = triangulation.as_navmesh();
    triangulation.simplify(0.01);
    let mesh: Mesh = triangulation.as_navmesh();
    assert!(dbg!(mesh_before.layers[0].polygons.len()) > dbg!(mesh.layers[0].polygons.len()));
    let resolution = 5;
    for i in 0..(10 * resolution) {
        for j in 0..(10 * resolution) {
            let point = vec2(i as f32 / resolution as f32, j as f32 / resolution as f32);
            assert_eq!(mesh.point_in_mesh(point), mesh_before.point_in_mesh(point));
        }
    }
}
