use super::*;

impl From<&Hsla> for Rgba {
    fn from(hsla: &Hsla) -> Rgba {
        let hue = hsla.hue / 360;
        let sat = clamp(hsla.sat, zero(), one());
        let lum = clamp(hsla.lum, zero(), one());
        if sat.is_zero() {
            let gray = lum * 255;
            Rgba::new(gray, gray, gray, hsla.alpha)
        } else {
            fn hue2rgb(p: Rational, q: Rational, t: Rational) -> Rational {
                let t = (t - t.floor()) * 6;
                match t.to_integer() {
                    0 => p + (q - p) * t,
                    1 | 2 => q,
                    3 => p + (p - q) * (t - 4),
                    _ => p,
                }
            }
            let q = if lum < Rational::new(1, 2) {
                lum * (sat + 1)
            } else {
                lum + sat - lum * sat
            };
            let p = lum * 2 - q;

            Rgba::new(
                hue2rgb(p, q, hue + Rational::new(1, 3)) * 255,
                hue2rgb(p, q, hue) * 255,
                hue2rgb(p, q, hue - Rational::new(1, 3)) * 255,
                hsla.alpha,
            )
        }
    }
}

impl From<&Rgba> for Hsla {
    fn from(rgba: &Rgba) -> Hsla {
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
