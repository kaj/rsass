use super::{get_color, make_call, Error, FunctionMap};
use crate::css::Value;
use crate::value::{Hsla, Hwba, Rgba, Unit};
use crate::Scope;
use num_rational::Rational;
use num_traits::{One, Signed, Zero};

pub fn register(f: &mut Scope) {
    def!(
        f,
        adjust(
            color, red, green, blue, hue, saturation, lightness, whiteness,
            blackness, alpha
        ),
        |s| {
            let rgba = get_color(s, "color")?;
            let c_add = |orig: Rational, name: &str| match s.get(name)? {
                Value::Null => Ok(orig),
                x => to_rational(x).map(|x| orig + x),
            };
            let h_adj = s.get("hue")?;
            let s_adj = s.get("saturation")?;
            let l_adj = s.get("lightness")?;
            let b_adj = s.get("blackness")?;
            let w_adj = s.get("whiteness")?;
            if h_adj.is_null()
                && s_adj.is_null()
                && l_adj.is_null()
                && b_adj.is_null()
                && w_adj.is_null()
            {
                let rgba = rgba.to_rgba();
                Ok(Rgba::new(
                    c_add(rgba.red(), "red")?,
                    c_add(rgba.green(), "green")?,
                    c_add(rgba.blue(), "blue")?,
                    c_add(rgba.alpha(), "alpha")?,
                )
                .into())
            } else if b_adj.is_null() && w_adj.is_null() {
                let hsla = rgba.to_hsla();
                let sl_add = |orig: Rational, x: Value| match x {
                    Value::Null => Ok(orig),
                    x => to_rational_percent(x).map(|x| orig + x),
                };
                Ok(Hsla::new(
                    c_add(hsla.hue(), "hue")?,
                    sl_add(hsla.sat(), s_adj)?,
                    sl_add(hsla.lum(), l_adj)?,
                    c_add(hsla.alpha(), "alpha")?,
                )
                .into())
            } else {
                let hwba = rgba.to_hwba();
                let sl_add = |orig: Rational, x: Value| match x {
                    Value::Null => Ok(orig),
                    x => to_rational_percent(x).map(|x| clamp_z1(orig + x)),
                };
                Ok(Hwba::new(
                    c_add(hwba.hue(), "hue")?,
                    sl_add(hwba.whiteness(), w_adj)?,
                    sl_add(hwba.blackness(), b_adj)?,
                    c_add(hwba.alpha(), "alpha")?,
                )
                .into())
            }
        }
    );
    def!(
        f,
        scale(
            color, red, green, blue, hue, saturation, lightness, whiteness,
            blackness, alpha
        ),
        |s| {
            let rgba = get_color(s, "color")?;
            let h_adj = s.get("hue")?;
            let s_adj = s.get("saturation")?;
            let l_adj = s.get("lightness")?;
            let b_adj = s.get("blackness")?;
            let w_adj = s.get("whiteness")?;
            let a_adj = s.get("alpha")?;

            let comb = |orig: Rational, x: Value, max: Rational| match x {
                Value::Null => Ok(orig),
                x => to_rational_percent(x).map(|x| {
                    if x.is_positive() {
                        orig + (max - orig) * x
                    } else {
                        orig + orig * x
                    }
                }),
            };
            let one = Rational::one();
            let ff = Rational::from_integer(255);
            if h_adj.is_null()
                && s_adj.is_null()
                && l_adj.is_null()
                && b_adj.is_null()
                && w_adj.is_null()
            {
                let rgba = rgba.to_rgba();
                Ok(Rgba::new(
                    comb(rgba.red(), s.get("red")?, ff)?,
                    comb(rgba.green(), s.get("green")?, ff)?,
                    comb(rgba.blue(), s.get("blue")?, ff)?,
                    comb(rgba.alpha(), a_adj, one)?,
                )
                .into())
            } else if b_adj.is_null() && w_adj.is_null() {
                let hsla = rgba.to_hsla();
                Ok(Hsla::new(
                    comb(hsla.hue(), h_adj, one)?,
                    comb(hsla.sat(), s_adj, one)?,
                    comb(hsla.lum(), l_adj, one)?,
                    comb(hsla.alpha(), a_adj, one)?,
                )
                .into())
            } else {
                let hwba = rgba.to_hwba();
                Ok(Hwba::new(
                    comb(hwba.hue(), h_adj, one)?,
                    comb(hwba.whiteness(), w_adj, one)?,
                    comb(hwba.blackness(), b_adj, one)?,
                    comb(hwba.alpha(), a_adj, one)?,
                )
                .into())
            }
        }
    );

    def!(f, opacity(color), |args| match args.get("color")? {
        Value::Color(ref col, _) => Ok(Value::scalar(col.get_alpha())),
        v => Ok(make_call("opacity", vec![v])),
    });
    def!(f, alpha(color), |args| match args.get("color")? {
        Value::Color(ref col, _) => Ok(Value::scalar(col.get_alpha())),
        v => Ok(make_call("alpha", vec![v])),
    });

    fn fade_in(color: Value, amount: Value) -> Result<Value, Error> {
        match (color, amount) {
            (Value::Color(mut col, _), Value::Numeric(v, ..)) => {
                col.set_alpha(col.get_alpha() + v.as_ratio()?);
                Ok(col.into())
            }
            (c, v) => Err(Error::badargs(&["color", "number"], &[&c, &v])),
        }
    }
    f.define_function(name!(_opacify), func2!(fade_in(color, amount)));

    fn fade_out(color: Value, amount: Value) -> Result<Value, Error> {
        match (color, amount) {
            (Value::Color(mut col, _), Value::Numeric(v, ..)) => {
                col.set_alpha(col.get_alpha() - v.as_ratio()?);
                Ok(col.into())
            }
            (c, v) => Err(Error::badargs(&["color", "number"], &[&c, &v])),
        }
    }
    f.define_function(name!(_fade_out), func2!(fade_out(color, amount)));

    def!(
        f,
        change(
            color, red, green, blue, hue, saturation, lightness, blackness,
            whiteness, alpha
        ),
        |s| {
            let rgba = get_color(s, "color")?;
            let h_adj = s.get("hue")?;
            let s_adj = s.get("saturation")?;
            let l_adj = s.get("lightness")?;
            let b_adj = s.get("blackness")?;
            let w_adj = s.get("whiteness")?;

            let c_or = |name: &str, orig: Rational| match s.get(name)? {
                Value::Null => Ok(orig),
                x => to_rational(x),
            };
            let a_or = |name: &str, orig: Rational| match s.get(name)? {
                Value::Null => Ok(orig),
                x => to_rational(x),
            };
            let sl_or = |x: Value, orig: Rational| match x {
                Value::Null => Ok(orig),
                x => to_rational_percent(x),
            };
            if h_adj.is_null()
                && s_adj.is_null()
                && l_adj.is_null()
                && b_adj.is_null()
                && w_adj.is_null()
            {
                let rgba = rgba.to_rgba();
                Ok(Rgba::new(
                    c_or("red", rgba.red())?,
                    c_or("green", rgba.green())?,
                    c_or("blue", rgba.blue())?,
                    a_or("alpha", rgba.alpha())?,
                )
                .into())
            } else if b_adj.is_null() && w_adj.is_null() {
                let hsla = rgba.to_hsla();
                Ok(Hsla::new(
                    a_or("hue", hsla.hue())?,
                    sl_or(s_adj, hsla.sat())?,
                    sl_or(l_adj, hsla.lum())?,
                    a_or("alpha", hsla.alpha())?,
                )
                .into())
            } else {
                let hwba = rgba.to_hwba();
                Ok(Hwba::new(
                    a_or("hue", hwba.hue())?,
                    sl_or(w_adj, hwba.whiteness())?,
                    sl_or(b_adj, hwba.blackness())?,
                    a_or("alpha", hwba.alpha())?,
                )
                .into())
            }
        }
    );
    def!(f, ie_hex_str(color), |s| {
        let (r, g, b, alpha) = get_color(s, "color")?.to_rgba().to_bytes();
        Ok(format!("#{:02X}{:02X}{:02X}{:02X}", alpha, r, g, b).into())
    });
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    for (gname, lname) in &[
        (name!(adjust_color), name!(adjust)),
        (name!(alpha), name!(alpha)),
        (name!(opacity), name!(opacity)),
        (name!(change_color), name!(change)),
        (name!(ie_hex_str), name!(ie_hex_str)),
        (name!(opacify), name!(_opacify)),
        (name!(fade_in), name!(_opacify)),
        (name!(scale_color), name!(scale)),
        (name!(transparentize), name!(_fade_out)),
        (name!(fade_out), name!(_fade_out)),
    ] {
        global.insert(gname.clone(), m.get_function(&lname).unwrap().clone());
    }
}

