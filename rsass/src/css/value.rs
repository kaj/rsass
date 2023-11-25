use super::util::IsNot;
use super::{is_calc_name, is_not, BinOp, CallArgs, CssString};
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
    BinOp(Box<BinOp>),
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

/// An `OrderMap` where both the keys and the values are css values.
pub type ValueMap = OrderMap<Value, Value>;

impl Value {
    /// Create a numeric value with no unit.
    pub fn scalar<T: Into<Number>>(v: T) -> Self {
        Value::Numeric(Numeric::scalar(v), true)
    }

    /// Check that the value is valid in css.
    pub fn valid_css(self) -> Result<Self, InvalidCss> {
        match self {
            Value::Numeric(ref num, _) => {
                if num.unit.valid_in_css() {
                    Ok(self)
                } else {
                    Err(InvalidCss::Value(self))
                }
            }
            Value::BinOp(op) => Ok(op.valid_css()?.into()),
            Value::UnaryOp(_, ref v) if v.is_calculation() => {
                Err(InvalidCss::UndefOp(self))
            }
            Value::Call(ref name, ref args) => {
                if name != "calc" {
                    for arg in &args.positional {
                        arg.clone().valid_css()?;
                    }
                }
                Ok(self)
            }
            Value::List(ref v, _, false) => {
                if v.is_empty() {
                    Err(InvalidCss::Value(self))
                } else {
                    Ok(self)
                }
            }
            Value::Function(..) | Value::Map(_) => {
                Err(InvalidCss::Value(self))
            }
            _ => Ok(self),
        }
    }

    /// Get the type name of this value.
    pub fn type_name(&self) -> &'static str {
        match *self {
            ref v if v.is_calculation() => "calculation",
            Value::ArgList(..) => "arglist",
            Value::Call(ref f, _) => {
                if f == "var" {
                    "variable"
                } else {
                    "string"
                }
            }
            Value::Color(..) => "color",
            Value::Literal(..) | Value::BinOp(_) => "string",
            Value::Map(..) => "map",
            Value::Numeric(..) => "number",
            Value::List(..) => "list",
            Value::Function(..) => "function",
            Value::True | Value::False => "bool",
            Value::Null => "null",
            _ => "unknown",
        }
    }

    /// Returns true if this is a css `calc(...)` function.
    pub fn is_calculation(&self) -> bool {
        match *self {
            Value::Call(ref name, _) if is_calc_name(name) => true,
            Value::Literal(ref s) if s.is_css_calc() => true,
            _ => false,
        }
    }

    pub(crate) fn to_string(&self, format: Format) -> String {
        let value = self.format(format);
        if self.needs_calc() {
            format!("calc({value})")
        } else {
            value.to_string()
        }
    }

    fn needs_calc(&self) -> bool {
        if let Value::Numeric(Numeric { value, unit: _ }, _) = self {
            !value.is_finite()
        } else {
            false
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
            Value::BinOp(op) => op.into_calculated(),
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
                list.iter().all(Value::is_null)
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

    /// Unquote this value.
    ///
    /// If the value is a quoted string, the content is unquoted.
    pub fn unquote(self) -> Value {
        match self {
            Value::Literal(s) => s.unquote().into(),
            Value::List(list, s, b) => Value::List(
                list.into_iter().map(Value::unquote).collect(),
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
                if args.trailing_comma {
                    vec.push(Value::Null);
                }
                vec
            }
            Value::List(v, _, _) => v,
            Value::Map(map) => map
                .into_iter()
                .map(|(k, v)| {
                    Value::List(vec![k, v], Some(ListSeparator::Space), false)
                })
                .collect(),
            Value::Paren(v) => v.iter_items(),
            v => vec![v],
        }
    }

    /// Get a reference to this `Value` bound to an output format.
    ///
    /// The bound reference implements `Display`, so it can be written
    /// with the rust `format!(...)` macros or coverted with the
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
    /// Format this value for error messages.
    pub fn introspect(&self) -> String {
        // Note: The replaceing here is a silly workaround for
        // https://github.com/sass/sass-spec/issues/1905
        self.to_string(Format::introspect())
            .replace("infinity * 1", "Infinity")
            .replace("NaN * 1", "NaN")
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
            (Value::True, Value::True)
            | (Value::False, Value::False)
            | (Value::Null, Value::Null) => true,
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
            (Value::BinOp(a), Value::BinOp(b)) => a == b,
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
impl TryFrom<Value> for CssString {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Literal(s) => Ok(s),
            Value::Call(name, args) => Ok(format!("{name}({args})").into()),
            v => Err(is_not(&v, "a string")),
        }
    }
}
impl TryFrom<Value> for String {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        CssString::try_from(value).map(CssString::take_value)
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
impl TryFrom<Value> for Color {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Color(col, _) => Ok(col),
            v => Err(is_not(&v, "a color")),
        }
    }
}

impl TryFrom<Value> for Numeric {
    type Error = IsNot;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Numeric(num, ..) => Ok(num),
            v => Err(IsNot::new(v, "a number")),
        }
    }
}

impl TryInto<OrderMap<CssString, Value>> for Value {
    type Error = ValueToMapError;
    fn try_into(self) -> Result<OrderMap<CssString, Value>, ValueToMapError> {
        match self {
            Value::Map(m) => m
                .into_iter()
                .map(|(k, v)| match (k, v) {
                    (Value::Literal(k), v) => Ok((k, v)),
                    (k, _v) => {
                        Err(ValueToMapError::Key(is_not(&k, "a string")))
                    }
                })
                .collect(),
            // An empty map and an empty list looks the same
            Value::List(ref l, ..) if l.is_empty() => Ok(OrderMap::new()),
            v => Err(ValueToMapError::Root(is_not(&v, "a map"))),
        }
    }
}

/// Specific error type for converting a `Value` to a `ValueMap`.
#[derive(Debug)]
pub enum ValueToMapError {
    /// The `Value` was not convertible to a `ValueMap`.
    Root(String),
    /// One key value was not convertible to the key type.
    Key(String),
}

/// An value is not valid for use in css.
#[derive(Debug)]
pub enum InvalidCss {
    /// Value contains an undefined operation.
    UndefOp(Value),
    /// Value isn't a valid css value.
    Value(Value),
    /// {} and {} have incompatible units.
    Incompat(Numeric, Numeric),
}

impl std::error::Error for InvalidCss {}
impl std::fmt::Display for InvalidCss {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidCss::UndefOp(v) => {
                write!(f, "Undefined operation \"{}\".", v.introspect())
            }
            InvalidCss::Value(v) => {
                write!(f, "{} isn't a valid CSS value.", v.introspect())
            }
            InvalidCss::Incompat(a, b) => write!(
                f,
                "{} and {} have incompatible units.",
                a.format(Format::introspect()),
                b.format(Format::introspect()),
            ),
        }
    }
}
