use colors::rgb_to_name;
use nom::{alphanumeric, digit, multispace};
use num_rational::Rational;
use std::fmt;
use std::str::{FromStr, from_utf8};

/// A sass value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    /// A call has a name and an argument (which may be multi).
    Call(String, Box<Value>),
    /// Sometimes an actual division, sometimes "a/b".
    /// In the later case, the booleans tell if there should be whitespace
    /// before / after the slash.
    Div(Box<Value>, Box<Value>, bool, bool),
    Literal(String),
    Minus(Box<Value>, Box<Value>),
    MultiSpace(Vec<Value>),
    MultiComma(Vec<Value>),
    /// A Numeric value is a rational value with a Unit (which may be
    /// Unit::None) and a flag which is true for calculated values and
    /// false for literal values.
    Numeric(Rational, Unit, bool),
    /// "(a/b) and a/b differs semantically.  Parens means the value
    /// should be evaluated numerically if possible, without parens /
    /// is not allways division.
    Paren(Box<Value>),
    Product(Box<Value>, Box<Value>),
    Sum(Box<Value>, Box<Value>),
    Variable(String),
    /// Both a numerical and original string representation,
    /// since case and length should be preserved (#AbC vs #aabbcc).
    HexColor(u8, u8, u8, Option<String>),
}

impl Value {
    pub fn is_calculated(&self) -> bool {
        match self {
            &Value::Numeric(_, _, calculated) => calculated,
            &Value::HexColor(_, _, _, None) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Unit {
    Percent,
    Pt,
    Px,
    Em,
    Ex,
    None,
}

impl fmt::Display for Unit {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Unit::Percent => write!(out, "%"),
            &Unit::Pt => write!(out, "pt"),
            &Unit::Px => write!(out, "px"),
            &Unit::Em => write!(out, "em"),
            &Unit::Ex => write!(out, "ex"),
            &Unit::None => Ok(()),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Literal(ref s) => write!(out, "{}", s),
            &Value::Numeric(ref v, ref u, ref _is_calculated) => {
                let n = v.numer().clone();
                let d = v.denom().clone();
                write!(out, "{}{}", n as f64 / d as f64, u)
            }
            &Value::HexColor(ref r, ref g, ref b, ref s) => {
                match s {
                    &Some(ref s) => write!(out, "{}", s),
                    &None => {
                        match rgb_to_name(*r, *g, *b) {
                            Some(name) => write!(out, "{}", name),
                            None => write!(out, "#{:02x}{:02x}{:02x}", r, g, b),
                        }
                    }
                }
            }
            &Value::MultiSpace(ref v) => {
                let t = v.iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(out, "{}", t)
            }
            &Value::MultiComma(ref v) => {
                let t = v.iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(out, "{}", t)
            }
            &Value::Call(ref name, ref arg) => write!(out, "{}({})", name, arg),
            x => write!(out, "TODO {:?}", x),
        }
    }
}

named!(pub value_expression<&[u8], Value>,
       chain!(v: separated_nonempty_list!(chain!(tag!(",") ~ multispace?,
                                                 || ()),
                                          space_list),
              || if v.len() == 1 {
                  v[0].clone()
              } else {
                  Value::MultiComma(v)
              }));

named!(pub space_list<&[u8], Value>,
       chain!(v: separated_nonempty_list!(multispace, single_expression),
              || if v.len() == 1 {
                  v[0].clone()
              } else {
                  Value::MultiSpace(v)
              }));

named!(pub single_expression<Value>,
       alt!(complete!(chain!(a: term_value ~ multispace? ~ tag!("+") ~
                             multispace? ~ b: single_expression,
                             || Value::Sum(Box::new(a), Box::new(b)))) |
            complete!(chain!(a: term_value ~ multispace? ~ tag!("-") ~
                             multispace? ~ b: term_value,
                             || Value::Minus(Box::new(a), Box::new(b)))) |
            term_value));

named!(term_value<Value>,
       alt!(complete!(chain!(a: single_value ~ multispace? ~ tag!("*") ~
                             multispace? ~ b: term_value,
                             || Value::Product(Box::new(a), Box::new(b)))) |
            complete!(chain!(a: single_value ~
                             s1: multispace? ~ tag!("/") ~ s2: multispace? ~
                             b: single_value ~
                             s3: multispace? ~ tag!("/") ~ s4: multispace? ~
                             c: single_value,
                             || Value::Div(
                                 Box::new(Value::Div(Box::new(a),
                                                     Box::new(b),
                                                     s1.is_some(),
                                                     s2.is_some())),
                                 Box::new(c),
                                 s3.is_some(),
                                 s4.is_some()))) |
            complete!(chain!(a: single_value ~
                             s1: multispace? ~ tag!("/") ~ s2: multispace? ~
                             b: single_value,
                             || Value::Div(Box::new(a), Box::new(b),
                                           s1.is_some(), s2.is_some()))) |
            single_value));

named!(single_value<&[u8], Value>,
       alt_complete!(
           chain!(r: is_a!("0123456789") ~ u: unit?,
                  || Value::Numeric(
                      Rational::from_str(from_utf8(r).unwrap()).unwrap(),
                      u.unwrap_or(Unit::None),
                      false,
                      )) |
           chain!(tag!("$") ~ name: alphanumeric,
                  || Value::Variable(from_utf8(name).unwrap().to_string())) |
           chain!(tag!("#") ~ r: hexchar2 ~ g: hexchar2 ~ b: hexchar2,
                  || Value::HexColor(from_hex(r),
                                     from_hex(g),
                                     from_hex(b),
                                     Some(format!("#{}{}{}",
                                                  from_utf8(r).unwrap(),
                                                  from_utf8(g).unwrap(),
                                                  from_utf8(b).unwrap())))) |
           chain!(tag!("#") ~ r: hexchar ~ g: hexchar ~ b: hexchar,
                  || Value::HexColor(from_hex(r) * 0x11,
                                     from_hex(g) * 0x11,
                                     from_hex(b) * 0x11,
                                     Some(format!("#{}{}{}",
                                                  from_utf8(r).unwrap(),
                                                  from_utf8(g).unwrap(),
                                                  from_utf8(b).unwrap())))) |
           chain!(tag!("rgb(") ~ multispace? ~
                  r: digit ~ tag!(",") ~ multispace? ~
                  g: digit ~ tag!(",") ~ multispace? ~
                  b: digit ~ multispace? ~ tag!(")"),
                  || Value::HexColor(from_dec(r),
                                     from_dec(g),
                                     from_dec(b),
                                     None)) |
           chain!(name: alphanumeric ~ tag!("(") ~
                  multispace? ~ arg: value_expression ~ multispace? ~
                  tag!(")"),
                  || Value::Call(from_utf8(name).unwrap().into(),
                                 Box::new(arg))) |
           chain!(val: is_not!("+-*/;,$()! \n\t"),
                  || Value::Literal(from_utf8(val).unwrap().to_string())) |
           chain!(tag!("(") ~ multispace? ~
                  val: value_expression ~ multispace? ~
                  tag!(")"),
                  || Value::Paren(Box::new(val)))));

named!(unit<&[u8], Unit>,
       alt!(value!(Unit::Percent, tag!("%")) |
            value!(Unit::Pt, tag!("pt")) |
            value!(Unit::Px, tag!("px")) |
            value!(Unit::Em, tag!("em")) |
            value!(Unit::Ex, tag!("ex")) |
            value!(Unit::None, tag!(""))));

named!(hexchar<&[u8], &[u8]>,
       recognize!(one_of!("0123456789ABCDEFabcdef")));

named!(hexchar2<&[u8], &[u8]>,
       recognize!(chain!(one_of!("0123456789ABCDEFabcdef") ~
                         one_of!("0123456789ABCDEFabcdef"),
                         || ()))
       );

fn from_hex(v: &[u8]) -> u8 {
    u8::from_str_radix(from_utf8(v).unwrap(), 16).unwrap()
}

fn from_dec(v: &[u8]) -> u8 {
    u8::from_str(from_utf8(v).unwrap()).unwrap()
}

#[cfg(test)]
mod test {
    use nom::IResult::*;
    use num_rational::Rational;
    use valueexpression::*;

