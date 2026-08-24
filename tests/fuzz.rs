//! Randomized ("fuzzy") tests for pathfinding.
//!
//! The other test files check a fixed list of start/goal pairs against precomputed
//! costs. That covers the queries someone thought about, and nothing else. These tests
//! throw random queries at the bundled meshes instead, and check the properties any
//! correct answer has to satisfy, so that a path which is valid but too long, or a length
//! that doesn't match the polyline it comes with, fails without anyone having to know
//! the right cost up front.
//!
//! Runs are deterministic. Set `POLYANYA_FUZZ_SEED` to replay a failure, and
//! `POLYANYA_FUZZ_ITERATIONS` to soak for longer than the default.

use std::sync::OnceLock;

use glam::Vec2;
use polyanya::{Mesh, Path, PolyanyaFile};

/// xorshift64*, so the tests carry no dependency for something this small.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        // xorshift is stuck at zero, and a seed of 0 is the one a user is most likely
        // to type by hand.
        Rng(seed ^ 0x9E37_79B9_7F4A_7C15)
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// A float in `[0, 1)`, from the 24 bits an `f32` can hold exactly.
    fn f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u32 << 24) as f32
    }

    fn range(&mut self, min: f32, max: f32) -> f32 {
        min + self.f32() * (max - min)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
}

