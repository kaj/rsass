use super::{is_special, CallError};
use crate::css::Value;
use crate::value::ListSeparator;

/// Channels data is either four values of parsable data, or one value
/// of "special" data, that is probably a sass list.
// only used temporarily for evaluating color function arguments.
#[allow(clippy::large_enum_variant)]
pub enum Channels {
    Data([Value; 4]),
    Special(Value),
}

impl TryFrom<Value> for Channels {
    type Error = ChaError;
    fn try_from(channels: Value) -> Result<Self, ChaError> {
        if is_special(&channels) {
            Ok(Channels::Special(channels))
        } else if let Value::List(vec, sep, bracketed) = channels.clone() {
            if bracketed {
                return Err(ChaError::Bracketed);
            }
            use crate::value::Operator::Div;
            use Value::{BinOp, Numeric};
            if sep == Some(ListSeparator::Comma) {
                return Err(ChaError::BadSep);
            }
            if sep == Some(ListSeparator::Slash) {
                match vec.as_slice() {
                    [Value::List(inner, i_s, i_b), a] => {
                        if *i_b {
                            return Err(ChaError::Bracketed);
                        }
                        if i_s.unwrap_or_default() != ListSeparator::Space {
                            return Err(ChaError::BadSep);
                        }
                        Ok(inner_channels(inner)?
                            .map(|[c1, c2, c3]| {
                                Channels::Data([c1, c2, c3, a.clone()])
                            })
                            .unwrap_or_else(|| Channels::Special(channels)))
                    }
                    [h, _a] => {
                        if is_special(h) {
                            Ok(Channels::Special(channels))
                        } else {
                            Err(ChaError::Missing1)
                        }
                    }
                    list => Err(ChaError::SlashBadNum(list.len())),
                }
            } else {
                match vec.as_slice() {
                    [r, g, BinOp(b, _, Div, _, a)] => Ok(
                        if let (b @ Numeric(..), a @ Numeric(..)) =
                            (&**b, &**a)
                        {
                            Channels::Data([
                                r.clone(),
                                g.clone(),
                                b.clone(),
                                a.clone(),
                            ])
                        } else {
                            Channels::Special(channels)
                        },
                    ),
                    other => Ok(inner_channels(other)?
                        .map(|[c1, c2, c3]| {
                            Channels::Data([c1, c2, c3, Value::Null])
                        })
                        .unwrap_or_else(|| Channels::Special(channels))),
                }
            }
        } else {
            Err(ChaError::Missing1)
        }
    }
}

fn inner_channels(
    channels: &[Value],
) -> Result<Option<[Value; 3]>, ChaError> {
    match channels {
        [h, s, l] => Ok(Some([h.clone(), s.clone(), l.clone()])),
        [] => Err(ChaError::Missing0),
        [h] => {
            if is_special(h) {
                Ok(None)
            } else {
                Err(ChaError::Missing1)
            }
        }
        [h, s] => {
            if is_special(h) || is_special(s) {
                Ok(None)
            } else {
                Err(ChaError::Missing2)
            }
        }
        list => Err(ChaError::BadNum(list.len())),
    }
}

pub enum ChaError {
    /// $channels must be an unbracketed list
    Bracketed,
    /// $channels must be a space-separated list
    BadSep,
    Missing0,
    Missing1,
    Missing2,
    /// "Only 3 elements allowed, but {} were passed",
    BadNum(usize),
    SlashBadNum(usize),
}

impl ChaError {
    pub fn conv(self, names: &[&'static str; 3]) -> CallError {
        match self {
            Self::Bracketed => {
                CallError::msg("$channels must be an unbracketed list.")
            }
            Self::BadSep => {
                CallError::msg("$channels must be a space-separated list.")
            }
            Self::Missing0 => {
                CallError::msg(format!("Missing element ${}.", names[0]))
            }
            Self::Missing1 => {
                CallError::msg(format!("Missing element ${}.", names[1]))
            }
            Self::Missing2 => {
                CallError::msg(format!("Missing element ${}.", names[2]))
            }
            Self::BadNum(n) => CallError::msg(format!(
                "Only 3 elements allowed, but {} were passed.",
                n
            )),
            Self::SlashBadNum(n) => CallError::msg(format!(
                "Only 2 slash-separated elements allowed, but {} {} passed.",
                n,
                if n == 1 { "was" } else { "were" },
            )),
        }
    }
}
