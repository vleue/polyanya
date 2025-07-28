use glam::{vec3, Vec3Swizzles};
use polyanya::{Mesh, RecastPolyMesh, RecastPolyMeshDetail};

macro_rules! assert_delta {
    ($x:expr, $y:expr) => {
        let val = $x.unwrap().length;
        if !((val - $y).abs() < 0.0001) {
            assert_eq!(val, $y);
        }
    };
}

#[test]
fn poly_mesh() {
    let mesh: Mesh = RecastPolyMesh::from_file("meshes/recast/poly_mesh.json")
        .try_into()
        .unwrap();

    let start = vec3(46.998413, 9.998184, 1.717747);
    let end = vec3(20.703018, 18.651773, -80.770203);

    let path = mesh.path(start.xz(), end.xz());
    assert!(path.is_some());
    assert_eq!(path.as_ref().unwrap().path.len(), 10);
    assert_delta!(path, 126.75868);
}

#[test]
fn detailed_mesh() {
    let mesh: Mesh = RecastPolyMeshDetail::from_file("meshes/recast/detail_mesh.json")
        .try_into()
        .unwrap();

    let start = vec3(46.998413, 9.998184, 1.717747);
    let end = vec3(20.703018, 18.651773, -80.770203);

    let path = mesh.path(start.xz(), end.xz());
    assert!(path.is_some());
    assert_eq!(path.as_ref().unwrap().path.len(), 10);
    assert_delta!(path, 126.75868);
}
