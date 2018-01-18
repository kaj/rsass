use nom::multispace;
use num_rational::Rational;
use num_traits::{One, Zero};
use ordermap::OrderMap;
use parser::formalargs::call_args;
use parser::strings::{sass_string, sass_string_dq, sass_string_ext,
                      sass_string_sq};
use parser::unit::unit;
use parser::util::{name, opt_spacelike, spacelike2};
use sass::{SassString, Value};
use std::str::{FromStr, from_utf8};
use value::{name_to_rgb, ListSeparator, Operator, Unit};

named!(pub value_expression<&[u8], Value>,
       do_parse!(
           result: separated_nonempty_list!(
               complete!(do_parse!(tag!(",") >> opt_spacelike >> ())),
               space_list) >>
           trail: many0!(do_parse!(opt_spacelike >> tag!(",") >>
                                   opt_spacelike >>
                                   ())) >>
           (if result.len() == 1 && trail.is_empty() {
               result.into_iter().next().unwrap()
           } else {
               Value::List(result, ListSeparator::Comma, false, false)
           })));

named!(pub space_list<&[u8], Value>,
       do_parse!(first: se_or_ext_string >>
                 list: fold_many0!(
                     pair!(opt!(multispace), se_or_ext_string),
                     (vec![first], false),
                     |(mut list, mut unreq): (Vec<Value>, bool), (s, item)| {
                         let mut appended = false;
                         if let (None, &Value::Literal(ref s2)) = (s, &item) {
                             if let Some(&mut Value::Literal(ref mut s1)) =
                                 list.last_mut()
                             {
                                 if s1.is_unquoted() && s2.is_unquoted() {
                                     s1.append(s2);
                                     appended = true;
                                 } else {
                                     unreq = true;
                                 }
                             } else {
                                 unreq = true;
                             }
                         }
                         if !appended {
                             list.push(item);
                         }
                         (list, unreq)
                     }) >>
                 (if list.0.len() == 1 {
                     list.0.into_iter().next().unwrap()
                 } else {
                     Value::List(list.0, ListSeparator::Space, false, list.1)
                 })));

named!(
    se_or_ext_string<Value>,
    alt_complete!(single_expression | map!(sass_string_ext, Value::Literal))
);

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

named!(
    term_value<Value>,
    do_parse!(
        a: single_value
            >> r:
                fold_many0!(
                    do_parse!(
                        s1: opt!(multispace)
                            >> op: alt_complete!(tag!("*") | tag!("/"))
                            >> s2: opt!(multispace)
                            >> b: opt!(single_value)
                            >> (s1.is_some(), op, s2.is_some(), b)
                    ),
                    a,
                    |a, (s1, op, s2, b)| {
                        let b: Option<Value> = b;
                        let b = b.unwrap_or(Value::Null);
                        if op == b"*" {
                            Value::BinOp(
                                Box::new(a),
                                Operator::Multiply,
                                Box::new(b),
                            )
                        } else {
                            Value::Div(Box::new(a), Box::new(b), s1, s2)
                        }
                    }
                ) >> (r)
    )
);

named!(pub single_value<&[u8], Value>,
       alt_complete!(
           value!(Value::True, tag!("true")) |
           value!(Value::False, tag!("false")) |
           do_parse!(tag!("[") >>
                     content: opt!(value_expression) >>
                     tag!("]") >>
                     (match content {
                         Some(Value::List(list, sep, false, _)) => {
                             Value::List(list, sep, true, false)
                         }
                         Some(single) => {
                             Value::List(vec![single],
                                         ListSeparator::Space,
                                         true,
                                         false)
                         }
                         None => {
                             Value::List(vec![],
                                         ListSeparator::Space,
                                         true,
                                         false)
                         }
                     })) |
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
           value!(Value::Null, tag!("null")) |
           // Really ugly special case ... sorry.
           value!(Value::Literal("-null".into()), tag!("-null")) |
           do_parse!(op: alt!(value!(Operator::Minus, tag!("-")) |
                              value!(Operator::Plus, tag!("+")) |
                              value!(Operator::Not,
                                     terminated!(tag!("not"),
                                                 spacelike2))) >>
                     opt_spacelike >>
                     v: single_value >>
                     (Value::UnaryOp(op, Box::new(v)))) |
           function_call |
           map!(sass_string, literal_or_color) |
           map!(sass_string_dq, Value::Literal) |
           map!(sass_string_sq, Value::Literal) |
           dictionary |
           map!(delimited!(preceded!(tag!("("), opt_spacelike),
                           opt!(value_expression),
                           terminated!(opt_spacelike, tag!(")"))),
                |val: Option<Value>| match val {
                    Some(v) => Value::Paren(Box::new(v)),
                    None => {
                        Value::List(vec![], ListSeparator::Space, false, false)
                    }
                })));

