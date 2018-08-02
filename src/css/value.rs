use css::CallArgs;
use error::Error;
use functions::SassFunction;
use num_rational::Rational;
use num_traits::{One, Signed, Zero};
use ordermap::OrderMap;
use std::fmt::{self, Write};
use value::{rgb_to_name, ListSeparator, Number, Operator, Quotes, Unit};

/// A sass value.
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
    Numeric(Number, Unit, bool),
    Color(Rational, Rational, Rational, Rational, Option<String>),
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
}

impl Value {
    pub fn scalar(v: isize) -> Self {
        Value::Numeric(Number::from_integer(v), Unit::None, false)
    }
    pub fn bool(v: bool) -> Self {
        if v {
            Value::True
        } else {
            Value::False
        }
    }
    pub fn black() -> Self {
        let z = Rational::zero();
        Value::Color(z, z, z, Rational::one(), Some("black".into()))
    }
    pub fn rgba(r: Rational, g: Rational, b: Rational, a: Rational) -> Self {
        fn cap(n: Rational, ff: &Rational) -> Rational {
            if n > *ff {
                *ff
            } else if n.is_negative() {
                Rational::zero()
            } else {
                n
            }
        }
        let ff = Rational::new(255, 1);
        let one = Rational::one();
        Value::Color(
            cap(r, &ff),
            cap(g, &ff),
            cap(b, &ff),
            cap(a, &one),
            None,
        )
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
            Value::Color(_, _, _, _, None) => true,
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
            Value::Literal(s, Quotes::None) => {
                Value::Literal(s, Quotes::None)
            }
            Value::Literal(s, _) => {
                let mut result = String::new();
                let mut iter = s.chars();
                while let Some(c) = iter.next() {
                    if c == '\\' {
                        match iter.next() {
                            Some('0') => result.push('\u{fffd}'),
                            Some(c) if (c >= '1' && c <= '9') => {
                                result.push((c as u8 - b'0') as char)
                            }
                            Some(c) if (c >= 'a' && c <= 'f') => {
                                result.push((c as u8 - b'a' + 10) as char)
                            }
                            Some(c) if (c >= 'A' && c <= 'F') => {
                                result.push((c as u8 - b'A' + 10) as char)
                            }
                            Some(c) => result.push(c),
                            None => result.push('\\'), // ??
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
                let mut iter = s.chars();
                while let Some(c) = iter.next() {
                    if c == '\\' {
                        match iter.next() {
                            Some('\\') => result.push_str("\\\\"),
                            Some('0') => result.push('\u{fffd}'),
                            Some(c) if (c >= '1' && c <= '9') => {
                                result.push((c as u8 - b'0') as char)
                            }
                            Some('a') | Some('A') => result.push_str("\\a"),
                            Some(c) if (c >= 'b' && c <= 'f') => {
                                result.push((c as u8 - b'a' + 10) as char)
                            }
                            Some(c) if (c >= 'B' && c <= 'F') => {
                                result.push((c as u8 - b'A' + 10) as char)
                            }
                            Some(c) => result.push(c),
                            None => result.push('\\'), // ??
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
                list.into_iter().map(|v| v.unrequote()).collect(),
                s.clone(),
                *b,
            ),
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
                }).collect(),
            v => vec![v],
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Bang(ref s) => write!(out, "!{}", s),
            Value::Literal(ref s, ref q) => match *q {
                Quotes::Double => write!(
                    out,
                    "\"{}\"",
                    s.chars()
                        .flat_map(|c| match c {
                            '"' => vec!['\\', '"'],
                            c => vec![c],
                        }).collect::<String>()
                ),
                Quotes::Single => write!(
                    out,
                    "'{}'",
                    s.chars()
                        .flat_map(|c| match c {
                            '\'' => vec!['\\', '\''],
                            c => vec![c],
                        }).collect::<String>()
                ),
                Quotes::None => write!(out, "{}", s),
            },
            Value::Function(ref n, ref _f) => {
                let name = n
                    .chars()
                    .flat_map(|c| match c {
                        '"' => vec!['\\', '"'],
                        c => vec![c],
                    }).collect::<String>();
                write!(out, "get-function(\"{}\")", name)
            }
            Value::Numeric(ref num, ref unit, _) => {
                num.fmt(out)?;
                unit.fmt(out)
            }
            Value::Color(ref r, ref g, ref b, ref a, ref name) => {
                if let Some(ref name) = *name {
                    write!(out, "{}", name)
                } else if *a >= Rational::from_integer(1) {
                    let r = r.round().to_integer() as u8;
                    let g = g.round().to_integer() as u8;
                    let b = b.round().to_integer() as u8;
                    // E.g. #ff00cc can be written #f0c in css.
                    // 0xff / 17 = 0xf (since 17 = 0x11).
                    let short = r % 17 == 0 && g % 17 == 0 && b % 17 == 0;
                    let hex_len = if short { 4 } else { 7 };
                    match rgb_to_name(r, g, b) {
                        Some(name)
                            if !(out.alternate() && name.len() > hex_len) =>
                        {
                            write!(out, "{}", name)
                        }
                        _ if out.alternate() && short => write!(
                            out,
                            "#{:x}{:x}{:x}",
                            r / 17,
                            g / 17,
                            b / 17
                        ),
                        _ => write!(out, "#{:02x}{:02x}{:02x}", r, g, b),
                    }
                } else if a.is_zero()
                    && r.is_zero()
                    && g.is_zero()
                    && b.is_zero()
                {
                    write!(out, "transparent")
                } else if out.alternate() {
                    write!(
                        out,
                        // I think the last {} should be {:#} here,
                        // but the test suite says otherwise.
                        "rgba({},{},{},{})",
                        r.round().to_integer() as u8,
                        g.round().to_integer() as u8,
                        b.round().to_integer() as u8,
                        Number::new(*a),
                    )
                } else {
                    write!(
                        out,
                        "rgba({}, {}, {}, {})",
                        r.round().to_integer() as u8,
                        g.round().to_integer() as u8,
                        b.round().to_integer() as u8,
                        Number::new(*a),
                    )
                }
            }
            Value::List(ref v, ref sep, brackets) => {
                let t = v
                    .iter()
                    .filter(|v| !v.is_null())
                    .map(|v| {
                        let needs_paren = match *v {
                            Value::List(_, _, false) => {
                                brackets && *sep == ListSeparator::Space
                            }
                            _ => false,
                        };
                        match (out.alternate(), needs_paren) {
                            (true, true) => format!("({:#})", v),
                            (true, false) => format!("{:#}", v),
                            (false, true) => format!("({})", v),
                            (false, false) => format!("{}", v),
                        }
                    }).collect::<Vec<_>>();
                let t = if *sep == ListSeparator::Comma && t.len() == 1 {
                    format!("{},", t[0])
                } else {
                    t.join(match *sep {
                        ListSeparator::Comma => {
                            if out.alternate() {
                                ","
                            } else {
                                ", "
                            }
                        }
                        ListSeparator::Space => " ",
                    })
                };
                if brackets {
                    out.write_str("[")?;
                }
                write!(out, "{}", t)?;
                if brackets {
                    out.write_str("]")?;
                }
                Ok(())
            }
            Value::Call(ref name, ref arg) => {
                write!(out, "{}({})", name, arg)
            }
            Value::BinOp(ref a, _, Operator::Plus, _, ref b) => {
                // The plus operator is also a concat operator
                a.fmt(out)?;
                b.fmt(out)
            }
            Value::BinOp(ref a, ref s1, ref op, ref s2, ref b) => {
                a.fmt(out)?;
                if *s1 {
                    out.write_char(' ')?;
                }
                op.fmt(out)?;
                if *s2 {
                    out.write_char(' ')?;
                }
                b.fmt(out)
            }
            Value::UnaryOp(ref op, ref v) => {
                op.fmt(out)?;
                v.fmt(out)
            }
            Value::True => write!(out, "true"),
            Value::False => write!(out, "false"),
            Value::Null => Ok(()),
            Value::Map(ref map) => write!(
                out,
                "({})",
                map.iter()
                    .map(|&(ref k, ref v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::UnicodeRange(ref s) => write!(out, "{}", s),
        }
    }
}
