use super::{Hsla, Hwba, RgbFormat, Rgba};

impl From<&Hsla> for Rgba {
    fn from(hsla: &Hsla) -> Self {
        let hue = hsla.hue() / 360.;
        let sat = hsla.sat();
        let lum = hsla.lum();
        if sat == 0. {
            let gray = lum * 255.;
            Self::new(gray, gray, gray, hsla.alpha(), RgbFormat::Name)
        } else {
            fn hue2rgb(p: f64, q: f64, t: f64) -> f64 {
                let t = (t - t.floor()) * 6.;
                match t as u8 {
                    0 => p + (q - p) * t,
                    1 | 2 => q,
                    3 => p + (p - q) * (t - 4.),
                    _ => p,
                }
            }
            let q = if lum < 0.5 {
                lum * (sat + 1.)
            } else {
                lum + sat - lum * sat
            };
            let p = lum * 2. - q;
            const THIRD: f64 = 1. / 3.;
            Self::new(
                hue2rgb(p, q, hue + THIRD) * 255.,
                hue2rgb(p, q, hue) * 255.,
                hue2rgb(p, q, hue - THIRD) * 255.,
                hsla.alpha(),
                RgbFormat::Name,
            )
        }
    }
}

impl From<&Hwba> for Rgba {
    fn from(hwba: &Hwba) -> Self {
        (&Hsla::from(hwba)).into() // TODO: Implement a direct conversion!
    }
}

impl From<&Hwba> for Hsla {
    fn from(hwba: &Hwba) -> Self {
        let w = hwba.whiteness();
        let b = hwba.blackness();
        let l = (1. - b + w) / 2.;
        let s = if l == 0. || l == 1. {
            0.
        } else {
            (1. - b - l) / f64::min(l, 1. - l)
        };
        let (hue, lum) = if w.is_finite() && b.is_finite() {
            (hwba.hue(), l)
        } else {
            (f64::NAN, f64::NAN)
        };
        Self::new(hue, s, lum, hwba.alpha(), false)
    }
}

impl From<&Rgba> for Hsla {
    fn from(rgba: &Rgba) -> Self {
        let (red, green, blue) =
            (rgba.red() / 255., rgba.green() / 255., rgba.blue() / 255.);
        let (max, min, largest) = max_min_largest(red, green, blue);

        if max == min {
            Self::new(0., 0., max, rgba.alpha(), false)
        } else {
            let d = max - min;
            let hue = match largest {
                0 => (green - blue) / d + if green < blue { 6. } else { 0. },
                1 => (blue - red) / d + 2.,
                _ => (red - green) / d + 4.,
            } * (360. / 6.);
            let mm = max + min;
            let sat = d / if mm > 1. { -mm + 2. } else { mm };
            Self::new(hue, sat, mm / 2., rgba.alpha(), false)
        }
    }
}

impl From<&Rgba> for Hwba {
    fn from(rgba: &Rgba) -> Self {
        let hsla = Hsla::from(rgba);
        Self::new(
            hsla.hue(),
            rgba.red().min(rgba.blue()).min(rgba.green()) / 255.,
            1. - rgba.red().max(rgba.blue()).max(rgba.green()) / 255.,
            hsla.alpha(),
        )
    }
}
impl From<&Hsla> for Hwba {
    fn from(hsla: &Hsla) -> Self {
        let rgba = Rgba::from(hsla);
        Self::new(
            hsla.hue(),
            rgba.red().min(rgba.blue()).min(rgba.green()) / 255.,
            1. - rgba.red().max(rgba.blue()).max(rgba.green()) / 255.,
            hsla.alpha(),
        )
    }
}

// Find which of three numbers are largest and smallest
fn max_min_largest(a: f64, b: f64, c: f64) -> (f64, f64, u32) {
    let (max, largest) = if a > b && a > c {
        (a, 0)
    } else if b > a && b > c {
        (b, 1)
    } else {
        (c, 2)
    };
    let min = a.min(b).min(c);
    (max, min, largest)
}
