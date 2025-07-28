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
    assert_eq!(
        path.as_ref().unwrap().path_through_polygons,
        vec![
            126, 114, 115, 116, 131, 128, 121, 124, 122, 93, 92, 96, 95, 90, 72, 71, 59, 54, 58,
            57, 45, 46, 49, 50, 47, 38, 39, 41, 30, 31, 32, 28, 24, 27, 21, 19, 25
        ]
    );
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
    assert_eq!(
        path.as_ref().unwrap().path_through_polygons,
        vec![
            309, 284, 288, 287, 286, 289, 321, 320, 312, 313, 300, 307, 306, 305, 304, 302, 301,
            232, 231, 240, 239, 238, 236, 235, 225, 226, 227, 228, 177, 178, 179, 180, 176, 151,
            152, 153, 140, 141, 142, 150, 149, 146, 145, 126, 127, 128, 134, 135, 136, 130, 129,
            115, 114, 116, 117, 118, 119, 122, 121, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103,
            88, 89, 72, 71, 70, 69, 68, 81, 80, 86, 63, 62, 59, 74, 75, 76
        ]
    );
    assert_delta!(path, 126.75868);
}
