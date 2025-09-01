use std::fs::File;

use glam::{vec3, Vec3Swizzles};
use polyanya::{Mesh, RecastFullMesh, RecastPolyMesh, RecastPolyMeshDetail};

macro_rules! assert_delta {
    ($x:expr, $y:expr) => {
        let val = $x.unwrap().length;
        if !((val - $y).abs() < 0.0001) {
            assert_eq!(val, $y);
        }
    };
}

#[test]
fn detailed_mesh() {
    let detailed_mesh: RecastPolyMeshDetail =
        serde_json::from_reader(File::open("meshes/recast/detail_mesh.json").unwrap()).unwrap();
    let mesh: Mesh = detailed_mesh.into();

    let start = vec3(46.998413, 9.998184, 1.717747);
    let end = vec3(20.703018, 18.651773, -80.770203);

    let path = mesh.path(start.xz(), end.xz());
    assert!(path.is_some());
    assert_eq!(path.as_ref().unwrap().path.len(), 10);
    assert_eq!(
        path.as_ref().unwrap().polygons(),
        vec![
            (0, 309),
            (0, 284),
            (0, 288),
            (0, 287),
            (0, 286),
            (0, 289),
            (0, 321),
            (0, 320),
            (0, 312),
            (0, 313),
            (0, 300),
            (0, 307),
            (0, 306),
            (0, 305),
            (0, 304),
            (0, 302),
            (0, 301),
            (0, 232),
            (0, 231),
            (0, 240),
            (0, 239),
            (0, 238),
            (0, 236),
            (0, 235),
            (0, 225),
            (0, 226),
            (0, 227),
            (0, 228),
            (0, 177),
            (0, 178),
            (0, 179),
            (0, 180),
            (0, 176),
            (0, 151),
            (0, 152),
            (0, 153),
            (0, 140),
            (0, 141),
            (0, 142),
            (0, 150),
            (0, 149),
            (0, 146),
            (0, 145),
            (0, 126),
            (0, 127),
            (0, 128),
            (0, 134),
            (0, 135),
            (0, 136),
            (0, 130),
            (0, 129),
            (0, 115),
            (0, 114),
            (0, 116),
            (0, 117),
            (0, 118),
            (0, 119),
            (0, 122),
            (0, 121),
            (0, 93),
            (0, 94),
            (0, 95),
            (0, 96),
            (0, 97),
            (0, 98),
            (0, 99),
            (0, 100),
            (0, 101),
            (0, 102),
            (0, 103),
            (0, 88),
            (0, 89),
            (0, 72),
            (0, 71),
            (0, 70),
            (0, 69),
            (0, 68),
            (0, 81),
            (0, 80),
            (0, 86),
            (0, 63),
            (0, 62),
            (0, 59),
            (0, 74),
            (0, 75),
            (0, 76)
        ]
    );
    assert_delta!(path, 126.75868);
}

#[test]
fn full_mesh() {
    let rasterised: RecastPolyMesh =
        serde_json::from_reader(File::open("meshes/recast/poly_mesh.json").unwrap()).unwrap();
    let detailed: RecastPolyMeshDetail =
        serde_json::from_reader(File::open("meshes/recast/detail_mesh.json").unwrap()).unwrap();
    let mesh: polyanya::Mesh = RecastFullMesh::new(rasterised, detailed).into();

    let start = vec3(46.998413, 9.998184, 1.717747);
    let end = vec3(20.703018, 18.651773, -80.770203);

    let path = mesh.path(start.xz(), end.xz());
    assert!(path.is_some());
    assert_eq!(path.as_ref().unwrap().path.len(), 10);
    assert_eq!(
        path.as_ref().unwrap().polygons(),
        vec![
            (0, 271),
            (1, 2),
            (1, 6),
            (1, 5),
            (1, 4),
            (1, 7),
            (0, 283),
            (0, 282),
            (0, 274),
            (0, 275),
            (2, 12),
            (2, 19),
            (2, 18),
            (2, 17),
            (2, 16),
            (2, 14),
            (2, 13),
            (2, 2),
            (2, 1),
            (2, 10),
            (2, 9),
            (2, 8),
            (2, 6),
            (2, 5),
            (0, 215),
            (0, 216),
            (0, 217),
            (0, 218),
            (4, 3),
            (4, 4),
            (4, 5),
            (4, 6),
            (4, 2),
            (0, 151),
            (0, 152),
            (0, 153),
            (0, 140),
            (0, 141),
            (0, 142),
            (0, 150),
            (0, 149),
            (0, 146),
            (0, 145),
            (0, 126),
            (0, 127),
            (0, 128),
            (0, 134),
            (0, 135),
            (0, 136),
            (0, 130),
            (0, 129),
            (0, 115),
            (0, 114),
            (0, 116),
            (0, 117),
            (0, 118),
            (0, 119),
            (0, 122),
            (0, 121),
            (0, 93),
            (0, 94),
            (0, 95),
            (0, 96),
            (0, 97),
            (0, 98),
            (0, 99),
            (0, 100),
            (0, 101),
            (0, 102),
            (0, 103),
            (0, 88),
            (0, 89),
            (0, 72),
            (0, 71),
            (0, 70),
            (0, 69),
            (0, 68),
            (0, 81),
            (0, 80),
            (0, 86),
            (0, 63),
            (0, 62),
            (0, 59),
            (0, 74),
            (0, 75),
            (0, 76)
        ]
    );
    assert_delta!(path, 126.75868);
}
