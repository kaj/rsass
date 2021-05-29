use super::{
    check, expected_to, get_checked, get_numeric, get_opt_check, Error,
    FunctionMap, Scope,
};
use crate::css::Value;
use crate::output::Format;
use crate::sass::Name;
use crate::value::{Number, Numeric, Quotes, Rational, Unit, UnitSet};
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::f64::consts::{E, PI};

/// Create the `sass:math` standard module.
///
/// Should conform to
/// [the specification](https://sass-lang.com/documentation/modules/math).
pub fn create_module() -> Scope {
    let mut f = Scope::builtin_module("sass:math");

    def!(f, div(number1, number2), |s| {
        let a = s.get("number1")?;
        let b = s.get("number2")?;
        use crate::value::Operator;
        match (a, b) {
            (Value::Color(a, _), Value::Numeric(b, _)) if b.is_no_unit() => {
                let bn = b.as_ratio()?;
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
        let val = get_numeric(s, "number")?;
        Ok(number(val.value.ceil(), val.unit))
    });
    def!(f, clamp(min, number, max), |s| {
        let min_v = get_numeric(s, "min")?;
        let check_numeric_compat_unit =
            |v: Value| -> Result<Numeric, String> {
                let v = check::numeric(v)?;
                if (v.is_no_unit() != min_v.is_no_unit())
                    || !v.unit.is_compatible(&min_v.unit)
                {
                    return Err(diff_units_msg(&v, &min_v, name!(min)));
                }
                Ok(v)
            };
        let mut num =
            get_checked(s, name!(number), check_numeric_compat_unit)?;
        let max_v = get_checked(s, name!(max), check_numeric_compat_unit)?;

        if num >= max_v {
            num = max_v;
        }
        if num <= min_v {
            num = min_v;
        }
        Ok(Value::Numeric(num, true))
    });
    def!(f, floor(number), |s| {
        let val = get_numeric(s, "number")?;
        Ok(number(val.value.floor(), val.unit))
    });
    def_va!(f, max(numbers), |s| match s.get("numbers")? {
        Value::List(v, _, _) => find_extreme(&v, Ordering::Greater),
        single_value => find_extreme(&[single_value], Ordering::Greater),
    });
    def_va!(f, min(numbers), |s| match s.get("numbers")? {
        Value::List(v, _, _) => find_extreme(&v, Ordering::Less),
        single_value => find_extreme(&[single_value], Ordering::Less),
    });
    def!(f, round(number), |s| {
        let val = get_numeric(s, "number")?;
        Ok(number(val.value.round(), val.unit))
    });

    // - - - Distance Functions - - -
    def!(f, abs(number), |s| {
        let v = get_numeric(s, "number")?;
        Ok(number(v.value.abs(), v.unit))
    });
    def_va!(f, hypot(number), |s| match s.get("number")? {
        Value::List(v, _, _) => {
            if let Some((first, rest)) = v.split_first() {
                let first = as_numeric(first)?;
                let mut sum = f64::from(first.value.clone()).powi(2);
                let unit = first.unit.clone();
                for (i, v) in rest.iter().enumerate() {
                    let num = as_numeric(v)?;
                    let scaled = num.as_unitset(&unit).ok_or_else(|| {
                        Error::BadArgument(
                            format!("numbers[{}]", i + 2).into(),
                            diff_units_msg(&num, &first, "numbers[1]".into()),
                        )
                    })?;
                    sum += f64::from(scaled).powi(2);
                }
                Ok(number(sum.sqrt(), unit))
            } else {
                Err(Error::error("At least one argument must be passed"))
            }
        }
        Value::Numeric(v, _) => Ok(number(v.value.abs(), v.unit)),
        Value::Null => {
            Err(Error::error("At least one argument must be passed"))
        }
        v => Err(Error::bad_arg(name!(number), &v, "is not a number")),
    });

    // - - - Exponential Functions - - -
    def!(f, log(number, base = b"null"), |s| {
        let num = get_unitless(s, "number")?;
        let base = get_opt_check(s, name!(base), check::unitless)?
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
        Ok(Value::scalar(get_radians(s, "number")?.cos()))
    });
    def!(f, sin(number), |s| {
        Ok(Value::scalar(get_radians(s, "number")?.sin()))
    });
    def!(f, tan(number), |s| {
        let ans = get_radians(s, "number")?.tan();
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
        let y = get_numeric(s, "y")?;
        let x = get_checked(s, name!(x), |v| {
            let v = check::numeric(v)?;
            v.as_unitset(&y.unit)
                .ok_or_else(|| diff_units_msg(&v, &y, name!(y)))
        })?;
        Ok(deg_value(f64::from(y.value).atan2(f64::from(x))))
    });

    // - - - Unit Functions - - -
    def!(f, compatible(number1, number2), |s| {
        let u1 = get_numeric(s, "number1")?.unit;
        let u2 = get_numeric(s, "number2")?.unit;
        Ok(u1.is_compatible(&u2).into())
    });
    def!(f, is_unitless(number), |s| {
        Ok((get_numeric(s, "number")?.is_no_unit()).into())
    });
    def!(f, unit(number), |s| {
        let mut unit = get_numeric(s, "number")?.unit;
        unit.simplify();
        Ok(Value::Literal(unit.to_string(), Quotes::Double))
    });

    // - - - Other Functions - - -
    def!(f, percentage(number), |s| {
        let val = get_checked(s, name!(number), check::unitless)?;
        Ok(Numeric::new(val * 100, Unit::Percent).into())
    });
    def!(f, random(limit = b"null"), |s| {
        match get_opt_check(s, name!(limit), |v| {
            let v = check::int(v)?;
            if v > 0 {
                Ok(v)
            } else {
                Err(format!("Must be greater than 0, was {}", v))
            }
        })? {
            None => {
                let rez = 1_000_000;
                Ok(Value::scalar(Rational::new(intrand(rez), rez)))
            }
            Some(bound) => Ok(Value::scalar(intrand(bound) + 1)),
        }
    });

    f.set_variable(name!(pi), Value::scalar(PI), false, false);
    f.set_variable(name!(e), Value::scalar(E), false, false);
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
        global.insert(gname.clone(), m.get_function(&lname).unwrap().clone());
    }
}

fn get_radians(s: &Scope, name: &str) -> Result<f64, Error> {
    get_checked(s, name.into(), |v| {
        let v = check::numeric(v)?;
        v.as_unit_def(Unit::Rad).map(Into::into).ok_or_else(|| {
            expected_to(&v, "have an angle unit (deg, grad, rad, turn)")
        })
    })
}

fn get_unitless(s: &Scope, name: &str) -> Result<f64, Error> {
    get_checked(s, name.into(), |v| Ok(check::unitless(v)?.into()))
}

// Only used by hypot function, which treats arguments as unnamed.
fn as_numeric(v: &Value) -> Result<Numeric, Error> {
    check::numeric(v.clone()).map_err(Error::error)
}

fn number(v: impl Into<Number>, unit: impl Into<UnitSet>) -> Value {
    Numeric::new(v.into(), unit).into()
}

/// convert f64 in radians (used by rust) to numeric Value in degrees
/// (used by sass).
fn deg_value(rad: f64) -> Value {
    number(rad.to_degrees(), Unit::Deg)
}

fn find_extreme(v: &[Value], pref: Ordering) -> Result<Value, Error> {
    find_extreme_inner(v, pref)?
        .ok_or_else(|| Error::error("At least one argument must be passed"))
        .map(Into::into)
}

fn find_extreme_inner(
    v: &[Value],
    pref: Ordering,
) -> Result<Option<Numeric>, Error> {
    if let Some((first, rest)) = v.split_first() {
        let va = check::numeric(first.clone()).map_err(Error::error)?;
        if let Some(vb) = find_extreme_inner(rest, pref)? {
            if let Some(o) = va.partial_cmp(&vb) {
                Ok(Some(if o == pref { va } else { vb }))
            } else if va.is_no_unit() || vb.is_no_unit() {
                if let Some(o) = va.value.partial_cmp(&vb.value) {
                    Ok(Some(if o == pref { va } else { vb }))
                } else {
                    Err(Error::error(format!(
                        "{} and {} could not be compared",
                        va.format(Format::introspect()),
                        vb.format(Format::introspect()),
                    )))
                }
            } else {
                Err(Error::error(format!(
                    "{} and {} have incompatible units",
                    va.format(Format::introspect()),
                    vb.format(Format::introspect()),
                )))
            }
        } else {
            Ok(Some(va))
        }
    } else {
        Ok(None)
    }
}

fn intrand(lim: i64) -> i64 {
    thread_rng().gen_range(0..lim)
}

fn diff_units_msg(
    one: &Numeric,
    other: &Numeric,
    other_name: Name,
) -> String {
    format!(
        "{} and ${}: {} have incompatible units{}",
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
