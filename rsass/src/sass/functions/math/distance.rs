use super::super::{check, CallError, FunctionMap, ResolvedArgs};
use super::{diff_units_msg, number};
use crate::css::{is_not, CallArgs, Value};
use crate::sass::functions::color::eval_inner;
use crate::sass::functions::CheckedArg;
use crate::value::Numeric;
use crate::{sass::functions::css_fn_arg, Scope};

pub fn in_module(module: &mut Scope) {
    def!(module, abs(number), sass_abs);
    def_va!(module, hypot(number), |s| {
        hypot(&s.get_map(name!(number), check::va_list)?)
    });
}

pub fn global(global: &mut FunctionMap) {
    def_va!(global, abs(args), |s| {
        let args = s.get_map(name!(args), CallArgs::from_value)?;
        if !args.named.is_empty() {
            let fa = FormalArgs::new(vec![one_arg!(number)]);
            return sass_abs(&eval_inner(&name!(abs), &fa, s, args)?);
        }
        if args.positional.len() > 1 {
            return Err(CallError::msg(format!(
                "Only 1 argument allowed, but {} were passed.",
                args.positional.len(),
            )));
        }
        let arg = args
            .positional
            .into_iter()
            .next()
            .ok_or_else(|| CallError::msg("Missing argument."))?;
        Numeric::try_from(arg)
            .map(|v| number(v.value.abs(), v.unit))
            .or_else(|e| match css_fn_arg(e.value().clone()) {
                Ok(v) => {
                    Ok(Value::Call("abs".into(), CallArgs::from_single(v)))
                }
                Err(_) => Err(e.to_string()).named(name!(number)),
            })
    });
    def_va!(global, hypot(number), |s| {
        let args = s.get_map(name!(number), check::va_list)?;
        match hypot(&args) {
            Ok(value) => Ok(value),
            Err(_) => {
                if args.is_empty() {
                    Err(CallError::msg("Missing argument."))
                } else {
                    let args = args
                        .into_iter()
                        .map(css_fn_arg)
                        .collect::<Result<_, _>>()?;
                    Ok(Value::Call("hypot".into(), CallArgs::from_list(args)))
                }
            }
        }
    });
}

fn sass_abs(s: &ResolvedArgs) -> Result<Value, CallError> {
    let v: Numeric = s.get(name!(number))?;
    Ok(number(v.value.abs(), v.unit))
}

fn hypot(args: &[Value]) -> Result<Value, CallError> {
    match args {
        [Value::Numeric(v, _)] => {
            Ok(number(v.value.clone().abs(), v.unit.clone()))
        }
        [v] => Err(is_not(v, "a number")).named(name!(number)),
        v => {
            if let Some((first, rest)) = v.split_first() {
                let first = as_numeric(first)?;
                let mut sum = f64::from(first.value.clone()).powi(2);
                let unit = first.unit.clone();
                if first.unit.is_percent() {
                    return Err(CallError::msg("Percentage not allowed"));
                }
                for (i, v) in rest.iter().enumerate() {
                    let v = as_numeric(v)?;
                    let scaled = v
                        .as_unitset(&unit)
                        .ok_or_else(|| {
                            diff_units_msg(&v, &first, "numbers[1]".into())
                        })
                        .named(format!("numbers[{}]", i + 2).into())?;
                    sum += f64::from(scaled).powi(2);
                }
                Ok(number(sum.sqrt(), unit))
            } else {
                Err(CallError::msg("At least one argument must be passed."))
            }
        }
    }
}

// Only used by hypot function, which treats arguments as unnamed.
fn as_numeric(v: &Value) -> Result<Numeric, CallError> {
    Numeric::try_from(v.clone()).map_err(CallError::msg)
}