fn seed() -> u64 {
    std::env::var("POLYANYA_FUZZ_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0x5EED_C0FF_EE00_1234)
}

fn iterations(default: usize) -> usize {
    std::env::var("POLYANYA_FUZZ_ITERATIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

/// Lengths here run from fractions of a unit (arena) to well over a thousand (aurora),
/// and `f32` accumulation over a long polyline drifts with the length. A fixed epsilon
/// would either be noise at the top of that range or unusable at the bottom.
/// How finely the segments between waypoints are sampled when checking that a path stays
/// on the mesh. Small enough to catch a corner cut across a wall, large enough that the
/// thousand-unit aurora paths don't dominate the test's runtime.
const SAMPLE_EVERY: f32 = 0.25;

fn tolerance(length: f32) -> f32 {
    1e-3_f32.max(length.abs() * 1e-4)
}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() <= tolerance(a.abs().max(b.abs()))
}

/// Axis aligned bounds of the whole mesh, in mesh coordinates (layer coordinates are
/// relative to the layer offset).
fn bounds(mesh: &Mesh) -> (Vec2, Vec2) {
    let mut min = Vec2::splat(f32::MAX);
    let mut max = Vec2::splat(f32::MIN);
    for layer in &mesh.layers {
        for vertex in &layer.vertices {
            min = min.min(vertex.coords + layer.offset);
            max = max.max(vertex.coords + layer.offset);
        }
    }
    (min, max)
}

/// Is the point really on the mesh, with no snapping?
///
/// [`Mesh::point_in_mesh`] deliberately answers yes up to [`Mesh::search_delta`] away, so a
/// query point can sit outside a wall and still be accepted, and the search then snaps it
/// to the closest polygon. Which polygon that is depends on where the query came from, so a
/// snapped endpoint gives two different (and both defensible) answers for the two
/// directions of the same query. That is the documented contract, not a bug, so the
/// generators have to stay off those points or the properties below test the snapping
/// instead of the search.
///
/// Points exactly on a vertex or an edge count as inside: they are on the mesh, and they
/// are the inputs worth fuzzing.
fn strictly_on_mesh(mesh: &Mesh, point: Vec2) -> bool {
    mesh.get_point_layer(point).iter().any(|coords| {
        let Some(layer) = coords.layer().and_then(|l| mesh.layers.get(l as usize)) else {
            return false;
        };
        let local = point - layer.offset;
        let corners = &layer.polygons[coords.polygon() as usize].vertices;
        let n = corners.len();
        n >= 3
            && (0..n).all(|i| {
                let a = layer.vertices[corners[i] as usize].coords;
                let b = layer.vertices[corners[(i + 1) % n] as usize].coords;
                // counter clockwise polygons: inside is to the left of every edge
                (b - a).perp_dot(local - a) >= -1e-5
            })
    })
}

/// A point somewhere in the mesh, sampled uniformly over the bounding box and rejected
/// until it lands on the mesh. Gives no weight to small polygons, which is the point:
/// it goes where the polygon seeded generator doesn't.
fn point_by_rejection(mesh: &Mesh, rng: &mut Rng, (min, max): (Vec2, Vec2)) -> Option<Vec2> {
    for _ in 0..64 {
        let point = Vec2::new(rng.range(min.x, max.x), rng.range(min.y, max.y));
        if strictly_on_mesh(mesh, point) {
            return Some(point);
        }
    }
    None
}

/// A point in a random polygon, as a random convex combination of its vertices. Always
/// on the mesh, and it hits every polygon with the same probability however small it is.
/// One time in four the weights are snapped so the point lands exactly on a vertex or in
/// the middle of an edge. Those are the degenerate, collinear inputs the search's fast
/// paths care about, and uniform sampling reaches them with probability zero.
fn point_in_random_polygon(mesh: &Mesh, rng: &mut Rng) -> Option<Vec2> {
    let layer_index = rng.below(mesh.layers.len());
    let layer = &mesh.layers[layer_index];
    // Deleted polygons are left in place as empty ones, so retry rather than give up.
    for _ in 0..16 {
        let polygon = &layer.polygons[rng.below(layer.polygons.len())];
        if polygon.vertices.is_empty() {
            continue;
        }
        let corners = polygon
            .vertices
            .iter()
            .map(|v| layer.vertices[*v as usize].coords + layer.offset)
            .collect::<Vec<_>>();

        let point = match rng.below(4) {
            // exactly on a vertex
            0 => corners[rng.below(corners.len())],
            // exactly in the middle of an edge
            1 => {
                let first = rng.below(corners.len());
                (corners[first] + corners[(first + 1) % corners.len()]) / 2.0
            }
            // anywhere inside
            _ => {
                let weights = corners.iter().map(|_| rng.f32()).collect::<Vec<_>>();
                let total: f32 = weights.iter().sum();
                if total <= 0.0 {
                    continue;
                }
                corners
                    .iter()
                    .zip(&weights)
                    .fold(Vec2::ZERO, |acc, (corner, weight)| acc + *corner * *weight)
                    / total
            }
        };
        // A convex combination of a convex polygon's corners is inside it, but rounding on
        // the way there can still leave it a hair outside, so it gets the same check as
        // anything else.
        if strictly_on_mesh(mesh, point) {
            return Some(point);
        }
    }
    None
}

fn random_point(mesh: &Mesh, rng: &mut Rng, bounds: (Vec2, Vec2)) -> Option<Vec2> {
    if rng.below(2) == 0 {
        point_by_rejection(mesh, rng, bounds)
    } else {
        point_in_random_polygon(mesh, rng)
    }
}

/// Everything needed to replay a failure, printed by every assertion.
struct Query {
    mesh: &'static str,
    seed: u64,
    from: Vec2,
    to: Vec2,
}

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} from {:?} to {:?} (replay with POLYANYA_FUZZ_SEED={})",
            self.mesh, self.from, self.to, self.seed
        )
    }
}

