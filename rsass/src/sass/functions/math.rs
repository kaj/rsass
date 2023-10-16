use super::{
    check, css_dim, expected_to, is_not, is_special, CallError, CheckedArg,
    FunctionMap, ResolvedArgs, Scope,
};
use crate::css::{BinOp, CallArgs, CssString, InvalidCss, Value};
use crate::output::Format;
use crate::parser::input_span;
use crate::sass::functions::{css_fn_arg, known_dim};
use crate::sass::Name;
use crate::value::{Number, Numeric, Quotes, Rational, Unit, UnitSet};
use std::cmp::Ordering;
use std::f64::consts::{E, PI};
use std::ops::Rem;

mod distance;
mod round;

/// Create the `sass:math` standard module.
///
/// Should conform to
/// [the specification](https://sass-lang.com/documentation/modules/math).
pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:math");

    def!(f, div(number1, number2), |s| {
        let a = s.get(name!(number1))?;
        let b = s.get(name!(number2))?;
        if let (Value::Numeric(a, _), Value::Numeric(b, _)) = (&a, &b) {
            Ok((a / b).into())
        } else {
            use crate::value::Operator;
            Ok(BinOp::new(a, false, Operator::Div, false, b).into())
        }
    });
    // - - - Boundig Functions - - -
    def!(f, ceil(number), |s| {
        let val: Numeric = s.get(name!(number))?;
        Ok(number(val.value.ceil(), val.unit))
    });
    def!(f, clamp(min, number, max), clamp_fn);
    def!(f, floor(number), |s| {
        let val: Numeric = s.get(name!(number))?;
        Ok(number(val.value.floor(), val.unit))
    });
    def_va!(f, max(numbers), |s| {
        let numbers = s.get_map(name!(numbers), check::va_list)?;
        find_extreme(&numbers, Ordering::Greater)
    });
    def_va!(f, min(numbers), |s| {
        let numbers = s.get_map(name!(numbers), check::va_list)?;
        find_extreme(&numbers, Ordering::Less)
    });
    def!(f, round(number), round::sass_round);

    // - - - Distance Functions - - -
    distance::in_module(&mut f);

    // - - - Exponential Functions - - -
    def!(f, exp(number), |s| {
        s.get_map(name!(number), radians)
            .map(|v| Value::scalar(v.exp()))
            .or_else(|e| fallback1(s, "exp", name!(number)).map_err(|_| e))
    });
    def!(f, log(number, base = b"null"), |s| {
        let num = get_unitless(s, "number");
        let base = s.get_opt_map(name!(base), check::unitless);
        match (num, base) {
            (Ok(num), Ok(base)) => {
                let base = base.map_or(E, Into::into);
                Ok(Value::scalar(num.log(base)))
            }
            (Err(e), _) | (_, Err(e)) => {
                let num = s.get_map(name!(number), expression);
                let base = s.get_map(name!(base), expression);
                if let (Ok(num), Ok(base)) = (num, base) {
                    Ok(Value::Call(
                        "log".into(),
                        CallArgs::from_list(vec![num, base]),
                    ))
                } else {
                    Err(e)
                }
            }
        }
    });
    def!(f, pow(base, exponent), |s| {
        let base = get_unitless(s, "base");
        let exponent = get_unitless(s, "exponent");
        match (base, exponent) {
            (Ok(base), Ok(exponent)) => {
                Ok(Value::scalar(base.powf(exponent)))
            }
            (Err(e), _) | (_, Err(e)) => {
                fallback2(s, "pow", name!(base), name!(exponent))
                    .map_err(|_| e)
            }
        }
    });
    def!(f, sqrt(number), |s| {
        get_unitless(s, "number")
            .map(|v| Value::scalar(v.sqrt()))
            .or_else(|e| fallback1(s, "sqrt", name!(number)).map_err(|_| e))
    });

    // - - - Trigonometric Functions - - -
    def!(f, cos(number), |s| {
        s.get_map(name!(number), radians)
            .map(|v| Value::scalar(v.cos()))
            .or_else(|e| fallback1(s, "cos", name!(number)).map_err(|_| e))
    });
    def!(f, sin(number), |s| {
        s.get_map(name!(number), radians)
            .map(|v| Value::scalar(v.sin()))
            .or_else(|e| fallback1(s, "sin", name!(number)).map_err(|_| e))
    });
    def!(f, tan(number), |s| {
        s.get_map(name!(number), radians)
            .map(|v| Value::scalar(v.tan()))
            .or_else(|e| fallback1(s, "tan", name!(number)).map_err(|_| e))
    });

    def!(f, acos(number), |s| {
        get_unitless(s, "number")
            .map(|v| deg_value(v.acos()))
            .or_else(|e| {
                s.get_map(name!(number), expression)
                    .map(|expr| {
                        Value::Call(
                            "acos".into(),
                            CallArgs::from_single(expr),
                        )
                    })
                    .map_err(|_| e)
            })
    });
    def!(f, asin(number), |s| {
        get_unitless(s, "number")
            .map(|v| deg_value(v.asin()))
            .or_else(|e| {
                s.get_map(name!(number), expression)
                    .map(|expr| {
                        Value::Call(
                            "asin".into(),
                            CallArgs::from_single(expr),
                        )
                    })
                    .map_err(|_| e)
            })
    });
    def!(f, atan(number), |s| {
        get_unitless(s, "number")
            .map(|v| deg_value(v.atan()))
            .or_else(|e| {
                s.get_map(name!(number), expression)
                    .map(|expr| {
                        Value::Call(
                            "atan".into(),
                            CallArgs::from_single(expr),
                        )
                    })
                    .map_err(|_| e)
            })
    });
    def!(f, atan2(y, x), |s| {
        let y: Numeric = s.get(name!(y))?;
        let x = s.get_map(name!(x), |v| {
            let v = Numeric::try_from(v)?;
            v.as_unitset(&y.unit)
                .ok_or_else(|| diff_units_msg(&v, &y, name!(y)))
        })?;
        Ok(deg_value(f64::from(y.value).atan2(f64::from(x))))
    });

    // - - - Unit Functions - - -
    def!(f, compatible(number1, number2), |s| {
        let u1 = s.get::<Numeric>(name!(number1))?.unit;
        let u2 = s.get::<Numeric>(name!(number2))?.unit;
        Ok(u1.is_compatible(&u2).into())
    });
    def!(f, is_unitless(number), |s| {
        Ok((s.get::<Numeric>(name!(number))?.is_no_unit()).into())
    });
    def!(f, unit(number), |s| {
        let mut unit = s.get::<Numeric>(name!(number))?.unit;
        unit.simplify();
        Ok(CssString::new(unit.to_string(), Quotes::Double).into())
    });

    // - - - Other Functions - - -
    def!(f, percentage(number), |s| {
        let val = s.get_map(name!(number), check::unitless)?;
        Ok(Numeric::new(val * 100, Unit::Percent).into())
    });
    def!(f, random(limit = b"null"), |s| {
        match s.get_opt_map(name!(limit), |v| {
            let v = check::int(v)?;
            if v > 0 {
                Ok(v)
            } else {
                Err(format!("Must be greater than 0, was {v}."))
            }
        })? {
            None => {
                let rez = 1_000_000;
                Ok(Value::scalar(Rational::new(intrand(rez), rez)))
            }
            Some(bound) => Ok(Value::scalar(intrand(bound) + 1)),
        }
    });

    f.define(name!(pi), Value::scalar(PI)).unwrap();
    f.define(name!(e), Value::scalar(E)).unwrap();
    f.define(name!(epsilon), Value::scalar(f64::EPSILON))
        .unwrap();
    f.define(name!(max_safe_integer), Value::scalar(9007199254740991f64))
        .unwrap();
    f.define(name!(min_safe_integer), Value::scalar(-9007199254740991f64))
        .unwrap();
    f.define(name!(max_number), Value::scalar(f64::MAX))
        .unwrap();
    f.define(
        name!(min_number),
        // Note: This value is the smallest number that a f32 can
        // express, but it is a "subnormal" number.  This is
        // compatible with dart sass, but maybe it would be better to
        // just use `f64::MIN_POSITIVE`, the smallest number an f64
        // can handle as a "normal" number.
        // See https://users.rust-lang.org/t/what-is-f64-min-positive/82725
        Value::scalar(
            2.0f64.powi(1 - (f64::MANTISSA_DIGITS as i32))
                * f64::MIN_POSITIVE,
        ),
    )
    .unwrap();
    f
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    for (gname, lname) in &[
        // - - - Boundig Functions - - -
        (name!(ceil), name!(ceil)),
        (name!(floor), name!(floor)),
        (name!(max), name!(max)),
        (name!(min), name!(min)),
        // - - - Exponential functions - - -
        (name!(exp), name!(exp)),
        (name!(log), name!(log)),
        (name!(pow), name!(pow)),
        (name!(sqrt), name!(sqrt)),
        // - - - Trigonometric functions - - -
        (name!(asin), name!(asin)),
        (name!(acos), name!(acos)),
        (name!(atan), name!(atan)),
        (name!(sin), name!(sin)),
        (name!(cos), name!(cos)),
        (name!(tan), name!(tan)),
        // - - - Unit Functions - - -
        (name!(comparable), name!(compatible)),
        (name!(unitless), name!(is_unitless)),
        (name!(unit), name!(unit)),
        // - - - Other Functions - - -
        (name!(percentage), name!(percentage)),
        (name!(random), name!(random)),
    ] {
        global.insert(gname.clone(), m.get_lfunction(lname));
    }

    // Functions behave somewhat differently in the global scope vs in the math module.
    distance::global(global);
    def!(global, clamp(min, number = b"null", max = b"null"), |s| {
        clamp_fn(s).or_else(|_| {
            let mut args = vec![s.get::<Value>(name!(min))?];
            if let Some(b) = s.get_opt(name!(number))? {
                args.push(b);
            }
            if let Some(c) = s.get_opt(name!(max))? {
                args.push(c);
            }
            if let Some((a, rest)) = args.split_first() {
                if let Some(adim) = css_dim(a) {
                    for b in rest {
                        if let Some(bdim) = css_dim(b) {
                            if adim != bdim {
                                return Err(CallError::incompatible_values(
                                    a, b,
                                ));
                            }
                        }
                    }
                }
            }
            Ok(Value::Call("clamp".into(), CallArgs::from_list(args)))
        })
    });
    def!(global, atan2(y, x), |s| {
        fn real_atan2(s: &ResolvedArgs) -> Result<Value, CallError> {
            let y: Numeric = s.get(name!(y))?;
            if y.unit.is_percent() {
                return Err(String::from("No percentage here"))
                    .named(name!(y));
            }
            let x = s.get_map(name!(x), |v| {
                let v = Numeric::try_from(v)?;
                v.as_unitset(&y.unit)
                    .ok_or_else(|| diff_units_msg(&v, &y, name!(y)))
            })?;
            Ok(deg_value(f64::from(y.value).atan2(f64::from(x))))
        }
        real_atan2(s).or_else(|e| {
            fallback2a(s, "atan2", name!(y), name!(x)).map_err(|_| e)
        })
    });
    def!(global, rem(y, x), |s| {
        fn real_rem(s: &ResolvedArgs) -> Result<Value, CallError> {
            let y: Numeric = s.get(name!(y))?;
            let x = s.get_map(name!(x), |v| {
                let v = Numeric::try_from(v)?;
                v.as_unitset(&y.unit)
                    .ok_or_else(|| diff_units_msg(&v, &y, name!(y)))
            })?;
            Ok(number(f64::from(y.value).rem(f64::from(x)), y.unit))
        }
        real_rem(s).or_else(|e| {
            fallback2a(s, "rem", name!(y), name!(x)).map_err(|_| e)
        })
    });
    def!(global, mod(y, x), |s| {
        fn real_mod(s: &ResolvedArgs) -> Result<Value, CallError> {
            let y: Numeric = s.get(name!(y))?;
            let x: Numeric = s.get(name!(x))?;
            let x = x.as_unitset(&y.unit)
                    .ok_or_else(|| CallError::msg(diff_units_msg2(&y, &x)))?;
            let unit = y.unit;
            let y = f64::from(y.value);
            let m = f64::from(x);
            let mut y = y.rem(m);
            if (y * m.signum()).is_sign_negative() {
                if m.is_finite() {
                    y += m;
                    y = y.rem(m);
                } else {
                    y = f64::NAN;
                }
            }
            let y = y.abs() * m.signum();
            Ok(number(y, unit))
        }
        real_mod(s).or_else(|e| {
            fallback2a(s, "mod", name!(y), name!(x)).map_err(|_| e)
        })
    });
    def_va!(global, round(kwargs), round::css_round);
    def!(global, sign(v), |s| {
        fn real_sign(s: &ResolvedArgs) -> Result<Value, CallError> {
            let v: Numeric = s.get(name!(v))?;
            Ok(number(v.value.signum(), v.unit))
        }
        real_sign(s)
            .or_else(|e| fallback1(s, "sign", name!(v)).map_err(|_| e))
    });
}

