use super::{
    check, expected_to, is_not, is_special, CallError, CheckedArg,
    FunctionMap, ResolvedArgs, Scope,
};
use crate::css::{CallArgs, CssString, Value};
use crate::output::Format;
use crate::parser::input_span;
use crate::sass::Name;
use crate::value::{Number, Numeric, Quotes, Rational, Unit, UnitSet};
use std::cmp::Ordering;
use std::f64::consts::{E, PI};

/// Create the `sass:math` standard module.
///
/// Should conform to
/// [the specification](https://sass-lang.com/documentation/modules/math).
pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:math");

    def!(f, div(number1, number2), |s| {
        let a = s.get(name!(number1))?;
        let b = s.get(name!(number2))?;
        use crate::value::Operator;
        match (a, b) {
            (Value::Color(a, _), Value::Numeric(b, _)) if b.is_no_unit() => {
                let bn = b
                    .as_ratio()
                    .map_err(|e| e.to_string())
                    .named(name!(number2))?;
                Ok((a.to_rgba().as_ref() / bn).into())
            }
            (Value::Numeric(ref a, _), Value::Numeric(ref b, _)) => {
                Ok((a / b).into())
            }
            (a, b) => Ok(Value::BinOp(
                Box::new(a),
                false,
                Operator::Div,
                false,
                Box::new(b),
            )
            .format(Format::introspect())
            .to_string()
            .into()),
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
    def!(f, round(number), |s| {
        let val: Numeric = s.get(name!(number))?;
        Ok(number(val.value.round(), val.unit))
    });

    // - - - Distance Functions - - -
    def!(f, abs(number), |s| {
        let v: Numeric = s.get(name!(number))?;
        Ok(number(v.value.abs(), v.unit))
    });
    def_va!(f, hypot(number), |s| match s
        .get_map(name!(number), check::va_list)?
        .as_slice()
    {
        [Value::Numeric(v, _)] =>
            Ok(number(v.value.clone().abs(), v.unit.clone())),
        [v] => Err(is_not(v, "a number")).named(name!(number)),
        v => {
            if let Some((first, rest)) = v.split_first() {
                let first = as_numeric(first)?;
                let mut sum = f64::from(first.value.clone()).powi(2);
                let unit = first.unit.clone();
                for (i, v) in rest.iter().enumerate() {
                    let num = as_numeric(v)?;
                    let scaled = num
                        .as_unitset(&unit)
                        .ok_or_else(|| {
                            diff_units_msg(&num, &first, "numbers[1]".into())
                        })
                        .named(format!("numbers[{}]", i + 2).into())?;
                    sum += f64::from(scaled).powi(2);
                }
                Ok(number(sum.sqrt(), unit))
            } else {
                Err(CallError::msg("At least one argument must be passed."))
            }
        }
    });

    // - - - Exponential Functions - - -
    def!(f, log(number, base = b"null"), |s| {
        let num = get_unitless(s, "number")?;
        let base = s
            .get_opt_map(name!(base), check::unitless)?
            .map(Into::into)
            .unwrap_or(E);
        Ok(Value::scalar(num.log(base)))
    });
    def!(f, pow(base, exponent), |s| {
        let base = get_unitless(s, "base")?;
        let exponent = get_unitless(s, "exponent")?;
        let result =
            if exponent.is_infinite() && (base.abs() - 1.0).abs() < 1e-7 {
                f64::NAN
            } else {
                base.powf(exponent)
            };
        Ok(Value::scalar(result))
    });
    def!(f, sqrt(number), |s| {
        Ok(Value::scalar(get_unitless(s, "number")?.sqrt()))
    });

    // - - - Trigonometric Functions - - -
    def!(f, cos(number), |s| {
        Ok(Value::scalar(s.get_map(name!(number), radians)?.cos()))
    });
    def!(f, sin(number), |s| {
        Ok(Value::scalar(s.get_map(name!(number), radians)?.sin()))
    });
    def!(f, tan(number), |s| {
        let ans = s.get_map(name!(number), radians)?.tan();
        let ans = if ans.abs() > 1e15 {
            ans.signum() * f64::INFINITY
        } else {
            ans
        };
        Ok(Value::scalar(ans))
    });

    def!(f, acos(number), |s| {
        Ok(deg_value(get_unitless(s, "number")?.acos()))
    });
    def!(f, asin(number), |s| {
        Ok(deg_value(get_unitless(s, "number")?.asin()))
    });
    def!(f, atan(number), |s| {
        Ok(deg_value(get_unitless(s, "number")?.atan()))
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
                Err(format!("Must be greater than 0, was {}.", v))
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
    f
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    for (gname, lname) in &[
        // - - - Boundig Functions - - -
        (name!(ceil), name!(ceil)),
        (name!(floor), name!(floor)),
        (name!(max), name!(max)),
        (name!(min), name!(min)),
        (name!(round), name!(round)),
        // - - - Distance Functions - - -
        (name!(abs), name!(abs)),
        // Exponential and trigonometric functions are not exposed

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
}

fn radians(v: Value) -> Result<f64, String> {
    let v = Numeric::try_from(v)?;
    v.as_unit_def(Unit::Rad).map(Into::into).ok_or_else(|| {
        expected_to(&v, "have an angle unit (deg, grad, rad, turn)")
    })
}

fn get_unitless(s: &ResolvedArgs, name: &str) -> Result<f64, CallError> {
    s.get_map(name.into(), check::unitless).map(Into::into)
}

// Only used by hypot function, which treats arguments as unnamed.
fn as_numeric(v: &Value) -> Result<Numeric, CallError> {
    Numeric::try_from(v.clone()).map_err(CallError::msg)
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
                    && crate::parser::value::number(input_span(
                        s.value().as_ref(),
                    ))
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
                Err(CallError::msg(format!(
                    "{} and {} have incompatible units.",
                    a.format(Format::introspect()),
                    b.format(Format::introspect()),
                )))
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