/// Check the properties of one path on its own: that it is well formed, that its length
/// describes it, and that it stays on the mesh. Run for both directions of a query, since
/// a search bug is free to show up in only one of them.
fn check_path(mesh: &Mesh, from: Vec2, to: Vec2, path: &Path, query: &Query) {
    // 2. no shortcut: nothing beats the straight line.
    assert!(
        path.length.is_finite(),
        "length is {}, {query}",
        path.length
    );
    let straight = from.distance(to);
    assert!(
        path.length >= straight - tolerance(straight),
        "length {} is shorter than the straight line {straight}, {query}",
        path.length
    );

    // 3. the path is well formed: it doesn't include the start, and it ends at the goal.
    assert!(!path.path.is_empty(), "empty path, {query}");
    let last = *path.path.last().unwrap();
    assert!(
        close(last.x, to.x) && close(last.y, to.y),
        "path ends at {last:?} instead of the goal, {query}"
    );

    // 4. the length matches the polyline it is handed out with. `length` is computed
    // from the search's own accumulators, not by summing `path`, so these really can
    // disagree.
    let summed = path
        .path
        .iter()
        .fold((0.0, from), |(total, previous), point| {
            (total + previous.distance(*point), *point)
        })
        .0;
    assert!(
        close(summed, path.length),
        "length {} but the path is {summed} long, {query}",
        path.length
    );

    // 5. the path stays on the mesh, and not just at its waypoints: a path that drops a
    // waypoint and cuts a corner through a wall has every remaining waypoint on the mesh
    // and only leaves it in between. Sampling can miss a crossing thinner than the step,
    // so this under-reports, but a sample that lands off the mesh is proof either way.
    let mut previous = from;
    for point in &path.path {
        assert!(
            mesh.point_in_mesh(*point),
            "waypoint {point:?} is off the mesh, {query}"
        );
        let steps = ((previous.distance(*point) / SAMPLE_EVERY).ceil() as usize).max(1);
        for step in 1..steps {
            let sample = previous.lerp(*point, step as f32 / steps as f32);
            assert!(
                mesh.point_in_mesh(sample),
                "the path leaves the mesh at {sample:?}, between {previous:?} and \
                 {point:?}, {query}"
            );
        }
        previous = *point;
    }

    // 6. polygon indices are in range, so `Path::path_with_height` can't index out of
    // bounds on this path.
    let polygons = path.polygons();
    assert!(
        !polygons.is_empty(),
        "path goes through no polygon, {query}"
    );
    for (layer_index, polygon_index) in &polygons {
        let layer = mesh
            .layers
            .get(*layer_index as usize)
            .unwrap_or_else(|| panic!("path goes through unknown layer {layer_index}, {query}"));
        assert!(
            (*polygon_index as usize) < layer.polygons.len(),
            "path goes through polygon {polygon_index} of layer {layer_index}, \
             which only has {} polygons, {query}",
            layer.polygons.len()
        );
    }
}

/// Check everything that has to hold for a path between two points on the mesh,
/// whatever the two points are.
fn check_query(mesh: &Mesh, query: &Query, third: Option<Vec2>) {
    let (from, to) = (query.from, query.to);
    let Some(path) = mesh.path(from, to) else {
        // 1. reachability is symmetric. The bundled maps are not fully connected, so
        // "there is always a path" isn't assertable, but this is.
        assert!(
            mesh.path(to, from).is_none(),
            "no path one way but a path the other way, {query}"
        );
        return;
    };
    check_path(mesh, from, to, &path, query);

    // 7. the cost is the same both ways.
    let back = mesh
        .path(to, from)
        .unwrap_or_else(|| panic!("path one way but none back, {query}"));
    check_path(
        mesh,
        to,
        from,
        &back,
        &Query {
            from: to,
            to: from,
            ..*query
        },
    );
    assert!(
        close(path.length, back.length),
        "costs {} one way and {} back, {query}",
        path.length,
        back.length
    );

    // 8. sub-paths of an optimal path are optimal: walking to the first turn and asking
    // again can't cost less than what the full path left for that leg, or the full path
    // was longer than it needed to be. A path that is valid but too long passes
    // everything above and fails here.
    //
    // Only that one direction is asserted. The other case, where the leg costs *more* than
    // the full path left for it, happens today without anything being wrong with the full
    // path: turns are on corner vertices, a query starting exactly on a vertex resolves
    // to one of the polygons around it, and from the wrong side of the corner the search
    // has to go the long way round. That predates the search optimisations (it reproduces
    // on 7869725), so it isn't this test's to fail on.
    if path.path.len() > 1 {
        let turn = path.path[0];
        let remaining = mesh
            .path(turn, to)
            .unwrap_or_else(|| panic!("no path from the turn {turn:?} to the goal, {query}"));
        let expected = path.length - from.distance(turn);
        assert!(
            remaining.length >= expected - tolerance(expected),
            "the leg from the first turn {turn:?} costs {}, less than the {expected} the \
             full path leaves for it, so the full path is not optimal, {query}",
            remaining.length
        );
    }

    // 9. the triangle inequality: no detour through a third point is ever cheaper.
    if let Some(third) = third {
        if let (Some(first), Some(second)) = (mesh.path(from, third), mesh.path(third, to)) {
            let detour = first.length + second.length;
            assert!(
                path.length <= detour + tolerance(detour),
                "going through {third:?} costs {detour}, less than the direct {}, {query}",
                path.length
            );
        }
    }
}

