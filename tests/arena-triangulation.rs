use glam::{vec2, Vec2};
use polyanya::{Mesh, Triangulation};

macro_rules! assert_delta {
    ($x:expr, $y:expr) => {
        let val = $x.unwrap().length;
        if !((val - $y).abs() < 0.0001) {
            assert_eq!(val, $y);
        }
    };
}

fn arena_mesh() -> Mesh {
    let mut triangulation = Triangulation::from_outer_edges(vec![
        vec2(1., 3.),
        vec2(2., 3.),
        vec2(2., 2.),
        vec2(3., 2.),
        vec2(3., 1.),
        vec2(15., 1.),
        vec2(15., 3.),
        vec2(18., 3.),
        vec2(18., 2.),
        vec2(19., 2.),
        vec2(19., 1.),
        vec2(20., 1.),
        vec2(20., 2.),
        vec2(23., 2.),
        vec2(23., 1.),
        vec2(26., 1.),
        vec2(26., 3.),
        vec2(29., 3.),
        vec2(29., 2.),
        vec2(30., 2.),
        vec2(30., 1.),
        vec2(31., 1.),
        vec2(31., 3.),
        vec2(34., 3.),
        vec2(34., 2.),
        vec2(35., 2.),
        vec2(35., 1.),
        vec2(47., 1.),
        vec2(47., 3.),
        vec2(48., 3.),
        vec2(48., 15.),
        vec2(47., 15.),
        vec2(47., 19.),
        vec2(48., 19.),
        vec2(48., 31.),
        vec2(47., 31.),
        vec2(47., 35.),
        vec2(48., 35.),
        vec2(48., 47.),
        vec2(47., 47.),
        vec2(47., 48.),
        vec2(35., 48.),
        vec2(35., 47.),
        vec2(31., 47.),
        vec2(31., 48.),
        vec2(30., 48.),
        vec2(30., 47.),
        vec2(29., 47.),
        vec2(29., 46.),
        vec2(26., 46.),
        vec2(26., 48.),
        vec2(24., 48.),
        vec2(24., 47.),
        vec2(23., 47.),
        vec2(23., 46.),
        vec2(20., 46.),
        vec2(20., 48.),
        vec2(19., 48.),
        vec2(19., 47.),
        vec2(15., 47.),
        vec2(15., 48.),
        vec2(3., 48.),
        vec2(3., 47.),
        vec2(1., 47.),
        vec2(1., 35.),
        vec2(2., 35.),
        vec2(2., 34.),
        vec2(3., 34.),
        vec2(3., 31.),
        vec2(1., 31.),
        vec2(1., 30.),
        vec2(3., 30.),
        vec2(3., 27.),
        vec2(2., 27.),
        vec2(2., 26.),
        vec2(1., 26.),
        vec2(1., 23.),
        vec2(2., 23.),
        vec2(2., 18.),
        vec2(3., 18.),
        vec2(3., 15.),
        vec2(1., 15.),
    ]);

    triangulation.add_obstacle(vec![
        vec2(15., 15.),
        vec2(19., 15.),
        vec2(19., 18.),
        vec2(18., 18.),
        vec2(18., 19.),
        vec2(15., 19.),
    ]);
    triangulation.add_obstacle(vec![
        vec2(31., 15.),
        vec2(35., 15.),
        vec2(35., 18.),
        vec2(34., 18.),
        vec2(34., 19.),
        vec2(31., 19.),
    ]);
    triangulation.add_obstacle(vec![
        vec2(15., 31.),
        vec2(19., 31.),
        vec2(19., 34.),
        vec2(18., 34.),
        vec2(18., 35.),
        vec2(15., 35.),
    ]);
    triangulation.add_obstacle(vec![
        vec2(31., 31.),
        vec2(35., 31.),
        vec2(35., 34.),
        vec2(34., 34.),
        vec2(34., 35.),
        vec2(31., 35.),
    ]);
    triangulation.add_obstacle(vec![
        vec2(23., 10.),
        vec2(23., 8.),
        vec2(24., 8.),
        vec2(24., 7.),
        vec2(26., 7.),
        vec2(26., 10.),
    ]);
    triangulation.as_navmesh().unwrap()
}

