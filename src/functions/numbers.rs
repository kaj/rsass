use super::{Error, SassFunction};
use css::Value;
use num_rational::Rational;
use num_traits::Signed;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use value::Unit;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, abs(number), |s| match s.get("number") {
        Value::Numeric(v, u, ..) => Ok(number(v.abs(), u)),
        v => Err(Error::badarg("number", &v)),
    });
    def!(f, ceil(number), |s| match s.get("number") {
        Value::Numeric(v, u, ..) => Ok(number(v.ceil(), u)),
        v => Err(Error::badarg("number", &v)),
    });
    def!(f, floor(number), |s| match s.get("number") {
        Value::Numeric(v, u, ..) => Ok(number(v.floor(), u)),
        v => Err(Error::badarg("number", &v)),
    });
    def!(f, percentage(number), |s| match s.get("number") {
        Value::Numeric(val, Unit::None, ..) => {
            Ok(number(val * Rational::from_integer(100), Unit::Percent))
        }
        v => Err(Error::badarg("number", &v)),
    });
    def!(f, round(number), |s| match s.get("number") {
        Value::Numeric(val, unit, ..) => Ok(number(val.round(), unit)),
        v => Err(Error::badarg("number", &v)),
    });
    def_va!(f, max(numbers), |s| match s.get("numbers") {
        Value::List(v, _, _) => Ok(find_extreme(&v, Ordering::Greater).clone()),
        single_value => Ok(single_value),
    });
    def_va!(f, min(numbers), |s| match s.get("numbers") {
        Value::List(v, _, _) => Ok(find_extreme(&v, Ordering::Less).clone()),
        single_value => Ok(single_value),
    });
    def!(f, random(limit), |s| match s.get("limit") {
        Value::Null => {
            let rez = 1000000;
            Ok(number(Rational::new(intrand(rez), rez), Unit::None))
        }
        Value::Numeric(val, unit, ..) => {
            let res = 1 + intrand(val.to_integer());
            Ok(number(Rational::from_integer(res), unit))
        }
        v => Err(Error::badarg("number or null", &v)),
    });
}

fn number(v: Rational, unit: Unit) -> Value {
    Value::Numeric(v, unit, false, true)
}

fn find_extreme(v: &[Value], pref: Ordering) -> &Value {
    match v.split_first() {
        Some((first, rest)) => {
            let second = find_extreme(rest, pref);
            match (first, second) {
                (&Value::Null, b) => b,
                (a, &Value::Null) => a,
                (&Value::Numeric(ref va, ref ua, ..),
                 &Value::Numeric(ref vb, ref ub, ..)) => {
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
