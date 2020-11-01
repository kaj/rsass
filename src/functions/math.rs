use super::{Error, Module, SassFunction, Scope};
use crate::css::Value;
use crate::value::{Number, Quotes, Unit};
use num_rational::Rational;
use num_traits::Signed;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

/// Create the `sass:math` standard module.
///
/// Should conform to
/// [the specification](https://sass-lang.com/documentation/modules/math).
pub fn create_module() -> Module {
    let mut f = Module::new();
    // TODO: Variable $pi and $e.

    // - - - Boundig Functions - - -
    def!(f, ceil(number), |s| {
        let (val, unit) = get_numeric(s, "number")?;
        Ok(number(val.value.ceil(), unit))
    });
    def!(f, clamp(min, number, max), |s| {
        let (min_v, min_u) = get_numeric(s, "min")?;
        let (mut num, mut u) = get_numeric(s, "number")?;
        let (max_v, max_u) = get_numeric(s, "max")?;
        if let Some(scale) = max_u.scale_to(&u) {
            if num.value >= &max_v.value * scale {
                num = max_v;
                u = max_u;
            }
        }
        if let Some(scale) = min_u.scale_to(&u) {
            if num.value <= &min_v.value * scale {
                num = min_v;
                u = min_u;
            }
        }
        Ok(number(num.value, u))
    });
    def!(f, floor(number), |s| {
        let (val, unit) = get_numeric(s, "number")?;
        Ok(number(val.value.floor(), unit))
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
        let (val, unit) = get_numeric(s, "number")?;
        Ok(number(val.value.round(), unit))
    });

    // - - - Distance Functions - - -
    def!(f, abs(number), |s| match s.get("number")? {
        Value::Numeric(v, u, ..) => Ok(number(v.value.abs(), u)),
        v => Err(Error::badarg("number", &v)),
    });
    def_va!(f, hypot(number), |s| match s.get("number")? {
        Value::List(v, _, _) => {
            if let Some((first, rest)) = v.split_first() {
                let (first, unit) = as_numeric(first)?;
                let mut sum = f64::from(first).powi(2);
                for v in rest {
                    let (vv, vu) = as_numeric(v)?;
                    if let Some(scale) = vu.scale_to(&unit) {
                        sum += f64::from(vv * scale).powi(2);
                    } else {
                        return Err(Error::badarg(&unit.to_string(), v));
                    }
                }
                Ok(float_value(sum.sqrt(), unit))
            } else {
                Err(Error::badarg("number", &Value::Null))
            }
        }
        Value::Numeric(v, u, ..) => Ok(number(v.value.abs(), u)),
        v => Err(Error::badarg("number", &v)),
    });

    // - - - Exponential Functions - - -
    def!(f, log(number, base), |s| {
        let num = match s.get("number")? {
            Value::Numeric(v, Unit::None, ..) => f64::from(v),
            v => return Err(Error::badarg("number", &v)),
        };
        let base = match s.get("base")? {
            Value::Numeric(v, Unit::None, ..) => f64::from(v),
            Value::Null => std::f64::consts::E,
            v => return Err(Error::badarg("number", &v)),
        };
        let result = num.log(base);
        Ok(float_value(result, Unit::None))
    });
    def!(f, pow(base, exponent), |s| {
        let base = match s.get("base")? {
            Value::Numeric(v, Unit::None, ..) => f64::from(v),
            v => return Err(Error::badarg("number", &v)),
        };
        let exponent = match s.get("exponent")? {
            Value::Numeric(v, Unit::None, ..) => f64::from(v),
            v => return Err(Error::badarg("number", &v)),
        };
        Ok(float_value(base.powf(exponent), Unit::None))
    });
    def!(f, sqrt(number), |s| {
        let number = match s.get("number")? {
            Value::Numeric(v, Unit::None, ..) => f64::from(v),
            v => return Err(Error::badarg("number", &v)),
        };
        Ok(float_value(number.sqrt(), Unit::None))
    });

    // - - - Trigonometric Functions - - -
    // TODO: cos, sin, tan, acos, asin, atan, atan2

    // - - - Unit Functions - - -
    def!(f, compatible(number1, number2), |s| {
        match (&s.get("number1")?, &s.get("number2")?) {
            (
                &Value::Numeric(_, ref u1, ..),
                &Value::Numeric(_, ref u2, ..),
            ) => Ok(((u1 == u2)
                || (*u1 == Unit::None || *u2 == Unit::None)
                || u2.scale_to(u1).is_some())
            .into()),
            (v1, v2) => Err(Error::badargs(&["number", "number"], &[v1, v2])),
        }
    });
    def!(f, is_unitless(number), |s| match s.get("number")? {
        Value::Numeric(_, unit, ..) => Ok((unit == Unit::None).into()),
        v => Err(Error::badarg("number", &v)),
    });
    def!(f, unit(number), |s| {
        let v = match s.get("number")? {
            Value::Numeric(_, ref unit, ..) => format!("{}", unit),
            _ => "".into(),
        };
        Ok(Value::Literal(v, Quotes::Double))
    });

    // - - - Other Functions - - -
    def!(f, percentage(number), |s| match s.get("number")? {
        Value::Numeric(val, Unit::None, _) => {
            Ok(number(val.value * 100, Unit::Percent))
        }
        v => Err(Error::badarg("number", &v)),
    });
    def!(f, random(limit), |s| match s.get("limit")? {
        Value::Null => {
            let rez = 1_000_000;
            Ok(number(Rational::new(intrand(rez), rez), Unit::None))
        }
        Value::Numeric(val, ..) => {
            let bound = val.to_integer();
            if bound > 0 {
                let res = 1 + intrand(bound);
                Ok(number(Rational::from_integer(res), Unit::None))
            } else {
                Err(Error::S("bound must be > 0".into()))
            }
        }
        v => Err(Error::badarg("number or null", &v)),
    });
    f
}

