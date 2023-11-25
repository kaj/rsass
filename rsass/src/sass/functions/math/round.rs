use super::{
    diff_units_msg2, known_dim, number, CallArgs, CallError, ResolvedArgs,
};
use crate::css::{CssString, Value};
use crate::output::Format;
use crate::sass::functions::color::eval_inner;
use crate::sass::FormalArgs;
use crate::value::{Number, Numeric, Quotes};

pub fn sass_round(s: &ResolvedArgs) -> Result<Value, CallError> {
    let val: Numeric = s.get(name!(number))?;
    Ok(number(val.value.round(), val.unit))
}

pub fn css_round(s: &ResolvedArgs) -> Result<Value, CallError> {
    let args = s.get_map(name!(kwargs), CallArgs::from_value)?;
    if !args.named.is_empty() {
        let fa = FormalArgs::new(vec![one_arg!(number)]);
        return sass_round(&eval_inner(&name!(round), &fa, s, args)?);
    }
    if args.positional.len() > 3 {
        return Err(CallError::msg(format!(
            "Only 3 arguments allowed, but {} were passed.",
            args.positional.len(),
        )));
    }
    let mut args = args.positional.into_iter();
    let (strategy, number, step) =
        match (args.next(), args.next(), args.next()) {
            (Some(v0), Some(v1), v2) => {
                match (Strategy::try_from(&v0), v1, v2) {
                    (Ok(_), num, None) if num.type_name() == "number" => {
                        return Err(CallError::msg(
                            "If strategy is not null, step is required.",
                        ))
                    }
                    (Ok(s), num, None) => (Some(s), num, None),
                    (Ok(s), num, Some(step)) => (Some(s), num, Some(step)),
                    (Err(()), num, Some(step)) => {
                        return if v0.type_name() == "variable" {
                            fallback(Some(v0), num, Some(step))
                        } else {
                            Err(CallError::msg(format!(
                        "{} must be either nearest, up, down or to-zero.",
                        v0.format(Format::introspect()),
                    )))
                        };
                    }
                    (Err(()), step, None) => (None, v0, Some(step)),
                }
            }
            (Some(v), None, _) => (None, v, None),
            (None, ..) => {
                return Err(CallError::msg("Missing argument."));
            }
        };
    real_round(strategy.unwrap_or_default(), &number, step.as_ref())
        .unwrap_or_else(|| fallback(strategy.map(Value::from), number, step))
}

fn real_round(
    strategy: Strategy,
    num: &Value,
    step: Option<&Value>,
) -> Option<Result<Value, CallError>> {
    let val = Numeric::try_from(num.clone()).ok()?;
    let step = match step.cloned().map(Numeric::try_from) {
        Some(Ok(v)) => {
            if let Some(step) = v.as_unitset(&val.unit) {
                Some(step)
            } else if known_dim(&val)
                .and_then(|dim1| known_dim(&v).map(|dim2| dim1 == dim2))
                .unwrap_or(true)
            {
                return None;
            } else {
                return Some(Err(CallError::msg(diff_units_msg2(&val, &v))));
            }
        }
        Some(Err(_)) => return None,
        None => None,
    };
    let (val, unit) = (val.value, val.unit);
    Some(Ok(number(
        if let Some(step) = step {
            if step.is_finite() {
                if (strategy == Strategy::ToZero) && step.is_negative() {
                    (&val / &step).abs().ceil() * step.abs() * val.signum()
                } else {
                    strategy.apply(val / step.abs()) * step.abs()
                }
            } else if val.is_finite() {
                if strategy == Strategy::Up && val > 0.into() {
                    Number::from(f64::INFINITY)
                } else if strategy == Strategy::Down && val < 0.into() {
                    Number::from(f64::NEG_INFINITY)
                } else {
                    val.signum() / Number::from(f64::INFINITY)
                }
            } else {
                Number::from(f64::NAN)
            }
        } else {
            strategy.apply(val)
        },
        unit,
    )))
}

fn fallback(
    strategy: Option<Value>,
    number: Value,
    step: Option<Value>,
) -> Result<Value, CallError> {
    let mut args = Vec::new();
    let is_single = strategy.is_none() && step.is_none();
    if let Some(v) = strategy {
        args.push(v);
    }
    if is_single && matches!(&number, Value::BinOp(_)) {
        return Err(CallError::msg(format!(
            "Single argument {} expected to be simplifiable.",
            number.format(Format::introspect()),
        )));
    }
    args.push(super::css_fn_arg(number)?);
    if let Some(step) = step {
        args.push(super::css_fn_arg(step)?);
    }
    Ok(Value::Call("round".into(), CallArgs::from_list(args)))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Strategy {
    Nearest,
    Up,
    ToZero,
    Down,
}

impl Strategy {
    fn apply(&self, val: Number) -> Number {
        match self {
            Self::Nearest => val.round(),
            Self::Up => val.ceil(),
            Self::ToZero => val.trunc(),
            Self::Down => val.floor(),
        }
    }
}

impl Default for Strategy {
    fn default() -> Self {
        Self::Nearest
    }
}

impl TryFrom<&CssString> for Strategy {
    type Error = ();

    fn try_from(value: &CssString) -> Result<Self, Self::Error> {
        if value.quotes() != Quotes::None {
            return Err(());
        }
        match value.value() {
            "nearest" => Ok(Self::Nearest),
            "up" => Ok(Self::Up),
            "to-zero" | "to_zero" => Ok(Self::ToZero),
            "down" => Ok(Self::Down),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Value> for Strategy {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Value::Literal(s) = value {
            s.try_into()
        } else {
            Err(())
        }
    }
}

impl From<Strategy> for Value {
    fn from(value: Strategy) -> Self {
        Value::from(match value {
            Strategy::Nearest => "nearest",
            Strategy::Up => "up",
            Strategy::ToZero => "to-zero",
            Strategy::Down => "down",
        })
    }
}