fn to_rational(v: Value) -> Result<Rational, Error> {
    match v {
        Value::Numeric(v, ..) => v.as_ratio(),
        v => Err(Error::badarg("number", &v)),
    }
}

/// Try to get percentage part (i.e. a number -1 .. 1) from a value.
///
/// If the value is a percentage, get it as part (e.g. 50% => 0.5).
/// If the value is in the correct range, return it as is.
/// Otherwise, guesstimate that it is a percenatage anyway and divide by 100.
fn to_rational_percent(v: Value) -> Result<Rational, Error> {
    match v {
        Value::Null => Ok(Rational::zero()),
        Value::Numeric(v, _) if v.unit == Unit::Percent => {
            Ok(v.as_ratio()? / 100)
        }
        Value::Numeric(v, ..) => {
            let v = v.as_ratio()?;
            Ok(if v.abs() < Rational::one() {
                v
            } else {
                v / 100
            })
        }
        v => Err(Error::badarg("number", &v)),
    }
}

/// Clamp a value into the zero - one range.
fn clamp_z1(v: Rational) -> Rational {
    if v < Rational::zero() {
        Rational::zero()
    } else if v > Rational::one() {
        Rational::one()
    } else {
        v
    }
}

#[cfg(test)]
mod test {
    use crate::variablescope::test::do_evaluate;

    #[test]
    fn ie_hex_str_a() {
        assert_eq!(do_evaluate(&[], b"ie-hex-str(#abc);"), "#FFAABBCC")
    }
    #[test]
    fn ie_hex_str_b() {
        assert_eq!(do_evaluate(&[], b"ie-hex-str(#3322BB);"), "#FF3322BB")
    }
    #[test]
    fn ie_hex_str_c() {
        assert_eq!(
            do_evaluate(&[], b"ie-hex-str(rgba(0, 255, 0, 0.5));"),
            "#8000FF00"
        )
    }
}