fn radians(v: Value) -> Result<f64, String> {
    let v = Numeric::try_from(v)?;
    v.as_unit_def(Unit::Rad).map(Into::into).ok_or_else(|| {
        expected_to(&v, "have an angle unit (deg, grad, rad, turn)")
    })
}

fn fallback1(
    s: &ResolvedArgs,
    name: &str,
    a1: Name,
) -> Result<Value, CallError> {
    s.get_map(a1, expression)
        .map(|expr| Value::Call(name.into(), CallArgs::from_single(expr)))
}

fn fallback2(
    s: &ResolvedArgs,
    name: &str,
    a1: Name,
    a2: Name,
) -> Result<Value, ()> {
    let (a1, a2) = match (get_expr(s, a1), get_expr(s, a2)) {
        (Ok(a1), Ok(a2)) => (a1, a2),
        (Ok(a1), Err(a2 @ Value::Numeric(..))) => (a1, a2),
        (Err(a1 @ Value::Numeric(..)), Ok(a2)) => (a1, a2),
        _ => return Err(()),
    };
    Ok(Value::Call(name.into(), CallArgs::from_list(vec![a1, a2])))
}
fn fallback2a(
    s: &ResolvedArgs,
    name: &str,
    a1: Name,
    a2: Name,
) -> Result<Value, ()> {
    let (a1, a2) = match (get_expr_a(s, a1), get_expr_a(s, a2)) {
        (Ok(a1), Ok(a2)) => {
            let dim1 = css_dim(&a1);
            let dim2 = css_dim(&a2);
            if (dim1 == dim2) || dim1.is_none() || dim2.is_none() {
                (a1, a2)
            } else {
                return Err(());
            }
        }
        _ => return Err(()),
    };
    Ok(Value::Call(name.into(), CallArgs::from_list(vec![a1, a2])))
}

