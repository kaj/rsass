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
    pub fn from_rgba(rgba: &super::Rgba) -> Self {
        let (red, green, blue) =
            (rgba.red / 255, rgba.green / 255, rgba.blue / 255);
        let (max, min, largest) = max_min_largest(red, green, blue);

        if max == min {
            Hsla {
                hue: zero(),
                sat: zero(),
                lum: max,
                alpha: rgba.alpha,
            }
        } else {
            let d = max - min;
            let hue = match largest {
                0 => (green - blue) / d + if green < blue { 6 } else { 0 },
                1 => (blue - red) / d + 2,
                _ => (red - green) / d + 4,
            } * (360 / 6);
            let mm = max + min;
            let sat = d / if mm > one() { -mm + 2 } else { mm };
            Hsla {
                hue,
                sat,
                lum: mm / 2,
                alpha: rgba.alpha,
            }
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

// Find which of three numbers are largest and smallest
fn max_min_largest(
    a: Rational,
    b: Rational,
    c: Rational,
) -> (Rational, Rational, u32) {
    let v = [(a, 0), (b, 1), (c, 2)];
    let max = v.iter().max().unwrap();
    let min = v.iter().min().unwrap();
    (max.0, min.0, max.1)
}
