use glam::vec2;
use polyanya::*;

#[test]
fn test_harmonia() {
    let vertices = vec![
        Vertex {
            coords: vec2(-25.0, -25.0),
            polygons: vec![6, 14, 4, 4294967295, 10, 9, 0, 8, 16],
            is_corner: true,
        },
        Vertex {
            coords: vec2(25.0, -25.0),
            polygons: vec![10, 4294967295, 12, 5, 2, 19, 1, 7],
            is_corner: true,
        },
        Vertex {
            coords: vec2(25.0, 25.0),
            polygons: vec![12, 4294967295, 15, 13, 11],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-25.0, 25.0),
            polygons: vec![4, 15, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.7, -32.3),
            polygons: vec![7, 1, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-0.9, -20.0),
            polygons: vec![18, 3, 4294967295, 1, 19],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-0.9, -20.0),
            polygons: vec![3, 17, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.8, -32.2),
            polygons: vec![16, 8, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.9, -32.7),
            polygons: vec![0, 4294967295, 8],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.7, -32.7),
            polygons: vec![9, 4294967295, 0],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.7, -32.7),
            polygons: vec![9, 10, 7, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-0.9, -19.8),
            polygons: vec![2, 5, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-0.6, -14.7),
            polygons: vec![5, 12, 11, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-0.6, -14.7),
            polygons: vec![13, 4294967295, 11],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-0.8, -14.7),
            polygons: vec![13, 15, 4, 14, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.0, -19.8),
            polygons: vec![14, 6, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.0, -19.9),
            polygons: vec![6, 16, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-0.9, -20.0033),
            polygons: vec![3, 18, 4294967295, 17],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.0, -20.0),
            polygons: vec![17, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-0.9, -20.0035),
            polygons: vec![18, 19, 2, 4294967295],
            is_corner: true,
        },
    ];
    let polygons = vec![
        Polygon {
            vertices: vec![8, 0, 9],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![4, 1, 5],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![19, 1, 11],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![6, 5, 17],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![3, 0, 14],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![11, 1, 12],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![0, 16, 15],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![10, 1, 4],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![0, 8, 7],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![9, 0, 10],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![1, 10, 0],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![12, 2, 13],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![2, 12, 1],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![13, 2, 14],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![15, 14, 0],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![3, 14, 2],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![0, 7, 16],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![6, 17, 18],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![5, 19, 17],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![19, 5, 1],
            is_one_way: false,
        },
    ];

    let mesh = Mesh::new(vertices, polygons).unwrap();

    let path_from = mesh.path(vec2(-1.5, -19.0), vec2(1.5, -19.0));
    eprintln!("{:?}", path_from);
    assert!(path_from.is_some());
    let path_to = mesh.path(vec2(1.5, -19.0), vec2(-1.5, -19.0));
    eprintln!("{:?}", path_to);
    assert!(path_to.is_some());
}

