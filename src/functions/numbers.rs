use super::{Error, SassFunction};
use num_rational::Rational;
use num_traits::Signed;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use unit::Unit;
use valueexpression::Value;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, abs(number), |s| match s.get("number") {
        Value::Numeric(v, u, _) => Ok(Value::Numeric(v.abs(), u, true)),
        v => Err(Error::badarg("number", &v)),
    });
    def!(f, ceil(number), |s| match s.get("number") {
        Value::Numeric(v, u, _) => Ok(Value::Numeric(v.ceil(), u, true)),
        v => Err(Error::badarg("number", &v)),
    });
    def!(f, floor(number), |s| match s.get("number") {
        Value::Numeric(v, u, _) => Ok(Value::Numeric(v.floor(), u, true)),
        v => Err(Error::badarg("number", &v)),
    });
    def!(f, percentage(number), |s| match s.get("number") {
        Value::Numeric(val, Unit::None, _) => {
            Ok(Value::Numeric(val * Rational::from_integer(100),
                              Unit::Percent,
                              true))
        }
        v => Err(Error::badarg("number", &v)),
    });
    def!(f, round(number), |s| match s.get("number") {
        Value::Numeric(val, unit, _) => {
            Ok(Value::Numeric(val.round(), unit, true))
        }
        v => Err(Error::badarg("number", &v)),
    });
    def_va!(f, max(numbers), |s| match s.get("numbers") {
        Value::List(v, _) => Ok(find_extreme(&v, Ordering::Greater).clone()),
        single_value => Ok(single_value),
    });
    def_va!(f, min(numbers), |s| match s.get("numbers") {
        Value::List(v, _) => Ok(find_extreme(&v, Ordering::Less).clone()),
        single_value => Ok(single_value),
    });
    def!(f, random(limit), |s| match s.get("limit") {
        Value::Null => {
            let rez = 1000000;
            Ok(Value::Numeric(Rational::new(intrand(rez), rez),
                              Unit::None,
                              true))
        }
        Value::Numeric(val, unit, _) => {
            let res = 1 + intrand(val.to_integer());
            Ok(Value::Numeric(Rational::from_integer(res), unit, true))
        }
        v => Err(Error::badarg("number or null", &v)),
    });
}

fn find_extreme(v: &[Value], pref: Ordering) -> &Value {
    match v.split_first() {
        Some((first, rest)) => {
            let second = find_extreme(rest, pref);
            match (first, second) {
                (&Value::Null, b) => b,
                (a, &Value::Null) => a,
                (&Value::Numeric(ref va, ref ua, _),
                 &Value::Numeric(ref vb, ref ub, _)) => {
                    if ua == &Unit::None || ua == ub || ub == &Unit::None {
                        if va.cmp(vb) == pref { first } else { second }
                    } else if ua.dimension() == ub.dimension() {
                        let sa = va * ua.scale_factor();
                        let sb = vb * ub.scale_factor();
                        if sa.cmp(&sb) == pref { first } else { second }
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
    use rand::thread_rng;
    use rand::distributions::{IndependentSample, Range};
    let between = Range::new(0, lim);
    let mut rng = thread_rng();
    between.ind_sample(&mut rng)
}
