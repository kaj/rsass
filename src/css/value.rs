use crate::css::CallArgs;
use crate::error::Error;
use crate::functions::SassFunction;
use crate::ordermap::OrderMap;
use crate::output::{Format, Formatted};
use crate::value::{ListSeparator, Number, Operator, Quotes, Rgba, Unit};
use num_bigint::BigInt;
use num_rational::Rational;
use std::convert::TryFrom;

/// A css value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    /// A special kind of escape.  Only really used for !important.
    Bang(String),
    /// An function call that was not evaluated.
    Call(String, CallArgs),
    /// A (callable?) function.
    Function(String, Option<SassFunction>),
    Literal(String, Quotes),
    /// A comma- or space separated list of values, with or without brackets.
    List(Vec<Value>, ListSeparator, bool),
    /// A Numeric value is a rational value with a Unit (which may be
    /// Unit::None) and flags.
    ///
    /// The boolean flag is true for calculated values and false for
    /// literal values.
    Numeric(Number<isize>, Unit, bool),
    /// A rational value with a Unit and flags, similar to Numeric, but with
    /// support for arbitrarily large numbers.
    NumericBig(Number<BigInt>, Unit, bool),
    Color(Rgba, Option<String>),
    Null,
    True,
    False,
    /// A binary operation, two operands and an operator.
    /// The booleans represents possible whitespace.
    BinOp(Box<Value>, bool, Operator, bool, Box<Value>),
    UnaryOp(Operator, Box<Value>),
    Map(OrderMap<Value, Value>),
    /// A unicode range for font selections. U+NN, U+N?, U+NN-MM.
    /// The string is the entire value, including the "U+" tag.
    UnicodeRange(String),
    Paren(Box<Value>),
}

impl Value {
    pub fn scalar<T: Into<Number<isize>>>(v: T) -> Self {
        Value::Numeric(v.into(), Unit::None, false)
    }
    pub fn bool(v: bool) -> Self {
        if v {
            Value::True
        } else {
            Value::False
        }
    }
    pub fn black() -> Self {
        Value::Color(Rgba::from_rgb(0, 0, 0), Some("black".into()))
    }
    pub fn rgba(r: Rational, g: Rational, b: Rational, a: Rational) -> Self {
        Value::Color(Rgba::new(r, g, b, a), None)
    }
    pub fn hsla(h: Rational, s: Rational, l: Rational, a: Rational) -> Self {
        Value::Color(Rgba::from_hsla(h / 360, s, l, a), None)
    }

