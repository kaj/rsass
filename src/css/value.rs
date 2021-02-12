use crate::css::CallArgs;
use crate::error::Error;
use crate::functions::SassFunction;
use crate::ordermap::OrderMap;
use crate::output::{Format, Formatted};
use crate::value::{Color, ListSeparator, Number, Operator, Quotes, Unit};
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
    /// A string literal.
    Literal(String, Quotes),
    /// A comma- or space separated list of values, with or without brackets.
    List(Vec<Value>, ListSeparator, bool),
    /// A Numeric value is a rational value with a Unit (which may be
    /// Unit::None) and flags.
    ///
    /// The boolean flag is true for calculated values and false for
    /// literal values.
    Numeric(Number, Unit, bool),
    /// A color value (and optionally, its source string).
    Color(Color, Option<String>),
    /// The null value.
    Null,
    /// The true boolean value.
    True,
    /// The false boolean value.
    False,
    /// A binary operation, two operands and an operator.
    /// The booleans represents possible whitespace.
    BinOp(Box<Value>, bool, Operator, bool, Box<Value>),
    /// A unary operator and its operand.
    UnaryOp(Operator, Box<Value>),
    /// A map of values.
    Map(OrderMap<Value, Value>),
    /// A unicode range for font selections. U+NN, U+N?, U+NN-MM.
    /// The string is the entire value, including the "U+" tag.
    UnicodeRange(String),
    /// A value in parenthesis.
    Paren(Box<Value>),
}

impl Value {
    /// Create a numeric value with no unit.
    pub fn scalar<T: Into<Number>>(v: T) -> Self {
        Value::Numeric(v.into(), Unit::None, false)
    }

    /// Get the type name of this value.
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

    /// Return true if this is a calculated value.
    ///
    /// The return of functions or operators are calculated, verbatim
    /// values are not.
    pub fn is_calculated(&self) -> bool {
        match *self {
            Value::Numeric(.., calculated) => calculated,
            Value::Color(_, None) => true,
            _ => false,
        }
    }

    /// Get this value, but marked as calculated.
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
        !matches!(self, Value::False | Value::Null)
    }

    /// Return true if this value is null.
    ///
    /// Note that an empty unquoted string and a list containing no
    /// non-null values is also considered null.
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

    /// Check if this value is numeric.
    ///
    /// If it is, get the number and unit, otherwise, get the value
    /// itself as error.
    pub fn numeric_value(self) -> Result<(Number, Unit), Self> {
        match self {
            Value::Numeric(num, unit, ..) => Ok((num, unit)),
            v => Err(v),
        }
    }

    /// Check that this value is an integer.
    pub fn integer_value(&self) -> Result<isize, Error> {
        match self {
            &Value::Numeric(ref num, ..) if num.is_integer() => num
                .to_integer()
                .ok_or_else(|| Error::bad_value("an integer", self)),
            v => Err(Error::bad_value("a number", v)),
        }
    }

    /// Unquote this value.
    ///
    /// If the value is a quoted string, the content is unquoted.
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
                                Some(&c) if c.is_ascii_hexdigit() => {
                                    val = val * 10 + u32::from(hexvalue(c));
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

    /// Get this value as iterable items.
    ///
    /// Lists and maps have iterable items, which are returned as a
    /// vector of values.  Other values are returned as a vec
    /// containing the value as a single item.
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

fn hexvalue(c: char) -> u8 {
    if ('0'..='9').contains(&c) {
        c as u8 - b'0'
    } else if ('a'..='f').contains(&c) {
        c as u8 - b'a' + 10
    } else if ('A'..='F').contains(&c) {
        c as u8 - b'A' + 10
    } else {
        0
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
                    a.value == (&b.value * scale)
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
impl<C: Into<Color>> From<C> for Value {
    fn from(c: C) -> Value {
        Value::Color(c.into(), None)
    }
}
