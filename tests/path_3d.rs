//! `Mesh::path_3d` and `impl From<Vec3> for Coords`.

use glam::{vec2, vec3, Vec3Swizzles};
use polyanya::{Coords, Mesh, Triangulation};

/// A mesh built from a [`Triangulation`] is flat: it carries no height information at all.
fn flat_mesh() -> Mesh {
    Triangulation::from_outer_edges(&[
        vec2(0.0, 0.0),
        vec2(10.0, 0.0),
        vec2(10.0, 10.0),
        vec2(0.0, 10.0),
    ])
    .as_navmesh()
}

/// The mesh is in the XZ plane, so `y` is the height and is not part of the 2D position.
#[test]
fn from_vec3_keeps_xz() {
    assert_eq!(
        Coords::from(vec3(1.0, 2.0, 3.0)),
        Coords::on_mesh(vec2(1.0, 3.0))
    );
}

/// Which is what makes a `Vec3` usable anywhere a `Coords` is taken.
#[test]
fn a_vec3_can_be_pathed_from() {
    let mesh = flat_mesh();
    let (start, end) = (vec3(1.0, 7.0, 1.0), vec3(9.0, -3.0, 9.0));

    assert_eq!(mesh.path(start, end), mesh.path(start.xz(), end.xz()));
}

/// There is no terrain to follow on a flat mesh, so there is nothing to return.
#[test]
fn path_3d_on_a_flat_mesh_is_none() {
    let mesh = flat_mesh();
    assert!(mesh
        .path_3d(vec3(1.0, 0.0, 1.0), vec3(9.0, 0.0, 9.0))
        .is_none());
}

#[test]
fn path_3d_follows_the_heights() {
    let mut mesh = flat_mesh();
    let vertices = mesh.layers[0].vertices.len();
    mesh.layers[0].set_heights(vec![3.0; vertices]).unwrap();

    // The `y` given doesn't have to be the mesh's: the point is snapped to it.
    let path = mesh
        .path_3d(vec3(1.0, 12.0, 1.0), vec3(9.0, -5.0, 9.0))
        .unwrap();

    assert_eq!(path, vec![vec3(9.0, 3.0, 9.0)]);
}

/// The four-step dance this replaces, done by hand.
#[cfg(feature = "recast")]
#[test]
fn path_3d_matches_doing_it_by_hand() {
    use polyanya::RecastPolyMeshDetail;
    use std::fs::File;

    let detailed_mesh: RecastPolyMeshDetail =
        serde_json::from_reader(File::open("meshes/recast/detail_mesh.json").unwrap()).unwrap();
    let mesh: Mesh = detailed_mesh.try_into().unwrap();

    let start = vec3(46.998413, 9.998184, 1.717747);
    let end = vec3(20.703018, 18.651773, -80.770_2);

    let from = mesh.get_closest_point_at_height(start, start.y).unwrap();
    let to = mesh.get_closest_point_at_height(end, end.y).unwrap();
    let by_hand = mesh
        .path(from, to)
        .unwrap()
        .path_with_height(
            from.position_with_height(&mesh).unwrap(),
            to.position_with_height(&mesh).unwrap(),
            &mesh,
        )
        .unwrap();

    assert_eq!(mesh.path_3d(start, end), Some(by_hand.clone()));

    // The route climbs, and it ends on the mesh rather than at the point asked for.
    assert!(by_hand.iter().any(|p| p.y != start.y));
    assert_eq!(
        *by_hand.last().unwrap(),
        to.position_with_height(&mesh).unwrap()
    );
}

/// The point of taking a `Vec3`: `(-0.25, 30.32)` is under a balcony, so it is both a spot
/// on the balcony at height 19.26 and a spot on the floor below at 8.76. The balcony is a
/// disconnected component; the floor is not. 2D can only answer for both at once, the
/// height picks one.
#[cfg(feature = "recast")]
#[test]
fn path_3d_picks_the_floor_by_height() {
    use polyanya::{RecastFullMesh, RecastPolyMesh, RecastPolyMeshDetail};
    use std::fs::File;

    let poly: RecastPolyMesh =
        serde_json::from_reader(File::open("meshes/recast/poly_mesh-large.json").unwrap()).unwrap();
    let detail: RecastPolyMeshDetail =
        serde_json::from_reader(File::open("meshes/recast/detail_mesh-large.json").unwrap())
            .unwrap();
    let mesh: Mesh = RecastFullMesh::new(poly, detail).try_into().unwrap();

    let goal = vec3(5.0, 7.3436, 24.86);

    // Standing on the floor: there is a path.
    let path = mesh.path_3d(vec3(-0.25, 8.76, 30.32), goal).unwrap();
    assert_eq!(*path.last().unwrap(), goal);

    // Standing on the balcony above the exact same spot: there is not.
    assert!(mesh.path_3d(vec3(-0.25, 19.26, 30.32), goal).is_none());

    // While in 2D the point is on both, so the query is answered from the reachable one.
    assert!(mesh.path(vec2(-0.25, 30.32), goal.xz()).is_some());
}
