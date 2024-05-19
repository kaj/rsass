use super::num_or_special::NumOrSpecial;
use super::{
    check, expected_to, unnamed, CallError, CheckedArg, FunctionMap,
    ResolvedArgs, Scope,
};
use crate::css::{BinOp, CallArgs, CssString, InvalidCss, Value};
use crate::output::Format;
use crate::sass::Name;
use crate::value::{CssDimensionSet, Numeric, Operator, Quotes, Unit};
use std::cmp::Ordering;
use std::f64::consts::{E, PI};

mod css;
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
        Ok(Numeric::new(val.value.ceil(), val.unit).into())
    });
    def!(f, clamp(min, number, max), |s| {
        let min_v = s.get::<Numeric>(name!(min))?;
        let check_numeric_compat_unit =
            |v: Value| -> Result<Numeric, String> {
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
    });
    def!(f, floor(number), |s| {
        let val: Numeric = s.get(name!(number))?;
        Ok(Numeric::new(val.value.floor(), val.unit).into())
    });
    def_va!(f, max(numbers), |s| {
        let numbers = unnamed(s.get_va(name!(numbers)))?;
        find_extreme(&numbers, Ordering::Greater)?
            .map_or_else(|| Ok(Value::call("max", numbers)), |v| Ok(v.into()))
    });
    def_va!(f, min(numbers), |s| {
        let numbers = unnamed(s.get_va(name!(numbers)))?;
        find_extreme(&numbers, Ordering::Less)?
            .map_or_else(|| Ok(Value::call("min", numbers)), |v| Ok(v.into()))
    });
    def!(f, round(number), round::sass_round);

    // - - - Distance Functions - - -
    distance::in_module(&mut f);

    // - - - Exponential Functions - - -
    def!(f, exp(number), |s| {
        Ok(Value::scalar(s.get_map(name!(number), unitless)?.exp()))
    });
    def!(f, log(number, base = b"null"), |s| {
        let num = s.get_map(name!(number), unitless)?;
        let base = s.get_opt_map(name!(base), unitless)?;
        Ok(Value::scalar(num.log(base.unwrap_or(E))))
    });
    def!(f, pow(base, exponent), |s| {
        let base = s.get_map(name!(base), unitless)?;
        let exponent = s.get_map(name!(exponent), unitless)?;
        Ok(Value::scalar(base.powf(exponent)))
    });
    def!(f, sqrt(number), |s| {
        Ok(Value::scalar(s.get_map(name!(number), unitless)?.sqrt()))
    });

    // - - - Trigonometric Functions - - -
    def!(f, cos(number), |s| {
        Ok(Value::scalar(s.get_map(name!(number), radians)?.cos()))
    });
    def!(f, sin(number), |s| {
        Ok(Value::scalar(s.get_map(name!(number), radians)?.sin()))
    });
    def!(f, tan(number), |s| {
        Ok(Value::scalar(s.get_map(name!(number), radians)?.tan()))
    });

    def!(f, acos(number), |s| {
        Ok(deg_value(s.get_map(name!(number), unitless)?.acos()))
    });
    def!(f, asin(number), |s| {
        Ok(deg_value(s.get_map(name!(number), unitless)?.asin()))
    });
    def!(f, atan(number), |s| {
        Ok(deg_value(s.get_map(name!(number), unitless)?.atan()))
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
        Ok(CssString::new(format!("{unit:#}"), Quotes::Double).into())
    });

    // - - - Other Functions - - -
    def!(f, percentage(number), |s| {
        let val = s.get_map(name!(number), check::unitless)?;
        Ok(Numeric::percentage(val).into())
    });
    def!(f, random(limit = b"null"), |s| {
        match s.get_opt_map(name!(limit), check::positive_int)? {
            None => Ok(Value::scalar(fastrand::f64())),
            Some(bound) => Ok(Value::scalar(fastrand::i64(0..bound) + 1)),
        }
    });

    f.define(name!(pi), Value::scalar(PI)).unwrap();
    f.define(name!(e), Value::scalar(E)).unwrap();
    f.define(name!(epsilon), Value::scalar(f64::EPSILON))
        .unwrap();
    f.define(
        name!(max_safe_integer),
        Value::scalar(9_007_199_254_740_991_f64),
    )
    .unwrap();
    f.define(
        name!(min_safe_integer),
        Value::scalar(-9_007_199_254_740_991_f64),
    )
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
    css::global(global);
    distance::global(global);
    def_va!(global, round(kwargs), round::css_round);
}

