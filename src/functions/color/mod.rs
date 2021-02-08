use super::{make_call, Error, Module, SassFunction};
use crate::{css::Value, value::Color, Scope};

mod hsl;
mod hwb;
mod other;
mod rgb;

pub fn create_module() -> Module {
    let mut f = Module::new();
    hsl::register(&mut f);
    hwb::register(&mut f);
    rgb::register(&mut f);
    other::register(&mut f);
    f
}

pub fn expose(m: &Module, global: &mut Module) {
    rgb::expose(m, global);
    hsl::expose(m, global);
    other::expose(m, global);
}

fn get_color(s: &dyn Scope, name: &str) -> Result<Color, Error> {
    match s.get(name)? {
        Value::Color(col, _) => Ok(col),
        value => Err(Error::badarg("color", &value)),
    }
}
