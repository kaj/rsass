use super::{make_call, Error, Module, SassFunction};
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
