use formalargs::FormalArgs;
use functions::SassFunction;
use num_rational::Rational;
use num_traits::Signed;
use std::collections::BTreeMap;
use valueexpression::{Unit, Value};
use variablescope::Scope;

pub fn register(f: &mut BTreeMap<&'static str, SassFunction>) {
    f.insert("abs",
             func!((number), |s: &Scope| {
        match s.get("number") {
            Value::Numeric(v, u, _) => Value::Numeric(v.abs(), u, true),
            v => panic!("abs function needs a number, got {}", v),
        }
    }));
    f.insert("ceil",
             func!((number), |s| {
        match s.get("number") {
            Value::Numeric(v, u, _) => Value::Numeric(v.ceil(), u, true),
            v => panic!("ceil function needs a number, got {}", v),
        }
    }));
    f.insert("floor",
             func!((number), |s| {
        match s.get("number") {
            Value::Numeric(v, u, _) => Value::Numeric(v.floor(), u, true),
            v => panic!("floor function needs a number, got {}", v),
        }
    }));
    f.insert("percentage",
             func!((number), |s| {
        match s.get("number") {
            Value::Numeric(val, Unit::None, _) => {
                Value::Numeric(val * Rational::from_integer(100),
                               Unit::Percent,
                               true)
            }
            v => panic!("percentage needs unitless number, got {}", v),
        }
    }));
    f.insert("round",
             func!((number), |s| {
        match s.get("number") {
            Value::Numeric(val, unit, _) => {
                Value::Numeric(val.round(), unit, true)
            }
            v => panic!("round function needs a number, got {}", v),
        }
    }));
    f.insert("random",
             func!((limit), |s| {
        match s.get("limit") {
            Value::Null => {
                let rez = 1000000;
                Value::Numeric(Rational::new(intrand(rez), rez),
                               Unit::None,
                               true)
            }
            Value::Numeric(val, unit, _) => {
                let res = 1 + intrand(val.to_integer());
                Value::Numeric(Rational::from_integer(res), unit, true)
            }
            v => panic!("random needs a unitless number, got {}", v),
        }
    }));
}

fn intrand(lim: isize) -> isize {
    use rand::thread_rng;
    use rand::distributions::{IndependentSample, Range};
    let between = Range::new(0, lim);
    let mut rng = thread_rng();
    between.ind_sample(&mut rng)
}
