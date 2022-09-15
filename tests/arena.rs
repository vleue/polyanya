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
    Mesh::new(
        vec![
            Vertex::from_coords(2, 2, vec![-1, 4]),
            Vertex::from_coords(1, 3, vec![15, -1]),
            Vertex::from_coords(2, 3, vec![1, 15, -1, 4, 0, 14]),
            Vertex::from_coords(3, 2, vec![2, 0, 4, -1, 5]),
            Vertex::from_coords(3, 1, vec![5, -1]),
            Vertex::from_coords(15, 1, vec![-1, 2, 5]),
            Vertex::from_coords(15, 3, vec![6, 3, 14, 0, 2, -1]),
            Vertex::from_coords(18, 3, vec![95, 37, 6, -1, 12, 7, 92]),
            Vertex::from_coords(18, 2, vec![-1, 12]),
            Vertex::from_coords(19, 1, vec![9, -1]),
            Vertex::from_coords(20, 1, vec![-1, 10, 9]),
            Vertex::from_coords(20, 2, vec![11, 101, 92, 7, 10, -1]),
            Vertex::from_coords(19, 2, vec![10, 7, 12, -1, 9]),
            Vertex::from_coords(23, 2, vec![11, -1, 62, 20, 100]),
            Vertex::from_coords(23, 1, vec![62, -1]),
            Vertex::from_coords(23, 8, vec![95, 92, 101, 104, -1]),
            Vertex::from_coords(24, 7, vec![108, -1, 104, 101, 11, 100]),
            Vertex::from_coords(24, 8, vec![-1, 104]),
            Vertex::from_coords(23, 10, vec![105, 106, 37, 95, -1]),
            Vertex::from_coords(15, 15, vec![37, 106, -1, 99, 91, 3, 6]),
            Vertex::from_coords(18, 18, vec![-1, 94]),
            Vertex::from_coords(19, 15, vec![105, 107, 42, -1, 106]),
            Vertex::from_coords(19, 18, vec![93, 94, -1, 42, 13]),
            Vertex::from_coords(18, 19, vec![117, 115, 8, -1, 94, 93]),
            Vertex::from_coords(15, 19, vec![17, 24, 98, 99, -1, 8]),
            Vertex::from_coords(3, 15, vec![3, 91, -1, 1, 14]),
            Vertex::from_coords(1, 15, vec![-1, 15, 1]),
            Vertex::from_coords(3, 18, vec![99, 98, 16, -1, 91]),
            Vertex::from_coords(2, 18, vec![16, -1]),
            Vertex::from_coords(2, 23, vec![24, 40, 39, -1, 16, 98]),
            Vertex::from_coords(1, 23, vec![39, 38, -1]),
            Vertex::from_coords(3, 48, vec![-1, 18]),
            Vertex::from_coords(15, 48, vec![18, 27, -1]),
            Vertex::from_coords(19, 48, vec![-1, 21]),
            Vertex::from_coords(20, 48, vec![-1, 21, 19]),
            Vertex::from_coords(24, 48, vec![52, -1]),
            Vertex::from_coords(24, 47, vec![36, 52, -1, 22, 50]),
            Vertex::from_coords(23, 46, vec![82, 50, 22, -1, 75, 114]),
            Vertex::from_coords(23, 47, vec![-1, 22]),
            Vertex::from_coords(19, 47, vec![19, 21, -1, 23]),
            Vertex::from_coords(20, 46, vec![47, 75, -1, 19, 23, 111]),
            Vertex::from_coords(15, 47, vec![112, 111, 23, -1, 27]),
            Vertex::from_coords(3, 47, vec![30, 112, 27, 18, -1, 26, 25]),
            Vertex::from_coords(1, 47, vec![28, 26, -1]),
            Vertex::from_coords(1, 35, vec![-1, 28]),
            Vertex::from_coords(2, 35, vec![25, 26, 28, -1, 29]),
            Vertex::from_coords(2, 34, vec![-1, 29]),
            Vertex::from_coords(3, 34, vec![30, 25, 29, -1, 110, 31]),
            Vertex::from_coords(1, 31, vec![32, 33, -1]),
            Vertex::from_coords(3, 31, vec![34, 110, -1, 33]),
            Vertex::from_coords(3, 30, vec![97, 34, 33, 32, -1]),
            Vertex::from_coords(1, 30, vec![-1, 32]),
            Vertex::from_coords(3, 27, vec![17, 97, -1, 41, 40, 24]),
            Vertex::from_coords(2, 27, vec![-1, 41]),
            Vertex::from_coords(1, 26, vec![-1, 38]),
            Vertex::from_coords(2, 26, vec![-1, 38, 39, 40, 41]),
            Vertex::from_coords(15, 31, vec![115, -1, 31, 110, 34, 97, 17, 8]),
            Vertex::from_coords(15, 35, vec![-1, 47, 111, 112, 30, 31]),
            Vertex::from_coords(18, 35, vec![114, 75, 47, -1, 113]),
            Vertex::from_coords(18, 34, vec![113, -1]),
            Vertex::from_coords(19, 31, vec![116, -1, 115, 117, 96]),
            Vertex::from_coords(19, 34, vec![82, 114, 113, -1, 116, 118, 119]),
            Vertex::from_coords(31, 31, vec![118, 116, 96, 51, 85, -1]),
            Vertex::from_coords(31, 35, vec![119, 118, -1, 45, 84]),
            Vertex::from_coords(34, 34, vec![-1, 35]),
            Vertex::from_coords(34, 35, vec![35, 88, 60, 86, 83, 45, -1]),
            Vertex::from_coords(35, 34, vec![-1, 90, 87, 88, 35]),
            Vertex::from_coords(35, 31, vec![-1, 85, 89, 74, 90]),
            Vertex::from_coords(47, 31, vec![74, 59, -1, 87, 90]),
            Vertex::from_coords(47, 35, vec![88, 87, -1, 54, 43, 60]),
            Vertex::from_coords(47, 47, vec![54, 56, -1, 44, 43]),
            Vertex::from_coords(35, 47, vec![86, 60, 43, 44, 55, -1]),
            Vertex::from_coords(31, 47, vec![83, 86, -1, 48, 46]),
            Vertex::from_coords(29, 46, vec![45, 83, 46, 49, -1, 84]),
            Vertex::from_coords(29, 47, vec![-1, 49]),
            Vertex::from_coords(30, 47, vec![48, 53, -1, 49, 46]),
            Vertex::from_coords(26, 46, vec![82, 119, 84, -1, 36, 50]),
            Vertex::from_coords(26, 48, vec![-1, 52, 36]),
            Vertex::from_coords(30, 48, vec![53, -1]),
            Vertex::from_coords(31, 48, vec![-1, 53, 48]),
            Vertex::from_coords(35, 48, vec![55, -1]),
            Vertex::from_coords(47, 48, vec![-1, 55, 44]),
            Vertex::from_coords(48, 47, vec![56, -1]),
            Vertex::from_coords(48, 35, vec![-1, 56, 54]),
            Vertex::from_coords(48, 31, vec![59, 58, -1]),
            Vertex::from_coords(48, 3, vec![-1, 72]),
            Vertex::from_coords(48, 15, vec![57, 72, -1]),
            Vertex::from_coords(48, 19, vec![-1, 58]),
            Vertex::from_coords(47, 19, vec![74, 89, 79, 78, -1, 58, 59]),
            Vertex::from_coords(47, 15, vec![73, 57, -1, 78, 81]),
            Vertex::from_coords(34, 19, vec![89, 85, 51, -1, 65, 79]),
            Vertex::from_coords(31, 19, vec![96, 117, 93, 13, -1, 51]),
            Vertex::from_coords(31, 15, vec![76, 77, 66, -1, 13, 42, 107]),
            Vertex::from_coords(34, 18, vec![-1, 65]),
            Vertex::from_coords(35, 18, vec![-1, 81, 78, 79, 65]),
            Vertex::from_coords(35, 15, vec![-1, 66, 80, 73, 81]),
            Vertex::from_coords(26, 10, vec![103, 76, 107, 105, -1]),
            Vertex::from_coords(26, 7, vec![103, -1, 108, 102, 109]),
            Vertex::from_coords(26, 3, vec![102, 108, 100, 20, -1]),
            Vertex::from_coords(26, 1, vec![-1, 20, 62]),
            Vertex::from_coords(29, 3, vec![109, 102, -1, 63, 67]),
            Vertex::from_coords(30, 2, vec![61, 67, 63, -1, 64]),
            Vertex::from_coords(29, 2, vec![-1, 63]),
            Vertex::from_coords(30, 1, vec![64, -1]),
            Vertex::from_coords(31, 1, vec![-1, 61, 64]),
            Vertex::from_coords(31, 3, vec![103, 109, 67, 61, -1, 77, 76]),
            Vertex::from_coords(34, 3, vec![80, 66, 77, -1, 71, 68]),
            Vertex::from_coords(34, 2, vec![-1, 71]),
            Vertex::from_coords(35, 1, vec![70, -1]),
            Vertex::from_coords(35, 2, vec![70, 69, 68, 71, -1]),
            Vertex::from_coords(47, 3, vec![73, 80, 68, 69, -1, 72, 57]),
            Vertex::from_coords(47, 1, vec![70, -1, 69]),
        ],
        vec![
            Polygon::using(3, vec![6, 2, 3, 2, 14, 4]),
            Polygon::using(3, vec![26, 2, 25, -1, 15, 14]),
            Polygon::using(3, vec![6, 3, 5, -1, 0, 5]),
            Polygon::using(3, vec![19, 25, 6, 6, 91, 14]),
            Polygon::using(3, vec![3, 2, 0, -1, 0, -1]),
            Polygon::using(3, vec![5, 3, 4, -1, 2, -1]),
            Polygon::using(3, vec![19, 6, 7, 37, 3, -1]),
            Polygon::using(3, vec![12, 11, 7, 12, 10, 92]),
            Polygon::using(3, vec![56, 24, 23, 115, 17, -1]),
            Polygon::using(3, vec![12, 9, 10, 10, -1, -1]),
            Polygon::using(3, vec![12, 10, 11, 7, 9, -1]),
            Polygon::using(3, vec![16, 11, 13, 100, 101, -1]),
            Polygon::using(3, vec![12, 7, 8, -1, 7, -1]),
            Polygon::using(3, vec![22, 92, 91, 93, 42, -1]),
            Polygon::using(3, vec![25, 2, 6, 3, 1, 0]),
            Polygon::using(3, vec![26, 1, 2, 1, -1, -1]),
            Polygon::using(3, vec![29, 28, 27, 98, -1, -1]),
            Polygon::using(3, vec![56, 52, 24, 8, 97, 24]),
            Polygon::using(3, vec![42, 32, 31, -1, 27, -1]),
            Polygon::using(3, vec![40, 34, 39, 23, -1, 21]),
            Polygon::using(3, vec![99, 98, 13, 62, -1, 100]),
            Polygon::using(3, vec![39, 34, 33, -1, 19, -1]),
            Polygon::using(3, vec![38, 37, 36, -1, -1, 50]),
            Polygon::using(3, vec![40, 39, 41, 111, 19, -1]),
            Polygon::using(3, vec![24, 52, 29, 98, 17, 40]),
            Polygon::using(3, vec![47, 42, 45, 29, 30, 26]),
            Polygon::using(3, vec![45, 42, 43, 28, 25, -1]),
            Polygon::using(3, vec![42, 41, 32, 18, 112, -1]),
            Polygon::using(3, vec![45, 43, 44, -1, 26, -1]),
            Polygon::using(3, vec![47, 45, 46, -1, 25, -1]),
            Polygon::using(3, vec![57, 42, 47, 31, 112, 25]),
            Polygon::using(3, vec![57, 47, 56, -1, 30, 110]),
            Polygon::using(3, vec![50, 48, 51, -1, 33, -1]),
            Polygon::using(3, vec![50, 49, 48, 32, 34, -1]),
            Polygon::using(3, vec![56, 49, 50, 97, 110, 33]),
            Polygon::using(3, vec![66, 65, 64, -1, 88, -1]),
            Polygon::using(3, vec![77, 36, 76, -1, 52, 50]),
            Polygon::using(3, vec![18, 19, 7, 95, 106, 6]),
            Polygon::using(3, vec![55, 54, 30, 39, -1, -1]),
            Polygon::using(3, vec![55, 30, 29, 40, 38, -1]),
            Polygon::using(3, vec![55, 29, 52, 41, 39, 24]),
            Polygon::using(3, vec![55, 52, 53, -1, 40, -1]),
            Polygon::using(3, vec![22, 21, 92, 13, -1, 107]),
            Polygon::using(3, vec![71, 69, 70, 44, 60, 54]),
            Polygon::using(3, vec![81, 71, 70, -1, 55, 43]),
            Polygon::using(3, vec![65, 73, 63, -1, 83, 84]),
            Polygon::using(3, vec![75, 73, 72, 48, 49, 83]),
            Polygon::using(3, vec![58, 40, 57, -1, 75, 111]),
            Polygon::using(3, vec![79, 75, 72, -1, 53, 46]),
            Polygon::using(3, vec![75, 74, 73, 46, -1, -1]),
            Polygon::using(3, vec![76, 36, 37, 82, 36, 22]),
            Polygon::using(3, vec![62, 91, 90, 85, 96, -1]),
            Polygon::using(3, vec![77, 35, 36, 36, -1, -1]),
            Polygon::using(3, vec![79, 78, 75, 48, -1, -1]),
            Polygon::using(3, vec![83, 70, 69, -1, 56, 43]),
            Polygon::using(3, vec![81, 80, 71, 44, -1, -1]),
            Polygon::using(3, vec![83, 82, 70, 54, -1, -1]),
            Polygon::using(3, vec![110, 86, 89, 73, 72, -1]),
            Polygon::using(3, vec![88, 87, 84, 59, -1, -1]),
            Polygon::using(3, vec![88, 84, 68, 74, 58, -1]),
            Polygon::using(3, vec![65, 69, 71, 86, 88, 43]),
            Polygon::using(3, vec![105, 101, 104, -1, 67, 64]),
            Polygon::using(3, vec![99, 13, 14, -1, 20, -1]),
            Polygon::using(3, vec![101, 100, 102, -1, 67, -1]),
            Polygon::using(3, vec![104, 101, 103, -1, 61, -1]),
            Polygon::using(3, vec![94, 90, 93, -1, 79, -1]),
            Polygon::using(3, vec![95, 92, 106, 80, -1, 77]),
            Polygon::using(3, vec![105, 100, 101, 61, 109, 63]),
            Polygon::using(3, vec![110, 106, 109, 69, 80, 71]),
            Polygon::using(3, vec![111, 110, 109, 70, -1, 68]),
            Polygon::using(3, vec![111, 109, 108, -1, 69, -1]),
            Polygon::using(3, vec![109, 106, 107, -1, 68, -1]),
            Polygon::using(3, vec![110, 85, 86, 57, -1, -1]),
            Polygon::using(3, vec![95, 110, 89, 81, 80, 57]),
            Polygon::using(3, vec![67, 88, 68, 90, 89, 59]),
            Polygon::using(3, vec![58, 37, 40, 47, 114, -1]),
            Polygon::using(3, vec![96, 105, 92, 107, 103, 77]),
            Polygon::using(3, vec![92, 105, 106, 66, 76, -1]),
            Polygon::using(3, vec![94, 89, 88, 79, 81, -1]),
            Polygon::using(3, vec![94, 88, 90, 65, 78, 89]),
            Polygon::using(3, vec![95, 106, 110, 73, 66, 68]),
            Polygon::using(3, vec![95, 89, 94, -1, 73, 78]),
            Polygon::using(3, vec![61, 76, 37, 114, 119, 50]),
            Polygon::using(3, vec![65, 72, 73, 45, 86, 46]),
            Polygon::using(3, vec![63, 73, 76, 119, 45, -1]),
            Polygon::using(3, vec![67, 62, 90, 89, -1, 51]),
            Polygon::using(3, vec![65, 71, 72, 83, 60, -1]),
            Polygon::using(3, vec![66, 68, 69, 88, 90, -1]),
            Polygon::using(3, vec![66, 69, 65, 35, 87, 60]),
            Polygon::using(3, vec![67, 90, 88, 74, 85, 79]),
            Polygon::using(3, vec![67, 68, 66, -1, 74, 87]),
            Polygon::using(3, vec![19, 27, 25, 3, 99, -1]),
            Polygon::using(3, vec![15, 7, 11, 101, 95, 7]),
            Polygon::using(3, vec![23, 22, 91, 117, 94, 13]),
            Polygon::using(3, vec![23, 20, 22, 93, -1, -1]),
            Polygon::using(3, vec![18, 7, 15, -1, 37, 92]),
            Polygon::using(3, vec![60, 91, 62, 116, 117, 51]),
            Polygon::using(3, vec![56, 50, 52, 17, 34, -1]),
            Polygon::using(3, vec![24, 29, 27, 99, 24, 16]),
            Polygon::using(3, vec![24, 27, 19, -1, 98, 91]),
            Polygon::using(3, vec![16, 13, 98, 108, 11, 20]),
            Polygon::using(3, vec![16, 15, 11, 11, 104, 92]),
            Polygon::using(3, vec![97, 98, 100, 109, 108, -1]),
            Polygon::using(3, vec![97, 105, 96, -1, 109, 76]),
            Polygon::using(3, vec![17, 15, 16, -1, -1, 101]),
            Polygon::using(3, vec![96, 21, 18, -1, 107, 106]),
            Polygon::using(3, vec![18, 21, 19, 37, 105, -1]),
            Polygon::using(3, vec![96, 92, 21, 105, 76, 42]),
            Polygon::using(3, vec![97, 16, 98, 102, -1, 100]),
            Polygon::using(3, vec![97, 100, 105, 103, 102, 67]),
            Polygon::using(3, vec![56, 47, 49, 34, 31, -1]),
            Polygon::using(3, vec![57, 40, 41, 112, 47, 23]),
            Polygon::using(3, vec![57, 41, 42, 30, 111, 27]),
            Polygon::using(3, vec![61, 58, 59, -1, 114, -1]),
            Polygon::using(3, vec![61, 37, 58, 113, 82, 75]),
            Polygon::using(3, vec![60, 56, 23, 117, -1, 8]),
            Polygon::using(3, vec![61, 60, 62, 118, -1, 96]),
            Polygon::using(3, vec![60, 23, 91, 96, 115, 93]),
            Polygon::using(3, vec![61, 62, 63, 119, 116, -1]),
            Polygon::using(3, vec![61, 63, 76, 82, 118, 84]),
        ],
    )
}

