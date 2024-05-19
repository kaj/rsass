use crate::css::{CssString, IsNot, Value};
use crate::output::Format;
use crate::parser::input_span;
use crate::value::Numeric;

#[derive(Clone, Debug)]
pub(crate) enum NumOrSpecial<T = Numeric> {
    Num(T),
    Special(Value),
}

impl NumOrSpecial {
    // A `try_from` with alternative error message.
    pub fn in_calc(value: Value) -> Result<Self, String> {
        Self::try_from(value).map_err(|e| {
            format!(
                "Value {} can't be used in a calculation.",
                e.value().format(Format::introspect())
            )
        })
    }
    pub fn try_map<F, T, E>(self, f: F) -> Result<NumOrSpecial<T>, E>
    where
        F: Fn(Numeric) -> Result<T, E>,
    {
        match self {
            Self::Num(n) => Ok(NumOrSpecial::Num(f(n)?)),
            Self::Special(s) => Ok(NumOrSpecial::Special(s)),
        }
    }
}

impl TryFrom<Value> for NumOrSpecial {
    type Error = IsNot;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            v @ (Value::Call(..) | Value::BinOp(..)) => Ok(Self::Special(v)),
            Value::Literal(s) if like_call_or_num(&s) => {
                Ok(Self::Special(Value::Literal(s)))
            }
            v => Ok(Self::Num(v.try_into()?)),
        }
    }
}

fn like_call_or_num(s: &CssString) -> bool {
    s.quotes().is_none()
        && ((s.value().contains('(') && s.value().ends_with(')'))
            || crate::parser::value::number(input_span(s.value()).borrow())
                .is_ok())
}

impl From<Numeric> for NumOrSpecial {
    fn from(value: Numeric) -> Self {
        Self::Num(value)
    }
}

impl From<NumOrSpecial> for Value {
    fn from(value: NumOrSpecial) -> Self {
        match value {
            NumOrSpecial::Num(n) => n.into(),
            NumOrSpecial::Special(v) => v,
        }
    }
}
