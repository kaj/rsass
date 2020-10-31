use super::{Error, Module, SassFunction};
use crate::css::Value;
use crate::value::{Number, Quotes, Unit};
use num_rational::Rational;
use num_traits::Signed;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

pub fn create_module() -> Module {
    let mut f = Module::new();
    // TODO: Variable $pi and $e.

    // - - - Boundig Functions - - -
    def!(f, ceil(number), |s| match s.get("number")? {
        Value::Numeric(v, u, ..) => Ok(number(v.value.ceil(), u)),
        v => Err(Error::badarg("number", &v)),
    });
    // TODO: clamp
    def!(f, floor(number), |s| match s.get("number")? {
        Value::Numeric(v, u, ..) => Ok(number(v.value.floor(), u)),
        v => Err(Error::badarg("number", &v)),
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
    def!(f, round(number), |s| match s.get("number")? {
        Value::Numeric(val, unit, _) => Ok(number(val.value.round(), unit)),
        v => Err(Error::badarg("number", &v)),
    });

    // - - - Distance Functions - - -
    def!(f, abs(number), |s| match s.get("number")? {
        Value::Numeric(v, u, ..) => Ok(number(v.value.abs(), u)),
        v => Err(Error::badarg("number", &v)),
    });
    // TODO: hypot

    // - - - Exponential Functions - - -
    // TODO: log, pow, sqrt

    // - - - Exponential Functions - - -
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
        // ("clamp", "clamp"),
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

fn number(v: Rational, unit: Unit) -> Value {
    Value::Numeric(Number::from(v), unit, true)
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
