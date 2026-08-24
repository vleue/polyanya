use glam::Vec2;

#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::instance::EdgeSide;

pub(crate) const EPSILON: f32 = 1e-4;

pub(crate) trait Vec2Helper {
    fn side(self, edge: (Vec2, Vec2)) -> EdgeSide;
    fn mirror(self, edge: (Vec2, Vec2)) -> Vec2;
    fn on_segment(self, segment: (Vec2, Vec2)) -> bool;
    fn in_bounding_box(self, segment: (Vec2, Vec2)) -> bool;
}

impl Vec2Helper for Vec2 {
    /// Determines where relative to the given line the point is.
    /// Returns [EdgeSide].
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    fn side(self, line: (Vec2, Vec2)) -> EdgeSide {
        let local_point = self - line.0;
        let local_line = line.1 - line.0;

        match local_line.perp_dot(local_point) {
            x if x.abs() < EPSILON => EdgeSide::Edge,
            x if x > 0.0 => EdgeSide::Left,
            _ => EdgeSide::Right,
        }
    }

    /// Mirrors a point across a line defined by two points.
    /// Returns the reflected point.
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    fn mirror(self, line: (Vec2, Vec2)) -> Vec2 {
        let local_point = self - line.0;
        let local_line = line.1 - line.0;

        let local_reflect_point = 2.0 * local_point.project_onto(local_line) - local_point;

        line.0 + local_reflect_point
    }

    /// Determines if a point lies on a line segment.
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    fn on_segment(self, segment: (Vec2, Vec2)) -> bool {
        // Check if the point is in the segment's bounding box and then check if it is on the line.
        // Just checking if the point is on the line is not sufficient because the point can be
        // outside the segment but still be on the line.
        self.in_bounding_box(segment) && (self.side(segment) == EdgeSide::Edge)
    }

    /// Determines if a point is in a bounding box.
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[inline(always)]
    fn in_bounding_box(self, segment: (Vec2, Vec2)) -> bool {
        const EPSILON: f32 = 1.0e-6;
        // Check if the point is in the segment's bounding box and then check if it is on the line.
        // Just checking if the point is on the line is not sufficient because the point can be
        // outside the segment but still be on the line.
        ((min(segment.0.x, segment.1.x) - EPSILON)..=(max(segment.0.x, segment.1.x) + EPSILON))
            .contains(&self.x)
            && ((min(segment.0.y, segment.1.y) - EPSILON)
                ..=(max(segment.0.y, segment.1.y) + EPSILON))
                .contains(&self.y)
    }
}

/// Fast floating point minimum.  This function matches the semantics of
///
/// ```no_compile
/// if x < y { x } else { y }
/// ```
///
/// which has efficient instruction sequences on many platforms (1 instruction on x86).  For most
/// values, it matches the semantics of `x.min(y)`; the special cases are:
///
/// ```text
/// min(-0.0, +0.0); +0.0
/// min(+0.0, -0.0): -0.0
/// min( NaN,  1.0):  1.0
/// min( 1.0,  NaN):  NaN
/// ```
#[inline(always)]
pub(crate) fn min(x: f32, y: f32) -> f32 {
    if x < y {
        x
    } else {
        y
    }
}

/// Fast floating point maximum.  This function matches the semantics of
///
/// ```no_compile
/// if x > y { x } else { y }
/// ```
///
/// which has efficient instruction sequences on many platforms (1 instruction on x86).  For most
/// values, it matches the semantics of `x.max(y)`; the special cases are:
///
/// ```text
/// max(-0.0, +0.0); +0.0
/// max(+0.0, -0.0): -0.0
/// max( NaN,  1.0):  1.0
/// max( 1.0,  NaN):  NaN
/// ```
#[inline(always)]
pub(crate) fn max(x: f32, y: f32) -> f32 {
    if x > y {
        x
    } else {
        y
    }
}

/// Is the root sitting on the interval itself?
///
/// The root normally sees the interval through a wedge bounded by the rays to its two ends,
/// and where the path bends is decided by which side of those rays the goal falls on. Put
/// the root on the interval and that wedge opens out into a half plane: the two rays point
/// opposite ways, so the goal lands outside both of them at once and the side tests stop
/// answering the question they are asked. Nothing blocks such a root, which is already on
/// the edge, so the path leaves it straight. The root on one of the ends is the common way
/// this happens, but landing anywhere along the interval does the same thing.
#[inline(always)]
fn root_on_interval(root: Vec2, interval: (Vec2, Vec2)) -> bool {
    root.on_segment(interval)
}

