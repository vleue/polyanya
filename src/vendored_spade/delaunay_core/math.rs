use crate::vendored_spade::{HasPosition, LineSideInfo, Point2, SpadeNum};
use num_traits::{zero, Float};

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Hash)]
pub struct PointProjection<S> {
    factor: S,
    length_2: S,
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Hash)]
pub enum InsertionError {
    TooSmall,

    TooLarge,

    NAN,
}

impl core::fmt::Display for InsertionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Debug>::fmt(self, f)
    }
}

impl std::error::Error for InsertionError {}

// Implementation note: These numbers come from the paper of Jonathan Richard Shewchuk:
// "The four predicates implemented for this report will not overflow nor underflow if
// their inputs have exponents in the range -[142, 201] and IEEE-745 double precision
// arithmetic is used."
// Source: Adaptive Precision Floating-Point Arithmetic and Fast Robust Geometric Predicates
//
// This suggests that the limit as is needlessly tight as spade only requires two of
// the four implemented predicates. There is unfortunately no motivation given for these
// limits, hence it is not obvious how those would need to be derived.
pub const MIN_ALLOWED_VALUE: f64 = 1.793662034335766e-43; // 1.0 * 2^-142

pub const MAX_ALLOWED_VALUE: f64 = 3.2138760885179806e60; // 1.0 * 2^201

pub fn validate_coordinate<S: SpadeNum>(value: S) -> Result<(), InsertionError> {
    let as_f64: f64 = value.into();
    if as_f64.is_nan() {
        Err(InsertionError::NAN)
    } else if as_f64.abs() < MIN_ALLOWED_VALUE && as_f64 != 0.0 {
        Err(InsertionError::TooSmall)
    } else if as_f64.abs() > MAX_ALLOWED_VALUE {
        Err(InsertionError::TooLarge)
    } else {
        Ok(())
    }
}

pub fn validate_vertex<V: HasPosition>(vertex: &V) -> Result<(), InsertionError> {
    let position = vertex.position();
    validate_coordinate(position.x)?;
    validate_coordinate(position.y)?;
    Ok(())
}

pub fn mitigate_underflow(position: Point2<f64>) -> Point2<f64> {
    Point2::new(
        mitigate_underflow_for_coordinate(position.x),
        mitigate_underflow_for_coordinate(position.y),
    )
}

fn mitigate_underflow_for_coordinate<S: SpadeNum>(coordinate: S) -> S {
    if coordinate != S::zero() && coordinate.abs().into() < MIN_ALLOWED_VALUE {
        S::zero()
    } else {
        coordinate
    }
}

impl<S: SpadeNum> PointProjection<S> {
    fn new(factor: S, length_2: S) -> Self {
        Self { factor, length_2 }
    }

    pub fn is_before_edge(&self) -> bool {
        self.factor < S::zero()
    }

    pub fn is_behind_edge(&self) -> bool {
        self.factor > self.length_2
    }

    pub fn is_on_edge(&self) -> bool {
        !self.is_before_edge() && !self.is_behind_edge()
    }

    pub fn reversed(&self) -> Self {
        Self {
            factor: self.length_2 - self.factor,
            length_2: self.length_2,
        }
    }
}

impl<S: SpadeNum + Float> PointProjection<S> {
    pub fn relative_position(&self) -> S {
        if self.length_2 >= zero() {
            self.factor / self.length_2
        } else {
            let l = -self.length_2;
            let f = -self.factor;
            (l - f) / l
        }
    }
}

pub fn nearest_point<S>(p1: Point2<S>, p2: Point2<S>, query_point: Point2<S>) -> Point2<S>
where
    S: SpadeNum + Float,
{
    let dir = p2.sub(p1);
    let s = project_point(p1, p2, query_point);
    if s.is_on_edge() {
        let relative_position = s.relative_position();
        p1.add(dir.mul(relative_position))
    } else if s.is_before_edge() {
        p1
    } else {
        p2
    }
}

pub fn project_point<S>(p1: Point2<S>, p2: Point2<S>, query_point: Point2<S>) -> PointProjection<S>
where
    S: SpadeNum,
{
    let dir = p2.sub(p1);
    PointProjection::new(query_point.sub(p1).dot(dir), dir.length2())
}

