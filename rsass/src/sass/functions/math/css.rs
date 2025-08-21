use super::{
    css_fn_arg, deg_value, diff_units_msg, expected_to, known_dim,
    known_dim_spec, num2radians, CallError, CheckedArg, FunctionMap,
    NumOrSpecial, ResolvedArgs,
};
use crate::css::{self, BinOp, Value};
use crate::sass::{ArgsError, Name};
use crate::value::{Numeric, Quotes, UnitSet};
use std::f64::consts::E;
use std::ops::Rem;

type Result<T, E = CallError> = std::result::Result<T, E>;

pub fn global(global: &mut FunctionMap) {
    def_va!(global, exp(number), |s| {
        Ok(match one_number_or_special(s, unitless)? {
            NumOrSpecial::Num(v) => Value::scalar(v.exp()),
            NumOrSpecial::Special(v) => Value::call("exp", [v]),
        })
    });
    def_va!(global, log(number), |s| {
        let mut args = args_iter(s)?;
        let num = required_arg(args.next())??;
        let base = args
            .next()
            .transpose()?
            .unwrap_or(NumOrSpecial::Num(Numeric::scalar(E)));
        check_excess_args(2, args.count())?;
        match (num, base) {
            (NumOrSpecial::Num(n), NumOrSpecial::Num(b)) => {
                let n = unitless(n).map_err(CallError::msg)?;
                let b = unitless(b).map_err(CallError::msg)?;
                Ok(Value::scalar(n.log(b)))
            }
            (n, b) => Ok(Value::call("log", [n, b])),
        }
    });
    def_va!(global, pow(number), |s| {
        match two_num_or_special(s)? {
            (NumOrSpecial::Num(base), NumOrSpecial::Num(exponent)) => {
                Ok(Value::scalar(
                    unitless(base)
                        .and_then(|b| unitless(exponent).map(|e| b.powf(e)))
                        .map_err(CallError::msg)?,
                ))
            }
            (base, exp) => Ok(Value::call("pow", [base, exp])),
        }
    });
    def_va!(global, sqrt(number), |s| {
        Ok(match one_number_or_special(s, unitless)? {
            NumOrSpecial::Num(v) => Value::scalar(v.sqrt()),
            NumOrSpecial::Special(v) => Value::call("sqrt", [v]),
        })
    });
    fn num_rad(v: Numeric) -> Result<f64> {
        num2radians(v).named(name!(number))
    }
    def_va!(global, sin(number), |s| {
        Ok(match one_number_or_special(s, Ok)? {
            NumOrSpecial::Num(v) => Value::scalar(num_rad(v)?.sin()),
            NumOrSpecial::Special(v) => Value::call("sin", [v]),
        })
    });
    def_va!(global, cos(number), |s| {
        Ok(match one_number_or_special(s, Ok)? {
            NumOrSpecial::Num(v) => Value::scalar(num_rad(v)?.cos()),
            NumOrSpecial::Special(v) => Value::call("cos", [v]),
        })
    });
    def_va!(global, tan(number), |s| {
        Ok(match one_number_or_special(s, Ok)? {
            NumOrSpecial::Num(v) => Value::scalar(num_rad(v)?.tan()),
            NumOrSpecial::Special(v) => Value::call("tan", [v]),
        })
    });
    def_va!(global, asin(number), |s| {
        Ok(match one_number_or_special(s, unitless)? {
            NumOrSpecial::Num(v) => deg_value(v.asin()),
            NumOrSpecial::Special(v) => Value::call("asin", [v]),
        })
    });
    def_va!(global, acos(number), |s| {
        Ok(match one_number_or_special(s, unitless)? {
            NumOrSpecial::Num(v) => deg_value(v.acos()),
            NumOrSpecial::Special(v) => Value::call("acos", [v]),
        })
    });
    def_va!(global, atan(number), |s| {
        Ok(match one_number_or_special(s, unitless)? {
            NumOrSpecial::Num(v) => deg_value(v.atan()),
            NumOrSpecial::Special(v) => Value::call("atan", [v]),
        })
    });
    def_va!(global, atan2(number), |s| {
        match two_num_or_special(s)? {
            (NumOrSpecial::Num(x), NumOrSpecial::Num(y)) => {
                if x.unit.is_percent() || y.unit.is_percent() {
                    Ok(Value::call("atan2", [x, y]))
                } else if let Some(x) = x.as_unitset(&y.unit) {
                    Ok(deg_value(f64::from(x).atan2(f64::from(y.value))))
                } else if !x.unit.is_known() || !y.unit.is_known() {
                    Ok(Value::call("atan2", [x, y]))
                } else {
                    Err(CallError::msg(diff_units_msg(&x, &y, name!(y))))
                }
            }
            (x, y) => Ok(Value::call("atan2", [x, y])),
        }
    });
    def_va!(global, sign(number), |s| {
        Ok(match one_number_or_special(s, Ok)? {
            NumOrSpecial::Num(v) => {
                Numeric::new(v.value.signum(), v.unit).into()
            }
            NumOrSpecial::Special(v) => Value::call("sign", [v]),
        })
    });
    def_va!(global, mod(number), |s| {
        match two_num_or_special(s)? {
            (NumOrSpecial::Num(y), NumOrSpecial::Num(x)) => {
                fn allow(u: &UnitSet) -> bool {
                    u.is_percent() || !u.is_known()
                }
                if let Some(x) = x.as_unitset(&y.unit) {
                    let unit = y.unit;
                    let m = f64::from(x);
                    let mut y = f64::from(y.value).rem(m);
                    if (y * m.signum()).is_sign_negative() {
                        if m.is_finite() {
                            y += m;
                            y = y.rem(m);
                        } else {
                            y = f64::NAN;
                        }
                    }
                    Ok(Numeric::new(y.abs() * m.signum(), unit).into())
                } else if allow(&y.unit) || allow(&x.unit) {
                    Ok(Value::call("mod", [y, x]))
                } else {
                    Err(CallError::incompatible_values(y, x))
                }
            },
            (x, y) => Ok(Value::call("mod", [x, y])),
        }
    });
    def_va!(global, rem(number), |s| {
        match two_num_or_special(s)? {
            (NumOrSpecial::Num(y), NumOrSpecial::Num(x)) => {
                fn allow(u: &UnitSet) -> bool {
                    u.is_percent() || !u.is_known()
                }
                if let Some(x) = x.as_unitset(&y.unit) {
                    let r = f64::from(y.value).rem(f64::from(x));
                    Ok(Numeric::new(r, y.unit).into())
                } else if allow(&y.unit) || allow(&x.unit) {
                    Ok(Value::call("rem", [y, x]))
                } else {
                    Err(CallError::incompatible_values(y, x))
                }
            }
            (x, y) => Ok(Value::call("rem", [x, y])),
        }
    });
    def_va!(global, clamp(number), |s| {
        let mut args = args_iter(s)?;
        let min = required_arg(args.next())??;
        let number = args.next().transpose()?;
        let max = args.next().transpose()?;
        check_excess_args(3, args.count())?;
        match (min, number, max) {
            (NumOrSpecial::Num(_min), None, _) => Err(missing_arg_nm(3, 1)),
            (NumOrSpecial::Num(_), Some(NumOrSpecial::Num(_)), None) => {
                Err(missing_arg_nm(3, 2))
            }
            (
                NumOrSpecial::Num(min),
                Some(NumOrSpecial::Num(mut number)),
                Some(NumOrSpecial::Num(max)),
            ) => {
                let min_d = known_dim(&min);
                let num_d = known_dim(&number);
                let max_d = known_dim(&max);
                if min_d.is_some() && num_d.is_some() && min_d != num_d {
                    Err(CallError::incompatible_values(min, number))
                } else if min_d.is_some() && max_d.is_some() && min_d != max_d
                {
                    Err(CallError::incompatible_values(min, max))
                } else if known_dim_spec(&min) == known_dim_spec(&number)
                    && known_dim_spec(&number) == known_dim_spec(&max)
                {
                    if number >= max {
                        number = max;
                    }
                    if number <= min {
                        number = min;
                    }
                    Ok(number.into())
                } else {
                    Ok(Value::call("clamp", [min, number, max]))
                }
            }
            (min, Some(number), Some(max)) => {
                Ok(Value::call("clamp", [min, number, max]))
            }
            (min, Some(number), None) => {
                Ok(Value::call("clamp", [min, number]))
            }
            (arg, None, _) => Ok(Value::call("clamp", [arg])),
        }
    });
    def_va!(global, calc(number), |s| {
        fn pre_calc(v: &Value) -> bool {
            match v {
                Value::Numeric(..) => true,
                Value::Call(ref name, _) => css::is_calc_name(name),
                Value::Literal(s) => s.is_css_calc(),
                Value::Paren(s) => pre_calc(s),
                _ => false,
            }
        }
        fn do_eval(v: Value) -> Result<Value> {
            match v {
                Value::Literal(s) if s.quotes() == Quotes::None => {
                    let s = s.value();
                    if s.eq_ignore_ascii_case("e") {
                        Ok(Value::scalar(std::f64::consts::E))
                    } else if s.eq_ignore_ascii_case("pi") {
                        Ok(Value::scalar(std::f64::consts::PI))
                    } else if s.eq_ignore_ascii_case("infinity") {
                        Ok(Value::scalar(f64::INFINITY))
                    } else if s.eq_ignore_ascii_case("-infinity") {
                        Ok(Value::scalar(-f64::INFINITY))
                    } else if s.eq_ignore_ascii_case("NaN") {
                        Ok(Value::scalar(f64::NAN))
                    } else if let Some(arg) = s
                        .strip_prefix("calc(")
                        .and_then(|s| s.strip_suffix(')'))
                    {
                        Ok(Value::Paren(Box::new(arg.into())))
                    } else {
                        Ok(s.into())
                    }
                }
                Value::Call(name, args) => {
                    if name == "calc" {
                        let arg = args.get_single().unwrap();
                        match do_eval(arg.clone())? {
                            Value::Literal(s) if s.is_name() => Ok(s.into()),
                            arg => Ok(Value::Paren(Box::new(arg))),
                        }
                    } else {
                        Ok(Value::Call(name, args))
                    }
                }
                Value::BinOp(op) => {
                    let a = do_eval(op.a().clone())?;
                    let b = do_eval(op.b().clone())?;
                    let op = op.op();
                    if let Ok(Some(result)) = op.eval(a.clone(), b.clone()) {
                        return Ok(result);
                    }
                    Ok(BinOp::new(a, true, op, true, b).into())
                }
                Value::Paren(v) => match v.as_ref() {
                    l @ Value::Paren(_) => Ok(l.clone()),
                    l @ Value::BinOp(..) => Ok(l.clone()),
                    _ => Ok(Value::Paren(v)),
                },
                Value::List(v, sep, bra) => {
                    fn seems_numeric(v: &Value) -> bool {
                        match v {
                            Value::Numeric(..) => true,
                            Value::Call(n, _) if n == "calc" => true,
                            Value::Literal(s) => {
                                // FIXME: This requires more ops, and
                                // are still a silly way to do it.
                                s.quotes() == Quotes::None
                                    && !s.value().starts_with('-')
                                    && !s.value().starts_with('+')
                                    && !s.value().ends_with('-')
                                    && !s.value().ends_with('+')
                            }
                            _ => false,
                        }
                    }
                    if v.windows(2).all(|p| p.iter().all(seems_numeric)) {
                        Err(CallError::msg("Missing math operator."))
                    } else {
                        Ok(Value::List(v, sep, bra))
                    }
                }
                v => NumOrSpecial::in_calc(v)
                    .map_err(CallError::msg)
                    .map(Value::from),
            }
        }
        let mut args = raw_args_iter(s)?;
        let v = required_arg(args.next())?;
        check_excess_args(1, args.count())?;

        let v = do_eval(v)?;
        if pre_calc(&v) {
            match v {
                Value::Paren(v) => Ok(*v),
                v => Ok(v),
            }
        } else {
            Ok(Value::call("calc", [css_fn_arg(v)?]))
        }
    });
}