/// Computes heuristic distance from a [`super::SearchNode`] represented by the given root and interval to the goal.
#[cfg_attr(feature = "tracing", instrument(skip_all))]
#[inline(always)]
pub(crate) fn heuristic(root: Vec2, goal: Vec2, interval: (Vec2, Vec2)) -> f32 {
    // This measures the very path [`turning_point`] describes, so it asks that function
    // where the path bends rather than working it out a second way of its own. The two used
    // to decide it separately and could disagree, and then the length reported for a path
    // was not the length of the path reported with it.
    match turning_point(root, goal, interval) {
        // The bend is on the interval, and mirroring is a reflection in that same line, so
        // the distance from the bend to the goal is the same whichever goal is used.
        Some(turn) => root.distance(turn) + turn.distance(goal),
        // Straight through, measured against the mirrored goal when the goal is on the
        // root's side of the interval, which is what makes this a taut path length.
        None => {
            let goal = if root.side(interval) == goal.side(interval) {
                goal.mirror(interval)
            } else {
                goal
            };
            root.distance(goal)
        }
    }
}

/// Returns the point at which the path between the given root and goal should turn, if any.
#[cfg_attr(feature = "tracing", instrument(skip_all))]
#[inline(always)]
pub(crate) fn turning_point(root: Vec2, goal: Vec2, interval: (Vec2, Vec2)) -> Option<Vec2> {
    // If the goal is on the same side of the interval with the root, then we mirror it.
    let goal = if root.side(interval) == goal.side(interval) {
        goal.mirror(interval)
    } else {
        goal
    };

    if root_on_interval(root, interval) {
        None
    } else if goal.side((root, interval.0)) == EdgeSide::Right {
        Some(interval.0)
    } else if goal.side((root, interval.1)) == EdgeSide::Left {
        Some(interval.1)
    } else {
        None
    }
}

/// Returns intersection time (which is the ratio of the supposed
/// intersection-defined segment "part" to the total segment length)
/// at which the given segment will be intersected by the given line.
#[cfg_attr(feature = "tracing", instrument(skip_all))]
#[inline(always)]
pub(crate) fn intersection_time(line: (Vec2, Vec2), segment: (Vec2, Vec2)) -> f32 {
    // What's happening here is that we're effectively finding a "partial" area (wedge product) defined by supposed
    // intersection (line end to segment end x full line), and then divide it by "total" area (full line x full segment).
    // Apparently this is equal to the so called intersection time, which is the ratio of the supposed segment
    // "part" defined by intersection to the total segment length. If the "part" is biggger than 1, then the line
    // does not intersect the segment but intersects the line on which the segment lies.
    let local_line = line.0 - line.1;

    (line.0 - segment.0).perp_dot(local_line) / (local_line).perp_dot(segment.0 - segment.1)
}

/// Returns the intersection point of the given line and segment, if it exists.
#[cfg_attr(feature = "tracing", instrument(skip_all))]
#[inline(always)]
pub(crate) fn line_intersect_segment(line: (Vec2, Vec2), segment: (Vec2, Vec2)) -> Option<Vec2> {
    let intersection_time = intersection_time(line, segment);

    if !(-EPSILON..=(1.0 + EPSILON)).contains(&intersection_time) || intersection_time.is_nan() {
        None
    } else {
        // The bounds above are deliberately a little wider than the segment, so that a line
        // passing within a hair of an end still counts as crossing it. The point handed back
        // has to be on the segment all the same: callers use it as an interval bound, and a
        // bound sitting just past the end makes the interval run backwards. A backwards
        // interval reads as a wide cone instead of an empty one, which lets the search see
        // through walls.
        Some(segment.0 + intersection_time.clamp(0.0, 1.0) * (segment.1 - segment.0))
    }
}

#[cfg(test)]
mod tests {
    use glam::{vec2, Vec2};

    use crate::{helpers::Vec2Helper, instance::EdgeSide};

    use super::{heuristic, line_intersect_segment, turning_point};

