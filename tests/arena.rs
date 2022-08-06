use polyanya::{Mesh, Polygon, Vertex};

fn arena_mesh() -> Mesh {
    Mesh {
        vertices: vec![
            Vertex::new(2, 2, vec![-1, 1]),
            Vertex::new(1, 3, vec![6, -1]),
            Vertex::new(2, 3, vec![5, 6, -1, 1, 0]),
            Vertex::new(3, 2, vec![2, 0, 1, -1]),
            Vertex::new(3, 1, vec![2, -1]),
            Vertex::new(15, 1, vec![-1, 2]),
            Vertex::new(15, 3, vec![-1, 5, 0, 2]),
            Vertex::new(18, 3, vec![4, 16, 5, -1]),
            Vertex::new(18, 2, vec![-1, 4]),
            Vertex::new(19, 1, vec![3, -1]),
            Vertex::new(20, 1, vec![-1, 3]),
            Vertex::new(20, 2, vec![16, 3, -1]),
            Vertex::new(19, 2, vec![3, 16, 4, -1]),
            Vertex::new(23, 2, vec![16, -1, 28]),
            Vertex::new(23, 1, vec![28, -1]),
            Vertex::new(23, 8, vec![5, 16, 39, -1]),
            Vertex::new(24, 7, vec![39, 16, 41, -1]),
            Vertex::new(24, 8, vec![-1, 39]),
            Vertex::new(23, 10, vec![5, -1, 54]),
            Vertex::new(15, 15, vec![5, -1, 46, 51]),
            Vertex::new(18, 18, vec![-1, 53]),
            Vertex::new(19, 15, vec![5, 54, -1]),
            Vertex::new(19, 18, vec![-1, 54, 52, 53]),
            Vertex::new(18, 19, vec![52, 38, -1, 53]),
            Vertex::new(15, 19, vec![46, -1, 38]),
            Vertex::new(3, 15, vec![5, 51, -1, 6]),
            Vertex::new(1, 15, vec![6, -1]),
            Vertex::new(3, 18, vec![51, 46, 7, -1]),
            Vertex::new(2, 18, vec![7, -1]),
            Vertex::new(2, 23, vec![46, 17, -1, 7]),
            Vertex::new(1, 23, vec![-1, 17]),
            Vertex::new(3, 48, vec![-1, 8]),
            Vertex::new(15, 48, vec![8, -1]),
            Vertex::new(19, 48, vec![-1, 9]),
            Vertex::new(20, 48, vec![9, -1]),
            Vertex::new(24, 48, vec![22, -1]),
            Vertex::new(24, 47, vec![21, 22, -1, 10]),
            Vertex::new(23, 46, vec![42, 47, 21, 10, -1, 27]),
            Vertex::new(23, 47, vec![-1, 10]),
            Vertex::new(19, 47, vec![45, 9, -1]),
            Vertex::new(20, 46, vec![27, -1, 9, 45]),
            Vertex::new(15, 47, vec![45, -1, 8]),
            Vertex::new(3, 47, vec![11, 45, 8, -1]),
            Vertex::new(1, 47, vec![11, -1]),
            Vertex::new(1, 35, vec![-1, 11]),
            Vertex::new(2, 35, vec![45, 11, -1, 12]),
            Vertex::new(2, 34, vec![-1, 12]),
            Vertex::new(3, 34, vec![-1, 13, 45, 12]),
            Vertex::new(1, 31, vec![14, -1]),
            Vertex::new(3, 31, vec![14, 13, -1]),
            Vertex::new(3, 30, vec![-1, 13, 14]),
            Vertex::new(1, 30, vec![-1, 14]),
            Vertex::new(3, 27, vec![18, 46, 13, -1]),
            Vertex::new(2, 27, vec![-1, 18]),
            Vertex::new(1, 26, vec![-1, 17]),
            Vertex::new(2, 26, vec![17, 46, 18, -1]),
            Vertex::new(15, 31, vec![38, -1, 13, 46]),
            Vertex::new(15, 35, vec![27, 45, 13, -1]),
            Vertex::new(18, 35, vec![42, 27, -1, 48]),
            Vertex::new(18, 34, vec![-1, 48]),
            Vertex::new(19, 31, vec![52, 54, 31, 47, -1, 38]),
            Vertex::new(19, 34, vec![-1, 47, 42, 48]),
            Vertex::new(31, 31, vec![47, 31, -1]),
            Vertex::new(31, 35, vec![19, 47, -1]),
            Vertex::new(34, 34, vec![-1, 15]),
            Vertex::new(34, 35, vec![19, -1, 15, 49]),
            Vertex::new(35, 34, vec![50, 49, 15, -1]),
            Vertex::new(35, 31, vec![-1, 31, 50]),
            Vertex::new(47, 31, vec![-1, 50, 31, 26]),
            Vertex::new(47, 35, vec![19, 49, 50, -1, 25]),
            Vertex::new(47, 47, vec![19, 25, -1, 24]),
            Vertex::new(35, 47, vec![-1, 19, 24]),
            Vertex::new(31, 47, vec![23, 19, -1]),
            Vertex::new(29, 46, vec![19, 20, -1, 47]),
            Vertex::new(29, 47, vec![-1, 20]),
            Vertex::new(30, 47, vec![19, 23, -1, 20]),
            Vertex::new(26, 46, vec![47, -1, 22, 21]),
            Vertex::new(26, 48, vec![22, -1]),
            Vertex::new(30, 48, vec![23, -1]),
            Vertex::new(31, 48, vec![23, -1]),
            Vertex::new(35, 48, vec![24, -1]),
            Vertex::new(47, 48, vec![24, -1]),
            Vertex::new(48, 47, vec![25, -1]),
            Vertex::new(48, 35, vec![25, -1]),
            Vertex::new(48, 31, vec![-1, 26]),
            Vertex::new(48, 3, vec![-1, 36]),
            Vertex::new(48, 15, vec![-1, 36]),
            Vertex::new(48, 19, vec![-1, 26]),
            Vertex::new(47, 19, vec![31, 43, 44, -1, 26]),
            Vertex::new(47, 15, vec![-1, 44, 37, 36]),
            Vertex::new(34, 19, vec![31, -1, 40, 43]),
            Vertex::new(31, 19, vec![54, -1, 31]),
            Vertex::new(31, 15, vec![54, 37, -1]),
            Vertex::new(34, 18, vec![-1, 40]),
            Vertex::new(35, 18, vec![44, 43, 40, -1]),
            Vertex::new(35, 15, vec![-1, 37, 44]),
            Vertex::new(26, 10, vec![54, -1, 37]),
            Vertex::new(26, 7, vec![41, 37, -1]),
            Vertex::new(26, 3, vec![37, 41, 16, 28, -1]),
            Vertex::new(26, 1, vec![-1, 28]),
            Vertex::new(29, 3, vec![37, -1, 29, 32]),
            Vertex::new(30, 2, vec![30, 32, 29, -1]),
            Vertex::new(29, 2, vec![-1, 29]),
            Vertex::new(30, 1, vec![30, -1]),
            Vertex::new(31, 1, vec![-1, 30]),
            Vertex::new(31, 3, vec![37, 32, 30, -1]),
            Vertex::new(34, 3, vec![37, -1, 35, 33]),
            Vertex::new(34, 2, vec![-1, 35]),
            Vertex::new(35, 1, vec![34, -1]),
            Vertex::new(35, 2, vec![34, 33, 35, -1]),
            Vertex::new(47, 3, vec![37, 33, 34, -1, 36]),
            Vertex::new(47, 1, vec![-1, 34]),
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
fn first_test() {
    let arena = arena_mesh();

    assert_eq!(arena.path_len([1.0, 11.0], [1.0, 12.0]), 1.0);
    assert_eq!(arena.path_len([1.0, 12.0], [1.0, 10.0]), 2.0);
    assert_eq!(arena.path_len([1.0, 13.0], [4.0, 12.0]), 3.41421);
    assert_eq!(arena.path_len([1.0, 3.0], [3.0, 1.0]), 3.41421);
    assert_eq!(arena.path_len([1.0, 3.0], [4.0, 3.0]), 3.0);
    assert_eq!(arena.path_len([1.0, 4.0], [4.0, 2.0]), 3.82843);
    assert_eq!(arena.path_len([1.0, 40.0], [2.0, 39.0]), 1.41421);
    assert_eq!(arena.path_len([1.0, 41.0], [1.0, 39.0]), 2.0);
    assert_eq!(arena.path_len([1.0, 41.0], [1.0, 44.0]), 3.0);
    assert_eq!(arena.path_len([1.0, 42.0], [4.0, 43.0]), 3.41421);
    assert_eq!(arena.path_len([1.0, 10.0], [7.0, 10.0]), 6.0);
    assert_eq!(arena.path_len([1.0, 11.0], [1.0, 4.0]), 7.0);
    assert_eq!(arena.path_len([1.0, 11.0], [7.0, 14.0]), 7.24264);
    assert_eq!(arena.path_len([1.0, 12.0], [5.0, 7.0]), 6.65685);
    assert_eq!(arena.path_len([1.0, 12.0], [6.0, 15.0]), 6.24264);
    assert_eq!(arena.path_len([1.0, 12.0], [8.0, 11.0]), 7.41421);
    assert_eq!(arena.path_len([1.0, 14.0], [1.0, 9.0]), 5.0);
    assert_eq!(arena.path_len([1.0, 24.0], [7.0, 26.0]), 6.82843);
    assert_eq!(arena.path_len([1.0, 25.0], [5.0, 25.0]), 4.0);
    assert_eq!(arena.path_len([1.0, 35.0], [5.0, 33.0]), 4.82843);
    assert_eq!(arena.path_len([1.0, 11.0], [4.0, 18.0]), 8.24264);
    assert_eq!(arena.path_len([1.0, 12.0], [12.0, 14.0]), 11.8284);
    assert_eq!(arena.path_len([1.0, 13.0], [4.0, 23.0]), 11.8284);
    assert_eq!(arena.path_len([1.0, 13.0], [5.0, 3.0]), 11.6569);
    assert_eq!(arena.path_len([1.0, 13.0], [6.0, 7.0]), 8.07107);
    assert_eq!(arena.path_len([1.0, 13.0], [7.0, 7.0]), 8.48528);
    assert_eq!(arena.path_len([1.0, 23.0], [7.0, 32.0]), 11.4853);
    assert_eq!(arena.path_len([1.0, 24.0], [11.0, 25.0]), 10.4142);
    assert_eq!(arena.path_len([1.0, 24.0], [6.0, 32.0]), 10.0711);
    assert_eq!(arena.path_len([1.0, 25.0], [9.0, 24.0]), 8.41421);
    assert_eq!(arena.path_len([1.0, 10.0], [11.0, 19.0]), 13.7279);
    assert_eq!(arena.path_len([1.0, 10.0], [13.0, 11.0]), 12.4142);
    assert_eq!(arena.path_len([1.0, 11.0], [10.0, 2.0]), 12.7279);
    assert_eq!(arena.path_len([1.0, 12.0], [11.0, 21.0]), 13.7279);
    assert_eq!(arena.path_len([1.0, 12.0], [13.0, 13.0]), 12.4142);
    assert_eq!(arena.path_len([1.0, 12.0], [14.0, 12.0]), 13.0);
    assert_eq!(arena.path_len([1.0, 12.0], [6.0, 25.0]), 15.0711);
    assert_eq!(arena.path_len([1.0, 13.0], [11.0, 3.0]), 14.1421);
    assert_eq!(arena.path_len([1.0, 13.0], [13.0, 11.0]), 12.8284);
    assert_eq!(arena.path_len([1.0, 14.0], [6.0, 23.0]), 12.2426);
    assert_eq!(arena.path_len([1.0, 10.0], [18.0, 11.0]), 17.4142);
    assert_eq!(arena.path_len([1.0, 11.0], [16.0, 14.0]), 16.2426);
    assert_eq!(arena.path_len([1.0, 12.0], [14.0, 2.0]), 17.1421);
    assert_eq!(arena.path_len([1.0, 12.0], [17.0, 13.0]), 16.4142);
    assert_eq!(arena.path_len([1.0, 12.0], [9.0, 28.0]), 19.3137);
    assert_eq!(arena.path_len([1.0, 13.0], [4.0, 30.0]), 18.8284);
    assert_eq!(arena.path_len([1.0, 13.0], [9.0, 26.0]), 16.8995);
    assert_eq!(arena.path_len([1.0, 14.0], [14.0, 22.0]), 16.3137);
    assert_eq!(arena.path_len([1.0, 23.0], [10.0, 8.0]), 19.3137);
    assert_eq!(arena.path_len([1.0, 23.0], [14.0, 9.0]), 19.9706);
    assert_eq!(arena.path_len([1.0, 10.0], [13.0, 29.0]), 23.9706);
    assert_eq!(arena.path_len([1.0, 10.0], [18.0, 22.0]), 21.9706);
    assert_eq!(arena.path_len([1.0, 10.0], [19.0, 18.0]), 22.1421);
    assert_eq!(arena.path_len([1.0, 10.0], [21.0, 2.0]), 23.3137);
    assert_eq!(arena.path_len([1.0, 10.0], [5.0, 32.0]), 23.6569);
    assert_eq!(arena.path_len([1.0, 10.0], [6.0, 29.0]), 21.0711);
    assert_eq!(arena.path_len([1.0, 11.0], [20.0, 7.0]), 20.6569);
    assert_eq!(arena.path_len([1.0, 11.0], [21.0, 17.0]), 23.0711);
    assert_eq!(arena.path_len([1.0, 11.0], [22.0, 16.0]), 23.0711);
    assert_eq!(arena.path_len([1.0, 11.0], [8.0, 29.0]), 20.8995);
    assert_eq!(arena.path_len([1.0, 10.0], [22.0, 22.0]), 25.9706);
    assert_eq!(arena.path_len([1.0, 10.0], [5.0, 33.0]), 24.6569);
    assert_eq!(arena.path_len([1.0, 11.0], [10.0, 32.0]), 24.7279);
    assert_eq!(arena.path_len([1.0, 11.0], [20.0, 31.0]), 27.8701);
    assert_eq!(arena.path_len([1.0, 11.0], [21.0, 23.0]), 24.9706);
    assert_eq!(arena.path_len([1.0, 11.0], [24.0, 14.0]), 24.2426);
    assert_eq!(arena.path_len([1.0, 11.0], [25.0, 4.0]), 26.8995);
    assert_eq!(arena.path_len([1.0, 11.0], [3.0, 36.0]), 25.8284);
    assert_eq!(arena.path_len([1.0, 12.0], [19.0, 27.0]), 24.2132);
    assert_eq!(arena.path_len([1.0, 12.0], [2.0, 37.0]), 26.2426);
    assert_eq!(arena.path_len([1.0, 10.0], [22.0, 31.0]), 29.6985);
    assert_eq!(arena.path_len([1.0, 10.0], [24.0, 27.0]), 30.0416);
    assert_eq!(arena.path_len([1.0, 10.0], [28.0, 15.0]), 29.0711);
    assert_eq!(arena.path_len([1.0, 10.0], [7.0, 39.0]), 31.4853);
    assert_eq!(arena.path_len([1.0, 11.0], [12.0, 35.0]), 28.5563);
    assert_eq!(arena.path_len([1.0, 11.0], [28.0, 18.0]), 29.8995);
    assert_eq!(arena.path_len([1.0, 11.0], [5.0, 40.0]), 30.6569);
    assert_eq!(arena.path_len([1.0, 12.0], [26.0, 3.0]), 28.7279);
    assert_eq!(arena.path_len([1.0, 12.0], [29.0, 14.0]), 28.8284);
    assert_eq!(arena.path_len([1.0, 12.0], [29.0, 6.0]), 30.4853);
    assert_eq!(arena.path_len([1.0, 10.0], [25.0, 36.0]), 35.9411);
    assert_eq!(arena.path_len([1.0, 10.0], [27.0, 25.0]), 32.2132);
    assert_eq!(arena.path_len([1.0, 10.0], [32.0, 4.0]), 33.4853);
    assert_eq!(arena.path_len([1.0, 10.0], [33.0, 4.0]), 34.4853);
    assert_eq!(arena.path_len([1.0, 11.0], [10.0, 42.0]), 34.7279);
    assert_eq!(arena.path_len([1.0, 11.0], [27.0, 28.0]), 33.0416);
    assert_eq!(arena.path_len([1.0, 11.0], [30.0, 2.0]), 32.7279);
    assert_eq!(arena.path_len([1.0, 11.0], [31.0, 3.0]), 33.3137);
    assert_eq!(arena.path_len([1.0, 11.0], [5.0, 42.0]), 32.6569);
    assert_eq!(arena.path_len([1.0, 12.0], [18.0, 37.0]), 32.8701);
    assert_eq!(arena.path_len([1.0, 10.0], [15.0, 43.0]), 38.799);
    assert_eq!(arena.path_len([1.0, 10.0], [21.0, 41.0]), 39.2843);
    assert_eq!(arena.path_len([1.0, 10.0], [27.0, 37.0]), 37.7696);
    assert_eq!(arena.path_len([1.0, 10.0], [29.0, 38.0]), 39.598);
    assert_eq!(arena.path_len([1.0, 10.0], [31.0, 25.0]), 36.2132);
    assert_eq!(arena.path_len([1.0, 10.0], [38.0, 13.0]), 38.2426);
    assert_eq!(arena.path_len([1.0, 10.0], [38.0, 6.0]), 38.6569);
    assert_eq!(arena.path_len([1.0, 10.0], [39.0, 6.0]), 39.6569);
    assert_eq!(arena.path_len([1.0, 10.0], [40.0, 9.0]), 39.4142);
    assert_eq!(arena.path_len([1.0, 11.0], [11.0, 43.0]), 36.1421);
    assert_eq!(arena.path_len([1.0, 10.0], [12.0, 47.0]), 41.5563);
    assert_eq!(arena.path_len([1.0, 10.0], [14.0, 47.0]), 42.3848);
    assert_eq!(arena.path_len([1.0, 10.0], [16.0, 46.0]), 42.2132);
    assert_eq!(arena.path_len([1.0, 10.0], [28.0, 41.0]), 42.1838);
    assert_eq!(arena.path_len([1.0, 10.0], [37.0, 21.0]), 40.5563);
    assert_eq!(arena.path_len([1.0, 10.0], [39.0, 24.0]), 43.799);
    assert_eq!(arena.path_len([1.0, 11.0], [16.0, 45.0]), 40.2132);
    assert_eq!(arena.path_len([1.0, 11.0], [21.0, 43.0]), 40.2843);
    assert_eq!(arena.path_len([1.0, 11.0], [32.0, 39.0]), 42.598);
    assert_eq!(arena.path_len([1.0, 11.0], [34.0, 29.0]), 40.4558);
    assert_eq!(arena.path_len([1.0, 10.0], [29.0, 43.0]), 44.598);
    assert_eq!(arena.path_len([1.0, 10.0], [43.0, 15.0]), 44.0711);
    assert_eq!(arena.path_len([1.0, 10.0], [43.0, 17.0]), 44.8995);
    assert_eq!(arena.path_len([1.0, 10.0], [45.0, 10.0]), 44.0);
    assert_eq!(arena.path_len([1.0, 10.0], [46.0, 15.0]), 47.0711);
    assert_eq!(arena.path_len([1.0, 10.0], [46.0, 3.0]), 47.8995);
    assert_eq!(arena.path_len([1.0, 11.0], [30.0, 45.0]), 46.0122);
    assert_eq!(arena.path_len([1.0, 11.0], [35.0, 41.0]), 46.4264);
    assert_eq!(arena.path_len([1.0, 11.0], [35.0, 42.0]), 46.8406);
    assert_eq!(arena.path_len([1.0, 11.0], [43.0, 3.0]), 45.3137);
    assert_eq!(arena.path_len([1.0, 10.0], [31.0, 46.0]), 48.4264);
    assert_eq!(arena.path_len([1.0, 10.0], [34.0, 46.0]), 49.669);
    assert_eq!(arena.path_len([1.0, 10.0], [35.0, 46.0]), 50.0833);
    assert_eq!(arena.path_len([1.0, 10.0], [36.0, 42.0]), 48.2548);
    assert_eq!(arena.path_len([1.0, 10.0], [38.0, 45.0]), 51.4975);
    assert_eq!(arena.path_len([1.0, 10.0], [46.0, 18.0]), 48.3137);
    assert_eq!(arena.path_len([1.0, 11.0], [37.0, 44.0]), 49.669);
    assert_eq!(arena.path_len([1.0, 11.0], [41.0, 35.0]), 49.9411);
    assert_eq!(arena.path_len([1.0, 11.0], [43.0, 27.0]), 48.6274);
    assert_eq!(arena.path_len([1.0, 11.0], [44.0, 25.0]), 48.799);
    assert_eq!(arena.path_len([1.0, 10.0], [41.0, 40.0]), 52.4264);
    assert_eq!(arena.path_len([1.0, 10.0], [43.0, 40.0]), 54.4264);
    assert_eq!(arena.path_len([1.0, 11.0], [39.0, 47.0]), 52.9117);
    assert_eq!(arena.path_len([1.0, 11.0], [42.0, 46.0]), 55.4975);
    assert_eq!(arena.path_len([1.0, 11.0], [45.0, 33.0]), 53.1127);
    assert_eq!(arena.path_len([1.0, 12.0], [43.0, 43.0]), 54.8406);
    assert_eq!(arena.path_len([1.0, 12.0], [44.0, 38.0]), 53.7696);
    assert_eq!(arena.path_len([1.0, 12.0], [46.0, 34.0]), 54.1127);
    assert_eq!(arena.path_len([1.0, 13.0], [42.0, 40.0]), 52.1838);
    assert_eq!(arena.path_len([1.0, 14.0], [46.0, 32.0]), 52.4558);
    assert_eq!(arena.path_len([1.0, 14.0], [44.0, 46.0]), 56.2548);
    assert_eq!(arena.path_len([1.0, 14.0], [46.0, 43.0]), 57.0122);
    assert_eq!(arena.path_len([1.0, 35.0], [46.0, 3.0]), 58.2548);
    assert_eq!(arena.path_len([1.0, 37.0], [43.0, 1.0]), 56.9117);
    assert_eq!(arena.path_len([1.0, 38.0], [43.0, 3.0]), 56.4975);
    assert_eq!(arena.path_len([1.0, 38.0], [47.0, 13.0]), 56.3553);
    assert_eq!(arena.path_len([1.0, 39.0], [47.0, 14.0]), 56.3553);
    assert_eq!(arena.path_len([1.0, 4.0], [38.0, 47.0]), 58.3259);
    assert_eq!(arena.path_len([1.0, 4.0], [41.0, 42.0]), 56.9117);
    assert_eq!(arena.path_len([1.0, 42.0], [44.0, 5.0]), 58.3259);
    assert_eq!(arena.path_len([1.0, 3.0], [41.0, 47.0]), 60.5685);
    assert_eq!(arena.path_len([1.0, 3.0], [47.0, 37.0]), 60.0833);
    assert_eq!(arena.path_len([1.0, 39.0], [46.0, 1.0]), 60.7401);
    assert_eq!(arena.path_len([1.0, 4.0], [43.0, 46.0]), 60.5685);
    assert_eq!(arena.path_len([1.0, 4.0], [44.0, 45.0]), 61.1543);
    assert_eq!(arena.path_len([1.0, 40.0], [47.0, 3.0]), 61.3259);
    assert_eq!(arena.path_len([1.0, 41.0], [46.0, 2.0]), 61.1543);
    assert_eq!(arena.path_len([1.0, 45.0], [47.0, 9.0]), 60.9117);
    assert_eq!(arena.path_len([1.0, 7.0], [47.0, 44.0]), 61.3259);
    assert_eq!(arena.path_len([1.0, 7.0], [47.0, 46.0]), 62.1543);
}
