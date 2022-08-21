use glam::Vec2;
use polyanya::{Mesh, Polygon, Vertex};

macro_rules! assert_delta {
    ($x:expr, $y:expr) => {
        let val = $x;
        if !((val - $y).abs() < 0.0001) {
            assert_eq!(val, $y);
        }
    };
}

fn arena_mesh() -> Mesh {
    Mesh {
        vertices: vec![
            Vertex::from_coords(2, 2, vec![-1, 1]),
            Vertex::from_coords(1, 3, vec![6, -1]),
            Vertex::from_coords(2, 3, vec![5, 6, -1, 1, 0]),
            Vertex::from_coords(3, 2, vec![2, 0, 1, -1]),
            Vertex::from_coords(3, 1, vec![2, -1]),
            Vertex::from_coords(15, 1, vec![-1, 2]),
            Vertex::from_coords(15, 3, vec![-1, 5, 0, 2]),
            Vertex::from_coords(18, 3, vec![4, 16, 5, -1]),
            Vertex::from_coords(18, 2, vec![-1, 4]),
            Vertex::from_coords(19, 1, vec![3, -1]),
            Vertex::from_coords(20, 1, vec![-1, 3]),
            Vertex::from_coords(20, 2, vec![16, 3, -1]),
            Vertex::from_coords(19, 2, vec![3, 16, 4, -1]),
            Vertex::from_coords(23, 2, vec![16, -1, 28]),
            Vertex::from_coords(23, 1, vec![28, -1]),
            Vertex::from_coords(23, 8, vec![5, 16, 39, -1]),
            Vertex::from_coords(24, 7, vec![39, 16, 41, -1]),
            Vertex::from_coords(24, 8, vec![-1, 39]),
            Vertex::from_coords(23, 10, vec![5, -1, 54]),
            Vertex::from_coords(15, 15, vec![5, -1, 46, 51]),
            Vertex::from_coords(18, 18, vec![-1, 53]),
            Vertex::from_coords(19, 15, vec![5, 54, -1]),
            Vertex::from_coords(19, 18, vec![-1, 54, 52, 53]),
            Vertex::from_coords(18, 19, vec![52, 38, -1, 53]),
            Vertex::from_coords(15, 19, vec![46, -1, 38]),
            Vertex::from_coords(3, 15, vec![5, 51, -1, 6]),
            Vertex::from_coords(1, 15, vec![6, -1]),
            Vertex::from_coords(3, 18, vec![51, 46, 7, -1]),
            Vertex::from_coords(2, 18, vec![7, -1]),
            Vertex::from_coords(2, 23, vec![46, 17, -1, 7]),
            Vertex::from_coords(1, 23, vec![-1, 17]),
            Vertex::from_coords(3, 48, vec![-1, 8]),
            Vertex::from_coords(15, 48, vec![8, -1]),
            Vertex::from_coords(19, 48, vec![-1, 9]),
            Vertex::from_coords(20, 48, vec![9, -1]),
            Vertex::from_coords(24, 48, vec![22, -1]),
            Vertex::from_coords(24, 47, vec![21, 22, -1, 10]),
            Vertex::from_coords(23, 46, vec![42, 47, 21, 10, -1, 27]),
            Vertex::from_coords(23, 47, vec![-1, 10]),
            Vertex::from_coords(19, 47, vec![45, 9, -1]),
            Vertex::from_coords(20, 46, vec![27, -1, 9, 45]),
            Vertex::from_coords(15, 47, vec![45, -1, 8]),
            Vertex::from_coords(3, 47, vec![11, 45, 8, -1]),
            Vertex::from_coords(1, 47, vec![11, -1]),
            Vertex::from_coords(1, 35, vec![-1, 11]),
            Vertex::from_coords(2, 35, vec![45, 11, -1, 12]),
            Vertex::from_coords(2, 34, vec![-1, 12]),
            Vertex::from_coords(3, 34, vec![-1, 13, 45, 12]),
            Vertex::from_coords(1, 31, vec![14, -1]),
            Vertex::from_coords(3, 31, vec![14, 13, -1]),
            Vertex::from_coords(3, 30, vec![-1, 13, 14]),
            Vertex::from_coords(1, 30, vec![-1, 14]),
            Vertex::from_coords(3, 27, vec![18, 46, 13, -1]),
            Vertex::from_coords(2, 27, vec![-1, 18]),
            Vertex::from_coords(1, 26, vec![-1, 17]),
            Vertex::from_coords(2, 26, vec![17, 46, 18, -1]),
            Vertex::from_coords(15, 31, vec![38, -1, 13, 46]),
            Vertex::from_coords(15, 35, vec![27, 45, 13, -1]),
            Vertex::from_coords(18, 35, vec![42, 27, -1, 48]),
            Vertex::from_coords(18, 34, vec![-1, 48]),
            Vertex::from_coords(19, 31, vec![52, 54, 31, 47, -1, 38]),
            Vertex::from_coords(19, 34, vec![-1, 47, 42, 48]),
            Vertex::from_coords(31, 31, vec![47, 31, -1]),
            Vertex::from_coords(31, 35, vec![19, 47, -1]),
            Vertex::from_coords(34, 34, vec![-1, 15]),
            Vertex::from_coords(34, 35, vec![19, -1, 15, 49]),
            Vertex::from_coords(35, 34, vec![50, 49, 15, -1]),
            Vertex::from_coords(35, 31, vec![-1, 31, 50]),
            Vertex::from_coords(47, 31, vec![-1, 50, 31, 26]),
            Vertex::from_coords(47, 35, vec![19, 49, 50, -1, 25]),
            Vertex::from_coords(47, 47, vec![19, 25, -1, 24]),
            Vertex::from_coords(35, 47, vec![-1, 19, 24]),
            Vertex::from_coords(31, 47, vec![23, 19, -1]),
            Vertex::from_coords(29, 46, vec![19, 20, -1, 47]),
            Vertex::from_coords(29, 47, vec![-1, 20]),
            Vertex::from_coords(30, 47, vec![19, 23, -1, 20]),
            Vertex::from_coords(26, 46, vec![47, -1, 22, 21]),
            Vertex::from_coords(26, 48, vec![22, -1]),
            Vertex::from_coords(30, 48, vec![23, -1]),
            Vertex::from_coords(31, 48, vec![23, -1]),
            Vertex::from_coords(35, 48, vec![24, -1]),
            Vertex::from_coords(47, 48, vec![24, -1]),
            Vertex::from_coords(48, 47, vec![25, -1]),
            Vertex::from_coords(48, 35, vec![25, -1]),
            Vertex::from_coords(48, 31, vec![-1, 26]),
            Vertex::from_coords(48, 3, vec![-1, 36]),
            Vertex::from_coords(48, 15, vec![-1, 36]),
            Vertex::from_coords(48, 19, vec![-1, 26]),
            Vertex::from_coords(47, 19, vec![31, 43, 44, -1, 26]),
            Vertex::from_coords(47, 15, vec![-1, 44, 37, 36]),
            Vertex::from_coords(34, 19, vec![31, -1, 40, 43]),
            Vertex::from_coords(31, 19, vec![54, -1, 31]),
            Vertex::from_coords(31, 15, vec![54, 37, -1]),
            Vertex::from_coords(34, 18, vec![-1, 40]),
            Vertex::from_coords(35, 18, vec![44, 43, 40, -1]),
            Vertex::from_coords(35, 15, vec![-1, 37, 44]),
            Vertex::from_coords(26, 10, vec![54, -1, 37]),
            Vertex::from_coords(26, 7, vec![41, 37, -1]),
            Vertex::from_coords(26, 3, vec![37, 41, 16, 28, -1]),
            Vertex::from_coords(26, 1, vec![-1, 28]),
            Vertex::from_coords(29, 3, vec![37, -1, 29, 32]),
            Vertex::from_coords(30, 2, vec![30, 32, 29, -1]),
            Vertex::from_coords(29, 2, vec![-1, 29]),
            Vertex::from_coords(30, 1, vec![30, -1]),
            Vertex::from_coords(31, 1, vec![-1, 30]),
            Vertex::from_coords(31, 3, vec![37, 32, 30, -1]),
            Vertex::from_coords(34, 3, vec![37, -1, 35, 33]),
            Vertex::from_coords(34, 2, vec![-1, 35]),
            Vertex::from_coords(35, 1, vec![34, -1]),
            Vertex::from_coords(35, 2, vec![34, 33, 35, -1]),
            Vertex::from_coords(47, 3, vec![37, 33, 34, -1, 36]),
            Vertex::from_coords(47, 1, vec![-1, 34]),
        ],
        polygons: vec![
            Polygon::new(3, vec![6, 2, 3, 2, 5, 1]),
            Polygon::new(3, vec![3, 2, 0, -1, 0, -1]),
            Polygon::new(4, vec![5, 6, 3, 4, -1, -1, 0, -1]),
            Polygon::new(4, vec![10, 11, 12, 9, -1, -1, 16, -1]),
            Polygon::new(3, vec![12, 7, 8, -1, 16, -1]),
            Polygon::new(
                8,
                vec![7, 15, 18, 21, 19, 25, 2, 6, -1, 16, -1, 54, -1, 51, 6, 0],
            ),
            Polygon::new(4, vec![2, 25, 26, 1, -1, 5, -1, -1]),
            Polygon::new(3, vec![29, 28, 27, 46, -1, -1]),
            Polygon::new(4, vec![42, 41, 32, 31, -1, 45, -1, -1]),
            Polygon::new(4, vec![39, 40, 34, 33, -1, 45, -1, -1]),
            Polygon::new(3, vec![38, 37, 36, -1, -1, 21]),
            Polygon::new(4, vec![45, 42, 43, 44, -1, 45, -1, -1]),
            Polygon::new(3, vec![47, 45, 46, -1, 45, -1]),
            Polygon::new(6, vec![50, 52, 56, 57, 47, 49, 14, -1, 46, -1, 45, -1]),
            Polygon::new(4, vec![50, 49, 48, 51, -1, 13, -1, -1]),
            Polygon::new(3, vec![66, 65, 64, -1, 49, -1]),
            Polygon::new(7, vec![7, 12, 11, 13, 98, 16, 15, 5, 4, 3, -1, 28, 41, 39]),
            Polygon::new(4, vec![30, 29, 55, 54, -1, -1, 46, -1]),
            Polygon::new(3, vec![55, 52, 53, -1, 46, -1]),
            Polygon::new(
                8,
                vec![
                    72, 75, 73, 63, 65, 69, 70, 71, -1, 23, 20, 47, -1, 49, 25, 24,
                ],
            ),
            Polygon::new(3, vec![75, 74, 73, 19, -1, -1]),
            Polygon::new(3, vec![76, 36, 37, 47, 22, 10]),
            Polygon::new(4, vec![36, 76, 77, 35, -1, 21, -1, -1]),
            Polygon::new(4, vec![75, 72, 79, 78, -1, 19, -1, -1]),
            Polygon::new(4, vec![71, 70, 81, 80, -1, 19, -1, -1]),
            Polygon::new(4, vec![70, 69, 83, 82, -1, 19, -1, -1]),
            Polygon::new(4, vec![84, 68, 88, 87, -1, -1, 31, -1]),
            Polygon::new(4, vec![58, 37, 40, 57, -1, 42, -1, 45]),
            Polygon::new(4, vec![99, 98, 13, 14, -1, -1, 16, -1]),
            Polygon::new(3, vec![101, 100, 102, -1, 32, -1]),
            Polygon::new(4, vec![104, 105, 101, 103, -1, -1, 32, -1]),
            Polygon::new(
                7,
                vec![62, 60, 91, 90, 88, 68, 67, -1, 47, 54, -1, 43, 26, 50],
            ),
            Polygon::new(3, vec![105, 100, 101, 30, 37, 29]),
            Polygon::new(3, vec![110, 106, 109, 34, 37, 35]),
            Polygon::new(4, vec![111, 110, 109, 108, -1, -1, 33, -1]),
            Polygon::new(3, vec![109, 106, 107, -1, 33, -1]),
            Polygon::new(4, vec![86, 89, 110, 85, -1, -1, 37, -1]),
            Polygon::new(
                10,
                vec![
                    97, 98, 100, 105, 106, 110, 89, 95, 92, 96, -1, 41, -1, 32, -1, 33, 36, 44, -1,
                    54,
                ],
            ),
            Polygon::new(4, vec![23, 60, 56, 24, -1, 52, -1, 46]),
            Polygon::new(3, vec![17, 15, 16, -1, -1, 16]),
            Polygon::new(3, vec![94, 90, 93, -1, 43, -1]),
            Polygon::new(3, vec![97, 16, 98, 37, -1, 16]),
            Polygon::new(3, vec![61, 37, 58, 48, 47, 27]),
            Polygon::new(3, vec![94, 88, 90, 40, 44, 31]),
            Polygon::new(4, vec![89, 88, 94, 95, 37, -1, 43, -1]),
            Polygon::new(
                7,
                vec![40, 39, 41, 42, 45, 47, 57, 27, 9, -1, 8, 11, 12, 13],
            ),
            Polygon::new(
                7,
                vec![52, 55, 29, 27, 19, 24, 56, 13, 18, 17, 7, 51, -1, 38],
            ),
            Polygon::new(
                7,
                vec![63, 73, 76, 37, 61, 60, 62, -1, 19, -1, 21, 42, -1, 31],
            ),
            Polygon::new(3, vec![61, 58, 59, -1, 42, -1]),
            Polygon::new(3, vec![66, 69, 65, 15, 50, 19]),
            Polygon::new(4, vec![68, 69, 66, 67, 31, -1, 49, -1]),
            Polygon::new(3, vec![19, 27, 25, 5, 46, -1]),
            Polygon::new(3, vec![23, 22, 60, 38, 53, 54]),
            Polygon::new(3, vec![23, 20, 22, 52, -1, -1]),
            Polygon::new(
                7,
                vec![21, 18, 96, 92, 91, 60, 22, -1, 5, -1, 37, -1, 31, 52],
            ),
        ],
    }
}

