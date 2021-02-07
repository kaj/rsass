use super::clamp;
use num_rational::Rational;
use num_traits::{one, zero};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hwba {
    pub hue: Rational,
    pub w: Rational,
    pub b: Rational,
    pub alpha: Rational,
}

impl Hwba {
    pub fn new(
        hue: Rational,
        w: Rational,
        b: Rational,
        alpha: Rational,
    ) -> Hwba {
        Hwba {
            hue,
            w,
            b,
            alpha: clamp(alpha, zero(), one()),
        }
    }
}