    pub fn type_name(&self) -> &'static str {
        match *self {
            Value::Color(..) => "color",
            Value::Literal(..) => "string",
            Value::Numeric(..) => "number",
            Value::List(..) => "list",
            Value::Function(..) => "function",
            Value::True | Value::False => "bool",
            Value::Null => "null",
            _ => "unknown",
        }
    }

    pub fn is_calculated(&self) -> bool {
        match *self {
            Value::Numeric(.., calculated) => calculated,
            Value::Color(_, None) => true,
            _ => false,
        }
    }

    pub fn into_calculated(self) -> Self {
        match self {
            Value::Numeric(num, unit, _) => Value::Numeric(num, unit, true),
            Value::List(v, sep, bracketed) => Value::List(
                v.into_iter().map(|i| i.into_calculated()).collect(),
                sep,
                bracketed,
            ),
            other => other,
        }
    }

    /// All values other than `False` and `Null` should be considered true.
    pub fn is_true(&self) -> bool {
        match *self {
            Value::False | Value::Null => false,
            _ => true,
        }
    }

    pub fn is_null(&self) -> bool {
        match *self {
            Value::Null => true,
            Value::List(ref list, _, false) => {
                list.iter().all(|v| v.is_null())
            }
            Value::Literal(ref s, Quotes::None) if s.is_empty() => true,
            Value::Paren(ref v) => v.is_null(),
            _ => false,
        }
    }

    pub fn integer_value(&self) -> Result<isize, Error> {
        match self {
            &Value::Numeric(ref num, ..) if num.is_integer() => {
                Ok(num.to_integer())
            }
            v => Err(Error::bad_value("integer", v)),
        }
    }

    pub fn unquote(self) -> Value {
        match self {
            Value::Literal(s, _quotes) => Value::Literal(s, Quotes::None),
            Value::List(list, s, b) => Value::List(
                list.into_iter().map(|v| v.unquote()).collect(),
                s,
                b,
            ),
            Value::Paren(v) => *v,
            v => v,
        }
    }
    pub fn unrequote(&self) -> Value {
        match *self {
            Value::Literal(ref s, Quotes::None) => {
                Value::Literal(s.clone(), Quotes::None)
            }
            Value::Literal(ref s, _) => {
                let mut result = String::new();
                let mut iter = s.chars().peekable();
                while let Some(c) = iter.next() {
                    if c == '\\' {
                        let mut val: u32 = 0;
                        let mut got_num = false;
                        let radix = 16;
                        let nextchar = loop {
                            match iter.peek() {
                                Some(c) if (c >= &'0' && c <= &'9') => {
                                    val = val * radix
                                        + u32::from(*c as u8 - b'0');
                                    got_num = true;
                                    iter.next();
                                }
                                Some(c) if (c >= &'a' && c <= &'f') => {
                                    val = val * radix
                                        + u32::from(*c as u8 - b'a' + 10);
                                    got_num = true;
                                    iter.next();
                                }
                                Some(c) if (c >= &'A' && c <= &'F') => {
                                    val = val * radix
                                        + u32::from(*c as u8 - b'A' + 10);
                                    got_num = true;
                                    iter.next();
                                }
                                Some(' ') if got_num => {
                                    iter.next();
                                    break (None);
                                }
                                Some(_) if !got_num => break (iter.next()),
                                _ => break (None),
                            }
                        };
                        if got_num {
                            if val == 0 {
                                result.push('\u{fffd}');
                            } else {
                                if let Ok(c) = char::try_from(val) {
                                    if c != '\n' {
                                        result.push(c);
                                    } else {
                                        result.push('\\');
                                        result.push('a');
                                    }
                                } else {
                                    result.push('\u{fffd}');
                                }
                            }
                        } /* else if nextchar == Some('\\') {
                              result.push('\\');
                          }
                          if let Some(nextchar) = nextchar {
                              result.push(nextchar)
                          }*/
                        match nextchar {
                            Some('\n') => {
                                result.push('\\');
                                result.push('n');
                            }
                            Some('\\') => {
                                result.push('\\');
                                result.push('\\');
                            }
                            Some(c) => {
                                result.push(c);
                            }
                            None => (),
                        }
                    } else {
                        result.push(c)
                    }
                }
                if result.contains('"') && !result.contains('\'') {
                    Value::Literal(result, Quotes::Single)
                } else {
                    Value::Literal(result, Quotes::Double)
                }
            }
            Value::List(ref list, ref s, ref b) => Value::List(
                list.iter().map(|v| v.unrequote()).collect(),
                s.clone(),
                *b,
            ),
            Value::Paren(ref v) => *v.clone(),
            ref v => v.clone(),
        }
    }

    pub fn iter_items(self) -> Vec<Value> {
        match self {
            Value::List(v, _, _) => v,
            Value::Map(map) => map
                .iter()
                .map(|&(ref k, ref v)| {
                    Value::List(
                        vec![k.clone(), v.clone()],
                        ListSeparator::Space,
                        false,
                    )
                })
                .collect(),
            Value::Paren(v) => v.iter_items(),
            v => vec![v],
        }
    }

    /// Get a reference to this `Value` bound to an output format.
    ///
    /// The bound referene implements `Display`, so it can be written
    /// with the rust `format!(...) macros or coverted with the
    /// `to_string()` method.
    ///
    /// # Example
    ///
    /// ```
    /// # use rsass::css::Value;
    /// assert_eq!(
    ///     Value::scalar(42).format(Default::default()).to_string(),
    ///     "42",
    /// );
    /// ```
    pub fn format(&self, format: Format) -> Formatted<Value> {
        Formatted {
            value: self,
            format,
        }
    }
}