#[test]
fn arena() {
    let arena = arena_mesh();

    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(1.0, 12.0)), 1.0);
    assert_delta!(arena.path(Vec2::new(1.0, 12.0), Vec2::new(1.0, 10.0)), 2.0);
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(4.0, 12.0)),
        3.16228
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 3.0), Vec2::new(3.0, 1.0)),
        3.41421
    );
    assert_delta!(arena.path(Vec2::new(1.0, 3.0), Vec2::new(4.0, 3.0)), 3.0);
    assert_delta!(
        arena.path(Vec2::new(1.0, 4.0), Vec2::new(4.0, 2.0)),
        3.60555
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 40.0), Vec2::new(2.0, 39.0)),
        1.41421
    );
    assert_delta!(arena.path(Vec2::new(1.0, 41.0), Vec2::new(1.0, 39.0)), 2.0);
    assert_delta!(arena.path(Vec2::new(1.0, 41.0), Vec2::new(1.0, 44.0)), 3.0);
    assert_delta!(
        arena.path(Vec2::new(1.0, 42.0), Vec2::new(4.0, 43.0)),
        3.16228
    );
    assert_delta!(arena.path(Vec2::new(1.0, 10.0), Vec2::new(7.0, 10.0)), 6.0);
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(1.0, 4.0)), 7.0);
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(7.0, 14.0)),
        6.7082
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(5.0, 7.0)),
        6.40312
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(6.0, 15.0)),
        5.83095
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(8.0, 11.0)),
        7.07107
    );
    assert_delta!(arena.path(Vec2::new(1.0, 14.0), Vec2::new(1.0, 9.0)), 5.0);
    assert_delta!(
        arena.path(Vec2::new(1.0, 24.0), Vec2::new(7.0, 26.0)),
        6.32456
    );
    assert_delta!(arena.path(Vec2::new(1.0, 25.0), Vec2::new(5.0, 25.0)), 4.0);
    assert_delta!(
        arena.path(Vec2::new(1.0, 35.0), Vec2::new(5.0, 33.0)),
        4.60555
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(4.0, 18.0)),
        7.63441
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(12.0, 14.0)),
        11.1803
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(4.0, 23.0)),
        10.8907
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(5.0, 3.0)),
        10.7703
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(6.0, 7.0)),
        7.81025
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(7.0, 7.0)),
        8.48528
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 23.0), Vec2::new(7.0, 32.0)),
        10.8167
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 24.0), Vec2::new(11.0, 25.0)),
        10.0499
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 24.0), Vec2::new(6.0, 32.0)),
        9.4365
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 25.0), Vec2::new(9.0, 24.0)),
        8.06226
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(11.0, 19.0)),
        13.4536
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(13.0, 11.0)),
        12.0416
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(10.0, 2.0)),
        12.7279
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(11.0, 21.0)),
        13.4536
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(13.0, 13.0)),
        12.0416
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(14.0, 12.0)),
        13.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(6.0, 25.0)),
        14.0459
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(11.0, 3.0)),
        14.1421
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(13.0, 11.0)),
        12.1655
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 14.0), Vec2::new(6.0, 23.0)),
        10.7801
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(18.0, 11.0)),
        17.0294
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(16.0, 14.0)),
        15.2971
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(14.0, 2.0)),
        16.4012
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(17.0, 13.0)),
        16.0312
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(9.0, 28.0)),
        17.9234
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(4.0, 30.0)),
        17.8617
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(9.0, 26.0)),
        15.3584
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 14.0), Vec2::new(14.0, 22.0)),
        15.2745
    );
    assert_delta!(arena.path(Vec2::new(1.0, 23.0), Vec2::new(10.0, 8.0)), 18.0);
    assert_delta!(
        arena.path(Vec2::new(1.0, 23.0), Vec2::new(14.0, 9.0)),
        19.4391
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(13.0, 29.0)),
        22.4722
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(18.0, 22.0)),
        20.8087
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(19.0, 18.0)),
        21.0575
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(21.0, 2.0)),
        21.5407
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(5.0, 32.0)),
        22.5024
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(6.0, 29.0)),
        19.703
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(20.0, 7.0)),
        19.4165
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(21.0, 17.0)),
        21.2675
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(22.0, 16.0)),
        21.6014
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(8.0, 29.0)),
        19.3382
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(22.0, 22.0)),
        24.2591
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(5.0, 33.0)),
        23.4959
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(10.0, 32.0)),
        22.8569
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(20.0, 31.0)),
        27.5862
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(21.0, 23.0)),
        23.3238
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(24.0, 14.0)),
        23.1948
    );
    assert_delta!(arena.path(Vec2::new(1.0, 11.0), Vec2::new(25.0, 4.0)), 25.0);
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(3.0, 36.0)),
        25.4721
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(19.0, 27.0)),
        23.4307
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(2.0, 37.0)),
        25.7678
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(22.0, 31.0)),
        29.6985
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(24.0, 27.0)),
        28.6007
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(28.0, 15.0)),
        27.4591
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(7.0, 39.0)),
        29.7162
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(12.0, 35.0)),
        26.4038
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(28.0, 18.0)),
        27.9259
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(5.0, 40.0)),
        29.552
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(26.0, 3.0)),
        26.5707
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(29.0, 14.0)),
        28.0713
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(29.0, 6.0)),
        28.6362
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(25.0, 36.0)),
        35.3836
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(27.0, 25.0)),
        30.0597
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(32.0, 4.0)),
        31.5753
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(33.0, 4.0)),
        32.5576
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(10.0, 42.0)),
        32.3648
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(27.0, 28.0)),
        31.0644
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(30.0, 2.0)),
        30.5347
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(31.0, 3.0)),
        31.0483
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(5.0, 42.0)),
        31.5461
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(18.0, 37.0)),
        30.5349
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(15.0, 43.0)),
        35.8469
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(21.0, 41.0)),
        37.1384
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(27.0, 37.0)),
        37.4833
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(29.0, 38.0)),
        39.598
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(31.0, 25.0)),
        33.7313
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(38.0, 13.0)),
        37.1214
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(38.0, 6.0)),
        37.2305
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(39.0, 6.0)),
        38.2281
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(40.0, 9.0)),
        39.0357
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(11.0, 43.0)),
        33.5926
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(12.0, 47.0)),
        38.6267
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(14.0, 47.0)),
        39.223
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(16.0, 46.0)),
        39.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(28.0, 41.0)),
        41.1096
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(37.0, 21.0)),
        37.6552
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(39.0, 24.0)),
        40.6133
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(16.0, 45.0)),
        37.1677
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(21.0, 43.0)),
        37.7849
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(32.0, 39.0)),
        41.7732
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(34.0, 29.0)),
        37.5954
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(29.0, 43.0)),
        43.2791
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(43.0, 15.0)),
        42.2966
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(43.0, 17.0)),
        42.6119
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(45.0, 10.0)),
        44.0
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(46.0, 15.0)),
        45.2769
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(46.0, 3.0)),
        45.5412
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(30.0, 45.0)),
        44.7117
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(35.0, 41.0)),
        45.3431
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(35.0, 42.0)),
        46.0109
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(43.0, 3.0)),
        42.7551
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(31.0, 46.0)),
        46.868
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(34.0, 46.0)),
        48.8365
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(35.0, 46.0)),
        49.5177
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(36.0, 42.0)),
        47.4236
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(38.0, 45.0)),
        50.9313
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(46.0, 18.0)),
        45.7674
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(37.0, 44.0)),
        48.8365
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(41.0, 35.0)),
        46.6573
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(43.0, 27.0)),
        45.245
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(44.0, 25.0)),
        45.3649
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(41.0, 40.0)),
        50.2316
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 10.0), Vec2::new(43.0, 40.0)),
        52.0087
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(39.0, 47.0)),
        52.345
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(42.0, 46.0)),
        53.9073
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 11.0), Vec2::new(45.0, 33.0)),
        49.2304
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(43.0, 43.0)),
        52.2243
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(44.0, 38.0)),
        50.3504
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 12.0), Vec2::new(46.0, 34.0)),
        50.0908
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 13.0), Vec2::new(42.0, 40.0)),
        49.2852
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 14.0), Vec2::new(46.0, 32.0)),
        48.4665
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 14.0), Vec2::new(44.0, 46.0)),
        53.6365
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 14.0), Vec2::new(46.0, 43.0)),
        53.6454
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 35.0), Vec2::new(46.0, 3.0)),
        55.4059
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 37.0), Vec2::new(43.0, 1.0)),
        55.3173
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 38.0), Vec2::new(43.0, 3.0)),
        54.6717
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 38.0), Vec2::new(47.0, 13.0)),
        52.3546
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 39.0), Vec2::new(47.0, 14.0)),
        52.3599
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 4.0), Vec2::new(38.0, 47.0)),
        56.7274
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 4.0), Vec2::new(41.0, 42.0)),
        55.3523
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 42.0), Vec2::new(44.0, 5.0)),
        56.7621
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 3.0), Vec2::new(41.0, 47.0)),
        59.4702
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 3.0), Vec2::new(47.0, 37.0)),
        57.2423
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 39.0), Vec2::new(46.0, 1.0)),
        58.8982
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 4.0), Vec2::new(43.0, 46.0)),
        59.4245
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 4.0), Vec2::new(44.0, 45.0)),
        59.5469
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 40.0), Vec2::new(47.0, 3.0)),
        59.0512
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 41.0), Vec2::new(46.0, 2.0)),
        59.5483
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 45.0), Vec2::new(47.0, 9.0)),
        58.6718
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 7.0), Vec2::new(47.0, 44.0)),
        59.3941
    );
    assert_delta!(
        arena.path(Vec2::new(1.0, 7.0), Vec2::new(47.0, 46.0)),
        60.4531
    );
}
