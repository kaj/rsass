use super::super::FunctionMap;
use super::other::{ColorOrCall, StrictChannels, WildChannels};
use super::{CallError, CheckedArg, ResolvedArgs};
use crate::Scope;
use crate::css::{CallArgs, Value};
use crate::value::{Hwba, Numeric};

pub fn register(f: &mut Scope) {
    def_va!(f, hwb(kwargs), hwb);
    def!(f, blackness(color), |s| {
        let color = s
            .get::<ColorOrCall>(name!(color))?
            .check_legacy_w("color.blackness")?;
        let hwb = color.to_hwba();
        Ok(Numeric::percentage(hwb.blackness()).into())
    });
    def!(f, whiteness(color), |s| {
        let color = s
            .get::<ColorOrCall>(name!(color))?
            .check_legacy_w("color.whiteness")?;
        let hwb = color.to_hwba();
        Ok(Numeric::percentage(hwb.whiteness()).into())
    });
}

pub fn expose(m: &Scope, global: &mut FunctionMap) {
    global.insert(name!(hwb), m.get_lfunction(&name!(hwb)));
}

fn hwb(s: &ResolvedArgs) -> Result<Value, CallError> {
    let args = s.get_map(name!(kwargs), CallArgs::from_value)?;
    let space = "hwb";
    let chan = WildChannels::from_args(space, s, &args)?;

    let chan =
        match StrictChannels::check_w(space, chan).named(name!(channels))? {
            Ok(strict) => strict,
            Err(wild) => return Ok(wild.into_call_or_str(space)),
        };

    let [Some(hue), Some(w), Some(b)] = &chan.channels[..] else {
        return Ok(chan.into_call(space));
    };
    let hue = f64::from(hue.clone().value);
    let w = f64::from(w.value.clone()) * 0.01;
    let b = f64::from(b.value.clone()) * 0.01;

    let a = chan.alpha.map(|v| f64::from(v.value)).unwrap_or(1.);

    // I don't really agree with this, but it makes tests pass.
    let hue = if w + b >= 1. { 0. } else { hue };
    let hwba = Hwba::new(hue, w, b, a);
    Ok(hwba.into())
}