#[test]
fn arena_scenario_ref_impl() {
    let arena = arena_mesh();

    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(1.0, 12.0)).len, 1.0);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(1.0, 10.0)).len, 2.0);
    assert_delta!(arena.path(Vec2::new(1.0, 13.0), Vec2::new(4.0, 12.0)).len, 3.16228);
    assert_delta!(arena.path(Vec2::new(1.0, 3.0), Vec2::new(3.0, 1.0)).len, 3.41421);
    assert_delta!(arena.path(Vec2::new(1.0, 3.0), Vec2::new(4.0, 3.0)).len, 3.0);
    assert_delta!(arena.path(Vec2::new(1.0, 4.0), Vec2::new(4.0, 2.0)).len, 3.60555);
    assert_delta!(arena.path(Vec2::new(1.0, 40.0), Vec2::new(2.0, 39.0)).len, 1.41421);
    assert_delta!(arena.path(Vec2::new(1.0, 41.0), Vec2::new(1.0, 39.0)).len, 2.0);
    assert_delta!(arena.path(Vec2::new(1.0, 41.0), Vec2::new(1.0, 44.0)).len, 3.0);
    assert_delta!(arena.path(Vec2::new(1.0, 42.0), Vec2::new(4.0, 43.0)).len, 3.16228);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(7.0, 10.0)).len, 6.0);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(1.0, 4.0)).len, 7.0);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(7.0, 14.0)).len, 6.7082);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(5.0, 7.0)).len, 6.40312);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(6.0, 15.0)).len, 5.83095);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(8.0, 11.0)).len, 7.07107);
    assert_delta!(arena.path(Vec2::new(1.0, 14.0), Vec2::new(1.0, 9.0)).len, 5.0);
    assert_delta!(arena.path(Vec2::new(1.0, 24.0), Vec2::new(7.0, 26.0)).len, 6.32456);
    assert_delta!(arena.path(Vec2::new(1.0, 25.0), Vec2::new(5.0, 25.0)).len, 4.0);
    assert_delta!(arena.path(Vec2::new(1.0, 35.0), Vec2::new(5.0, 33.0)).len, 4.60555);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(4.0, 18.0)).len, 7.63441);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(12.0, 14.0)).len, 11.1803);
    assert_delta!(arena.path(Vec2::new(1.0, 13.0), Vec2::new(4.0, 23.0)).len, 10.8907);
    assert_delta!(arena.path(Vec2::new(1.0, 13.0), Vec2::new(5.0, 3.0)).len, 10.7703);
    assert_delta!(arena.path(Vec2::new(1.0, 13.0), Vec2::new(6.0, 7.0)).len, 7.81025);
    assert_delta!(arena.path(Vec2::new(1.0, 13.0), Vec2::new(7.0, 7.0)).len, 8.48528);
    assert_delta!(arena.path(Vec2::new(1.0, 23.0), Vec2::new(7.0, 32.0)).len, 10.8167);
    assert_delta!(arena.path(Vec2::new(1.0, 24.0), Vec2::new(11.0, 25.0)).len, 10.0499);
    assert_delta!(arena.path(Vec2::new(1.0, 24.0), Vec2::new(6.0, 32.0)).len, 9.4365);
    assert_delta!(arena.path(Vec2::new(1.0, 25.0), Vec2::new(9.0, 24.0)).len, 8.06226);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(11.0, 19.0)).len, 13.4536);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(13.0, 11.0)).len, 12.0416);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(10.0, 2.0)).len, 12.7279);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(11.0, 21.0)).len, 13.4536);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(13.0, 13.0)).len, 12.0416);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(14.0, 12.0)).len, 13.0);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(6.0, 25.0)).len, 14.0459);
    assert_delta!(arena.path(Vec2::new(1.0, 13.0), Vec2::new(11.0, 3.0)).len, 14.1421);
    assert_delta!(arena.path(Vec2::new(1.0, 13.0), Vec2::new(13.0, 11.0)).len, 12.1655);
    assert_delta!(arena.path(Vec2::new(1.0, 14.0), Vec2::new(6.0, 23.0)).len, 10.7801);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(18.0, 11.0)).len, 17.0294);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(16.0, 14.0)).len, 15.2971);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(14.0, 2.0)).len, 16.4012);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(17.0, 13.0)).len, 16.0312);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(9.0, 28.0)).len, 17.9234);
    assert_delta!(arena.path(Vec2::new(1.0, 13.0), Vec2::new(4.0, 30.0)).len, 17.8617);
    assert_delta!(arena.path(Vec2::new(1.0, 13.0), Vec2::new(9.0, 26.0)).len, 15.3584);
    assert_delta!(arena.path(Vec2::new(1.0, 14.0), Vec2::new(14.0, 22.0)).len, 15.2745);
    assert_delta!(arena.path(Vec2::new(1.0, 23.0), Vec2::new(10.0, 8.0)).len, 18.0);
    assert_delta!(arena.path(Vec2::new(1.0, 23.0), Vec2::new(14.0, 9.0)).len, 19.4391);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(13.0, 29.0)).len, 22.4722);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(18.0, 22.0)).len, 20.8087);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(19.0, 18.0)).len, 21.0575);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(21.0, 2.0)).len, 21.5407);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(5.0, 32.0)).len, 22.5024);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(6.0, 29.0)).len, 19.703);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(20.0, 7.0)).len, 19.4165);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(21.0, 17.0)).len, 21.2675);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(22.0, 16.0)).len, 21.6014);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(8.0, 29.0)).len, 19.3382);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(22.0, 22.0)).len, 24.2591);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(5.0, 33.0)).len, 23.4959);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(10.0, 32.0)).len, 22.8569);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(20.0, 31.0)).len, 27.5862);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(21.0, 23.0)).len, 23.3238);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(24.0, 14.0)).len, 23.1948);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(25.0, 4.0)).len, 25.0);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(3.0, 36.0)).len, 25.4721);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(19.0, 27.0)).len, 23.4307);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(2.0, 37.0)).len, 25.7678);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(22.0, 31.0)).len, 29.6985);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(24.0, 27.0)).len, 28.6007);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(28.0, 15.0)).len, 27.4591);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(7.0, 39.0)).len, 29.7162);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(12.0, 35.0)).len, 26.4038);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(28.0, 18.0)).len, 27.9259);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(5.0, 40.0)).len, 29.552);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(26.0, 3.0)).len, 26.5707);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(29.0, 14.0)).len, 28.0713);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(29.0, 6.0)).len, 28.6362);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(25.0, 36.0)).len, 35.3836);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(27.0, 25.0)).len, 30.0597);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(32.0, 4.0)).len, 31.5753);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(33.0, 4.0)).len, 32.5576);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(10.0, 42.0)).len, 32.3648);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(27.0, 28.0)).len, 31.0644);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(30.0, 2.0)).len, 30.5347);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(31.0, 3.0)).len, 31.0483);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(5.0, 42.0)).len, 31.5461);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(18.0, 37.0)).len, 30.5349);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(15.0, 43.0)).len, 35.8469);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(21.0, 41.0)).len, 37.1384);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(27.0, 37.0)).len, 37.4833);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(29.0, 38.0)).len, 39.598);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(31.0, 25.0)).len, 33.7313);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(38.0, 13.0)).len, 37.1214);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(38.0, 6.0)).len, 37.2305);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(39.0, 6.0)).len, 38.2281);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(40.0, 9.0)).len, 39.0357);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(11.0, 43.0)).len, 33.5926);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(12.0, 47.0)).len, 38.6267);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(14.0, 47.0)).len, 39.223);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(16.0, 46.0)).len, 39.0);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(28.0, 41.0)).len, 41.1096);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(37.0, 21.0)).len, 37.6552);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(39.0, 24.0)).len, 40.6133);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(16.0, 45.0)).len, 37.1677);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(21.0, 43.0)).len, 37.7849);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(32.0, 39.0)).len, 41.7732);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(34.0, 29.0)).len, 37.5954);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(29.0, 43.0)).len, 43.2791);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(43.0, 15.0)).len, 42.2966);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(43.0, 17.0)).len, 42.6119);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(45.0, 10.0)).len, 44.0);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(46.0, 15.0)).len, 45.2769);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(46.0, 3.0)).len, 45.5412);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(30.0, 45.0)).len, 44.7117);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(35.0, 41.0)).len, 45.3431);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(35.0, 42.0)).len, 46.0109);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(43.0, 3.0)).len, 42.7551);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(31.0, 46.0)).len, 46.868);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(34.0, 46.0)).len, 48.8365);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(35.0, 46.0)).len, 49.5177);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(36.0, 42.0)).len, 47.4236);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(38.0, 45.0)).len, 50.9313);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(46.0, 18.0)).len, 45.7674);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(37.0, 44.0)).len, 48.8365);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(41.0, 35.0)).len, 46.6573);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(43.0, 27.0)).len, 45.245);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(44.0, 25.0)).len, 45.3649);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(41.0, 40.0)).len, 50.2316);
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(43.0, 40.0)).len, 52.0087);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(39.0, 47.0)).len, 52.345);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(42.0, 46.0)).len, 53.9073);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(45.0, 33.0)).len, 49.2304);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(43.0, 43.0)).len, 52.2243);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(44.0, 38.0)).len, 50.3504);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(46.0, 34.0)).len, 50.0908);
    assert_delta!(arena.path(Vec2::new(1.0, 13.0), Vec2::new(42.0, 40.0)).len, 49.2852);
    assert_delta!(arena.path(Vec2::new(1.0, 14.0), Vec2::new(46.0, 32.0)).len, 48.4665);
    assert_delta!(arena.path(Vec2::new(1.0, 14.0), Vec2::new(44.0, 46.0)).len, 53.6365);
    assert_delta!(arena.path(Vec2::new(1.0, 14.0), Vec2::new(46.0, 43.0)).len, 53.6454);
    assert_delta!(arena.path(Vec2::new(1.0, 35.0), Vec2::new(46.0, 3.0)).len, 55.4059);
    assert_delta!(arena.path(Vec2::new(1.0, 37.0), Vec2::new(43.0, 1.0)).len, 55.3173);
    assert_delta!(arena.path(Vec2::new(1.0, 38.0), Vec2::new(43.0, 3.0)).len, 54.6717);
    assert_delta!(arena.path(Vec2::new(1.0, 38.0), Vec2::new(47.0, 13.0)).len, 52.3546);
    assert_delta!(arena.path(Vec2::new(1.0, 39.0), Vec2::new(47.0, 14.0)).len, 52.3599);
    assert_delta!(arena.path(Vec2::new(1.0, 4.0), Vec2::new(38.0, 47.0)).len, 56.7274);
    assert_delta!(arena.path(Vec2::new(1.0, 4.0), Vec2::new(41.0, 42.0)).len, 55.3523);
    assert_delta!(arena.path(Vec2::new(1.0, 42.0), Vec2::new(44.0, 5.0)).len, 56.7621);
    assert_delta!(arena.path(Vec2::new(1.0, 3.0), Vec2::new(41.0, 47.0)).len, 59.4702);
    assert_delta!(arena.path(Vec2::new(1.0, 3.0), Vec2::new(47.0, 37.0)).len, 57.2423);
    assert_delta!(arena.path(Vec2::new(1.0, 39.0), Vec2::new(46.0, 1.0)).len, 58.8982);
    assert_delta!(arena.path(Vec2::new(1.0, 4.0), Vec2::new(43.0, 46.0)).len, 59.4245);
    assert_delta!(arena.path(Vec2::new(1.0, 4.0), Vec2::new(44.0, 45.0)).len, 59.5469);
    assert_delta!(arena.path(Vec2::new(1.0, 40.0), Vec2::new(47.0, 3.0)).len, 59.0512);
    assert_delta!(arena.path(Vec2::new(1.0, 41.0), Vec2::new(46.0, 2.0)).len, 59.5483);
    assert_delta!(arena.path(Vec2::new(1.0, 45.0), Vec2::new(47.0, 9.0)).len, 58.6718);
    assert_delta!(arena.path(Vec2::new(1.0, 7.0), Vec2::new(47.0, 44.0)).len, 59.3941);
    assert_delta!(arena.path(Vec2::new(1.0, 7.0), Vec2::new(47.0, 46.0)).len, 60.4531);
}
