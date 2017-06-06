
use colors::name_to_rgb;
use nom::multispace;
use num_rational::Rational;
use num_traits::Zero;
use parser::formalargs::call_args;
use parser::unit::unit;
use parser::util::{is_name_char, name, opt_spacelike, spacelike2};
use std::str::{FromStr, from_utf8};
use unit::Unit;
use value::{ListSeparator, Quotes, Value};
use value::operator::Operator;

named!(pub value_expression<&[u8], Value>,
       do_parse!(
           result: separated_nonempty_list!(
               do_parse!(tag!(",") >> opt_spacelike >> ()),
               space_list) >>
           trail: many0!(do_parse!(opt_spacelike >> tag!(",") >>
                                   opt_spacelike >>
                                   ())) >>
           (if result.len() == 1 && trail.is_empty() {
               result.into_iter().next().unwrap()
           } else {
               Value::List(result, ListSeparator::Comma)
           })));

named!(pub space_list<&[u8], Value>,
       do_parse!(first: single_expression >>
                 list: fold_many0!(
                     preceded!(opt!(multispace), single_expression),
                     vec![first],
                     |mut list: Vec<Value>, item| { list.push(item); list }) >>
                 (if list.len() == 1 {
                     list.into_iter().next().unwrap()
                 } else {
                     Value::List(list, ListSeparator::Space)
                 })));

named!(pub single_expression<Value>,
       do_parse!(a: logic_expression >>
                 r: fold_many0!(
                     do_parse!(opt!(multispace) >>
                               op: alt_complete!(
                                   value!(Operator::And,
                                          preceded!(tag!("and"),
                                                    spacelike2)) |
                                   value!(Operator::Or,
                                          preceded!(tag!("or"),
                                                    spacelike2))) >>
                               opt!(multispace) >>
                               b: single_expression >>
                               (op, b)),
                     a,
                     |a, (op, b)| Value::BinOp(Box::new(a), op, Box::new(b))) >>
                 (r)));

named!(pub logic_expression<Value>,
       do_parse!(a: sum_expression >>
                 r: fold_many0!(
                     do_parse!(opt!(multispace) >>
                               op: alt_complete!(
                                   value!(Operator::Equal, tag!("==")) |
                                   value!(Operator::NotEqual, tag!("!=")) |
                                   value!(Operator::GreaterE, tag!(">=")) |
                                   value!(Operator::Greater, tag!(">")) |
                                   value!(Operator::LesserE, tag!("<=")) |
                                   value!(Operator::Lesser, tag!("<"))) >>
                               opt!(multispace) >>
                               b: sum_expression >>
                               (op, b)),
                     a,
                     |a, (op, b)| Value::BinOp(Box::new(a), op, Box::new(b))) >>
                 (r)));

named!(pub sum_expression<Value>,
       do_parse!(a: term_value >>
                 r: fold_many0!(
                     alt_complete!(
                         do_parse!(op: alt_complete!(
                                       value!(Operator::Plus, tag!("+")) |
                                       value!(Operator::Minus, tag!("-"))) >>
                                   opt!(spacelike2) >>
                                   b: term_value >>
                                   (op, b)) |
                         do_parse!(spacelike2 >>
                                   op: alt_complete!(
                                       value!(Operator::Plus, tag!("+")) |
                                       value!(Operator::Minus, tag!("-"))) >>
                                   spacelike2 >>
                                   b: term_value >>
                                   (op, b))),
                     a,
                     |a, (op, b)| Value::BinOp(Box::new(a), op, Box::new(b))) >>
                 (r)));

named!(term_value<Value>,
       do_parse!(a: single_value >>
                 r: fold_many0!(
                     do_parse!(s1: opt!(multispace) >>
                               op: alt_complete!(tag!("*") | tag!("/")) >>
                               s2: opt!(multispace) >>
                               b: opt!(single_value) >>
                               (s1.is_some(), op, s2.is_some(), b)),
                     a,
                     |a, (s1, op, s2, b)| {
                         let b: Option<Value> = b;
                         let b = b.unwrap_or(Value::Null);
                         if op == b"*" {
                             Value::BinOp(Box::new(a),
                                          Operator::Multiply,
                                          Box::new(b))
                         } else {
                             Value::Div(Box::new(a), Box::new(b), s1, s2)
                         }
                     }) >>
                 (r)));