#[test]
fn arena_scenario_ref_impl() {
    let arena = arena_mesh();

    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(1.0, 12.0)).len,
        1.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(1.0, 10.0)).len,
        2.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(4.0, 12.0)).len,
        3.16228
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 3.0), Vec2::new(3.0, 1.0)).len,
        3.41421
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 3.0), Vec2::new(4.0, 3.0)).len,
        3.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 4.0), Vec2::new(4.0, 2.0)).len,
        3.60555
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 40.0), Vec2::new(2.0, 39.0)).len,
        1.41421
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 41.0), Vec2::new(1.0, 39.0)).len,
        2.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 41.0), Vec2::new(1.0, 44.0)).len,
        3.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 42.0), Vec2::new(4.0, 43.0)).len,
        3.16228
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(7.0, 10.0)).len,
        6.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(1.0, 4.0)).len,
        7.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(7.0, 14.0)).len,
        6.7082
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(5.0, 7.0)).len,
        6.40312
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(6.0, 15.0)).len,
        5.83095
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(8.0, 11.0)).len,
        7.07107
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 14.0), Vec2::new(1.0, 9.0)).len,
        5.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 24.0), Vec2::new(7.0, 26.0)).len,
        6.32456
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 25.0), Vec2::new(5.0, 25.0)).len,
        4.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 35.0), Vec2::new(5.0, 33.0)).len,
        4.60555
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(4.0, 18.0)).len,
        7.63441
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(12.0, 14.0)).len,
        11.1803
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(4.0, 23.0)).len,
        10.8907
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(5.0, 3.0)).len,
        10.7703
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(6.0, 7.0)).len,
        7.81025
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(7.0, 7.0)).len,
        8.48528
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 23.0), Vec2::new(7.0, 32.0)).len,
        10.8167
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 24.0), Vec2::new(11.0, 25.0)).len,
        10.0499
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 24.0), Vec2::new(6.0, 32.0)).len,
        9.4365
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 25.0), Vec2::new(9.0, 24.0)).len,
        8.06226
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(11.0, 19.0)).len,
        13.4536
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(13.0, 11.0)).len,
        12.0416
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(10.0, 2.0)).len,
        12.7279
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(11.0, 21.0)).len,
        13.4536
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(13.0, 13.0)).len,
        12.0416
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(14.0, 12.0)).len,
        13.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(6.0, 25.0)).len,
        14.0459
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(11.0, 3.0)).len,
        14.1421
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(13.0, 11.0)).len,
        12.1655
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 14.0), Vec2::new(6.0, 23.0)).len,
        10.7801
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(18.0, 11.0)).len,
        17.0294
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(16.0, 14.0)).len,
        15.2971
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(14.0, 2.0)).len,
        16.4012
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(17.0, 13.0)).len,
        16.0312
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(9.0, 28.0)).len,
        17.9234
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(4.0, 30.0)).len,
        17.8617
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(9.0, 26.0)).len,
        15.3584
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 14.0), Vec2::new(14.0, 22.0)).len,
        15.2745
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 23.0), Vec2::new(10.0, 8.0)).len,
        18.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 23.0), Vec2::new(14.0, 9.0)).len,
        19.4391
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(13.0, 29.0)).len,
        22.4722
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(18.0, 22.0)).len,
        20.8087
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(19.0, 18.0)).len,
        21.0575
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(21.0, 2.0)).len,
        21.5407
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(5.0, 32.0)).len,
        22.5024
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(6.0, 29.0)).len,
        19.703
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(20.0, 7.0)).len,
        19.4165
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(21.0, 17.0)).len,
        21.2675
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(22.0, 16.0)).len,
        21.6014
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(8.0, 29.0)).len,
        19.3382
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(22.0, 22.0)).len,
        24.2591
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(5.0, 33.0)).len,
        23.4959
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(10.0, 32.0)).len,
        22.8569
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(20.0, 31.0)).len,
        27.5862
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(21.0, 23.0)).len,
        23.3238
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(24.0, 14.0)).len,
        23.1948
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(25.0, 4.0)).len,
        25.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(3.0, 36.0)).len,
        25.4721
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(19.0, 27.0)).len,
        23.4307
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(2.0, 37.0)).len,
        25.7678
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(22.0, 31.0)).len,
        29.6985
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(24.0, 27.0)).len,
        28.6007
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(28.0, 15.0)).len,
        27.4591
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(7.0, 39.0)).len,
        29.7162
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(12.0, 35.0)).len,
        26.4038
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(28.0, 18.0)).len,
        27.9259
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(5.0, 40.0)).len,
        29.552
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(26.0, 3.0)).len,
        26.5707
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(29.0, 14.0)).len,
        28.0713
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(29.0, 6.0)).len,
        28.6362
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(25.0, 36.0)).len,
        35.3836
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(27.0, 25.0)).len,
        30.0597
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(32.0, 4.0)).len,
        31.5753
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(33.0, 4.0)).len,
        32.5576
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(10.0, 42.0)).len,
        32.3648
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(27.0, 28.0)).len,
        31.0644
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(30.0, 2.0)).len,
        30.5347
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(31.0, 3.0)).len,
        31.0483
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(5.0, 42.0)).len,
        31.5461
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(18.0, 37.0)).len,
        30.5349
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(15.0, 43.0)).len,
        35.8469
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(21.0, 41.0)).len,
        37.1384
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(27.0, 37.0)).len,
        37.4833
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(29.0, 38.0)).len,
        39.598
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(31.0, 25.0)).len,
        33.7313
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(38.0, 13.0)).len,
        37.1214
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(38.0, 6.0)).len,
        37.2305
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(39.0, 6.0)).len,
        38.2281
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(40.0, 9.0)).len,
        39.0357
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(11.0, 43.0)).len,
        33.5926
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(12.0, 47.0)).len,
        38.6267
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(14.0, 47.0)).len,
        39.223
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(16.0, 46.0)).len,
        39.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(28.0, 41.0)).len,
        41.1096
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(37.0, 21.0)).len,
        37.6552
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(39.0, 24.0)).len,
        40.6133
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(16.0, 45.0)).len,
        37.1677
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(21.0, 43.0)).len,
        37.7849
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(32.0, 39.0)).len,
        41.7732
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(34.0, 29.0)).len,
        37.5954
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(29.0, 43.0)).len,
        43.2791
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(43.0, 15.0)).len,
        42.2966
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(43.0, 17.0)).len,
        42.6119
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(45.0, 10.0)).len,
        44.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(46.0, 15.0)).len,
        45.2769
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(46.0, 3.0)).len,
        45.5412
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(30.0, 45.0)).len,
        44.7117
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(35.0, 41.0)).len,
        45.3431
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(35.0, 42.0)).len,
        46.0109
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(43.0, 3.0)).len,
        42.7551
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(31.0, 46.0)).len,
        46.868
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(34.0, 46.0)).len,
        48.8365
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(35.0, 46.0)).len,
        49.5177
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(36.0, 42.0)).len,
        47.4236
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(38.0, 45.0)).len,
        50.9313
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(46.0, 18.0)).len,
        45.7674
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(37.0, 44.0)).len,
        48.8365
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(41.0, 35.0)).len,
        46.6573
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(43.0, 27.0)).len,
        45.245
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(44.0, 25.0)).len,
        45.3649
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(41.0, 40.0)).len,
        50.2316
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(43.0, 40.0)).len,
        52.0087
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(39.0, 47.0)).len,
        52.345
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(42.0, 46.0)).len,
        53.9073
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(45.0, 33.0)).len,
        49.2304
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(43.0, 43.0)).len,
        52.2243
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(44.0, 38.0)).len,
        50.3504
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(46.0, 34.0)).len,
        50.0908
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(42.0, 40.0)).len,
        49.2852
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 14.0), Vec2::new(46.0, 32.0)).len,
        48.4665
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 14.0), Vec2::new(44.0, 46.0)).len,
        53.6365
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 14.0), Vec2::new(46.0, 43.0)).len,
        53.6454
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 35.0), Vec2::new(46.0, 3.0)).len,
        55.4059
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 37.0), Vec2::new(43.0, 1.0)).len,
        55.3173
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 38.0), Vec2::new(43.0, 3.0)).len,
        54.6717
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 38.0), Vec2::new(47.0, 13.0)).len,
        52.3546
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 39.0), Vec2::new(47.0, 14.0)).len,
        52.3599
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 4.0), Vec2::new(38.0, 47.0)).len,
        56.7274
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 4.0), Vec2::new(41.0, 42.0)).len,
        55.3523
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 42.0), Vec2::new(44.0, 5.0)).len,
        56.7621
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 3.0), Vec2::new(41.0, 47.0)).len,
        59.4702
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 3.0), Vec2::new(47.0, 37.0)).len,
        57.2423
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 39.0), Vec2::new(46.0, 1.0)).len,
        58.8982
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 4.0), Vec2::new(43.0, 46.0)).len,
        59.4245
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 4.0), Vec2::new(44.0, 45.0)).len,
        59.5469
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 40.0), Vec2::new(47.0, 3.0)).len,
        59.0512
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 41.0), Vec2::new(46.0, 2.0)).len,
        59.5483
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 45.0), Vec2::new(47.0, 9.0)).len,
        58.6718
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 7.0), Vec2::new(47.0, 44.0)).len,
        59.3941
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 7.0), Vec2::new(47.0, 46.0)).len,
        60.4531
    );
}
