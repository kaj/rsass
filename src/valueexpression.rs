use nom::{alphanumeric, multispace};
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
    Multi(Vec<Value>),
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
}

impl Value {
    pub fn is_calculated(&self) -> bool {
        if let &Value::Numeric(_, _, calculated) = self {
            calculated
        } else {
            false
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
            &Value::Multi(ref v) => {
                let t = v.iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<_>>()
                    .join(" ");
                // NOTE: This is an ugly workaround and should not be needed.
                let t = t.replace(" ,", ",");
                write!(out, "{}", t)
            }
            &Value::Call(ref name, ref arg) => write!(out, "{}({})", name, arg),
            x => write!(out, "TODO {:?}", x),
        }
    }
}

named!(pub value_expression<&[u8], Value>,
       chain!(v: many1!(chain!(expr: single_expression ~ multispace?,
                               || expr)),
              || if v.len() == 1 {
                  v[0].clone()
              } else {
                  Value::Multi(v)
              }));

named!(single_expression<Value>,
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
           chain!(name: alphanumeric ~ tag!("(") ~
                  multispace? ~ arg: value_expression ~ multispace? ~
                  tag!(")"),
                  || Value::Call(from_utf8(name).unwrap().into(),
                                 Box::new(arg))) |
           chain!(tag!("$") ~ name: alphanumeric,
                   || Value::Variable(from_utf8(name).unwrap().to_string())) |
            chain!(val: is_not!("+-*/;$() \n\t"),
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
                            Value::Multi(vec![Value::Literal("red".into()),
                                              Value::Literal("blue".into())])
                                ))))
    }

    #[test]
    fn multi_expression() {
        assert_eq!(value_expression(b"15/10 2 3;"),
                   Done(&b";"[..],
                        Value::Multi(vec![Value::Div(Box::new(scalar(15)),
                                                     Box::new(scalar(10)),
                                                     false,
                                                     false),
                                          scalar(2),
                                          scalar(3)])))
    }
    fn scalar(v: isize) -> Value {
        Value::Numeric(Rational::from_integer(v), Unit::None, false)
    }
}