    #[test]
    fn simple_number() {
        assert_eq!(value_expression(b"4;"),
                   Done(&b";"[..],
                        Value::Numeric (
                            Rational::from_integer(4),
                            Unit::None,
                            false,
                        )))
    }

    #[test]
    fn simple_value_literal() {
        assert_eq!(value_expression(b"red;"),
                   Done(&b";"[..], Value::Literal("red".into())))
    }

    #[test]
    fn simple_value_variable() {
        assert_eq!(value_expression(b"$red;"),
                   Done(&b";"[..], Value::Variable("red".into())))
    }
    #[test]
    fn paren_literal() {
        assert_eq!(value_expression(b"(red);"),
                   Done(&b";"[..],
                        Value::Paren(Box::new(Value::Literal("red".into())))))
    }

    #[test]
    fn paren_multi() {
        assert_eq!(value_expression(b"(red blue);"),
                   Done(&b";"[..],
                        Value::Paren(Box::new(
                            Value::MultiSpace(vec![
                                Value::Literal("red".into()),
                                Value::Literal("blue".into())])
                                ))))
    }

    #[test]
    fn multi_expression() {
        assert_eq!(value_expression(b"15/10 2 3;"),
                   Done(&b";"[..],
                        Value::MultiSpace(vec![
                            Value::Div(Box::new(scalar(15)),
                                       Box::new(scalar(10)),
                                       false,
                                       false),
                            scalar(2),
                            scalar(3)])))
    }

    #[test]
    fn color_short() {
        assert_eq!(value_expression(b"#AbC;"),
                   Done(&b";"[..],
                        Value::HexColor(170, 187, 204, Some("#AbC".into()))))
    }

    #[test]
    fn color_long() {
        assert_eq!(value_expression(b"#AaBbCc;"),
                   Done(&b";"[..],
                        Value::HexColor(170, 187, 204, Some("#AaBbCc".into()))))
    }

    #[test]
    fn color_rgb() {
        assert_eq!(value_expression(b"rgb(1,1,1);"),
                   Done(&b";"[..],
                        Value::HexColor(1, 1, 1, None))) // TODO Origin!
    }

    fn scalar(v: isize) -> Value {
        Value::Numeric(Rational::from_integer(v), Unit::None, false)
    }
}
