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

pub fn expose(meta: &Module, global: &mut Module) {
    rgb::expose(meta, global);
    hsl::expose(meta, global);
    other::expose(meta, global);
}
