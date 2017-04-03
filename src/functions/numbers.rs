use functions::{SassFunction, badarg};
use num_rational::Rational;
use num_traits::Signed;
use std::collections::BTreeMap;
use unit::Unit;
use valueexpression::Value;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    def!(f, abs(number), |s| match s.get("number") {
        Value::Numeric(v, u, _) => Ok(Value::Numeric(v.abs(), u, true)),
        v => Err(badarg("number", &v)),
    });
    def!(f, ceil(number), |s| match s.get("number") {
        Value::Numeric(v, u, _) => Ok(Value::Numeric(v.ceil(), u, true)),
        v => Err(badarg("number", &v)),
    });
    def!(f, floor(number), |s| match s.get("number") {
        Value::Numeric(v, u, _) => Ok(Value::Numeric(v.floor(), u, true)),
        v => Err(badarg("number", &v)),
    });
    def!(f, percentage(number), |s| match s.get("number") {
        Value::Numeric(val, Unit::None, _) => {
            Ok(Value::Numeric(val * Rational::from_integer(100),
                              Unit::Percent,
                              true))
        }
        v => Err(badarg("number", &v)),
    });
    def!(f, round(number), |s| match s.get("number") {
        Value::Numeric(val, unit, _) => {
            Ok(Value::Numeric(val.round(), unit, true))
        }
        v => Err(badarg("number", &v)),
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
        v => Err(badarg("number or null", &v)),
    });
}

fn intrand(lim: isize) -> isize {
    use rand::thread_rng;
    use rand::distributions::{IndependentSample, Range};
    let between = Range::new(0, lim);
    let mut rng = thread_rng();
    between.ind_sample(&mut rng)
}
