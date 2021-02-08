use super::*;

impl From<&Hsla> for Rgba {
    fn from(hsla: &Hsla) -> Rgba {
        let hue = hsla.hue() / 360;
        let sat = hsla.sat();
        let lum = hsla.lum();
        if sat.is_zero() {
            let gray = lum * 255;
            Rgba::new(gray, gray, gray, hsla.alpha())
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
                hsla.alpha(),
            )
        }
    }
}

impl From<&Hwba> for Rgba {
    fn from(hwba: &Hwba) -> Rgba {
        (&Hsla::from(hwba)).into() // TODO: Implement a direct conversion!
    }
}

impl From<&Hwba> for Hsla {
    fn from(hwba: &Hwba) -> Hsla {
        let w = hwba.whiteness();
        let b = hwba.blackness();
        let l = (Rational::one() - b + w) / 2;
        let s = if l.is_zero() || l.is_one() {
            zero()
        } else {
            (Rational::one() - b - l) / std::cmp::min(l, Rational::one() - l)
        };
        Hsla::new(hwba.hue(), s, l, hwba.alpha())
    }
}

impl From<&Rgba> for Hsla {
    fn from(rgba: &Rgba) -> Hsla {
        let (red, green, blue) =
            (rgba.red() / 255, rgba.green() / 255, rgba.blue() / 255);
        let (max, min, largest) = max_min_largest(red, green, blue);

        if max == min {
            Hsla::new(zero(), zero(), max, rgba.alpha())
        } else {
            let d = max - min;
            let hue = match largest {
                0 => (green - blue) / d + if green < blue { 6 } else { 0 },
                1 => (blue - red) / d + 2,
                _ => (red - green) / d + 4,
            } * (360 / 6);
            let mm = max + min;
            let sat = d / if mm > one() { -mm + 2 } else { mm };
            Hsla::new(hue, sat, mm / 2, rgba.alpha())
        }
    }
}

impl From<&Rgba> for Hwba {
    fn from(rgba: &Rgba) -> Hwba {
        let hsla = Hsla::from(rgba);
        let arr = [rgba.red(), rgba.blue(), rgba.green()];
        Hwba::new(
            hsla.hue(),
            *arr.iter().min().unwrap() / 255,
            Rational::one() - *arr.iter().max().unwrap() / 255,
            hsla.alpha(),
        )
    }
}
impl From<&Hsla> for Hwba {
    fn from(hsla: &Hsla) -> Hwba {
        let rgba = Rgba::from(hsla);
        let arr = [rgba.red(), rgba.blue(), rgba.green()];
        Hwba::new(
            hsla.hue(),
            Rational::one() - *arr.iter().max().unwrap() / 255,
            *arr.iter().min().unwrap() / 255,
            hsla.alpha(),
        )
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