    #[test]
    fn test_on_side() {
        assert_eq!(
            Vec2Helper::side(
                Vec2::new(0.0, 0.5),
                (Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0))
            ),
            EdgeSide::Left
        );
        assert_eq!(
            Vec2Helper::side(
                Vec2::new(0.0, -0.5),
                (Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0))
            ),
            EdgeSide::Right
        );
        assert_eq!(
            Vec2Helper::side(
                Vec2::new(1.0, 0.0),
                (Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0))
            ),
            EdgeSide::Right
        );
        assert_eq!(
            Vec2Helper::side(
                Vec2::new(0.0, 1.0),
                (Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0))
            ),
            EdgeSide::Left
        );
        assert_eq!(
            Vec2Helper::side(
                Vec2::new(2.0, 2.0),
                (Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0))
            ),
            EdgeSide::Edge
        );

        // Test epsilon value is large enough
        assert_eq!(
            Vec2Helper::side(
                Vec2::new(5.5852313, 5.388011),
                (Vec2::new(9.56, 7.42), Vec2::new(1.54, 3.32))
            ),
            EdgeSide::Edge
        );
        // Test epsilon value is small enough
        assert_ne!(
            Vec2Helper::side(
                Vec2::new(1.8266357, 1.2239377),
                (Vec2::new(1.775, 1.275), Vec2::new(1.775, 1.175))
            ),
            EdgeSide::Edge
        );
    }

    #[test]
    fn test_mirror() {
        assert_eq!(
            Vec2Helper::mirror(
                Vec2::new(1.0, 0.0),
                (Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0))
            ),
            Vec2::new(-1.0, 0.0)
        );
        assert_eq!(
            Vec2Helper::mirror(
                Vec2::new(-1.0, 0.0),
                (Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0))
            ),
            Vec2::new(1.0, 0.0)
        );
    }

    #[test]
    fn test_heuristic() {
        assert_eq!(
            heuristic(
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 1.0),
                (Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0))
            ),
            2.0_f32.sqrt()
        );
        assert_eq!(
            heuristic(
                Vec2::new(0.0, 0.0),
                Vec2::new(2.0, -1.0),
                (Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0))
            ),
            1.0 + 2.0_f32.sqrt()
        );
        assert_eq!(
            heuristic(
                Vec2::new(0.0, 0.0),
                Vec2::new(-1.0, 2.0),
                (Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0))
            ),
            1.0 + 2.0_f32.sqrt()
        );
        assert_eq!(
            heuristic(
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, -1.0),
                (Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0))
            ),
            2.0
        );
    }

    #[test]
    fn test_line_intersect() {
        assert_eq!(
            line_intersect_segment(
                (Vec2::new(0.0, 0.5), Vec2::new(0.5, 0.5)),
                (Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0))
            ),
            Some(Vec2::new(1.0, 0.5))
        );
        assert_eq!(
            line_intersect_segment(
                (Vec2::new(0.0, 0.0), Vec2::new(0.5, 0.8)),
                (Vec2::new(1.0, 0.0), Vec2::new(1.0, 0.2))
            ),
            None
        );
        assert_eq!(
            line_intersect_segment(
                (Vec2::new(0.0, 0.8), Vec2::new(0.5, 0.0)),
                (Vec2::new(1.0, 0.0), Vec2::new(1.0, 0.2))
            ),
            None
        );
    }

    #[test]
    fn test_line_intersect_colinear() {
        assert_eq!(
            line_intersect_segment(
                (Vec2::new(0.0, 0.5), Vec2::new(0.5, 0.5)),
                (Vec2::new(-1.0, 0.5), Vec2::new(1.0, 0.5))
            ),
            None
        );
    }

    #[test]
    fn test_on_segment() {
        let segment = (vec2(0.0, 0.0), vec2(-9.0, 0.0));
        let p = vec2(-0.30399954, 5.9604645e-8);

        assert!(p.on_segment(segment));
    }

    /// The bounds are wider than the segment so a line passing within a hair of an end
    /// still counts as crossing it, but the point that comes back has to be on the
    /// segment. Callers use it as an interval bound, and one just past the end makes the
    /// interval run backwards, which reads as a wide cone instead of an empty one and lets
    /// the search cut corners through walls.
    #[test]
    fn line_intersect_segment_lands_on_the_segment() {
        // taken from a search on `meshes/v2/aurora-merged.mesh`: the line crosses the
        // segment's own line at t = 1.0000586, a hair past the end
        let segment = (vec2(564.0, 192.0), vec2(557.0, 168.0));
        let line = (vec2(606.0, 144.0), vec2(558.12524, 167.44727));

        let intersection = line_intersect_segment(line, segment).unwrap();
        assert!(
            intersection.on_segment(segment),
            "{intersection:?} is not on {segment:?}"
        );
    }

    /// [`heuristic`] measures the path [`turning_point`] describes, so the two have to
    /// agree about where that path bends. Both degenerate shapes below used to break one
    /// of them: the root sitting on the interval leaves the side tests with a half plane
    /// to work with instead of a wedge, and a root, interval and goal in a line leave
    /// `intersection_time` dividing by nearly nothing.
    #[test]
    fn heuristic_measures_the_turning_point_path() {
        let cases = [
            // root on the interval, from `meshes/v2/arena.mesh`
            (
                vec2(18.5, 25.0),
                vec2(15.0, 47.0),
                (vec2(18.0, 19.0), vec2(19.0, 31.0)),
            ),
            // root on one end of the interval, from `meshes/v2/arena.mesh`
            (
                vec2(35.0, 15.0),
                vec2(30.626049, 19.191982),
                (vec2(34.0, 3.0), vec2(35.0, 15.0)),
            ),
            // root, interval and goal nearly in a line, from `meshes/v3/scene_mp_2p_01.mesh`
            (
                vec2(-3.98, -66.47),
                vec2(-3.885617, -72.63431),
                (vec2(-3.891216, -72.26863), vec2(-3.880018, -72.99999)),
            ),
        ];

        for (root, goal, interval) in cases {
            let walked = match turning_point(root, goal, interval) {
                Some(turn) => root.distance(turn) + turn.distance(goal),
                None => root.distance(goal),
            };
            let estimated = heuristic(root, goal, interval);
            assert!(
                (walked - estimated).abs() < 1e-3,
                "root {root:?} interval {interval:?} goal {goal:?}: \
                 turning point says the path is {walked} long, heuristic says {estimated}"
            );
        }
    }
}
