use super::{CallArgs, CssString};
use crate::error::Error;
use crate::ordermap::OrderMap;
use crate::output::{Format, Formatted};
use crate::sass::Function;
use crate::value::{Color, ListSeparator, Number, Numeric, Operator};

/// A css value.
#[derive(Clone, Debug, Eq, PartialOrd)]
pub enum Value {
    /// A special kind of escape.  Only really used for !important.
    Bang(String),
    /// An function call that was not evaluated.
    Call(String, CallArgs),
    /// A (callable?) function.
    Function(String, Option<Function>),
    /// A string literal.
    Literal(CssString),
    /// A comma- or space separated list of values, with or without brackets.
    List(Vec<Value>, Option<ListSeparator>, bool),
    /// A Numeric value is a rational value with a Unit (which may be
    /// Unit::None) and flags.
    ///
    /// The boolean flag is true for calculated values and false for
    /// literal values.
    Numeric(Numeric, bool),
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
    Map(ValueMap),
    /// A unicode range for font selections. U+NN, U+N?, U+NN-MM.
    /// The string is the entire value, including the "U+" tag.
    UnicodeRange(String),
    /// A value in parenthesis.
    Paren(Box<Value>),
    /// An argumentt list
    ArgList(super::CallArgs),
}

/// An OrderMap where both the keys and the values are css values.
pub type ValueMap = OrderMap<Value, Value>;

impl Value {
    /// Create a numeric value with no unit.
    pub fn scalar<T: Into<Number>>(v: T) -> Self {
        Value::Numeric(Numeric::scalar(v), true)
    }

    /// Get the type name of this value.
    pub fn type_name(&self) -> &'static str {
        match *self {
            Value::ArgList(..) => "arglist",
            Value::Color(..) => "color",
            Value::Literal(..) => "string",
            Value::Map(..) => "map",
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
    ///
    /// Make sure arithmetic operators are evaluated.
    pub fn into_calculated(self) -> Self {
        match self {
            Value::Numeric(num, _) => Value::Numeric(num, true),
            Value::BinOp(a, s1, op, s2, b) => {
                match op.eval((&*a).clone().into_calculated(), (&*b).clone())
                {
                    Some(v) => v,
                    None => Value::BinOp(a, s1, op, s2, b),
                }
            }
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
            Value::Literal(ref s) => s.is_null(),
            Value::Paren(ref v) => v.is_null(),
            _ => false,
        }
    }

    /// Check if this value is numeric.
    ///
    /// If it is, get the number and unit, otherwise, get the value
    /// itself as error.
    pub fn numeric_value(self) -> Result<Numeric, Self> {
        match self {
            Value::Numeric(num, ..) => Ok(num),
            v => Err(v),
        }
    }

    /// Check that this value is an integer.
    #[deprecated]
    pub fn integer_value(&self) -> Result<i64, Error> {
        match self {
            &Value::Numeric(ref num, ..) => num
                .value
                .clone()
                .into_integer()
                .map_err(|_| Error::bad_value("an integer", self)),
            v => Err(Error::bad_value("a number", v)),
        }
    }

    /// Unquote this value.
    ///
    /// If the value is a quoted string, the content is unquoted.
    pub fn unquote(self) -> Value {
        match self {
            Value::Literal(s) => s.unquote().into(),
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
            Value::ArgList(args) => {
                let mut vec = args.positional;
                // I'm not sure that including the named arguments after the
                // positional is the right thing to do here.
                vec.extend(args.named.into_iter().map(|(k, v)| {
                    Value::List(
                        vec![Value::from(k.as_ref()), v],
                        Some(ListSeparator::Space),
                        false,
                    )
                }));
                vec
            }
            Value::List(v, _, _) => v,
            Value::Map(map) => map
                .iter()
                .map(|&(ref k, ref v)| {
                    Value::List(
                        vec![k.clone(), v.clone()],
                        Some(ListSeparator::Space),
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
            (Value::Numeric(a, _), Value::Numeric(b, _)) => a == b,
            (Value::Literal(a), Value::Literal(b)) => a == b,
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
                a.is_empty() && b.is_empty()
            }
            (Value::Map(a), Value::List(b, ..)) => {
                a.is_empty() && b.is_empty()
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
impl From<CssString> for Value {
    fn from(s: CssString) -> Value {
        Value::Literal(s.pref_dquotes())
    }
}
impl From<&str> for Value {
    fn from(s: &str) -> Value {
        String::from(s).into()
    }
}
impl From<String> for Value {
    fn from(s: String) -> Value {
        CssString::from(s).into()
    }
}
impl From<Numeric> for Value {
    fn from(v: Numeric) -> Value {
        Value::Numeric(v, true)
    }
}

impl<C: Into<Color>> From<C> for Value {
    fn from(c: C) -> Value {
        Value::Color(c.into(), None)
    }
}
