use super::util::IsNot;
use super::{is_calc_name, is_not, BinOp, CallArgs, CssString};
use crate::ordermap::OrderMap;
use crate::output::{Format, Formatted};
use crate::sass::Function;
use crate::value::{Color, ListSeparator, Number, Numeric, Operator};

/// A css value.
#[derive(Clone, Debug, PartialOrd)]
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

#[cfg(test)]
mod test_sizes {
    use super::*;
    use crate::testutil::test_size;

    test_size!(CssString, 32);
    test_size!(CallArgs, 56);
    test_size!(Color, 48);
    test_size!(Value, 80);
}

/// An `OrderMap` where both the keys and the values are css values.
pub type ValueMap = OrderMap<Value, Value>;

impl Value {
    /// Create a numeric value with no unit.
    pub fn scalar<T: Into<Number>>(v: T) -> Self {
        Self::Numeric(Numeric::scalar(v), true)
    }

    pub(crate) fn call<T: Into<Self>, I>(name: &str, args: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self::Call(name.into(), CallArgs::from_iter(args))
    }

    /// Check that the value is valid in css.
    pub fn valid_css(self) -> Result<Self, InvalidCss> {
        match self {
            Self::Numeric(ref num, _) => {
                if num.unit.valid_in_css() {
                    Ok(self)
                } else {
                    Err(InvalidCss::Value(self))
                }
            }
            Self::BinOp(op) => Ok(op.valid_css()?.into()),
            Self::UnaryOp(_, ref v) if v.is_calculation() => {
                Err(InvalidCss::UndefOp(self))
            }
            Self::Call(ref name, ref args) => {
                if name != "calc" {
                    for arg in &args.positional {
                        arg.clone().valid_css()?;
                    }
                }
                Ok(self)
            }
            Self::List(ref v, _, false) => {
                if v.is_empty() {
                    Err(InvalidCss::Value(self))
                } else {
                    Ok(self)
                }
            }
            Self::Function(..) | Self::Map(_) => Err(InvalidCss::Value(self)),
            _ => Ok(self),
        }
    }

