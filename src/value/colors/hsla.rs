use super::clamp;
use num_rational::Rational;
use num_traits::{one, zero, Signed};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hsla {
    pub hue: Rational,
    pub sat: Rational,
    pub lum: Rational,
    pub alpha: Rational,
}

impl Hsla {
    pub fn new(
        hue: Rational,
        sat: Rational,
        lum: Rational,
        alpha: Rational,
    ) -> Hsla {
        Hsla {
            hue: deg_mod(hue),
            sat,
            lum,
            alpha: clamp(alpha, zero(), one()),
        }
    }
}

/// Value is an angle in degrees, return same angle, but 0 <= value < 360.x
fn deg_mod(value: Rational) -> Rational {
    let turn = Rational::from_integer(360);
    let value = value % turn;
    if value.is_negative() {
        value + turn
    } else {
        value
    }
}
