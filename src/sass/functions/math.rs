use super::{
    check, get_checked, get_numeric, get_opt_check, Error, FunctionMap, Scope,
};
use crate::css::Value;
use crate::output::Format;
use crate::value::{Number, Numeric, Quotes, Unit, UnitSet};
use num_rational::Rational;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::f64::consts::{E, PI};

/// Create the `sass:math` standard module.
///
/// Should conform to
/// [the specification](https://sass-lang.com/documentation/modules/math).
pub fn create_module() -> Scope {
    let f = Scope::builtin_module("sass:math");

    // - - - Boundig Functions - - -
    def!(f, ceil(number), |s| {
        let val = get_numeric(s, "number")?;
        Ok(number(val.value.ceil(), val.unit))
    });
    def!(f, clamp(min, number, max), |s| {
        let min_v = get_numeric(s, "min")?;
        let mut num = get_numeric(s, "number")?;
        let max_v = get_numeric(s, "max")?;
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
        Value::List(v, _, _) => {
            Ok(find_extreme(&v, Ordering::Greater).clone())
        }
        single_value => Ok(single_value),
    });
    def_va!(f, min(numbers), |s| match s.get("numbers")? {
        Value::List(v, _, _) => Ok(find_extreme(&v, Ordering::Less).clone()),
        single_value => Ok(single_value),
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
                    let scaled = num
                        .as_unitset(&unit)
                        .ok_or_else(|| Error::S(format!(
                            "Error: $numbers[{}]: {} and $numbers[1]: {} have incompatible units{}.",
                            i + 2,
                            v.format(Format::introspect()),
                            first.format(Format::introspect()),
                            if unit.is_none() || num.is_no_unit() {
                                " (one has units and the other doesn't)"
                            } else {
                                ""
                            }
                        )))?;
                    sum += f64::from(scaled).powi(2);
                }
                Ok(number(sum.sqrt(), unit))
            } else {
                Err(Error::S(
                    "Error: At least one argument must be passed.".into(),
                ))
            }
        }
        Value::Numeric(v, _) => Ok(number(v.value.abs(), v.unit)),
        Value::Null => {
            Err(Error::S(
                "Error: At least one argument must be passed.".into(),
            ))
        }
        v => Err(Error::bad_arg(name!(number), &v, "is not a number")),
    });

    // - - - Exponential Functions - - -
    def!(f, log(number, base = b"null"), |s| {
        let num = get_unitless(s, "number")?;
        let base = get_base(s)?;
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
        let x = get_numeric(s, "x")?;
        let x = x.as_unitset(&y.unit).unwrap_or(x.value);
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
        let v = match s.get("number")? {
            Value::Numeric(v, _) => format!("{}", v.unit),
            _ => "".into(),
        };
        Ok(Value::Literal(v, Quotes::Double))
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
            format!(
                "Expected {} to have an angle unit (deg, grad, rad, turn)",
                v.format(Format::introspect())
            )
        })
    })
}

fn get_unitless(s: &Scope, name: &str) -> Result<f64, Error> {
    get_checked(s, name.into(), |v| Ok(check::unitless(v)?.into()))
}

fn get_base(s: &Scope) -> Result<f64, Error> {
    Ok(
        get_opt_check(s, name!(base), |v| Ok(check::unitless(v)?.into()))?
            .unwrap_or(std::f64::consts::E),
    )
}

// Only used by hypot function, which treats arguments as unnamed.
fn as_numeric(v: &Value) -> Result<Numeric, Error> {
    check::numeric(v.clone()).map_err(|e| Error::S(format!("Error: {}.", e)))
}

fn number(v: impl Into<Number>, unit: impl Into<UnitSet>) -> Value {
    Numeric::new(v.into(), unit).into()
}

/// convert f64 in radians (used by rust) to numeric Value in degrees
/// (used by sass).
fn deg_value(rad: f64) -> Value {
    number(rad.to_degrees(), Unit::Deg)
}

fn find_extreme(v: &[Value], pref: Ordering) -> &Value {
    match v.split_first() {
        Some((first, rest)) => {
            let second = find_extreme(rest, pref);
            match (first, second) {
                (&Value::Null, b) => b,
                (a, &Value::Null) => a,
                (&Value::Numeric(ref va, _), &Value::Numeric(ref vb, _)) => {
                    if let Some(o) = va.partial_cmp(vb) {
                        if o == pref {
                            first
                        } else {
                            second
                        }
                    } else if va.is_no_unit() || vb.is_no_unit() {
                        if let Some(o) = va.value.partial_cmp(&vb.value) {
                            if o == pref {
                                first
                            } else {
                                second
                            }
                        } else {
                            &NULL_VALUE
                        }
                    } else {
                        &NULL_VALUE
                    }
                }
                (_, _) => &NULL_VALUE,
            }
        }
        None => &NULL_VALUE,
    }
}

static NULL_VALUE: Value = Value::Null;

fn intrand(lim: isize) -> isize {
    thread_rng().gen_range(0..lim)
}