use super::channels::Channels;
use super::{
    check_alpha, check_channel, check_pct_range, eval_inner, is_not,
    is_special, make_call, CallError, CheckedArg, FunctionMap, ResolvedArgs,
};
use crate::css::{CallArgs, Value};
use crate::sass::{ArgsError, FormalArgs, Name};
use crate::value::{Color, Rational, RgbFormat, Rgba};
use crate::Scope;
use num_traits::{one, Zero};

pub fn register(f: &mut Scope) {
    def_va!(f, _rgb(kwargs), |s| do_rgba(&name!(rgb), s));
    def_va!(f, _rgba(kwargs), |s| do_rgba(&name!(rgba), s));
    def!(f, red(color), |s| {
        let c: Color = s.get(name!(color))?;
        Ok(Value::scalar(c.to_rgba().red().round()))
    });
    def!(f, green(color), |s| {
        let c: Color = s.get(name!(color))?;
        Ok(Value::scalar(c.to_rgba().green().round()))
    });
    def!(f, blue(color), |s| {
        let c: Color = s.get(name!(color))?;
        Ok(Value::scalar(c.to_rgba().blue().round()))
    });
    def!(f, mix(color1, color2, weight = b"50%"), |s| {
        let a: Color = s.get(name!(color1))?;
        let a = a.to_rgba();
        let b: Color = s.get(name!(color2))?;
        let b = b.to_rgba();
        let w = s.get_map(name!(weight), check_pct_range)?;
        let one: Rational = one();

        let w_a = {
            let wa = a.alpha() - b.alpha();
            let w2 = w * 2 - 1;
            let divis = w2 * wa + 1;
            if divis.is_zero() {
                w
            } else {
                (((w2 + wa) / divis) + 1) / 2
            }
        };
        let w_b = one - w_a;

        let m_c = |c_a, c_b| w_a * c_a + w_b * c_b;
        Ok(Rgba::new(
            m_c(a.red(), b.red()),
            m_c(a.green(), b.green()),
            m_c(a.blue(), b.blue()),
            a.alpha() * w + b.alpha() * (one - w),
            RgbFormat::Name,
        )
        .into())
    });
    def!(f, invert(color, weight = b"100%"), |s| {
        match s.get(name!(color))? {
            Value::Color(col, _) => {
                let rgba = col.to_rgba();
                let w = s.get_map(name!(weight), check_pct_range)?;
                let inv = |v: Rational| -(v - 255) * w + v * -(w - 1);
                Ok(Rgba::new(
                    inv(rgba.red()),
                    inv(rgba.green()),
                    inv(rgba.blue()),
                    rgba.alpha(),
                    rgba.source(),
                )
                .into())
            }
            col => {
                let w = s.get_map(name!(weight), check_pct_range)?;
                if w == one() {
                    match col {
                        v @ Value::Numeric(..) => {
                            Ok(make_call("invert", vec![v]))
                        }
                        v => Err(is_not(&v, "a color")).named(name!(color)),
                    }
                } else {
                    Err(CallError::msg("Only one argument may be passed to the plain-CSS invert() function."))
                }
            }
        }
    });
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    for (gname, lname) in &[
        (name!(rgb), name!(_rgb)),
        (name!(rgba), name!(_rgba)),
        (name!(blue), name!(blue)),
        (name!(green), name!(green)),
        (name!(invert), name!(invert)),
        (name!(mix), name!(mix)),
        (name!(red), name!(red)),
    ] {
        global.insert(gname.clone(), m.get_lfunction(lname));
    }
}

fn do_rgba(fn_name: &Name, s: &ResolvedArgs) -> Result<Value, CallError> {
    let a1 = FormalArgs::new(vec![one_arg!(channels)]);
    let a1b = FormalArgs::new(vec![one_arg!(color), one_arg!(alpha)]);
    let a2 = FormalArgs::new(vec![
        one_arg!(red),
        one_arg!(green),
        one_arg!(blue = b"null"),
        one_arg!(alpha = b"null"),
    ]);
    let a2_show = FormalArgs::new(vec![
        one_arg!(red),
        one_arg!(green),
        one_arg!(blue),
        one_arg!(alpha),
    ]);
    let args = s.get_map(name!(kwargs), CallArgs::from_value)?;
    match eval_inner(fn_name, &a1, s, args.clone()) {
        Ok(s) => Channels::try_from(s.get::<Value>(name!(channels))?)
            .map_err(|e| e.conv(&["red", "green", "blue"]))
            .and_then(|c| match c {
                Channels::Data([h, s, l, a]) => {
                    rgba_from_values(fn_name, h, s, l, a)
                }
                Channels::Special(channels) => {
                    Ok(make_call(fn_name.as_ref(), vec![channels]))
                }
            }),
        Err(err @ CallError::Args(ArgsError::Missing(_), _)) => Err(err),
        Err(_) => match eval_inner(fn_name, &a1b, s, args.clone()) {
            Ok(s) => {
                let c = s.get(name!(color))?;
                let a = s.get(name!(alpha))?;
                if is_special(&c) || is_special(&a) {
                    if let Ok(c) = Color::try_from(c.clone()) {
                        let c = c.to_rgba();
                        Ok(make_call(
                            fn_name.as_ref(),
                            vec![
                                Value::scalar(c.red()),
                                Value::scalar(c.green()),
                                Value::scalar(c.blue()),
                                a,
                            ],
                        ))
                    } else {
                        Ok(make_call(fn_name.as_ref(), vec![c, a]))
                    }
                } else {
                    let mut c = Color::try_from(c).named(name!(color))?;
                    c.set_alpha(check_alpha(a).named(name!(alpha))?);
                    c.reset_source();
                    Ok(c.into())
                }
            }
            Err(_) => {
                let s = eval_inner(fn_name, &a2_show, s, args.clone())
                    .or_else(|e| {
                        eval_inner(fn_name, &a2, s, args).map_err(|_| e)
                    })?;
                rgba_from_values(
                    fn_name,
                    s.get(name!(red))?,
                    s.get(name!(green))?,
                    s.get(name!(blue))?,
                    s.get(name!(alpha))?,
                )
            }
        },
    }
}

fn rgba_from_values(
    fn_name: &Name,
    r: Value,
    g: Value,
    b: Value,
    a: Value,
) -> Result<Value, CallError> {
    if is_special(&r) || is_special(&g) || is_special(&b) || is_special(&a) {
        Ok(make_call(fn_name.as_ref(), vec![r, g, b, a]))
    } else {
        Ok(Rgba::new(
            check_channel(r).named(name!(red))?,
            check_channel(g).named(name!(green))?,
            check_channel(b).named(name!(blue))?,
            check_alpha(a).named(name!(alpha))?,
            RgbFormat::Rgb,
        )
        .into())
    }
}

#[cfg(test)]
mod test {
    use crate::variablescope::test::do_evaluate;

    #[test]
    fn test_high() {
        assert_eq!(
            do_evaluate(&[], b"rgb(150%, 300, 256);"),
            "rgb(255, 255, 255)"
        );
    }

    #[test]
    fn test_low() {
        assert_eq!(do_evaluate(&[], b"rgb(-3, -2%, 0);"), "rgb(0, 0, 0)");
    }
    #[test]
    fn test_mid() {
        assert_eq!(
            do_evaluate(&[], b"rgb(50%, 255/2, 25% + 25);"),
            "rgb(128, 128, 128)"
        );
    }
}