fn get_expr(s: &ResolvedArgs, name: Name) -> Result<Value, Value> {
    match s.get(name) {
        Ok(v @ (Value::BinOp(_) | Value::Call(..))) => Ok(v),
        Ok(v) => Err(v),
        Err(_) => Err(Value::Null),
    }
}
fn get_expr_a(s: &ResolvedArgs, name: Name) -> Result<Value, Value> {
    match s.get(name) {
        Ok(v @ (Value::BinOp(_) | Value::Call(..) | Value::Numeric(..))) => {
            Ok(v)
        }
        Ok(v) => Err(v),
        Err(_) => Err(Value::Null),
    }
}

fn expression(v: Value) -> Result<Value, String> {
    match v {
        v @ (Value::BinOp(_) | Value::Call(..)) => Ok(v),
        _ => Err("Expected expression".into()),
    }
}

fn get_unitless(s: &ResolvedArgs, name: &str) -> Result<f64, CallError> {
    s.get_map(name.into(), check::unitless).map(Into::into)
}

fn number(v: impl Into<Number>, unit: impl Into<UnitSet>) -> Value {
    Numeric::new(v.into(), unit).into()
}

/// convert f64 in radians (used by rust) to numeric Value in degrees
/// (used by sass).
fn deg_value(rad: f64) -> Value {
    number(rad.to_degrees(), Unit::Deg)
}

