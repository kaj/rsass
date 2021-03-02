use super::{Error, FunctionMap};
use crate::css::{CallArgs, Value};
use crate::sass::Name;
use crate::{value::Color, Scope};
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
