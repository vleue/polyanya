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

    pub fn is_on_left_side(&self) -> bool {
        self.signed_side > 0.0
    }

    pub fn is_on_right_side(&self) -> bool {
        self.signed_side < 0.0
    }

    pub fn is_on_left_side_or_on_line(&self) -> bool {
        self.signed_side >= 0.0
    }

    pub fn is_on_right_side_or_on_line(self) -> bool {
        self.signed_side <= 0.0
    }

    #[inline]
    pub fn is_on_line(self) -> bool {
        self.signed_side.abs() == 0.0
    }

    pub fn reversed(self) -> LineSideInfo {
        LineSideInfo {
            signed_side: -self.signed_side,
        }
    }
}