fn fuzz_mesh(name: &'static str, mesh: &Mesh, count: usize) {
    let seed = seed();
    eprintln!("fuzzing {name} with {count} queries, POLYANYA_FUZZ_SEED={seed}");
    let mut rng = Rng::new(seed);
    let bounds = bounds(mesh);

    let mut ran = 0;
    for _ in 0..count {
        let (Some(from), Some(to)) = (
            random_point(mesh, &mut rng, bounds),
            random_point(mesh, &mut rng, bounds),
        ) else {
            continue;
        };
        let third = random_point(mesh, &mut rng, bounds);
        check_query(
            mesh,
            &Query {
                mesh: name,
                seed,
                from,
                to,
            },
            third,
        );
        ran += 1;
    }

    // Guard against the generators quietly failing and the test passing on nothing.
    assert!(
        ran > count / 2,
        "only {ran} of {count} queries could be generated on {name}"
    );
}

fn mesh_from(path: &str) -> Mesh {
    PolyanyaFile::from_file(path).try_into().unwrap()
}

#[test]
fn fuzz_arena() {
    fuzz_mesh(
        "arena",
        &mesh_from("meshes/v2/arena.mesh"),
        iterations(2000),
    );
}

#[test]
fn fuzz_scene_mp_2p_01() {
    fuzz_mesh(
        "scene_mp_2p_01",
        &mesh_from("meshes/v3/scene_mp_2p_01.mesh"),
        iterations(500),
    );
}

#[test]
fn fuzz_aurora() {
    fuzz_mesh("aurora", aurora_mesh(), iterations(200));
}

/// Aurora is big enough (34707 vertices) that loading it is worth doing once for the
/// whole binary.
fn aurora_mesh() -> &'static Mesh {
    static AURORA: OnceLock<Mesh> = OnceLock::new();
    AURORA.get_or_init(|| mesh_from("meshes/v2/aurora-merged.mesh"))
}

/// Points well outside the mesh. The search snaps a query back to the closest point
/// within `search_delta * search_steps`, so far away points have to come back as "no
/// path" rather than panicking, looping, or answering with a path that starts nowhere.
#[test]
fn fuzz_off_mesh_queries() {
    let seed = seed();
    eprintln!("fuzzing off-mesh queries, POLYANYA_FUZZ_SEED={seed}");
    let mut rng = Rng::new(seed);
    let mesh = aurora_mesh();
    let (min, max) = bounds(mesh);
    let size = max - min;

    for _ in 0..iterations(500) {
        // one full mesh width past an edge, at least
        let outside = Vec2::new(
            rng.range(max.x + size.x, max.x + 10.0 * size.x),
            rng.range(max.y + size.y, max.y + 10.0 * size.y),
        );
        assert!(
            !mesh.point_in_mesh(outside),
            "{outside:?} is somehow in the mesh, POLYANYA_FUZZ_SEED={seed}"
        );

        let inside = point_in_random_polygon(mesh, &mut rng).unwrap();
        assert_eq!(
            mesh.path(outside, inside),
            None,
            "path from outside {outside:?} to {inside:?}, POLYANYA_FUZZ_SEED={seed}"
        );
        assert_eq!(
            mesh.path(inside, outside),
            None,
            "path from {inside:?} to outside {outside:?}, POLYANYA_FUZZ_SEED={seed}"
        );
    }
}
