use std::io::{self, BufRead};

use glam::Vec2;
use polyanya::{Mesh, PolyanyaFile};

struct Scenario {
    from: Vec2,
    to: Vec2,
}

struct Scenarios(Vec<Scenario>);

impl Scenarios {
    pub fn from_file(path: &str) -> Scenarios {
        let file = std::fs::File::open(path).unwrap();

        Scenarios(
            io::BufReader::new(file)
                .lines()
                .skip(1)
                .flatten()
                .map(|line| {
                    let mut split = line.split('\t');
                    Scenario {
                        from: Vec2::new(
                            split.nth(4).unwrap().parse::<i32>().unwrap() as f32,
                            split.next().unwrap().parse::<i32>().unwrap() as f32,
                        ),
                        to: Vec2::new(
                            split.next().unwrap().parse::<i32>().unwrap() as f32,
                            split.next().unwrap().parse::<i32>().unwrap() as f32,
                        ),
                    }
                })
                .collect(),
        )
    }
}

fn main() {
    use tracing_subscriber::layer::SubscriberExt;

    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default()),
    )
    .expect("set up the subscriber");

    let mut args = std::env::args();
    args.next();

    let mesh: Mesh = PolyanyaFile::from_file(&args.next().unwrap()).into();

    for scenario in Scenarios::from_file(&args.next().unwrap()).0 {
        mesh.path(scenario.from, scenario.to).unwrap();
    }
}
