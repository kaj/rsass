use css::CallArgs;
use error::Error;
use num_rational::Rational;
use num_traits::{One, Signed, Zero};
use ordermap::OrderMap;
use std::fmt;
use value::{ListSeparator, Operator, Quotes, Unit, rgb_to_name};

/// A sass value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Call(String, CallArgs),
    /// The booleans tell if there should be whitespace
    /// before / after the slash.
    Div(Box<Value>, Box<Value>, bool, bool),
    Literal(String, Quotes),
    /// A comma- or space separated list of values, with or without brackets.
    List(Vec<Value>, ListSeparator, bool),
    /// A Numeric value is a rational value with a Unit (which may be
    /// Unit::None) and flags.
    ///
    /// The first flag is true for values with an explicit + sign.
    ///
    /// The second flag is true for calculated values and false for
    /// literal values.
    Numeric(Rational, Unit, bool, bool),
    Color(Rational, Rational, Rational, Rational, Option<String>),
    Null,
    True,
    False,
    /// A binary operation, two operands and an operator.
    BinOp(Box<Value>, Operator, Box<Value>),
    UnaryOp(Operator, Box<Value>),
    Map(OrderMap<Value, Value>),
}

impl Value {
    pub fn scalar(v: isize) -> Self {
        Value::Numeric(Rational::from_integer(v), Unit::None, false, false)
    }
    pub fn bool(v: bool) -> Self {
        if v { Value::True } else { Value::False }
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
        Value::Color(cap(r, &ff), cap(g, &ff), cap(b, &ff), cap(a, &one), None)
    }

