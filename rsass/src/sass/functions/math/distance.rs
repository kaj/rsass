use super::super::color::eval_inner;
use super::super::{CallError, CheckedArg, FunctionMap, ResolvedArgs};
use super::css::{check_excess_args, required_arg};
use super::{css_fn_arg, diff_units_msg, NumOrSpecial};
use crate::css::{is_not, CallArgs, Value};
use crate::value::Numeric;
use crate::Scope;

pub fn in_module(module: &mut Scope) {
    def!(module, abs(number), sass_abs);
    def_va!(module, hypot(number), |s| {
        hypot(&s.get_va(name!(number))?)
    });
}

pub fn global(global: &mut FunctionMap) {
    def_va!(global, abs(args), |s| {
        let args = s.get_map(name!(args), CallArgs::from_value)?;
        if !args.named.is_empty() {
            let fa = FormalArgs::new(vec![one_arg!(number)]);
            return sass_abs(&eval_inner(&name!(abs), &fa, s, args)?);
        }
        let mut args = args.positional.into_iter();
        let arg = required_arg(args.next())?;
        check_excess_args(1, args.count())?;
        match NumOrSpecial::try_from(arg).named(name!(number))? {
            NumOrSpecial::Num(v) => {
                Ok(Numeric::new(v.value.abs(), v.unit).into())
            }
            NumOrSpecial::Special(arg) => Ok(Value::call("abs", [arg])),
        }
    });
    def_va!(global, hypot(number), |s| {
        let args = s.get_va(name!(number))?;
        match hypot(&args) {
            Ok(value) => Ok(value),
            Err(_) => {
                if args.is_empty() {
                    Err(CallError::msg("Missing argument."))
                } else {
                    let args = args
                        .into_iter()
                        .map(css_fn_arg)
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(Value::call("hypot", args))
                }
            }
        }
    });
}

fn sass_abs(s: &ResolvedArgs) -> Result<Value, CallError> {
    let v: Numeric = s.get(name!(number))?;
    Ok(Numeric::new(v.value.abs(), v.unit).into())
}

fn hypot(args: &[Value]) -> Result<Value, CallError> {
    match args {
        [Value::Numeric(v, _)] => {
            Ok(Numeric::new(v.value.clone().abs(), v.unit.clone()).into())
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
                Ok(Numeric::new(sum.sqrt(), unit).into())
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
