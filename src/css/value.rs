use crate::css::CallArgs;
use crate::error::Error;
use crate::functions::SassFunction;
use crate::ordermap::OrderMap;
use crate::output::{Format, Formatted};
use crate::value::{ListSeparator, Number, Operator, Quotes, Rgba, Unit};
use num_rational::Rational;
use std::convert::TryFrom;

/// A css value.
#[derive(Clone, Debug, Eq, PartialOrd)]
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
    Numeric(Number, Unit, bool),
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
    pub fn scalar<T: Into<Number>>(v: T) -> Self {
        Value::Numeric(v.into(), Unit::None, false)
    }
    #[deprecated(since = "0.15.1", note = "please use From<bool> instead.")]
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
    #[deprecated]
    pub fn rgba(r: Rational, g: Rational, b: Rational, a: Rational) -> Self {
        Value::Color(Rgba::new(r, g, b, a), None)
    }
    #[deprecated]
    pub fn hsla(h: Rational, s: Rational, l: Rational, a: Rational) -> Self {
        Value::Color(Rgba::from_hsla(h, s, l, a), None)
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
            &Value::Numeric(ref num, ..) if num.is_integer() => num
                .to_integer()
                .ok_or_else(|| Error::bad_value("an integer", self)),
            v => Err(Error::bad_value("a number", v)),
        }
    }

    pub fn unquote(self) -> Value {
        match self {
            Value::Literal(s, Quotes::None) => {
                Value::Literal(s, Quotes::None)
            }
            Value::Literal(s, _) => {
                let mut result = String::new();
                let mut iter = s.chars().peekable();
                while let Some(c) = iter.next() {
                    if c == '\\' {
                        let mut val: u32 = 0;
                        let mut got_num = false;
                        let nextchar = loop {
                            match iter.peek() {
                                Some(&c) if (c >= '0' && c <= '9') => {
                                    val =
                                        val * 10 + u32::from(c as u8 - b'0');
                                    got_num = true;
                                    iter.next();
                                }
                                Some(&c) if (c >= 'a' && c <= 'f') => {
                                    val = val * 10
                                        + u32::from(c as u8 - b'a' + 10);
                                    got_num = true;
                                    iter.next();
                                }
                                Some(&c) if (c >= 'A' && c <= 'F') => {
                                    val = val * 10
                                        + u32::from(c as u8 - b'A' + 10);
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
                            if let Ok(c) = char::try_from(val) {
                                result.push(c);
                            } else {
                                result.push('\u{fffd}');
                            }
                        }
                        match nextchar {
                            Some('\n') => {
                                result.push('\\');
                                result.push('a');
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
                Value::Literal(result, Quotes::None)
            }
            Value::List(list, s, b) => Value::List(
                list.into_iter().map(|v| v.unquote()).collect(),
                s,
                b,
            ),
            Value::Paren(v) => *v,
            v => v,
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

/// Some Values are equal according to spec even with some
/// implementation differences.
impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (&self, other) {
            (Value::Bang(a), Value::Bang(b)) => a == b,
            (Value::Numeric(a, au, _), Value::Numeric(b, bu, _)) => {
                if au == bu {
                    a == b
                } else if let Some(scale) = bu.scale_to(au) {
                    a == &(b * &scale)
                } else {
                    false
                }
            }
            (Value::Literal(a, aq), Value::Literal(b, bq)) => {
                if aq == bq {
                    a == b
                } else {
                    let a = if aq.is_none() {
                        a.replace('\\', "\\\\")
                    } else {
                        a.clone()
                    };
                    let b = if bq.is_none() {
                        b.replace('\\', "\\\\")
                    } else {
                        b.clone()
                    };
                    a == b
                }
            }
            (Value::Null, Value::Null) => true,
            (Value::True, Value::True) => true,
            (Value::False, Value::False) => true,
            (Value::Color(a, _), Value::Color(b, _)) => a == b,
            (Value::Call(af, aa), Value::Call(bf, ba)) => {
                af == bf && aa == ba
            }
            (Value::Function(a, abody), Value::Function(b, bbody)) => {
                a == b && abody == bbody
            }
            (Value::List(av, asep, ab), Value::List(bv, bsep, bb)) => {
                av == bv && asep == bsep && ab == bb
            }
            (Value::Map(a), Value::Map(b)) => a == b,
            (Value::UnaryOp(a, av), Value::UnaryOp(b, bv)) => {
                a == b && av == bv
            }
            (
                Value::BinOp(aa, _, ao, _, ab),
                Value::BinOp(ba, _, bo, _, bb),
            ) => ao == bo && aa == ba && ab == bb,
            (Value::UnicodeRange(a), Value::UnicodeRange(b)) => a == b,
            (Value::Paren(a), Value::Paren(b)) => a == b,
            (Value::List(a, ..), Value::Map(b)) => {
                a.is_empty() && b.len() == 0
            }
            (Value::Map(a), Value::List(b, ..)) => {
                a.len() == 0 && b.is_empty()
            }
            _ => false,
        }
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Value {
        match v {
            true => Value::True,
            false => Value::False,
        }
    }
}
impl From<&str> for Value {
    fn from(s: &str) -> Value {
        Value::Literal(s.into(), Quotes::None)
    }
}
impl From<String> for Value {
    fn from(s: String) -> Value {
        Value::Literal(s, Quotes::None)
    }
}
impl From<Rgba> for Value {
    fn from(c: Rgba) -> Value {
        Value::Color(c, None)
    }
}
