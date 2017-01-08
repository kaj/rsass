use colors::{name_to_rgb, rgb_to_name};
use formalargs::{CallArgs, call_args};
use nom::multispace;
use num_rational::Rational;
use parseutil::name;
use std::fmt;
use std::str::{FromStr, from_utf8};

/// A sass value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    /// A call has a name and an argument (which may be multi).
    Call(String, CallArgs),
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
    Color(u8, u8, u8, Rational, Option<String>),
    Null,
}

impl Value {
    pub fn is_calculated(&self) -> bool {
        match self {
            &Value::Numeric(_, _, calculated) => calculated,
            &Value::Color(_, _, _, _, None) => true,
            _ => false,
        }
    }

    /// All values other than `false` and `null` should be considered true.
    /// Not properly implemented yet, because if limited Value type.
    pub fn is_true(&self) -> bool {
        match self {
            &Value::Literal(ref s) => s != "false",
            _ => true,
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
                write!(out, "{}{}", rational2str(v, false), u)
            }
            &Value::Color(ref r, ref g, ref b, ref a, ref s) => {
                match s {
                    &Some(ref s) => write!(out, "{}", s),
                    &None => {
                        if a >= &Rational::from_integer(1) {
                            match rgb_to_name(*r, *g, *b) {
                                Some(name) => write!(out, "{}", name),
                                None => {
                                    write!(out, "#{:02x}{:02x}{:02x}", r, g, b)
                                }
                            }
                        } else {
                            write!(out,
                                   "rgba({}, {}, {}, {})",
                                   r,
                                   g,
                                   b,
                                   rational2str(a, false))
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

fn rational2str(r: &Rational, skipzero: bool) -> String {
    let n = r.numer().clone();
    let d = r.denom().clone();
    let mut result = format!("{}", n as f64 / d as f64);
    if skipzero && result.starts_with("0.") {
        result.remove(0);
    }
    result
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
       do_parse!(a: term_value >>
                 r: fold_many0!(
                     do_parse!(opt!(multispace) >>
                               op: alt_complete!(tag!("+") | tag!("-")) >>
                               opt!(multispace) >>
                               b: term_value >>
                               (op, b)),
                     a,
                     |a, (op, b)| {
                         if op == b"+" {
                             Value::Sum(Box::new(a), Box::new(b))
                         } else {
                             Value::Minus(Box::new(a), Box::new(b))
                         }
                     }) >>
                 (r)));

named!(term_value<Value>,
       do_parse!(a: single_value >>
                 r: fold_many0!(
                     do_parse!(s1: opt!(multispace) >>
                               op: alt_complete!(tag!("*") | tag!("/")) >>
                               s2: opt!(multispace) >>
                               b: single_value >>
                               (s1.is_some(), op, s2.is_some(), b)),
                     a,
                     |a, (s1, op, s2, b)| {
                         if op == b"*" {
                             Value::Product(Box::new(a), Box::new(b))
                         } else {
                             Value::Div(Box::new(a), Box::new(b), s1, s2)
                         }
                     }) >>
                 (r)));

named!(single_value<&[u8], Value>,
       alt_complete!(
           chain!(r: is_a!("0123456789") ~
                  d: opt!(preceded!(tag!("."), is_a!("0123456789"))) ~
                  u: unit?,
                  || {
                      let mut n =
                          Rational::from_str(from_utf8(r).unwrap()).unwrap();
                      if let Some(d) = d {
                          let ten: isize = 10;
                          n = n +
                              Rational::from_str(from_utf8(d).unwrap()).unwrap()
                              / Rational::from_integer(ten.pow(d.len() as u32));
                      }
                      Value::Numeric(n,
                                     u.unwrap_or(Unit::None),
                                     false)
                  }) |
           chain!(tag!(".") ~
                  d: is_a!("0123456789") ~
                  u: unit?,
                  || {
                      let ten: isize = 10;
                      let n = Rational::from_str(from_utf8(d).unwrap()).unwrap()
                          / Rational::from_integer(ten.pow(d.len() as u32));
                      Value::Numeric(n,
                                     u.unwrap_or(Unit::None),
                                     false)
                  }) |
           chain!(tag!("$") ~ name: name, || Value::Variable(name)) |
           chain!(tag!("#") ~ r: hexchar2 ~ g: hexchar2 ~ b: hexchar2,
                  || Value::Color(from_hex(r),
                                  from_hex(g),
                                  from_hex(b),
                                  Rational::from_integer(1),
                                  Some(format!("#{}{}{}",
                                               from_utf8(r).unwrap(),
                                               from_utf8(g).unwrap(),
                                               from_utf8(b).unwrap())))) |
           chain!(tag!("#") ~ r: hexchar ~ g: hexchar ~ b: hexchar,
                  || Value::Color(from_hex(r) * 0x11,
                                  from_hex(g) * 0x11,
                                  from_hex(b) * 0x11,
                                  Rational::from_integer(1),
                                  Some(format!("#{}{}{}",
                                               from_utf8(r).unwrap(),
                                               from_utf8(g).unwrap(),
                                               from_utf8(b).unwrap())))) |
           chain!(name: name ~ args: call_args, || Value::Call(name, args)) |
           chain!(val: is_not!("+-*/;,$()! \n\t"),
                  || {
                      let val = from_utf8(val).unwrap().to_string();
                      if let Some((r, g, b)) = name_to_rgb(&val) {
                          Value::Color(r, g, b, Rational::from_integer(1),
                                       Some(val))
                      } else {
                          Value::Literal(val)
                      }
                  }) |
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
        assert_eq!(value_expression(b"rad;"),
                   Done(&b";"[..], Value::Literal("rad".into())))
    }

    #[test]
    fn simple_value_literal_color() {
        let one = Rational::from_integer(1);
        assert_eq!(value_expression(b"red;"),
                   Done(&b";"[..],
                        Value::Color(0xff, 0, 0, one, Some("red".into()))))
    }

    #[test]
    fn simple_value_variable() {
        assert_eq!(value_expression(b"$red;"),
                   Done(&b";"[..], Value::Variable("red".into())))
    }
    #[test]
    fn paren_literal() {
        assert_eq!(value_expression(b"(rad);"),
                   Done(&b";"[..],
                        Value::Paren(Box::new(Value::Literal("rad".into())))))
    }

    #[test]
    fn paren_multi() {
        assert_eq!(value_expression(b"(rod bloe);"),
                   Done(&b";"[..],
                        Value::Paren(Box::new(
                            Value::MultiSpace(vec![
                                Value::Literal("rod".into()),
                                Value::Literal("bloe".into())])
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
    fn double_div() {
        assert_eq!(value_expression(b"15/5/3;"),
                   Done(&b";"[..],
                        Value::Div(
                            Box::new(Value::Div(Box::new(scalar(15)),
                                                Box::new(scalar(5)),
                                                false,
                                                false)),
                            Box::new(scalar(3)),
                            false,
                            false)))
    }

    #[test]
    fn color_short() {
        assert_eq!(value_expression(b"#AbC;"),
                   Done(&b";"[..],
                        Value::Color(170, 187, 204,
                                        Rational::from_integer(1),
                                        Some("#AbC".into()))))
    }

    #[test]
    fn color_long() {
        assert_eq!(value_expression(b"#AaBbCc;"),
                   Done(&b";"[..],
                        Value::Color(170, 187, 204,
                                        Rational::from_integer(1),
                                        Some("#AaBbCc".into()))))
    }

    fn scalar(v: isize) -> Value {
        Value::Numeric(Rational::from_integer(v), Unit::None, false)
    }
}