    pub fn type_name(&self) -> &'static str {
        match *self {
            Value::Color(..) => "color",
            Value::Literal(..) => "string",
            Value::Numeric(..) => "number",
            Value::List(..) => "list",
            Value::True | Value::False => "bool",
            Value::Null => "null",
            _ => "unknown",
        }
    }

    pub fn is_calculated(&self) -> bool {
        match *self {
            Value::Numeric(_, _, _, calculated) => calculated,
            Value::Color(_, _, _, _, None) => true,
            _ => false,
        }
    }

    pub fn into_calculated(self) -> Self {
        match self {
            Value::Numeric(v, unit, with_sign, _) => {
                Value::Numeric(v, unit, with_sign, true)
            }
            Value::Literal(s, Quotes::Single) => {
                Value::Literal(s, Quotes::Double)
            }
            Value::List(v, sep, bracketed) => {
                Value::List(v.into_iter()
                                .map(|i| i.into_calculated())
                                .collect(),
                            sep,
                            bracketed)
            }
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
            Value::List(ref list, _, false) => list.iter().all(|v| v.is_null()),
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
            Value::Literal(s, Quotes::None) => Value::Literal(s, Quotes::None),
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
            Value::List(list, s, b) => {
                Value::List(list.into_iter().map(|v| v.unquote()).collect(),
                            s,
                            b)
            }
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
                Value::Literal(result, Quotes::Double)
            }
            Value::List(ref list, ref s, ref b) => {
                Value::List(list.into_iter().map(|v| v.unrequote()).collect(),
                            s.clone(),
                            *b)
            }
            ref v => v.clone(),
        }
    }

    pub fn iter_items(self) -> Vec<Value> {
        match self {
            Value::List(v, _, _) => v,
            Value::Map(map) => {
                map.iter()
                    .map(|&(ref k, ref v)| {
                             Value::List(vec![k.clone(), v.clone()],
                                         ListSeparator::Space,
                                         false)
                         })
                    .collect()
            }
            v => vec![v],
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Literal(ref s, ref q) => {
                match *q {
                    Quotes::Double => {
                        write!(out,
                               "\"{}\"",
                               s.chars()
                                   .flat_map(|c| match c {
                                                 '"' => vec!['\\', '"'],
                                                 c => vec![c],
                                             })
                                   .collect::<String>())
                    }
                    Quotes::Single => {
                        write!(out,
                               "'{}'",
                               s.chars()
                                   .flat_map(|c| match c {
                                                 '\'' => vec!['\\', '\''],
                                                 c => vec![c],
                                             })
                                   .collect::<String>())
                    }
                    Quotes::None => write!(out, "{}", s),
                }
            }
            Value::Numeric(ref v, ref u, ref with_sign, _) => {
                let short = out.alternate();
                write!(out, "{}{}", rational2str(v, *with_sign, short), u)
            }
            Value::Color(ref r, ref g, ref b, ref a, ref s) => {
                let r = r.round().to_integer() as u8;
                let g = g.round().to_integer() as u8;
                let b = b.round().to_integer() as u8;
                if let Some(ref s) = *s {
                    write!(out, "{}", s)
                } else if a >= &Rational::from_integer(1) {
                    if out.alternate() {
                        // E.g. #ff00cc can be written #f0c in css.
                        // 0xff / 17 = 0xf (since 17 = 0x11).
                        let hex = if r % 17 == 0 && g % 17 == 0 &&
                                     b % 17 == 0 {
                            format!("#{:x}{:x}{:x}", r / 17, g / 17, b / 17)
                        } else {
                            format!("#{:02x}{:02x}{:02x}", r, g, b)
                        };
                        match rgb_to_name(r, g, b) {
                            Some(name) if name.len() <= hex.len() => {
                                write!(out, "{}", name)
                            }
                            _ => write!(out, "{}", hex),
                        }
                    } else if let Some(name) = rgb_to_name(r, g, b) {
                        write!(out, "{}", name)
                    } else {
                        write!(out, "#{:02x}{:02x}{:02x}", r, g, b)
                    }
                } else if a.is_zero() && r.is_zero() && g.is_zero() &&
                          b.is_zero() {
                    write!(out, "transparent")
                } else if out.alternate() {
                    write!(out,
                           "rgba({},{},{},{})",
                           r,
                           g,
                           b,
                           rational2str(a, false, false))
                } else {
                    write!(out,
                           "rgba({}, {}, {}, {})",
                           r,
                           g,
                           b,
                           rational2str(a, false, false))
                }
            }
            Value::List(ref v, ref sep, brackets) => {
                let t = v.iter()
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
                    })
                    .collect::<Vec<_>>();
                let t = if *sep == ListSeparator::Comma && t.len() == 1 {
                    format!("{},", t[0])
                } else {
                    t.join(match *sep {
                               ListSeparator::Comma => {
                                   if out.alternate() { "," } else { ", " }
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
            Value::Div(ref a, ref b, s1, s2) => {
                a.fmt(out)?;
                if s1 {
                    out.write_str(" ")?;
                }
                out.write_str("/")?;
                if s2 {
                    out.write_str(" ")?;
                }
                b.fmt(out)
            }
            Value::Call(ref name, ref arg) => write!(out, "{}({})", name, arg),
            Value::BinOp(ref a, Operator::Plus, ref b) => {
                // The plus operator is also a concat operator
                a.fmt(out)?;
                b.fmt(out)
            }
            Value::BinOp(ref a, ref op, ref b) => {
                a.fmt(out)?;
                op.fmt(out)?;
                b.fmt(out)
            }
            Value::UnaryOp(ref op, ref v) => {
                op.fmt(out)?;
                v.fmt(out)
            }
            Value::True => write!(out, "true"),
            Value::False => write!(out, "false"),
            Value::Null => Ok(()),
            Value::Map(_) => panic!("Formatting a map not supported"),
        }
    }
}

fn rational2str(r: &Rational, with_sign: bool, skipzero: bool) -> String {
    if r.is_integer() {
        if with_sign {
            format!("{:+}", r.numer())
        } else {
            format!("{}", r.numer())
        }
    } else {
        let prec = Rational::from_integer(100_000);
        let v = (r * prec).round() / prec;
        let v = *v.numer() as f64 / *v.denom() as f64;
        let mut result = if with_sign {
            format!("{:+}", v)
        } else {
            format!("{}", v)
        };
        if skipzero && result.starts_with("0.") {
            result.remove(0);
        }
        result
    }
}
