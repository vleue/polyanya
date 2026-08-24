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
        if (val - $y).abs() >= 0.0001 {
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
        .map_while(Result::ok)
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
    let mut path: String = "meshes/v3/scene_mp_2p_01.mesh".into();
    let mesh = v3_mesh(&path);
    path.push_str(".scen");
    let scenarios = load_v3_scenario(&path);

    for (i, scenario) in scenarios.iter().enumerate() {
        // Only test the first 20 scenarios
        if i >= 20 {
            break;
        }
        assert_delta!(mesh.path(scenario.start, scenario.goal), scenario.cost);
    }
}

/// A path has to stay inside the mesh between its turns, and the same query has to cost
/// the same whichever feature set is compiled in.
///
/// The successor walk used to locate the edge it came in through by matching coordinates
/// with a tolerance, and this mesh has a triangle whose two corners are 0.008 apart,
/// closer than that tolerance. Matching the wrong corner started the walk an edge early,
/// which handed one edge's observability to the next and let the root propagate where it
/// should have had to turn a corner. The resulting path cut across a wall, and it looked
/// *shorter* than the real one, so it won under `detailed-layers`, which keeps every path
/// the search reaches and takes the shortest.
#[test]
fn path_stays_in_the_mesh_around_close_together_corners() {
    let mesh = v3_mesh("meshes/v3/scene_mp_2p_01.mesh");
    let from = Vec2::new(-89.27931, -9.702867);
    let to = Vec2::new(35.1644, -103.2862);

    let path = mesh.path(from, to).unwrap();

    let mut previous = from;
    for point in &path.path {
        let steps = ((previous.distance(*point) / 0.25).ceil() as usize).max(1);
        for step in 1..steps {
            let sample = previous.lerp(*point, step as f32 / steps as f32);
            assert!(
                mesh.point_in_mesh(sample),
                "the path leaves the mesh at {sample:?}, between {previous:?} and {point:?}"
            );
        }
        previous = *point;
    }

    assert_delta!(Some(path), 169.52078);
}