pub fn distance_2<S>(p1: Point2<S>, p2: Point2<S>, query_point: Point2<S>) -> S
where
    S: SpadeNum + Float,
{
    let nn = nearest_point(p1, p2, query_point);
    query_point.sub(nn).length2()
}

fn to_robust_coord<S: SpadeNum>(point: Point2<S>) -> robust::Coord<S> {
    robust::Coord {
        x: point.x,
        y: point.y,
    }
}

pub fn contained_in_circumference<S>(
    v1: Point2<S>,
    v2: Point2<S>,
    v3: Point2<S>,
    p: Point2<S>,
) -> bool
where
    S: SpadeNum,
{
    let v1 = to_robust_coord(v1);
    let v2 = to_robust_coord(v2);
    let v3 = to_robust_coord(v3);
    let p = to_robust_coord(p);

    // incircle expects all vertices to be ordered CW for right-handed systems.
    // For consistency, the public interface of this method will expect the points to be
    // ordered ccw.
    robust::incircle(v3, v2, v1, p) < 0.0
}

pub fn is_ordered_ccw<S>(p1: Point2<S>, p2: Point2<S>, query_point: Point2<S>) -> bool
where
    S: SpadeNum,
{
    let query = side_query(p1, p2, query_point);
    query.is_on_left_side_or_on_line()
}

pub fn side_query<S>(p1: Point2<S>, p2: Point2<S>, query_point: Point2<S>) -> LineSideInfo
where
    S: SpadeNum,
{
    let p1 = to_robust_coord(p1);
    let p2 = to_robust_coord(p2);
    let query_point = to_robust_coord(query_point);

    let result = robust::orient2d(p1, p2, query_point);
    LineSideInfo::from_determinant(result)
}

fn side_query_inaccurate<S>(from: Point2<S>, to: Point2<S>, query_point: Point2<S>) -> LineSideInfo
where
    S: SpadeNum,
{
    let q = query_point;
    let determinant = (to.x - from.x) * (q.y - from.y) - (to.y - from.y) * (q.x - from.x);
    LineSideInfo::from_determinant(determinant.into())
}

pub(crate) fn intersects_edge_non_collinear<S>(
    from0: Point2<S>,
    to0: Point2<S>,
    from1: Point2<S>,
    to1: Point2<S>,
) -> bool
where
    S: SpadeNum,
{
    let other_from = side_query(from0, to0, from1);
    let other_to = side_query(from0, to0, to1);
    let self_from = side_query(from1, to1, from0);
    let self_to = side_query(from1, to1, to0);

    assert!(
        ![&other_from, &other_to, &self_from, &self_to]
            .iter()
            .all(|q| q.is_on_line()),
        "intersects_edge_non_collinear: Given edge is collinear."
    );

    other_from != other_to && self_from != self_to
}

pub fn distance_2_triangle<S>(vertices: [Point2<S>; 3], query_point: Point2<S>) -> S
where
    S: SpadeNum + Float,
{
    for i in 0..3 {
        let from = vertices[i];
        let to = vertices[(i + 1) % 3];
        if side_query_inaccurate(from, to, query_point).is_on_right_side() {
            return distance_2(from, to, query_point);
        }
    }
    // The point lies within the triangle
    S::zero()
}

pub fn circumcenter<S>(positions: [Point2<S>; 3]) -> (Point2<S>, S)
where
    S: SpadeNum + Float,
{
    let [v0, v1, v2] = positions;
    let b = v1.sub(v0);
    let c = v2.sub(v0);

    let one = S::one();
    let two = one + one;
    let d = two * (b.x * c.y - c.x * b.y);
    let len_b = b.dot(b);
    let len_c = c.dot(c);
    let d_inv: S = one / d;

    let x = (len_b * c.y - len_c * b.y) * d_inv;
    let y = (-len_b * c.x + len_c * b.x) * d_inv;
    let result = Point2::new(x, y);
    (result.add(v0), x * x + y * y)
}

pub fn triangle_area<S>(positions: [Point2<S>; 3]) -> S
where
    S: SpadeNum,
{
    let [v0, v1, v2] = positions;
    let b = v1.sub(v0);
    let c = v2.sub(v0);
    (b.x * c.y - b.y * c.x).abs() * 0.5.into()
}
