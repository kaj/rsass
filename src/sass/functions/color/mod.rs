use super::{Error, FunctionMap};
use crate::css::{CallArgs, Value};
use crate::sass::Name;
use crate::value::{Color, Quotes, Rational};
use crate::Scope;
use num_traits::{One, Signed};
mod hsl;
mod hwb;
mod other;
mod rgb;

pub fn create_module() -> Scope {
    let mut f = Scope::new_global(Default::default());
    hsl::register(&mut f);
    hwb::register(&mut f);
    rgb::register(&mut f);
    other::register(&mut f);
    f
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    rgb::expose(m, global);
    hsl::expose(m, global);
    other::expose(m, global);
}

fn get_color(s: &Scope, name: &str) -> Result<Color, Error> {
    match s.get(name)? {
        Value::Color(col, _) => Ok(col),
        value => {
            Err(Error::bad_arg(Name::from(name), &value, "is not a color"))
        }
    }
}
fn get_rational(s: &Scope, name: &str) -> Result<Rational, Error> {
    match s.get(name)? {
        Value::Numeric(v, ..) => v.as_ratio(),
        v => Err(Error::bad_arg(Name::from(name), &v, "is not a number")),
    }
}
fn get_rational_pct(s: &Scope, name: &str) -> Result<Rational, Error> {
    match s.get(name)? {
        Value::Numeric(v, _) if v.unit.is_percent() => {
            Ok(v.as_ratio()? / 100)
        }
        Value::Numeric(v, ..) => {
            let v = v.as_ratio()?;
            Ok(if v.abs() < Rational::one() {
                v
            } else {
                v / 100
            })
        }
        v => Err(Error::bad_arg(Name::from(name), &v, "is not a number")),
    }
}

fn get_opt_rational(
    s: &Scope,
    name: &str,
) -> Result<Option<Rational>, Error> {
    match s.get(name)? {
        Value::Numeric(v, ..) => Some(v.as_ratio()).transpose(),
        Value::Null => Ok(None),
        v => Err(Error::bad_arg(Name::from(name), &v, "is not a number")),
    }
}

fn nospecial_value<F>(
    v: &Value,
    name: Name,
    f: F,
) -> Result<Option<Rational>, Error>
where
    F: Fn(&Value, Name) -> Result<Rational, Error>,
{
    match v {
        Value::Call(..) => Ok(None),
        Value::Literal(s, Quotes::None) if looks_like_call(s) => Ok(None),
        Value::BinOp(..) => Ok(None),
        h => f(h, name).map(Some),
    }
}

fn looks_like_call(s: &str) -> bool {
    s.contains('(') && s.ends_with(')')
}

fn make_call(name: &str, args: Vec<Value>) -> Value {
    Value::Call(
        name.into(),
        CallArgs::new(
            args.into_iter()
                .filter(|v| v != &Value::Null)
                .map(|v| (None, v))
                .collect(),
        ),
    )
}