named!(
    variable<Value>,
    do_parse!(tag!("$") >> name: name >> (Value::Variable(name)))
);

named!(pub function_call<Value>,
       do_parse!(name: sass_string >> args: call_args >>
                 (Value::Call(name, args))));

fn literal_or_color(s: SassString) -> Value {
    if let Some(val) = s.single_raw() {
        if let Some((r, g, b)) = name_to_rgb(val) {
            return Value::Color(
                r,
                g,
                b,
                Rational::one(),
                Some(val.to_string()),
            );
        }
    }
    Value::Literal(s)
}

fn decimals_to_rational(d: &[u8]) -> Rational {
    Rational::new(
        from_utf8(d).unwrap().parse().unwrap(),
        10_isize.pow(d.len() as u32),
    )
}

named!(hexchar, recognize!(one_of!("0123456789ABCDEFabcdef")));

named!(
    hexchar2,
    recognize!(do_parse!(
        one_of!("0123456789ABCDEFabcdef") >> one_of!("0123456789ABCDEFabcdef")
            >> ()
    ))
);

fn from_hex(v: &[u8]) -> Rational {
    Rational::from_integer(u8::from_str_radix(from_utf8(v).unwrap(), 16)
        .unwrap() as isize)
}

named!(pub dictionary<Value>,
       map!(delimited!(preceded!(tag!("("), opt_spacelike),
                       separated_nonempty_list!(
                           delimited!(opt_spacelike, tag!(","),
                                      opt_spacelike),
                           do_parse!(k: single_value >>
                                     opt_spacelike >>
                                     tag!(":") >>
                                     opt_spacelike >>
                                     v: space_list >>
                                     (k, v))),
                       terminated!(opt_spacelike, tag!(")"))),
            |items| {
                let mut map = OrderMap::new();
                for (k, v) in items {
                    map.insert(k, v);
                }
                Value::Map(map)
            }));

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::*;
    use num_rational::Rational;
    use num_traits::{One, Zero};
    use sass::CallArgs;
    use sass::Value::*;
    use variablescope::GlobalScope;

    #[test]
    fn simple_number() {
        check_expr("4;", number(4, 1))
    }

    #[test]
    fn simple_number_neg() {
        check_expr("-4;", number(-4, 1))
    }

    #[test]
    fn simple_number_pos() {
        check_expr("+4;", Numeric(Rational::new(4, 1), Unit::None, true, false))
    }

    #[test]
    fn simple_number_dec() {
        check_expr("4.34;", number(434, 100))
    }
    #[test]
    fn simple_number_onlydec() {
        check_expr(".34;", number(34, 100))
    }
    #[test]
    fn simple_number_onlydec_neg() {
        check_expr("-.34;", number(-34, 100))
    }
    #[test]
    fn simple_number_onlydec_pos() {
        check_expr(
            "+.34;",
            Numeric(Rational::new(34, 100), Unit::None, true, false),
        )
    }

    fn number(nom: isize, denom: isize) -> Value {
        Numeric(Rational::new(nom, denom), Unit::None, false, false)
    }

    #[test]
    fn simple_value_literal() {
        check_expr("rad;", Literal("rad".into()))
    }

    #[test]
    fn simple_value_literal_color() {
        check_expr(
            "red;",
            Color(
                Rational::new(255, 1),
                Rational::zero(),
                Rational::zero(),
                Rational::one(),
                Some("red".into()),
            ),
        )
    }

    #[test]
    fn simple_value_variable() {
        check_expr("$red;", Variable("red".into()))
    }

    #[test]
    fn paren_literal() {
        check_expr("(rad);", Paren(Box::new(Literal("rad".into()))))
    }

    #[test]
    fn paren_multi() {
        check_expr(
            "(rod bloe);",
            Paren(Box::new(List(
                vec![Literal("rod".into()), Literal("bloe".into())],
                ListSeparator::Space,
                false,
                false,
            ))),
        )
    }

    #[test]
    fn paren_multi_comma() {
        check_expr(
            "(rod, bloe);",
            Paren(Box::new(List(
                vec![Literal("rod".into()), Literal("bloe".into())],
                ListSeparator::Comma,
                false,
                false,
            ))),
        )
    }

    #[test]
    fn multi_comma() {
        check_expr(
            "rod, bloe;",
            List(
                vec![Literal("rod".into()), Literal("bloe".into())],
                ListSeparator::Comma,
                false,
                false,
            ),
        )
    }

    #[test]
    fn paren_multi_comma_trailing() {
        check_expr(
            "(rod, bloe, );",
            Paren(Box::new(List(
                vec![Literal("rod".into()), Literal("bloe".into())],
                ListSeparator::Comma,
                false,
                false,
            ))),
        )
    }

    #[test]
    fn multi_comma_trailing() {
        check_expr(
            "rod, bloe, ;",
            List(
                vec![Literal("rod".into()), Literal("bloe".into())],
                ListSeparator::Comma,
                false,
                false,
            ),
        )
    }

    #[test]
    fn call_no_args() {
        check_expr("foo();", Call("foo".into(), CallArgs::default()))
    }

    #[test]
    fn call_one_arg() {
        check_expr(
            "foo(17);",
            Call("foo".into(), CallArgs::new(vec![(None, Value::scalar(17))])),
        )
    }

    #[test]
    fn multi_expression() {
        check_expr(
            "15/10 2 3;",
            List(
                vec![
                    Div(
                        Box::new(Value::scalar(15)),
                        Box::new(Value::scalar(10)),
                        false,
                        false,
                    ),
                    Value::scalar(2),
                    Value::scalar(3),
                ],
                ListSeparator::Space,
                false,
                false,
            ),
        )
    }

    #[test]
    fn double_div() {
        check_expr(
            "15/5/3;",
            Div(
                Box::new(Div(
                    Box::new(Value::scalar(15)),
                    Box::new(Value::scalar(5)),
                    false,
                    false,
                )),
                Box::new(Value::scalar(3)),
                false,
                false,
            ),
        )
    }

    #[test]
    fn color_short() {
        check_expr(
            "#AbC;",
            Color(
                Rational::new(170, 1),
                Rational::new(187, 1),
                Rational::new(204, 1),
                Rational::one(),
                Some("#AbC".into()),
            ),
        )
    }

    #[test]
    fn color_long() {
        check_expr(
            "#AaBbCc;",
            Color(
                Rational::new(170, 1),
                Rational::new(187, 1),
                Rational::new(204, 1),
                Rational::one(),
                Some("#AaBbCc".into()),
            ),
        )
    }

    #[test]
    fn parse_bracket_array() {
        check_expr(
            "[foo bar];",
            List(
                vec![Literal("foo".into()), Literal("bar".into())],
                ListSeparator::Space,
                true,
                false,
            ),
        )
    }

    #[test]
    fn parse_bracket_comma_array() {
        check_expr(
            "[foo, bar];",
            List(
                vec![Literal("foo".into()), Literal("bar".into())],
                ListSeparator::Comma,
                true,
                false,
            ),
        )
    }

    #[test]
    fn parse_bracket_empty_array() {
        check_expr("[];", List(vec![], ListSeparator::Space, true, false))
    }

    #[test]
    fn map_nq() {
        check_expr(
            "(foo: bar, baz: 17);",
            Map(vec![
                (Literal("foo".into()), Literal("bar".into())),
                (Literal("baz".into()), Value::scalar(17)),
            ].into_iter()
                .collect()),
        )
    }

    fn check_expr(expr: &str, value: Value) {
        assert_eq!(value_expression(expr.as_bytes()), Done(&b";"[..], value))
    }

    #[test]
    fn parse_extended_literal() {
        let t = value_expression_eof(b"http://#{\")\"}.com/");
        if let &Done(rest, ref result) = &t {
            assert_eq!(
                (format!("{}", result.evaluate(&GlobalScope::new())), rest),
                ("http://).com/".to_string(), &b""[..])
            );
        } else {
            assert_eq!(format!("{:?}", t), "Done")
        }
    }
    #[test]
    fn parse_extended_literal_in_arg() {
        let t = value_expression_eof(b"url(http://#{\")\"}.com/)");
        if let &Done(rest, ref result) = &t {
            assert_eq!(
                (format!("{}", result.evaluate(&GlobalScope::new())), rest),
                ("url(http://).com/)".to_string(), &b""[..])
            );
        } else {
            assert_eq!(format!("{:?}", t), "Done")
        }
    }
    #[test]
    fn parse_extended_literal_in_arg_2() {
        let t = value_expression_eof(b"url(//#{\")\"}.com/)");
        if let &Done(rest, ref result) = &t {
            assert_eq!(
                (format!("{}", result.evaluate(&GlobalScope::new())), rest),
                ("url(//).com/)".to_string(), &b""[..])
            );
        } else {
            assert_eq!(format!("{:?}", t), "Done")
        }
    }

    named!(
        value_expression_eof<Value>,
        terminated!(value_expression, eof!())
    );

}
