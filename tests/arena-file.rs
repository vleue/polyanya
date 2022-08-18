use polyanya::Mesh;

macro_rules! assert_delta {
    ($x:expr, $y:expr) => {
        let val = $x;
        if !((val - $y).abs() < 0.0001) {
            assert_eq!(val, $y);
        }
    };
}

fn arena_mesh() -> Mesh {
    Mesh::from_file("meshes/arena-merged.mesh".into())
}

#[test]
fn arena_file() {
    let arena = arena_mesh();

    assert_delta!(arena.path([1.0, 11.0], [1.0, 12.0]).len, 1.0);
    assert_delta!(arena.path([1.0, 12.0], [1.0, 10.0]).len, 2.0);
    assert_delta!(arena.path([1.0, 13.0], [4.0, 12.0]).len, 3.16228);
    assert_delta!(arena.path([1.0, 3.0], [3.0, 1.0]).len, 3.41421);
    assert_delta!(arena.path([1.0, 3.0], [4.0, 3.0]).len, 3.0);
    assert_delta!(arena.path([1.0, 4.0], [4.0, 2.0]).len, 3.60555);
    assert_delta!(arena.path([1.0, 40.0], [2.0, 39.0]).len, 1.41421);
    assert_delta!(arena.path([1.0, 41.0], [1.0, 39.0]).len, 2.0);
    assert_delta!(arena.path([1.0, 41.0], [1.0, 44.0]).len, 3.0);
    assert_delta!(arena.path([1.0, 42.0], [4.0, 43.0]).len, 3.16228);
    assert_delta!(arena.path([1.0, 10.0], [7.0, 10.0]).len, 6.0);
    assert_delta!(arena.path([1.0, 11.0], [1.0, 4.0]).len, 7.0);
    assert_delta!(arena.path([1.0, 11.0], [7.0, 14.0]).len, 6.7082);
    assert_delta!(arena.path([1.0, 12.0], [5.0, 7.0]).len, 6.40312);
    assert_delta!(arena.path([1.0, 12.0], [6.0, 15.0]).len, 5.83095);
    assert_delta!(arena.path([1.0, 12.0], [8.0, 11.0]).len, 7.07107);
    assert_delta!(arena.path([1.0, 14.0], [1.0, 9.0]).len, 5.0);
    assert_delta!(arena.path([1.0, 24.0], [7.0, 26.0]).len, 6.32456);
    assert_delta!(arena.path([1.0, 25.0], [5.0, 25.0]).len, 4.0);
    assert_delta!(arena.path([1.0, 35.0], [5.0, 33.0]).len, 4.60555);
    assert_delta!(arena.path([1.0, 11.0], [4.0, 18.0]).len, 7.63441);
    assert_delta!(arena.path([1.0, 12.0], [12.0, 14.0]).len, 11.1803);
    assert_delta!(arena.path([1.0, 13.0], [4.0, 23.0]).len, 10.8907);
    assert_delta!(arena.path([1.0, 13.0], [5.0, 3.0]).len, 10.7703);
    assert_delta!(arena.path([1.0, 13.0], [6.0, 7.0]).len, 7.81025);
    assert_delta!(arena.path([1.0, 13.0], [7.0, 7.0]).len, 8.48528);
    assert_delta!(arena.path([1.0, 23.0], [7.0, 32.0]).len, 10.8167);
    assert_delta!(arena.path([1.0, 24.0], [11.0, 25.0]).len, 10.0499);
    assert_delta!(arena.path([1.0, 24.0], [6.0, 32.0]).len, 9.4365);
    assert_delta!(arena.path([1.0, 25.0], [9.0, 24.0]).len, 8.06226);
    assert_delta!(arena.path([1.0, 10.0], [11.0, 19.0]).len, 13.4536);
    assert_delta!(arena.path([1.0, 10.0], [13.0, 11.0]).len, 12.0416);
    assert_delta!(arena.path([1.0, 11.0], [10.0, 2.0]).len, 12.7279);
    assert_delta!(arena.path([1.0, 12.0], [11.0, 21.0]).len, 13.4536);
    assert_delta!(arena.path([1.0, 12.0], [13.0, 13.0]).len, 12.0416);
    assert_delta!(arena.path([1.0, 12.0], [14.0, 12.0]).len, 13.0);
    assert_delta!(arena.path([1.0, 12.0], [6.0, 25.0]).len, 14.0459);
    assert_delta!(arena.path([1.0, 13.0], [11.0, 3.0]).len, 14.1421);
    assert_delta!(arena.path([1.0, 13.0], [13.0, 11.0]).len, 12.1655);
    assert_delta!(arena.path([1.0, 14.0], [6.0, 23.0]).len, 10.7801);
    assert_delta!(arena.path([1.0, 10.0], [18.0, 11.0]).len, 17.0294);
    assert_delta!(arena.path([1.0, 11.0], [16.0, 14.0]).len, 15.2971);
    assert_delta!(arena.path([1.0, 12.0], [14.0, 2.0]).len, 16.4012);
    assert_delta!(arena.path([1.0, 12.0], [17.0, 13.0]).len, 16.0312);
    assert_delta!(arena.path([1.0, 12.0], [9.0, 28.0]).len, 17.9234);
    assert_delta!(arena.path([1.0, 13.0], [4.0, 30.0]).len, 17.8617);
    assert_delta!(arena.path([1.0, 13.0], [9.0, 26.0]).len, 15.3584);
    assert_delta!(arena.path([1.0, 14.0], [14.0, 22.0]).len, 15.2745);
    assert_delta!(arena.path([1.0, 23.0], [10.0, 8.0]).len, 18.0);
    assert_delta!(arena.path([1.0, 23.0], [14.0, 9.0]).len, 19.4391);
    assert_delta!(arena.path([1.0, 10.0], [13.0, 29.0]).len, 22.4722);
    assert_delta!(arena.path([1.0, 10.0], [18.0, 22.0]).len, 20.8087);
    assert_delta!(arena.path([1.0, 10.0], [19.0, 18.0]).len, 21.0575);
    assert_delta!(arena.path([1.0, 10.0], [21.0, 2.0]).len, 21.5407);
    assert_delta!(arena.path([1.0, 10.0], [5.0, 32.0]).len, 22.5024);
    assert_delta!(arena.path([1.0, 10.0], [6.0, 29.0]).len, 19.703);
    assert_delta!(arena.path([1.0, 11.0], [20.0, 7.0]).len, 19.4165);
    assert_delta!(arena.path([1.0, 11.0], [21.0, 17.0]).len, 21.2675);
    assert_delta!(arena.path([1.0, 11.0], [22.0, 16.0]).len, 21.6014);
    assert_delta!(arena.path([1.0, 11.0], [8.0, 29.0]).len, 19.3382);
    assert_delta!(arena.path([1.0, 10.0], [22.0, 22.0]).len, 24.2591);
    assert_delta!(arena.path([1.0, 10.0], [5.0, 33.0]).len, 23.4959);
    assert_delta!(arena.path([1.0, 11.0], [10.0, 32.0]).len, 22.8569);
    assert_delta!(arena.path([1.0, 11.0], [20.0, 31.0]).len, 27.5862);
    assert_delta!(arena.path([1.0, 11.0], [21.0, 23.0]).len, 23.3238);
    assert_delta!(arena.path([1.0, 11.0], [24.0, 14.0]).len, 23.1948);
    assert_delta!(arena.path([1.0, 11.0], [25.0, 4.0]).len, 25.0);
    assert_delta!(arena.path([1.0, 11.0], [3.0, 36.0]).len, 25.4721);
    assert_delta!(arena.path([1.0, 12.0], [19.0, 27.0]).len, 23.4307);
    assert_delta!(arena.path([1.0, 12.0], [2.0, 37.0]).len, 25.7678);
    assert_delta!(arena.path([1.0, 10.0], [22.0, 31.0]).len, 29.6985);
    assert_delta!(arena.path([1.0, 10.0], [24.0, 27.0]).len, 28.6007);
    assert_delta!(arena.path([1.0, 10.0], [28.0, 15.0]).len, 27.4591);
    assert_delta!(arena.path([1.0, 10.0], [7.0, 39.0]).len, 29.7162);
    assert_delta!(arena.path([1.0, 11.0], [12.0, 35.0]).len, 26.4038);
    assert_delta!(arena.path([1.0, 11.0], [28.0, 18.0]).len, 27.9259);
    assert_delta!(arena.path([1.0, 11.0], [5.0, 40.0]).len, 29.552);
    assert_delta!(arena.path([1.0, 12.0], [26.0, 3.0]).len, 26.5707);
    assert_delta!(arena.path([1.0, 12.0], [29.0, 14.0]).len, 28.0713);
    assert_delta!(arena.path([1.0, 12.0], [29.0, 6.0]).len, 28.6362);
    assert_delta!(arena.path([1.0, 10.0], [25.0, 36.0]).len, 35.3836);
    assert_delta!(arena.path([1.0, 10.0], [27.0, 25.0]).len, 30.0597);
    assert_delta!(arena.path([1.0, 10.0], [32.0, 4.0]).len, 31.5753);
    assert_delta!(arena.path([1.0, 10.0], [33.0, 4.0]).len, 32.5576);
    assert_delta!(arena.path([1.0, 11.0], [10.0, 42.0]).len, 32.3648);
    assert_delta!(arena.path([1.0, 11.0], [27.0, 28.0]).len, 31.0644);
    assert_delta!(arena.path([1.0, 11.0], [30.0, 2.0]).len, 30.5347);
    assert_delta!(arena.path([1.0, 11.0], [31.0, 3.0]).len, 31.0483);
    assert_delta!(arena.path([1.0, 11.0], [5.0, 42.0]).len, 31.5461);
    assert_delta!(arena.path([1.0, 12.0], [18.0, 37.0]).len, 30.5349);
    assert_delta!(arena.path([1.0, 10.0], [15.0, 43.0]).len, 35.8469);
    assert_delta!(arena.path([1.0, 10.0], [21.0, 41.0]).len, 37.1384);
    assert_delta!(arena.path([1.0, 10.0], [27.0, 37.0]).len, 37.4833);
    assert_delta!(arena.path([1.0, 10.0], [29.0, 38.0]).len, 39.598);
    assert_delta!(arena.path([1.0, 10.0], [31.0, 25.0]).len, 33.7313);
    assert_delta!(arena.path([1.0, 10.0], [38.0, 13.0]).len, 37.1214);
    assert_delta!(arena.path([1.0, 10.0], [38.0, 6.0]).len, 37.2305);
    assert_delta!(arena.path([1.0, 10.0], [39.0, 6.0]).len, 38.2281);
    assert_delta!(arena.path([1.0, 10.0], [40.0, 9.0]).len, 39.0357);
    assert_delta!(arena.path([1.0, 11.0], [11.0, 43.0]).len, 33.5926);
    assert_delta!(arena.path([1.0, 10.0], [12.0, 47.0]).len, 38.6267);
    assert_delta!(arena.path([1.0, 10.0], [14.0, 47.0]).len, 39.223);
    assert_delta!(arena.path([1.0, 10.0], [16.0, 46.0]).len, 39.0);
    assert_delta!(arena.path([1.0, 10.0], [28.0, 41.0]).len, 41.1096);
    assert_delta!(arena.path([1.0, 10.0], [37.0, 21.0]).len, 37.6552);
    assert_delta!(arena.path([1.0, 10.0], [39.0, 24.0]).len, 40.6133);
    assert_delta!(arena.path([1.0, 11.0], [16.0, 45.0]).len, 37.1677);
    assert_delta!(arena.path([1.0, 11.0], [21.0, 43.0]).len, 37.7849);
    assert_delta!(arena.path([1.0, 11.0], [32.0, 39.0]).len, 41.7732);
    assert_delta!(arena.path([1.0, 11.0], [34.0, 29.0]).len, 37.5954);
    assert_delta!(arena.path([1.0, 10.0], [29.0, 43.0]).len, 43.2791);
    assert_delta!(arena.path([1.0, 10.0], [43.0, 15.0]).len, 42.2966);
    assert_delta!(arena.path([1.0, 10.0], [43.0, 17.0]).len, 42.6119);
    assert_delta!(arena.path([1.0, 10.0], [45.0, 10.0]).len, 44.0);
    assert_delta!(arena.path([1.0, 10.0], [46.0, 15.0]).len, 45.2769);
    assert_delta!(arena.path([1.0, 10.0], [46.0, 3.0]).len, 45.5412);
    assert_delta!(arena.path([1.0, 11.0], [30.0, 45.0]).len, 44.7117);
    assert_delta!(arena.path([1.0, 11.0], [35.0, 41.0]).len, 45.3431);
    assert_delta!(arena.path([1.0, 11.0], [35.0, 42.0]).len, 46.0109);
    assert_delta!(arena.path([1.0, 11.0], [43.0, 3.0]).len, 42.7551);
    assert_delta!(arena.path([1.0, 10.0], [31.0, 46.0]).len, 46.868);
    assert_delta!(arena.path([1.0, 10.0], [34.0, 46.0]).len, 48.8365);
    assert_delta!(arena.path([1.0, 10.0], [35.0, 46.0]).len, 49.5177);
    assert_delta!(arena.path([1.0, 10.0], [36.0, 42.0]).len, 47.4236);
    assert_delta!(arena.path([1.0, 10.0], [38.0, 45.0]).len, 50.9313);
    assert_delta!(arena.path([1.0, 10.0], [46.0, 18.0]).len, 45.7674);
    assert_delta!(arena.path([1.0, 11.0], [37.0, 44.0]).len, 48.8365);
    assert_delta!(arena.path([1.0, 11.0], [41.0, 35.0]).len, 46.6573);
    assert_delta!(arena.path([1.0, 11.0], [43.0, 27.0]).len, 45.245);
    assert_delta!(arena.path([1.0, 11.0], [44.0, 25.0]).len, 45.3649);
    assert_delta!(arena.path([1.0, 10.0], [41.0, 40.0]).len, 50.2316);
    assert_delta!(arena.path([1.0, 10.0], [43.0, 40.0]).len, 52.0087);
    assert_delta!(arena.path([1.0, 11.0], [39.0, 47.0]).len, 52.345);
    assert_delta!(arena.path([1.0, 11.0], [42.0, 46.0]).len, 53.9073);
    assert_delta!(arena.path([1.0, 11.0], [45.0, 33.0]).len, 49.2304);
    assert_delta!(arena.path([1.0, 12.0], [43.0, 43.0]).len, 52.2243);
    assert_delta!(arena.path([1.0, 12.0], [44.0, 38.0]).len, 50.3504);
    assert_delta!(arena.path([1.0, 12.0], [46.0, 34.0]).len, 50.0908);
    assert_delta!(arena.path([1.0, 13.0], [42.0, 40.0]).len, 49.2852);
    assert_delta!(arena.path([1.0, 14.0], [46.0, 32.0]).len, 48.4665);
    assert_delta!(arena.path([1.0, 14.0], [44.0, 46.0]).len, 53.6365);
    assert_delta!(arena.path([1.0, 14.0], [46.0, 43.0]).len, 53.6454);
    assert_delta!(arena.path([1.0, 35.0], [46.0, 3.0]).len, 55.4059);
    assert_delta!(arena.path([1.0, 37.0], [43.0, 1.0]).len, 55.3173);
    assert_delta!(arena.path([1.0, 38.0], [43.0, 3.0]).len, 54.6717);
    assert_delta!(arena.path([1.0, 38.0], [47.0, 13.0]).len, 52.3546);
    assert_delta!(arena.path([1.0, 39.0], [47.0, 14.0]).len, 52.3599);
    assert_delta!(arena.path([1.0, 4.0], [38.0, 47.0]).len, 56.7274);
    assert_delta!(arena.path([1.0, 4.0], [41.0, 42.0]).len, 55.3523);
    assert_delta!(arena.path([1.0, 42.0], [44.0, 5.0]).len, 56.7621);
    assert_delta!(arena.path([1.0, 3.0], [41.0, 47.0]).len, 59.4702);
    assert_delta!(arena.path([1.0, 3.0], [47.0, 37.0]).len, 57.2423);
    assert_delta!(arena.path([1.0, 39.0], [46.0, 1.0]).len, 58.8982);
    assert_delta!(arena.path([1.0, 4.0], [43.0, 46.0]).len, 59.4245);
    assert_delta!(arena.path([1.0, 4.0], [44.0, 45.0]).len, 59.5469);
    assert_delta!(arena.path([1.0, 40.0], [47.0, 3.0]).len, 59.0512);
    assert_delta!(arena.path([1.0, 41.0], [46.0, 2.0]).len, 59.5483);
    assert_delta!(arena.path([1.0, 45.0], [47.0, 9.0]).len, 58.6718);
    assert_delta!(arena.path([1.0, 7.0], [47.0, 44.0]).len, 59.3941);
    assert_delta!(arena.path([1.0, 7.0], [47.0, 46.0]).len, 60.4531);
}