named!(pub single_value<&[u8], Value>,
       alt_complete!(
           value!(Value::True, tag!("true")) |
           value!(Value::False, tag!("false")) |
           do_parse!(sign: opt!(alt!(tag!("-") | tag!("+"))) >>
                     r: is_a!("0123456789") >>
                     d: opt!(preceded!(tag!("."), is_a!("0123456789"))) >>
                     u: opt!(unit) >>
                     (Value::Numeric(
                         {
                             let d = Rational::from_str(
                                 from_utf8(r).unwrap()).unwrap() +
                                 d.map(decimals_to_rational)
                                 .unwrap_or_else(Rational::zero);
                             if sign == Some(b"-") { -d } else { d }
                         },
                         u.unwrap_or(Unit::None),
                         sign == Some(b"+"),
                         false))) |
           do_parse!(sign: opt!(alt!(tag!("-") | tag!("+"))) >>
                     tag!(".") >>
                     d: is_a!("0123456789") >>
                     u: opt!(unit) >>
                     (Value::Numeric(
                         {
                             let d = decimals_to_rational(d);
                             if sign == Some(b"-") { -d } else { d }
                         },
                         u.unwrap_or(Unit::None),
                         sign == Some(b"+"),
                         false))) |
           variable |
           do_parse!(tag!("#") >> r: hexchar2 >> g: hexchar2 >> b: hexchar2 >>
                     (Value::Color(from_hex(r),
                                   from_hex(g),
                                   from_hex(b),
                                   Rational::from_integer(1),
                                   Some(format!("#{}{}{}",
                                                from_utf8(r).unwrap(),
                                                from_utf8(g).unwrap(),
                                                from_utf8(b).unwrap()))))) |
           do_parse!(tag!("#") >> r: hexchar >> g: hexchar >> b: hexchar >>
                     (Value::Color(from_hex(r) * Rational::new(17, 1),
                                   from_hex(g) * Rational::new(17, 1),
                                   from_hex(b) * Rational::new(17, 1),
                                   Rational::from_integer(1),
                                   Some(format!("#{}{}{}",
                                                from_utf8(r).unwrap(),
                                                from_utf8(g).unwrap(),
                                                from_utf8(b).unwrap()))))) |
           // Really ugly special case ... sorry.
           value!(Value::Literal("-null".into(), Quotes::None), tag!("-null")) |
           do_parse!(op: alt!(value!(Operator::Minus, tag!("-")) |
                              value!(Operator::Plus, tag!("+")) |
                              value!(Operator::Not,
                                     terminated!(tag!("not"),
                                                 spacelike2))) >>
                     opt_spacelike >>
                     v: single_value >>
                     (Value::UnaryOp(op, Box::new(v)))) |
           function_call |
           unquoted_literal |
           map!(tag!("\"\""),
                |_| Value::Literal("".into(), Quotes::Double)) |
           quoted_string |
           map!(tag!("''"),
                |_| Value::Literal("".into(), Quotes::Single)) |
           singlequoted_string |
           map!(delimited!(preceded!(tag!("("), opt_spacelike),
                           opt!(value_expression),
                           terminated!(opt_spacelike, tag!(")"))),
                |val: Option<Value>| match val {
                    Some(v) => Value::Paren(Box::new(v)),
                    None => Value::List(vec![], ListSeparator::Space),
                })));

named!(variable<Value>,
       do_parse!(tag!("$") >>  name: name >> (Value::Variable(name))));

named!(pub interpolation<Value>,
       map!(delimited!(tag!("#{"), value_expression, tag!("}")),
            |v| Value::Interpolation(Box::new(v))));

named!(unquoted_literal<Value>,
       do_parse!(t: alt!(interpolation | unquoted_literal_part) >>
                 first: expr_res!(ok_as_literal(t)) >>
                 all: fold_many0!(
                     alt!(interpolation | function_call |
                          unquoted_literal_part |
                          map!(preceded!(tag!("//"),
                                         take_while!(is_ext_str_char)),
                               |v| Value::Literal(
                                   format!("//{}", from_utf8(v).unwrap()),
                                   Quotes::None))),
                     first,
                     |a, b| {
                         Value::BinOp(Box::new(a), Operator::Plus, Box::new(b))
                     }) >>
                 (all)));

fn ok_as_literal(s: Value) -> Result<Value, bool> {
    if s != Value::Literal("-".into(), Quotes::None) {
        Ok(s)
    } else {
        Err(false)
    }
}

