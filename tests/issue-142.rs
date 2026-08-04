use std::collections::{HashSet, VecDeque};

use glam::{vec2, Vec2};
use polyanya::{Mesh, Trimesh};

/// Two points 2 units apart, in the region the reporter of #142 pointed at.
const FROM: Vec2 = Vec2::new(-5.982365, 32.459633);
const TO: Vec2 = Vec2::new(-6.479097, 30.450762);

fn stairs() -> Mesh {
    let raw = std::fs::read_to_string("meshes/issue-142.trimesh").unwrap();
    let mut vertices = Vec::new();
    let mut triangles = Vec::new();
    for line in raw.lines() {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let x = parts.next().unwrap().parse().unwrap();
                let y = parts.next().unwrap().parse().unwrap();
                vertices.push(vec2(x, y));
            }
            Some("t") => {
                let a = parts.next().unwrap().parse().unwrap();
                let b = parts.next().unwrap().parse().unwrap();
                let c = parts.next().unwrap().parse().unwrap();
                triangles.push([a, b, c]);
            }
            _ => {}
        }
    }
    let mut mesh: Mesh = Trimesh {
        vertices,
        triangles,
    }
    .try_into()
    .unwrap();
    mesh.bake();
    mesh
}

/// Flood fill over the same adjacency the search walks: two polygons are
/// neighbours when they share an edge, that is when both of its vertices list
/// the same polygon.
fn reachable(mesh: &Mesh, from: u32, to: u32) -> bool {
    let layer = &mesh.layers[0];
    let mut seen = HashSet::from([from]);
    let mut queue = VecDeque::from([from]);
    while let Some(polygon) = queue.pop_front() {
        if polygon == to {
            return true;
        }
        let ring = &layer.polygons[polygon as usize].vertices;
        for position in 0..ring.len() {
            let a = &layer.vertices[ring[position] as usize];
            let b = &layer.vertices[ring[(position + 1) % ring.len()] as usize];
            for &next in &a.polygons {
                if next != u32::MAX && b.polygons.contains(&next) && seen.insert(next) {
                    queue.push_back(next);
                }
            }
        }
    }
    false
}

#[test]
fn a_reachable_target_two_units_away_is_reached() {
    let mesh = stairs();
    let start = mesh.get_closest_point(FROM).unwrap();
    let goal = mesh.get_closest_point(TO).unwrap();
    assert!(
        reachable(&mesh, start.polygon(), goal.polygon()),
        "the fixture is meant to have the two ends connected"
    );

    let path = mesh
        .path(FROM, TO)
        .expect("a path exists but was not found");
    assert!(path.length >= FROM.distance(TO));
    assert_eq!(path.path.last(), Some(&TO));
}

/// The same mesh, both ways round. Before the fix one direction answered in
/// microseconds and the other burned the whole iteration limit, which is what
/// made the report read as "A to B and B to C work, A to C does not".
#[test]
fn both_directions_are_reached() {
    let mesh = stairs();
    assert!(mesh.path(FROM, TO).is_some());
    assert!(mesh.path(TO, FROM).is_some());
}