    /// Get the type name of this value.
    pub fn type_name(&self) -> &'static str {
        match *self {
            ref v if v.is_calculation() => "calculation",
            Self::ArgList(..) => "arglist",
            Self::Call(ref f, _) => {
                if f == "var" {
                    "variable"
                } else {
                    "string"
                }
            }
            Self::Color(..) => "color",
            Self::Literal(..) | Self::BinOp(_) => "string",
            Self::Map(..) => "map",
            Self::Numeric(..) => "number",
            Self::List(..) => "list",
            Self::Function(..) => "function",
            Self::True | Self::False => "bool",
            Self::Null => "null",
            _ => "unknown",
        }
    }

    /// Returns true if this is a css `calc(...)` function.
    pub fn is_calculation(&self) -> bool {
        match *self {
            Self::Call(ref name, _) if is_calc_name(name) => true,
            Self::Literal(ref s) if s.is_css_calc() => true,
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
        if let Self::Numeric(Numeric { value, unit }, _) = self {
            !value.is_finite() || !unit.valid_in_css()
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
            Self::Numeric(.., calculated) => calculated,
            Self::Color(_, None) => true,
            _ => false,
        }
    }

    /// Get this value, but marked as calculated.
    ///
    /// Make sure arithmetic operators are evaluated.
    pub fn into_calculated(self) -> Self {
        match self {
            Self::Numeric(num, _) => Self::Numeric(num, true),
            Self::BinOp(op) => op.into_calculated(),
            other => other,
        }
    }

    /// All values other than `False` and `Null` should be considered true.
    pub fn is_true(&self) -> bool {
        !matches!(self, Self::False | Self::Null)
    }

    /// Return true if this value is null.
    ///
    /// Note that an empty unquoted string and a list containing no
    /// non-null values is also considered null.
    pub fn is_null(&self) -> bool {
        match *self {
            Self::Null => true,
            Self::List(ref list, _, false) => list.iter().all(Self::is_null),
            Self::Literal(ref s) => s.is_null(),
            Self::Paren(ref v) => v.is_null(),
            _ => false,
        }
    }

    /// Check if this value is numeric.
    ///
    /// If it is, get the number and unit, otherwise, get the value
    /// itself as error.
    pub fn numeric_value(self) -> Result<Numeric, Self> {
        match self {
            Self::Numeric(num, ..) => Ok(num),
            v => Err(v),
        }
    }

    /// Unquote this value.
    ///
    /// If the value is a quoted string, the content is unquoted.
    pub fn unquote(self) -> Self {
        match self {
            Self::Literal(s) => s.unquote().into(),
            Self::List(list, s, b) => Self::List(
                list.into_iter().map(Self::unquote).collect(),
                s,
                b,
            ),
            Self::Paren(v) => *v,
            v => v,
        }
    }

    /// Get this value as iterable items.
    ///
    /// Lists and maps have iterable items, which are returned as a
    /// vector of values.  Other values are returned as a vec
    /// containing the value as a single item.
    pub fn iter_items(self) -> Vec<Self> {
        match self {
            Self::ArgList(args) => {
                let mut vec = args.positional;
                // I'm not sure that including the named arguments after the
                // positional is the right thing to do here.
                vec.extend(args.named.into_iter().map(|(k, v)| {
                    Self::List(
                        vec![Self::from(k.as_ref()), v],
                        Some(ListSeparator::Space),
                        false,
                    )
                }));
                if args.trailing_comma {
                    vec.push(Self::Null);
                }
                vec
            }
            Self::List(v, _, _) => v,
            Self::Map(map) => map
                .into_iter()
                .map(|(k, v)| {
                    Self::List(vec![k, v], Some(ListSeparator::Space), false)
                })
                .collect(),
            Self::Paren(v) => v.iter_items(),
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
    pub fn format(&self, format: Format) -> Formatted<'_, Self> {
        Formatted {
            value: self,
            format,
        }
    }
    /// Format this value for error messages.
    pub fn introspect(&self) -> String {
        self.to_string(Format::introspect())
    }
}

/// Some Values are equal according to spec even with some
/// implementation differences.
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (&self, other) {
            (Self::Bang(a), Self::Bang(b)) => a == b,
            (Self::Numeric(a, _), Self::Numeric(b, _)) => a == b,
            (Self::Literal(a), Self::Literal(b)) => a == b,
            (Self::True, Self::True)
            | (Self::False, Self::False)
            | (Self::Null, Self::Null) => true,
            (Self::Color(a, _), Self::Color(b, _)) => a == b,
            (Self::Call(af, aa), Self::Call(bf, ba)) => af == bf && aa == ba,
            (Self::Function(a, abody), Self::Function(b, bbody)) => {
                a == b && abody == bbody
            }
            (Self::List(av, asep, ab), Self::List(bv, bsep, bb)) => {
                av == bv && asep == bsep && ab == bb
            }
            (Self::Map(a), Self::Map(b)) => a == b,
            (Self::UnaryOp(a, av), Self::UnaryOp(b, bv)) => {
                a == b && av == bv
            }
            (Self::BinOp(a), Self::BinOp(b)) => a == b,
            (Self::UnicodeRange(a), Self::UnicodeRange(b)) => a == b,
            (Self::Paren(a), Self::Paren(b)) => a == b,
            (Self::List(a, ..), Self::Map(b)) => a.is_empty() && b.is_empty(),
            (Self::Map(a), Self::List(b, ..)) => a.is_empty() && b.is_empty(),
            _ => false,
        }
    }
}
impl Eq for Value {}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        match v {
            true => Self::True,
            false => Self::False,
        }
    }
}
impl From<CssString> for Value {
    fn from(s: CssString) -> Self {
        Self::Literal(s.pref_dquotes())
    }
}
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        String::from(s).into()
    }
}
impl From<String> for Value {
    fn from(s: String) -> Self {
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
    type Error = Self;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        CssString::try_from(value).map(CssString::take_value)
    }
}

impl From<Numeric> for Value {
    fn from(v: Numeric) -> Self {
        Self::Numeric(v, true)
    }
}

impl<C: Into<Color>> From<C> for Value {
    fn from(c: C) -> Self {
        Self::Color(c.into(), None)
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

impl TryInto<OrderMap<CssString, Self>> for Value {
    type Error = ValueToMapError;
    fn try_into(self) -> Result<OrderMap<CssString, Self>, ValueToMapError> {
        match self {
            Self::Map(m) => m
                .into_iter()
                .map(|(k, v)| match (k, v) {
                    (Self::Literal(k), v) => Ok((k, v)),
                    (k, _v) => {
                        Err(ValueToMapError::Key(is_not(&k, "a string")))
                    }
                })
                .collect(),
            // An empty map and an empty list looks the same
            Self::List(ref l, ..) if l.is_empty() => Ok(OrderMap::new()),
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
            Self::UndefOp(v) => {
                write!(f, "Undefined operation \"{}\".", v.introspect())
            }
            Self::Value(v) => {
                write!(f, "{} isn't a valid CSS value.", v.introspect())
            }
            Self::Incompat(a, b) => write!(
                f,
                "{} and {} have incompatible units.",
                Value::from(a.clone()).introspect(),
                Value::from(b.clone()).introspect(),
            ),
        }
    }
}
