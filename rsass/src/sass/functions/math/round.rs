use super::super::CheckedArg as _;
use super::css::required_arg;
use super::{known_dim, CallArgs, CallError, NumOrSpecial, ResolvedArgs};
use crate::css::{CssString, Value};
use crate::output::Format;
use crate::sass::functions::color::eval_inner;
use crate::sass::{ArgsError, FormalArgs};
use crate::value::{Number, Numeric, Quotes};

pub fn sass_round(s: &ResolvedArgs) -> Result<Value, CallError> {
    let val: Numeric = s.get(name!(number))?;
    Ok(Numeric::new(val.value.round(), val.unit).into())
}

pub fn css_round(s: &ResolvedArgs) -> Result<Value, CallError> {
    let args = s.get_map(name!(kwargs), CallArgs::from_value)?;
    if !args.named.is_empty() {
        let fa = FormalArgs::new(vec![one_arg!(number)]);
        return sass_round(&eval_inner(&name!(round), &fa, s, args)?);
    }
    if args.positional.len() > 3 {
        return Err(CallError::msg(ArgsError::TooMany(
            3,
            args.positional.len(),
        )));
    }
    let mut args = args.positional.into_iter();
    let (strategy, number, step) = match (
        Strategy::try_from(required_arg(args.next())?),
        args.next(),
        args.next(),
    ) {
        (Ok(_), Some(num), None) if num.type_name() == "number" => {
            return Err(CallError::msg(
                "If strategy is not null, step is required.",
            ))
        }
        (Ok(s), Some(num), None) => (Some(s), num, None),
        (Ok(s), Some(num), Some(step)) => (Some(s), num, Some(step)),
        (Ok(_), None, _) => return Err(CallError::msg("xyzzy")),
        (Err(v0), Some(num), Some(step)) => {
            return if v0.type_name() == "variable" {
                Ok(Value::call("round", [v0, num, step]))
            } else {
                Err(CallError::msg(format!(
                    "{} must be either nearest, up, down or to-zero.",
                    v0.format(Format::introspect()),
                )))
            };
        }
        (Err(v0), Some(step), None) => (None, v0, Some(step)),
        (Err(arg @ Value::BinOp(_)), None, _) => {
            return Err(CallError::msg(format!(
                "Single argument {} expected to be simplifiable.",
                arg.format(Format::introspect()),
            )));
        }
        (Err(v), None, _) => (None, v, None),
    };
    let number = NumOrSpecial::try_from(number).named(name!(number))?;
    let step = step
        .map(NumOrSpecial::in_calc)
        .transpose()
        .map_err(CallError::msg)?;
    match (number, step) {
        (NumOrSpecial::Num(num), Some(NumOrSpecial::Num(step))) => {
            if let Some(step) = step.as_unitset(&num.unit) {
                Ok(real_round(strategy.unwrap_or_default(), num, Some(step)))
            } else if known_dim(&num)
                .and_then(|dim1| known_dim(&step).map(|dim2| dim1 == dim2))
                .unwrap_or(true)
            {
                Ok(fallback(strategy, num.into(), Some(step.into())))
            } else {
                Err(CallError::incompatible_values(num, step))
            }
        }
        (NumOrSpecial::Num(num), None) => {
            Ok(real_round(strategy.unwrap_or_default(), num, None))
        }
        (number, step) => Ok(fallback(strategy, number, step)),
    }
}

fn real_round(
    strategy: Strategy,
    val: Numeric,
    step: Option<Number>,
) -> Value {
    let (val, unit) = (val.value, val.unit);
    Value::from(Numeric::new(
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
    ))
}

fn fallback(
    strategy: Option<Strategy>,
    number: NumOrSpecial,
    step: Option<NumOrSpecial>,
) -> Value {
    match (strategy.map(Value::from), number, step) {
        (Some(a1), a2, Some(a3)) => {
            Value::call("round", [a1, a2.into(), a3.into()])
        }
        (Some(a1), a2, None) => Value::call("round", [a1, a2.into()]),
        (None, a1, Some(a2)) => Value::call("round", [a1, a2]),
        (None, a1, None) => Value::call("round", [a1]),
    }
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

impl TryFrom<CssString> for Strategy {
    type Error = CssString;

    fn try_from(value: CssString) -> Result<Self, Self::Error> {
        if value.quotes() != Quotes::None {
            return Err(value);
        }
        match value.value() {
            "nearest" => Ok(Self::Nearest),
            "up" => Ok(Self::Up),
            "to-zero" | "to_zero" => Ok(Self::ToZero),
            "down" => Ok(Self::Down),
            _ => Err(value),
        }
    }
}

impl TryFrom<Value> for Strategy {
    type Error = Value;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Literal(s) => s.try_into().map_err(Value::Literal),
            other => Err(other),
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