fn find_extreme(v: &[Value], pref: Ordering) -> Result<Value, CallError> {
    let as_call = || {
        Value::Call(
            if pref == Ordering::Greater {
                "max"
            } else {
                "min"
            }
            .into(),
            CallArgs::from_list(v.to_vec()),
        )
    };
    if v.iter().any(is_special) {
        return Ok(as_call());
    }
    match find_extreme_inner(v, pref) {
        Ok(Some(v)) => Ok(v.into()),
        Ok(None) => {
            Err(CallError::msg("At least one argument must be passed."))
        }
        Err(ExtremeError::NonNumeric(v)) => {
            if let Value::Literal(s) = &v {
                if s.quotes().is_none()
                    && crate::parser::value::number(
                        input_span(s.value()).borrow(),
                    )
                    .is_ok()
                {
                    return Ok(as_call());
                }
            }
            if v.type_name() == "unknown" {
                Ok(as_call())
            } else {
                Err(CallError::msg(is_not(&v, "a number")))
            }
        }
        Err(ExtremeError::Incompatible(a, b)) => {
            let a_dim = a.unit.css_dimension();
            let b_dim = b.unit.css_dimension();
            if a_dim.is_empty() || b_dim.is_empty() || a_dim == b_dim {
                Ok(as_call())
            } else {
                Err(CallError::msg(InvalidCss::Incompat(a, b)))
            }
        }
        Err(_) => Ok(as_call()),
    }
}

