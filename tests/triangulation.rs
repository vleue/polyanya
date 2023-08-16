use glam::vec2;
use polyanya::{Mesh, Triangulation};

#[test]
fn is_in_mesh() {
    let mut triangulation = Triangulation::from_outer_edges(vec![
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
    let mut triangulation = Triangulation::from_outer_edges(vec![
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
fn is_in_mesh_overlapping() {
    let mut triangulation = Triangulation::from_outer_edges(vec![
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
    triangulation.merge_overlapping_obstacles();
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