fn num2radians(v: Numeric) -> Result<f64, String> {
    v.as_unit_def(Unit::Rad).map(Into::into).ok_or_else(|| {
        expected_to(v, "have an angle unit (deg, grad, rad, turn)")
    })
}

fn radians(v: Value) -> Result<f64, String> {
    num2radians(v.try_into()?)
}

fn unitless(value: Value) -> Result<f64, String> {
    check::unitless(value).map(Into::into)
}

/// convert f64 in radians (used by rust) to numeric Value in degrees
/// (used by sass).
fn deg_value(rad: f64) -> Value {
    Numeric::new(rad.to_degrees(), Unit::Deg).into()
}

fn find_extreme(
    v: &[NumOrSpecial],
    pref: Ordering,
) -> Result<Option<Numeric>, ExtremeError> {
    let mut v = v.iter();
    let found = v.next().ok_or(ExtremeError::OneRequired)?;
    let mut found = match found {
        NumOrSpecial::Num(found) => found,
        _ => return Ok(None),
    };
    for v in v {
        let v = match v {
            NumOrSpecial::Num(v) => v,
            _ => return Ok(None),
        };
        if let Some(o) = cmp2(found, v) {
            found = if o == pref { found } else { v };
        } else if may_cmp_css(found, v) {
            return Ok(None);
        } else {
            return Err(ExtremeError::Incompatible(found.clone(), v.clone()));
        }
    }
    Ok(Some(found.clone()))
}

fn cmp2(a: &Numeric, b: &Numeric) -> Option<Ordering> {
    a.partial_cmp(b).or_else(|| {
        if a.is_no_unit() || b.is_no_unit() {
            a.value.partial_cmp(&b.value)
        } else {
            None
        }
    })
}

fn may_cmp_css(a: &Numeric, b: &Numeric) -> bool {
    let a_dim = a.unit.css_dimension();
    let b_dim = b.unit.css_dimension();
    a_dim.is_empty() || b_dim.is_empty() || a_dim == b_dim
}

#[derive(Debug)]
enum ExtremeError {
    OneRequired,
    Incompatible(Numeric, Numeric),
}

impl From<ExtremeError> for CallError {
    fn from(value: ExtremeError) -> Self {
        match value {
            ExtremeError::OneRequired => {
                Self::msg("At least one argument must be passed.")
            }
            ExtremeError::Incompatible(a, b) => {
                Self::msg(InvalidCss::Incompat(a, b))
            }
        }
    }
}

fn css_fn_arg(v: Value) -> Result<Value, CallError> {
    match v {
        Value::Literal(s) if s.quotes() == Quotes::None => Ok(s.into()),
        Value::BinOp(op) => {
            let a = css_fn_arg(op.a().clone())?;
            let b = css_fn_arg(op.b().clone())?;
            let op = op.op();
            if let (Some(adim), Some(bdim)) = (css_dim(&a), css_dim(&b)) {
                if (op == Operator::Plus || op == Operator::Minus)
                    && adim != bdim
                {
                    return Err(CallError::incompatible_values(a, b));
                }
            }
            Ok(BinOp::new(a, true, op, true, b).into())
        }
        Value::Paren(v) => match v.as_ref() {
            l @ Value::Paren(_) => Ok(l.clone()),
            l @ Value::BinOp(..) => Ok(l.clone()),
            _ => Ok(Value::Paren(v)),
        },
        list @ Value::List(..) => {
            // FIXME: Check if list seems good, as above?
            Ok(list)
        }
        v => NumOrSpecial::in_calc(v)
            .and_then(|ns| {
                ns.try_map(|num| {
                    if num.unit.valid_in_css() {
                        Ok(num)
                    } else {
                        Err(format!(
                        "Number {} isn't compatible with CSS calculations.",
                        Value::from(num).introspect()
                    ))
                    }
                })
            })
            .map_err(CallError::msg)
            .map(Value::from),
    }
}

// Note: None here is for unknown, e.g. the dimension of something that is not a number.
fn css_dim(v: &Value) -> Option<CssDimensionSet> {
    match v {
        Value::Numeric(num, _) => known_dim(num),
        _ => None,
    }
}
fn known_dim(v: &Numeric) -> Option<CssDimensionSet> {
    let u = &v.unit;
    if u.is_known() && !u.is_percent() {
        Some(u.css_dimension())
    } else {
        None
    }
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