fn find_extreme_inner(
    v: &[Value],
    pref: Ordering,
) -> Result<Option<Numeric>, ExtremeError> {
    if let Some((first, rest)) = v.split_first() {
        let va = Numeric::try_from(first.clone())
            .map_err(|_| ExtremeError::NonNumeric(first.clone()))?;
        if let Some(vb) = find_extreme_inner(rest, pref)? {
            if let Some(o) = va.partial_cmp(&vb) {
                Ok(Some(if o == pref { va } else { vb }))
            } else if va.is_no_unit() || vb.is_no_unit() {
                if let Some(o) = va.value.partial_cmp(&vb.value) {
                    Ok(Some(if o == pref { va } else { vb }))
                } else {
                    Err(ExtremeError::Incomparable(va, vb))
                }
            } else {
                Err(ExtremeError::Incompatible(va, vb))
            }
        } else {
            Ok(Some(va))
        }
    } else {
        Ok(None)
    }
}

#[derive(Debug)]
enum ExtremeError {
    NonNumeric(Value),
    Incompatible(Numeric, Numeric),
    Incomparable(Numeric, Numeric),
}

fn intrand(lim: i64) -> i64 {
    fastrand::i64(0..lim)
}

fn diff_units_msg(
    one: &Numeric,
    other: &Numeric,
    other_name: Name,
) -> String {
    format!(
        "{} and ${}: {} have incompatible units{}.",
        one.format(Format::introspect()),
        other_name,
        other.format(Format::introspect()),
        if one.is_no_unit() || other.is_no_unit() {
            " (one has units and the other doesn't)"
        } else {
            ""
        }
    )
}

fn diff_units_msg2(one: &Numeric, other: &Numeric) -> String {
    format!(
        "{} and {} are incompatible.",
        one.format(Format::introspect()),
        other.format(Format::introspect()),
    )
}

pub(crate) fn clamp_fn(s: &ResolvedArgs) -> Result<Value, CallError> {
    let min_v = s.get::<Numeric>(name!(min))?;
    let check_numeric_compat_unit = |v: Value| -> Result<Numeric, String> {
        let v = Numeric::try_from(v)?;
        if (v.is_no_unit() != min_v.is_no_unit())
            || !v.unit.is_compatible(&min_v.unit)
        {
            return Err(diff_units_msg(&v, &min_v, name!(min)));
        }
        Ok(v)
    };
    let mut num = s.get_map(name!(number), check_numeric_compat_unit)?;
    let max_v = s.get_map(name!(max), check_numeric_compat_unit)?;

    if num >= max_v {
        num = max_v;
    }
    if num <= min_v {
        num = min_v;
    }
    Ok(Value::Numeric(num, true))
}
