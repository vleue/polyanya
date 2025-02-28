use num_traits::{Num, Signed};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A coordinate type that can be used with a triangulation.
///
/// Internally, most calculations are performed after converting the type into a `f64`.
/// However, changing this to `f32` will reduce the required storage space slightly.
///
/// This type should usually be either `f32` or `f64`.
pub trait SpadeNum:
    Num + PartialOrd + Into<f64> + From<f32> + Copy + Signed + core::fmt::Debug
{
}

impl<T> SpadeNum for T where
    T: Num + PartialOrd + Into<f64> + From<f32> + Copy + Signed + core::fmt::Debug
{
}

/// A two-dimensional point.
///
/// This is the basic type used for defining positions.
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Default, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde")
)]
pub struct Point2<S> {
    /// The point's x coordinate
    pub x: S,
    /// The point's y coordinate
    pub y: S,
}

impl<S> Point2<S> {
    /// Creates a new point.
    #[inline]
    pub const fn new(x: S, y: S) -> Self {
        Point2 { x, y }
    }
}

impl<S: SpadeNum> Point2<S> {
    /// Returns the squared distance of this point and another point.
    #[inline]
    pub fn distance_2(&self, other: Self) -> S {
        self.sub(other).length2()
    }

    pub(crate) fn to_f64(self) -> Point2<f64> {
        Point2::new(self.x.into(), self.y.into())
    }

    pub(crate) fn mul(&self, factor: S) -> Self {
        Point2 {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    pub(crate) fn add(&self, other: Self) -> Self {
        Point2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub(crate) fn length2(&self) -> S {
        self.x * self.x + self.y * self.y
    }

    pub(crate) fn sub(&self, other: Self) -> Self {
        Point2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub(crate) fn dot(&self, other: Self) -> S {
        self.x * other.x + self.y * other.y
    }

    pub(crate) fn all_component_wise(&self, other: Self, f: impl Fn(S, S) -> bool) -> bool {
        f(self.x, other.x) && f(self.y, other.y)
    }
}

impl<S: SpadeNum> From<Point2<S>> for [S; 2] {
    #[inline]
    fn from(point: Point2<S>) -> Self {
        [point.x, point.y]
    }
}

impl<S: SpadeNum> From<Point2<S>> for (S, S) {
    #[inline]
    fn from(point: Point2<S>) -> (S, S) {
        (point.x, point.y)
    }
}

impl<S: SpadeNum> From<[S; 2]> for Point2<S> {
    #[inline]
    fn from(source: [S; 2]) -> Self {
        Self::new(source[0], source[1])
    }
}

impl<S: SpadeNum> From<(S, S)> for Point2<S> {
    #[inline]
    fn from(source: (S, S)) -> Self {
        Self::new(source.0, source.1)
    }
}

/// An object with a position.
///
/// Vertices need to implement this trait to allow being inserted into triangulations.
pub trait HasPosition {
    /// The number type used by this coordinate type.
    type Scalar: SpadeNum;

    /// Returns the position of this object.
    ///
    /// **Note**: It is assumed that the position doesn't change once it has been
    /// inserted into a triangulation. Failing this requirement can lead to crashes,
    /// invalid results or endless loops.
    fn position(&self) -> Point2<Self::Scalar>;
}

impl<S: SpadeNum> HasPosition for Point2<S> {
    type Scalar = S;

    fn position(&self) -> Point2<S> {
        *self
    }
}