fn one_number_or_special<F, T>(
    s: &ResolvedArgs,
    f: F,
) -> Result<NumOrSpecial<T>>
where
    F: Fn(Numeric) -> Result<T, String>,
{
    let mut args = args_iter(s)?;
    let arg = required_arg(args.next())??;
    check_excess_args(1, args.count())?;
    arg.try_map(f).map_err(CallError::msg)
}

fn raw_args_iter(s: &ResolvedArgs) -> Result<impl Iterator<Item = Value>> {
    Ok(s.get_va::<Value>(name!(number))?.into_iter())
}

fn args_iter(
    s: &ResolvedArgs,
) -> Result<impl Iterator<Item = Result<NumOrSpecial>>> {
    Ok(raw_args_iter(s)?.map(|v| v.try_into().map_err(CallError::msg)))
}

fn two_num_or_special(
    s: &ResolvedArgs,
) -> Result<(NumOrSpecial, NumOrSpecial)> {
    let mut args = args_iter(s)?;
    let arg1 = required_arg(args.next())??;
    let arg2 = args.next().ok_or_else(|| missing_arg_nm(2, 1))??;
    check_excess_args(2, args.count())?;
    Ok((arg1, arg2))
}

pub(super) fn required_arg<T>(arg: Option<T>) -> Result<T> {
    arg.ok_or_else(|| bad_argument("Missing argument."))
}

fn missing_arg_nm(n: usize, m: usize) -> CallError {
    bad_argument(format!(
        "{n} arguments required, but only {m} {} passed.",
        if m == 1 { "was" } else { "were" },
    ))
}

pub(super) fn check_excess_args(prev: usize, remain: usize) -> Result<()> {
    if remain > 0 {
        Err(bad_argument(ArgsError::TooMany(prev, prev + remain)))
    } else {
        Ok(())
    }
}

fn bad_argument<S: ToString>(msg: S) -> CallError {
    CallError::BadArgument(Name::from_static(""), msg.to_string())
}

fn unitless(val: Numeric) -> Result<f64, String> {
    if val.is_no_unit() {
        Ok(val.value.into())
    } else {
        Err(expected_to(val, "have no units"))
    }
}
