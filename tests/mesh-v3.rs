use std::io::{BufRead, BufReader};

use glam::Vec2;
use polyanya::{Mesh, PolyanyaFile};

struct Scenario {
    pub start: Vec2,
    pub goal: Vec2,
    pub cost: f32,
}

macro_rules! assert_delta {
    ($x:expr, $y:expr) => {
        let val = $x.unwrap().length;
        if !((val - $y).abs() < 0.0001) {
            assert_eq!(val, $y);
        }
    };
}

fn v3_mesh(path: &str) -> Mesh {
    PolyanyaFile::from_file(path).try_into().unwrap()
}

fn load_v3_scenario(path: &str) -> Vec<Scenario> {
    let file = std::fs::File::open(path).unwrap();

    let mut lines = BufReader::new(file).lines();
    // Check the header
    if lines.next().unwrap().unwrap() != "version 1" {
        panic!("bad file, version header does not match 'version 1'")
    }
    lines
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut values = line.split_whitespace().skip(4);
            let start = Vec2::new(
                values.next().unwrap().parse().unwrap(),
                values.next().unwrap().parse().unwrap(),
            );
            let goal = Vec2::new(
                values.next().unwrap().parse().unwrap(),
                values.next().unwrap().parse().unwrap(),
            );
            let cost: f32 = values.next().unwrap().parse().unwrap();
            Scenario { start, goal, cost }
        })
        .collect()
}

#[test]
/// Test that loading a basic mesh works.
fn load_v3() {
    let path: String = "meshes/v3/cube.mesh".into();
    let _ = v3_mesh(&path);
}

#[test]
fn v3_scenario() {
    // TODO: consider shortening this test.
    let mut path: String = "meshes/v3/scene_mp_2p_01.mesh".into();
    let mesh = v3_mesh(&path);
    path.push_str(".scen");
    let scenarios = load_v3_scenario(&path);

    for scenario in scenarios {
        assert_delta!(mesh.path(scenario.start, scenario.goal), scenario.cost);
    }
}