pub fn expose(math: &Module, global: &mut Module) {
    for (gname, lname) in &[
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
        global.insert(gname, math.get(lname).unwrap().clone());
    }
}

fn get_numeric(
    s: &dyn Scope,
    name: &str,
) -> Result<(Number<isize>, Unit), Error> {
    match s.get(name)? {
        Value::Numeric(v, u, ..) => Ok((v, u)),
        v => Err(Error::badarg("number", &v)),
    }
}
fn as_numeric(v: &Value) -> Result<(Number<isize>, Unit), Error> {
    match v {
        Value::Numeric(v, u, ..) => Ok((v.clone(), u.clone())),
        v => Err(Error::badarg("number", &v)),
    }
}

fn number(v: Rational, unit: Unit) -> Value {
    Value::Numeric(Number::from(v), unit, true)
}

fn float_value(val: f64, unit: Unit) -> Value {
    if let Some(result) = Rational::approximate_float(val) {
        number(result, unit)
    } else {
        if val.is_infinite() {
            if val.is_sign_negative() {
                "-Infinity".into()
            } else {
                "Infinity".into()
            }
        } else {
            val.to_string().into()
        }
    }
}

fn find_extreme(v: &[Value], pref: Ordering) -> &Value {
    match v.split_first() {
        Some((first, rest)) => {
            let second = find_extreme(rest, pref);
            match (first, second) {
                (&Value::Null, b) => b,
                (a, &Value::Null) => a,
                (
                    &Value::Numeric(ref va, ref ua, _),
                    &Value::Numeric(ref vb, ref ub, _),
                ) => {
                    if ua == &Unit::None || ua == ub || ub == &Unit::None {
                        if va.cmp(vb) == pref {
                            first
                        } else {
                            second
                        }
                    } else if let Some(scale) = ub.scale_to(ua) {
                        if va.value.cmp(&(vb.value * scale)) == pref {
                            first
                        } else {
                            second
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
    thread_rng().gen_range(0, lim)
}