named!(pub extended_literal<Value>,
       map!(take_while1!(is_ext_str_char),
            |v| Value::Literal(from_utf8(v).unwrap().into(), Quotes::None)));

named!(pub function_call<Value>,
       do_parse!(name: name >> args: call_args >>
                 (Value::Call(name, args))));

fn is_ext_str_char(c: u8) -> bool {
    is_name_char(c) || c == b'*' || c == b'+' || c == b',' ||
    c == b'.' || c == b'/' || c == b':' || c == b'=' ||
    c == b'?' || c == b'|'
}

named!(unquoted_literal_part<Value>,
       map!(unquoted_literal_part_part,
            |val: String| {
                if val == "null" {
                    Value::Null
                } else if let Some((r, g, b)) = name_to_rgb(&val) {
                    Value::Color(r, g, b, Rational::from_integer(1), Some(val))
                } else {
                    Value::Literal(val, Quotes::None)
                }
            }));

named!(unquoted_literal_part_part<String>,
       switch!(take_backslash,
               true => map!(take!(1),
                            |v| format!("\\{}", from_utf8(v).unwrap())) |
               false => map!(is_not!("\\+*/=;,$(){{}}! \n\t'\"#"),
                             |v| from_utf8(v).unwrap().to_string())));

named!(take_backslash<bool>,
       map!(opt!(tag!("\\")), |v: Option<&[u8]>| v.is_some()));

// a quoted string may contain interpolations
named!(pub quoted_string<Value>,
       do_parse!(tag!("\"") >>
                 first: simple_dqs_part >>
                 all: fold_many0!(
                     alt!(interpolation | nonempty_dqs_part),
                     first,
                     |a, b| {
                         Value::BinOp(Box::new(a), Operator::Plus, Box::new(b))
                     }) >>
                 tag!("\"") >> (all)));

named!(simple_dqs_part<Value>,
       map!(escaped!(is_not!("\\\"#"), '\\', take!(1)),
            |s| Value::Literal(unescape(from_utf8(s).unwrap()),
                               Quotes::Double)));
named!(nonempty_dqs_part<Value>,
       map!(verify!(escaped!(is_not!("\\\"#"), '\\', take!(1)),
                    |s: &[u8]| !s.is_empty()),
            |s| Value::Literal(unescape(from_utf8(s).unwrap()),
                               Quotes::Double)));

// a quoted string may contain interpolations
named!(pub singlequoted_string<Value>,
       do_parse!(tag!("'") >>
                 first: simple_sqs_part >>
                 all: fold_many0!(
                     alt!(interpolation | nonempty_sqs_part),
                     first,
                     |a, b| {
                         Value::BinOp(Box::new(a), Operator::Plus, Box::new(b))
                     }) >>
                 tag!("'") >> (all)));

named!(simple_sqs_part<Value>,
       map!(escaped!(is_not!("\\'#"), '\\', take!(1)),
            |s| Value::Literal(unescape(from_utf8(s).unwrap()),
                               Quotes::Single)));
named!(nonempty_sqs_part<Value>,
       map!(verify!(escaped!(is_not!("\\'#"), '\\', take!(1)),
                    |s: &[u8]| !s.is_empty()),
            |s| Value::Literal(unescape(from_utf8(s).unwrap()),
                               Quotes::Single)));

fn decimals_to_rational(d: &[u8]) -> Rational {
    Rational::new(from_utf8(d).unwrap().parse().unwrap(),
                  10_isize.pow(d.len() as u32))
}

named!(hexchar, recognize!(one_of!("0123456789ABCDEFabcdef")));

named!(hexchar2,
       recognize!(do_parse!(one_of!("0123456789ABCDEFabcdef") >>
                            one_of!("0123456789ABCDEFabcdef") >> ())));

fn from_hex(v: &[u8]) -> Rational {
    Rational::from_integer(u8::from_str_radix(from_utf8(v).unwrap(), 16)
                               .unwrap() as isize)
}

fn unescape(s: &str) -> String {
    let mut i = s.chars();
    let mut result = String::new();
    while let Some(c) = i.next() {
        result.push(match c {
                        '\\' => {
                            match i.next() {
                                Some(c) => c,
                                None => '\\',
                            }
                        }
                        c => c,
                    });
    }
    result
}
