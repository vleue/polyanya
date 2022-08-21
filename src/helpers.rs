use glam::Vec2;

#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::EdgeSide;

const EPSILON: f32 = 1e-2;

pub(crate) trait Vec2Helper {
    fn side(self, edge: (Vec2, Vec2)) -> EdgeSide;
    fn mirror(self, edge: (Vec2, Vec2)) -> Vec2;
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
}

#[cfg_attr(feature = "tracing", instrument(skip_all))]
#[inline(always)]
pub(crate) fn on_segment(point: Vec2, segment: (Vec2, Vec2)) -> bool {
    (segment.0.x.min(segment.1.x)..=segment.0.x.max(segment.1.x)).contains(&point.x)
        && (segment.0.y.min(segment.1.y)..=segment.0.y.max(segment.1.y)).contains(&point.y)
        && (point.side(segment) == EdgeSide::Edge)
}

// i should be counterclockwise from r
#[cfg_attr(feature = "tracing", instrument(skip_all))]
#[inline(always)]
pub(crate) fn heuristic(root: Vec2, goal: Vec2, interval: (Vec2, Vec2)) -> f32 {
    let goal = if root.side(interval) == goal.side(interval) {
        goal.mirror(interval)
    } else {
        goal
    };

    if root == interval.0 || root == interval.1 {
        root.distance(goal)
    } else {
        let local_root = root - interval.0;
        let local_goal = goal - interval.0;
        let goal_distance = goal - root;
        let local_interval_end = interval.1 - interval.0;
        // TODO sort out readability in heuristic function
        let lr_num = local_goal.perp_dot(local_root);
        let denom = goal_distance.perp_dot(local_interval_end);
        match lr_num / denom {
            x if x < 0.0 => root.distance(interval.0) + interval.0.distance(goal),
            x if x > 1.0 => root.distance(interval.1) + interval.1.distance(goal),
            _ => root.distance(goal),
        }
    }
}

#[cfg_attr(feature = "tracing", instrument(skip_all))]
pub(crate) fn turning_on(root: Vec2, goal: Vec2, interval: (Vec2, Vec2)) -> Option<Vec2> {
    let goal = if root.side(interval) == goal.side(interval) {
        goal.mirror(interval)
    } else {
        goal
    };
    if root == interval.0 {
        None
    } else if goal.side((root, interval.0)) == EdgeSide::Right {
        Some(interval.0)
    } else if goal.side((root, interval.1)) == EdgeSide::Left {
        Some(interval.1)
    } else {
        None
    }
}

#[cfg_attr(feature = "tracing", instrument(skip_all))]
#[inline(always)]
pub(crate) fn line_intersect_segment(line: (Vec2, Vec2), segment: (Vec2, Vec2)) -> Option<Vec2> {
    // What's happening here is that we're effectively finding a "partial" area (wedge product) defined by supposed
    // intersection (line end to segment end x full line), and then divide it by "total" area (full line x full segment).
    // Apparently this is equal to the so called intersection time, which is the ratio of the supposed segment
    // "part" defined by intersection to the total segment length. If the "part" is biggger than 1, then the line
    // does not intersect the segment but intersects the line on which the segment lies.
    let intersection_time = (line.0 - segment.0).perp_dot(line.0 - line.1)
        / (line.0 - line.1).perp_dot(segment.0 - segment.1);

    if !(0.0..=1.0).contains(&intersection_time) || intersection_time.is_nan() {
        None
    } else {
        Some(segment.0 + intersection_time * (segment.1 - segment.0))
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::{helpers::Vec2Helper, EdgeSide};

    use super::{heuristic, line_intersect_segment};

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
}