#[test]
fn test_harmonia_2() {
    let vertices = vec![
        Vertex {
            coords: vec2(-250.0, -250.0),
            polygons: vec![7, 5, 15, 16, 13, 3, 4294967295, 9, 8, 0],
            is_corner: true,
        },
        Vertex {
            coords: vec2(250.0, -250.0),
            polygons: vec![2, 20, 6, 9, 4294967295, 11, 4],
            is_corner: true,
        },
        Vertex {
            coords: vec2(250.0, 250.0),
            polygons: vec![11, 4294967295, 14, 12, 10],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-250.0, 250.0),
            polygons: vec![3, 14, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.7396733, -32.28912),
            polygons: vec![19, 4294967295, 6, 20],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.0437447, -20.765087),
            polygons: vec![1, 4294967295, 18],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.193472, -20.756046),
            polygons: vec![17, 15, 5, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.8894006, -32.28008),
            polygons: vec![5, 7, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.9151752, -32.706886),
            polygons: vec![0, 4294967295, 7],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.7707951, -32.715603),
            polygons: vec![8, 4294967295, 0],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.7654479, -32.715927),
            polygons: vec![8, 9, 6, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.0306296, -20.550608),
            polygons: vec![1, 2, 4, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-0.67642397, -14.758028),
            polygons: vec![4, 11, 10, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-0.6817711, -14.757701),
            polygons: vec![12, 4294967295, 10],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-0.8261443, -14.748873),
            polygons: vec![12, 14, 3, 13, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.1803501, -20.541452),
            polygons: vec![13, 16, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.1934686, -20.75599),
            polygons: vec![17, 4294967295, 16, 15],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.1187924, -20.760555),
            polygons: vec![17, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.0437481, -20.765144),
            polygons: vec![19, 18, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-1.043748, -20.765142),
            polygons: vec![18, 19, 20, 2, 1],
            is_corner: false,
        },
    ];
    let polygons = vec![
        Polygon {
            vertices: vec![8, 0, 9],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![5, 19, 11],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![1, 11, 19],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![3, 0, 14],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![11, 1, 12],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![7, 6, 0],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![10, 1, 4],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![0, 8, 7],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![9, 0, 10],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![1, 10, 0],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![12, 2, 13],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![2, 12, 1],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![13, 2, 14],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![15, 14, 0],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![3, 14, 2],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![16, 0, 6],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![16, 15, 0],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![16, 6, 17],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![5, 18, 19],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![18, 4, 19],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![4, 1, 19],
            is_one_way: false,
        },
    ];
    let mesh = Mesh::new(vertices, polygons).unwrap();

    let path_from = mesh.path(vec2(-1.5, -19.0), vec2(1.5, -19.0));
    eprintln!("{:?}", path_from);
    assert!(path_from.is_some());
    let path_to = mesh.path(vec2(1.5, -19.0), vec2(-1.5, -19.0));
    eprintln!("{:?}", path_to);
    assert!(path_to.is_some());
}

