use super::{Error, Module, SassFunction, Scope};
use crate::css::Value;
use crate::value::{Number, Numeric, Quotes, Unit};
use num_rational::Rational;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::f64::consts::{E, PI};

/// Create the `sass:math` standard module.
///
/// Should conform to
/// [the specification](https://sass-lang.com/documentation/modules/math).
pub fn create_module() -> Module {
    let mut f = Module::new();
    // TODO: Variable $pi and $e.

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
                let mut sum = f64::from(first.value).powi(2);
                let unit = first.unit;
                for v in rest {
                    let scaled = as_numeric(v)?
                        .as_unit(unit.clone())
                        .ok_or_else(|| Error::badarg(&unit.to_string(), v))?;
                    sum += f64::from(scaled).powi(2);
                }
                Ok(number(sum.sqrt(), unit))
            } else {
                Err(Error::badarg("number", &Value::Null))
            }
        }
        Value::Numeric(v, _) => Ok(number(v.value.abs(), v.unit)),
        v => Err(Error::badarg("number", &v)),
    });

    // - - - Exponential Functions - - -
    def!(f, log(number, base), |s| {
        let num = get_unitless(s, "number")?;
        let base = get_unitless_or(s, "base", std::f64::consts::E)?;
        let result = num.log(base);
        Ok(number(result, Unit::None))
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
        Ok(number(result, Unit::None))
    });
    def!(f, sqrt(number), |s| {
        Ok(number(get_unitless(s, "number")?.sqrt(), Unit::None))
    });

    // - - - Trigonometric Functions - - -
    def!(f, cos(number), |s| {
        Ok(number(get_radians(s, "number")?.cos(), Unit::None))
    });
    def!(f, sin(number), |s| {
        Ok(number(get_radians(s, "number")?.sin(), Unit::None))
    });
    def!(f, tan(number), |s| {
        let ans = get_radians(s, "number")?.tan();
        let ans = if ans.abs() > 1e15 {
            ans.signum() * f64::INFINITY
        } else {
            ans
        };
        Ok(number(ans, Unit::None))
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
        let x = x.as_unit(y.unit).unwrap_or(x.value);
        Ok(deg_value(f64::from(y.value).atan2(f64::from(x))))
    });

    // - - - Unit Functions - - -
    def!(f, compatible(number1, number2), |s| {
        let u1 = get_numeric(s, "number1")?.unit;
        let u2 = get_numeric(s, "number2")?.unit;
        let ans = u1 == Unit::None
            || u2 == Unit::None
            || u1.dimension() == u2.dimension();
        Ok(ans.into())
    });
    def!(f, is_unitless(number), |s| {
        Ok((get_numeric(s, "number")?.unit == Unit::None).into())
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
        let val = s.get("number")?;
        let val = val
            .clone()
            .numeric_value()
            .and_then(|v| v.as_unit(Unit::Percent).ok_or(val))
            .map_err(|v| Error::badarg("number", &v))?;
        Ok(Numeric::new(val, Unit::Percent).into())
    });
    def!(f, random(limit), |s| match s.get("limit")? {
        Value::Null => {
            let rez = 1_000_000;
            Ok(number(Rational::new(intrand(rez), rez), Unit::None))
        }
        Value::Numeric(val, _) => {
            let bound = val
                .value
                .to_integer()
                .ok_or_else(|| Error::S("bound must be > 0".into()))?;
            if bound > 0 {
                let res = 1 + intrand(bound);
                Ok(number(Rational::from_integer(res), Unit::None))
            } else {
                Err(Error::S("bound must be > 0".into()))
            }
        }
        v => Err(Error::badarg("number or null", &v)),
    });

    f.set_variable(name!(pi), number(PI, Unit::None));
    f.set_variable(name!(e), number(E, Unit::None));
    f
}

pub fn expose(m: &Module, global: &mut Module) {
    for &(gname, lname) in &[
        // - - - Boundig Functions - - -
        ("ceil", "ceil"),
        ("floor", "floor"),
        ("max", "max"),
        ("min", "min"),
        ("round", "round"),
        // - - - Distance Functions - - -
        ("abs", "abs"),
        // Exponential and trigonometric functions are not exposed

        // - - - Unit Functions - - -
        ("comparable", "compatible"),
        ("unitless", "is_unitless"),
        ("unit", "unit"),
        // - - - Other Functions - - -
        ("percentage", "percentage"),
        ("random", "random"),
    ] {
        global.expose(gname, m, lname);
    }
}

fn get_numeric(s: &dyn Scope, name: &str) -> Result<Numeric, Error> {
    s.get(name)?
        .numeric_value()
        .map_err(|v| Error::badarg("number", &v))
}

fn get_radians(s: &dyn Scope, name: &str) -> Result<f64, Error> {
    let v = get_numeric(s, name)?;
    if v.unit == Unit::None {
        Ok(f64::from(v.value))
    } else if let Some(scaled) = v.as_unit(Unit::Rad) {
        Ok(f64::from(scaled))
    } else {
        Err(Error::badarg("angle", &v.into()))
    }
}

fn get_unitless(s: &dyn Scope, name: &str) -> Result<f64, Error> {
    let v = get_numeric(s, name)?;
    if v.unit == Unit::None {
        Ok(f64::from(v.value))
    } else {
        Err(Error::badarg("unitless", &v.into()))
    }
}

fn get_unitless_or(
    s: &dyn Scope,
    name: &str,
    default: f64,
) -> Result<f64, Error> {
    match s.get(name)? {
        Value::Numeric(v, _) => {
            if v.unit == Unit::None {
                Ok(f64::from(v.value))
            } else {
                Err(Error::badarg("unitless", &v.into()))
            }
        }
        Value::Null => Ok(default),
        v => Err(Error::badarg("number", &v)),
    }
}

fn as_numeric(v: &Value) -> Result<Numeric, Error> {
    match v {
        Value::Numeric(v, _) => Ok(v.clone()),
        v => Err(Error::badarg("number", &v)),
    }
}

fn number<T: Into<Number>>(v: T, unit: Unit) -> Value {
    Value::Numeric(Numeric::new(v.into(), unit), true)
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
                    } else if va.unit == Unit::None || vb.unit == Unit::None {
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
