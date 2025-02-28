/// Describes on which side of a line a point lies.
///
/// Created by [DirectedEdgeHandle::side_query](crate::handles::DirectedEdgeHandle::side_query)
#[derive(Debug, Clone, Copy)]
pub struct LineSideInfo {
    signed_side: f64,
}

impl PartialEq for LineSideInfo {
    fn eq(&self, other: &LineSideInfo) -> bool {
        if self.is_on_line() || other.is_on_line() {
            self.is_on_line() && other.is_on_line()
        } else {
            self.is_on_right_side() == other.is_on_right_side()
        }
    }
}

impl LineSideInfo {
    #[inline]
    pub(crate) fn from_determinant(s: f64) -> LineSideInfo {
        LineSideInfo { signed_side: s }
    }

    /// Returns `true` if a point lies on the left side of a line.
    ///
    /// For left-handed coordinate systems, this method returns if a point lies on the right side of a line.
    /// This method returns `false` if the point lies exactly on the line.
    pub fn is_on_left_side(&self) -> bool {
        self.signed_side > 0.0
    }

    /// Returns `true` if a point lies on the right side of a line.
    ///
    /// For left-handed coordinate systems, this method returns if a point lies on the left side of a line.
    /// This method returns `false` if the point lies exactly on the line.
    pub fn is_on_right_side(&self) -> bool {
        self.signed_side < 0.0
    }

    /// Returns `true` if a point lies on the left side of a line or is on the line itself.
    ///
    /// For left-handed coordinate systems, this method returns if a point lies on the left side of a line.
    pub fn is_on_left_side_or_on_line(&self) -> bool {
        self.signed_side >= 0.0
    }

    /// Returns `true` if a point lies on the right side of a line or is on the line itself.
    ///
    /// For left handed coordinate systems, this method returns if a point lies on the left side of a line.
    pub fn is_on_right_side_or_on_line(self) -> bool {
        self.signed_side <= 0.0
    }

    /// Returns `true` if a point lies exactly on this line.
    #[inline]
    pub fn is_on_line(self) -> bool {
        self.signed_side.abs() == 0.0
    }

    /// Returns the opposite of this `LineSideInfo`.
    pub fn reversed(self) -> LineSideInfo {
        LineSideInfo {
            signed_side: -self.signed_side,
        }
    }
}