#[test]
fn test_harmonia_3() {
    let vertices = vec![
        Vertex {
            coords: vec2(-250.0, -250.0),
            polygons: vec![16, 15, 14, 9, 1, 0, 4294967295, 27, 6, 4, 10],
            is_corner: true,
        },
        Vertex {
            coords: vec2(250.0, -250.0),
            polygons: vec![27, 4294967295, 21, 19, 24, 28, 29, 35],
            is_corner: true,
        },
        Vertex {
            coords: vec2(250.0, 250.0),
            polygons: vec![48, 21, 4294967295, 49],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-250.0, 250.0),
            polygons: vec![45, 49, 4294967295, 0, 3, 40, 39],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-26.281681, -11.768875),
            polygons: vec![3, 0, 1, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-27.447628, -27.625782),
            polygons: vec![12, 4294967295, 1, 9],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-27.442286, -27.626175),
            polygons: vec![12, 13, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-26.132084, -11.779874),
            polygons: vec![22, 44, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-26.233156, -11.181979),
            polygons: vec![42, 41, 2, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-26.238499, -11.181586),
            polygons: vec![2, 40, 3, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-26.981728, -33.50971),
            polygons: vec![10, 4, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-15.284523, -33.031345),
            polygons: vec![6, 36, 26, 4294967295, 4],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-15.284741, -33.025993),
            polygons: vec![26, 34, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-26.987858, -33.359837),
            polygons: vec![33, 8, 18, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-27.415167, -33.52207),
            polygons: vec![17, 11, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-27.414948, -33.527424),
            polygons: vec![11, 16, 10, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-27.341402, -33.245174),
            polygons: vec![18, 8, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-27.297852, -27.634348),
            polygons: vec![22, 4294967295, 8, 33, 5, 43],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-27.447826, -27.630701),
            polygons: vec![13, 12, 9, 14, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-27.372833, -27.631283),
            polygons: vec![13, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-27.491396, -33.244007),
            polygons: vec![14, 15, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-27.49301, -33.451904),
            polygons: vec![15, 16, 11, 17, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-27.418013, -33.452488),
            polygons: vec![17, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-27.342405, -33.374336),
            polygons: vec![18, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-11.266649, -32.54793),
            polygons: vec![24, 19, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-10.625031, -27.672806),
            polygons: vec![46, 4294967295, 19, 21, 48, 47, 38],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-10.6303425, -27.672108),
            polygons: vec![37, 4294967295, 46],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-10.773749, -27.653233),
            polygons: vec![37, 7, 43, 5, 20, 23, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-11.415367, -32.528362),
            polygons: vec![23, 32, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-11.295723, -32.72779),
            polygons: vec![31, 30, 25, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-11.290412, -32.72849),
            polygons: vec![25, 28, 24, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-15.142953, -33.022785),
            polygons: vec![35, 29, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-11.360234, -32.793568),
            polygons: vec![25, 30, 4294967295, 29, 28],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-11.3605585, -32.78822),
            polygons: vec![30, 31, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-11.364771, -32.718704),
            polygons: vec![31, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-15.152026, -32.87306),
            polygons: vec![32, 23, 20, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-11.431057, -32.647583),
            polygons: vec![32, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-15.291501, -32.881508),
            polygons: vec![20, 5, 33, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-15.283375, -33.025925),
            polygons: vec![26, 36, 4294967295, 34],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-15.287586, -32.956425),
            polygons: vec![34, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-15.283051, -33.031273),
            polygons: vec![35, 4294967295, 36, 6, 27],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-14.250227, -11.015068),
            polygons: vec![45, 39, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-26.16431, -11.112088),
            polygons: vec![39, 40, 2, 41, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-26.164267, -11.117445),
            polygons: vec![41, 42, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-26.1637, -11.187086),
            polygons: vec![42, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-14.249005, -11.165064),
            polygons: vec![7, 4294967295, 44, 22, 43],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-26.09397, -11.261521),
            polygons: vec![44, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-13.807747, -11.16147),
            polygons: vec![38, 4294967295, 7, 37, 46],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-13.808924, -11.016831),
            polygons: vec![38, 47, 4294967295],
            is_corner: true,
        },
        Vertex {
            coords: vec2(-13.808968, -11.011475),
            polygons: vec![47, 48, 49, 45, 4294967295],
            is_corner: true,
        },
    ];
    let polygons = vec![
        Polygon {
            vertices: vec![4, 3, 0],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![5, 4, 0],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![9, 8, 42],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![9, 3, 4],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![10, 0, 11],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![17, 37, 27],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![11, 0, 40],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![45, 27, 47],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![16, 13, 17],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![18, 5, 0],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![15, 0, 10],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![15, 14, 21],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![5, 18, 6],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![6, 18, 19],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![20, 18, 0],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![21, 20, 0],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![21, 0, 15],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![21, 14, 22],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![13, 16, 23],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![24, 1, 25],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![35, 27, 37],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![2, 25, 1],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![17, 45, 7],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![28, 27, 35],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![30, 1, 24],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![30, 29, 32],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![12, 11, 38],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![1, 40, 0],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![32, 1, 30],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![32, 31, 1],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![33, 32, 29],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![33, 29, 34],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![28, 35, 36],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![17, 13, 37],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![12, 38, 39],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![40, 1, 31],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![11, 40, 38],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![26, 47, 27],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![47, 25, 48],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![42, 41, 3],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![42, 3, 9],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![43, 42, 8],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![43, 8, 44],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![45, 17, 27],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![7, 45, 46],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![3, 41, 49],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![25, 47, 26],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![48, 25, 49],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![2, 49, 25],
            is_one_way: false,
        },
        Polygon {
            vertices: vec![49, 2, 3],
            is_one_way: false,
        },
    ];
    let mesh = Mesh::new(vertices, polygons).unwrap();

    let path_from = mesh.path(vec2(-20.197222, -18.893484), vec2(-32.41096, -21.870886));
    eprintln!("{:?}", path_from);
    assert!(path_from.is_some());
    let path_to = mesh.path(vec2(-32.41096, -21.870886), vec2(-20.197222, -18.893484));
    eprintln!("{:?}", path_to);
    assert!(path_to.is_some());
}
