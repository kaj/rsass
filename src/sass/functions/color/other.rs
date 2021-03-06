use super::{
    get_color, get_opt_rational, get_rational, make_call, Error, FunctionMap,
    Name,
};
use crate::css::Value;
use crate::value::{Hsla, Hwba, Rgba};
use crate::Scope;
use num_rational::Rational;
use num_traits::{One, Signed};

pub fn register(f: &mut Scope) {
    def!(
        f,
        adjust(
            color,
            red = b"null",
            green = b"null",
            blue = b"null",
            hue = b"null",
            saturation = b"null",
            lightness = b"null",
            whiteness = b"null",
            blackness = b"null",
            alpha = b"null"
        ),
        |s| {
            fn opt_add(a: Rational, b: Option<Rational>) -> Rational {
                if let Some(b) = b {
                    a + b
                } else {
                    a
                }
            }
            let rgba = get_color(s, "color")?;
            let h_adj = get_opt_rational(s, "hue")?;
            let s_adj = get_opt_rational_pct(s, "saturation")?;
            let l_adj = get_opt_rational_pct(s, "lightness")?;
            let b_adj = get_opt_rational_pct(s, "blackness")?;
            let w_adj = get_opt_rational_pct(s, "whiteness")?;
            let a_adj = get_opt_rational(s, "alpha")?;
            if h_adj.is_none()
                && s_adj.is_none()
                && l_adj.is_none()
                && b_adj.is_none()
                && w_adj.is_none()
            {
                let rgba = rgba.to_rgba();
                Ok(Rgba::new(
                    opt_add(rgba.red(), get_opt_rational(s, "red")?),
                    opt_add(rgba.green(), get_opt_rational(s, "green")?),
                    opt_add(rgba.blue(), get_opt_rational(s, "blue")?),
                    opt_add(rgba.alpha(), get_opt_rational(s, "alpha")?),
                )
                .into())
            } else if b_adj.is_none() && w_adj.is_none() {
                let hsla = rgba.to_hsla();
                Ok(Hsla::new(
                    opt_add(hsla.hue(), h_adj),
                    opt_add(hsla.sat(), s_adj),
                    opt_add(hsla.lum(), l_adj),
                    opt_add(hsla.alpha(), a_adj),
                )
                .into())
            } else {
                let hwba = rgba.to_hwba();
                Ok(Hwba::new(
                    opt_add(hwba.hue(), h_adj),
                    opt_add(hwba.whiteness(), w_adj),
                    opt_add(hwba.blackness(), b_adj),
                    opt_add(hwba.alpha(), a_adj),
                )
                .into())
            }
        }
    );
    def!(
        f,
        scale(
            color,
            red = b"null",
            green = b"null",
            blue = b"null",
            hue = b"null",
            saturation = b"null",
            lightness = b"null",
            whiteness = b"null",
            blackness = b"null",
            alpha = b"null"
        ),
        |s| {
            let rgba = get_color(s, "color")?;
            let h_adj = get_opt_rational_pct(s, "hue")?;
            let s_adj = get_opt_rational_pct(s, "saturation")?;
            let l_adj = get_opt_rational_pct(s, "lightness")?;
            let b_adj = get_opt_rational_pct(s, "blackness")?;
            let w_adj = get_opt_rational_pct(s, "whiteness")?;
            let a_adj = get_opt_rational_pct(s, "alpha")?;

            let cmb = |orig: Rational, x: Option<Rational>, max: Rational| {
                match x {
                    None => orig,
                    Some(x) => {
                        if x.is_positive() {
                            orig + (max - orig) * x
                        } else {
                            orig + orig * x
                        }
                    }
                }
            };
            let one = Rational::one();
            let ff = Rational::from_integer(255);
            if h_adj.is_none()
                && s_adj.is_none()
                && l_adj.is_none()
                && b_adj.is_none()
                && w_adj.is_none()
            {
                let rgba = rgba.to_rgba();
                Ok(Rgba::new(
                    cmb(rgba.red(), get_opt_rational_pct(s, "red")?, ff),
                    cmb(rgba.green(), get_opt_rational_pct(s, "green")?, ff),
                    cmb(rgba.blue(), get_opt_rational_pct(s, "blue")?, ff),
                    cmb(rgba.alpha(), a_adj, one),
                )
                .into())
            } else if b_adj.is_none() && w_adj.is_none() {
                let hsla = rgba.to_hsla();
                Ok(Hsla::new(
                    cmb(hsla.hue(), h_adj, one),
                    cmb(hsla.sat(), s_adj, one),
                    cmb(hsla.lum(), l_adj, one),
                    cmb(hsla.alpha(), a_adj, one),
                )
                .into())
            } else {
                let hwba = rgba.to_hwba();
                Ok(Hwba::new(
                    cmb(hwba.hue(), h_adj, one),
                    cmb(hwba.whiteness(), w_adj, one),
                    cmb(hwba.blackness(), b_adj, one),
                    cmb(hwba.alpha(), a_adj, one),
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

    def!(f, _fade_in(color, amount), |s| {
        let mut col = get_color(s, "color")?;
        let amount = get_rational(s, "amount")?;
        col.set_alpha(col.get_alpha() + amount);
        Ok(col.into())
    });
    def!(f, _fade_out(color, amount), |s| {
        let mut col = get_color(s, "color")?;
        let amount = get_rational(s, "amount")?;
        col.set_alpha(col.get_alpha() - amount);
        Ok(col.into())
    });

    def!(
        f,
        change(
            color,
            red = b"null",
            green = b"null",
            blue = b"null",
            hue = b"null",
            saturation = b"null",
            lightness = b"null",
            blackness = b"null",
            whiteness = b"null",
            alpha = b"null"
        ),
        |s| {
            let rgba = get_color(s, "color")?;
            let h_adj = get_opt_rational(s, "hue")?;
            let s_adj = get_opt_rational_pct(s, "saturation")?;
            let l_adj = get_opt_rational_pct(s, "lightness")?;
            let b_adj = get_opt_rational_pct(s, "blackness")?;
            let w_adj = get_opt_rational_pct(s, "whiteness")?;

            if h_adj.is_none()
                && s_adj.is_none()
                && l_adj.is_none()
                && b_adj.is_none()
                && w_adj.is_none()
            {
                let rgba = rgba.to_rgba();
                Ok(Rgba::new(
                    get_opt_rational(s, "red")?.unwrap_or_else(|| rgba.red()),
                    get_opt_rational(s, "green")?
                        .unwrap_or_else(|| rgba.green()),
                    get_opt_rational(s, "blue")?
                        .unwrap_or_else(|| rgba.blue()),
                    get_opt_rational(s, "alpha")?
                        .unwrap_or_else(|| rgba.alpha()),
                )
                .into())
            } else if b_adj.is_none() && w_adj.is_none() {
                let hsla = rgba.to_hsla();
                Ok(Hsla::new(
                    h_adj.unwrap_or_else(|| hsla.hue()),
                    s_adj.unwrap_or_else(|| hsla.sat()),
                    l_adj.unwrap_or_else(|| hsla.lum()),
                    get_opt_rational(s, "alpha")?
                        .unwrap_or_else(|| hsla.alpha()),
                )
                .into())
            } else {
                let hwba = rgba.to_hwba();
                Ok(Hwba::new(
                    h_adj.unwrap_or_else(|| hwba.hue()),
                    w_adj.unwrap_or_else(|| hwba.whiteness()),
                    b_adj.unwrap_or_else(|| hwba.blackness()),
                    get_opt_rational(s, "alpha")?
                        .unwrap_or_else(|| hwba.alpha()),
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
        (name!(opacify), name!(_fade_in)),
        (name!(fade_in), name!(_fade_in)),
        (name!(scale_color), name!(scale)),
        (name!(transparentize), name!(_fade_out)),
        (name!(fade_out), name!(_fade_out)),
    ] {
        global.insert(gname.clone(), m.get_function(&lname).unwrap().clone());
    }
}

fn get_opt_rational_pct(
    s: &Scope,
    name: &str,
) -> Result<Option<Rational>, Error> {
    match s.get(name)? {
        Value::Numeric(v, _) if v.unit.is_percent() => {
            Ok(Some(v.as_ratio()? / 100))
        }
        Value::Numeric(v, ..) => {
            let v = v.as_ratio()?;
            Ok(Some(if v.abs() < Rational::one() {
                v
            } else {
                v / 100
            }))
        }
        Value::Null => Ok(None),
        v => Err(Error::bad_arg(Name::from(name), &v, "is not a number")),
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
