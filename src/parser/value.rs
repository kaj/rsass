use super::formalargs::call_args;
use super::strings::{name, sass_string_dq, sass_string_ext, sass_string_sq};
use super::unit::unit;
use super::util::{opt_spacelike, spacelike2};
use super::{input_to_string, sass_string};
use crate::sass::{SassString, Value};
use crate::value::{ListSeparator, Number, Operator, Rgba};
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{multispace0, multispace1, one_of};
use nom::combinator::{map, opt, value};
use nom::sequence::{pair, preceded, terminated, tuple};
use nom::*;
use num_rational::Rational;
use std::str::from_utf8;

named!(pub value_expression<Value>,
       do_parse!(
           result: separated_nonempty_list!(
               complete!(preceded!(tag(","), opt_spacelike)),
               space_list) >>
           trail: many0!(delimited!(opt_spacelike, tag(","),
                                    opt_spacelike)) >>
           (if result.len() == 1 && trail.is_empty() {
               result.into_iter().next().unwrap()
           } else {
               Value::List(result, ListSeparator::Comma, false, false)
           })));

named!(pub space_list<Value>,
       do_parse!(first: se_or_ext_string >>
                 list: fold_many0!(
                     pair!(multispace0, se_or_ext_string),
                     (vec![first], false),
                     |(mut list, mut unreq): (Vec<Value>, bool), (s, item)| {
                         let mut appended = false;
                         if let (b"", &Value::Literal(ref s2)) = (s, &item) {
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
    alt!(single_expression | map!(sass_string_ext, Value::Literal))
);

named!(
    single_expression<Value>,
    do_parse!(
        a: logic_expression
            >> r: fold_many0!(
                pair!(
                    delimited!(
                        multispace0,
                        alt!(
                            value!(Operator::And, tag("and"))
                                | value!(Operator::Or, tag("or"))
                        ),
                        multispace1
                    ),
                    single_expression
                ),
                a,
                |a, (op, b)| Value::BinOp(
                    Box::new(a),
                    false,
                    op,
                    false,
                    Box::new(b)
                )
            )
            >> (r)
    )
);

named!(
    logic_expression<Value>,
    do_parse!(
        a: sum_expression
            >> r: fold_many0!(
                pair!(
                    delimited!(
                        multispace0,
                        alt!(
                            value!(Operator::Equal, tag("=="))
                                | value!(Operator::NotEqual, tag("!="))
                                | value!(Operator::GreaterE, tag(">="))
                                | value!(Operator::Greater, tag(">"))
                                | value!(Operator::LesserE, tag("<="))
                                | value!(Operator::Lesser, tag("<"))
                        ),
                        multispace0
                    ),
                    sum_expression
                ),
                a,
                |a, (op, b)| Value::BinOp(
                    Box::new(a),
                    false,
                    op,
                    false,
                    Box::new(b)
                )
            )
            >> (r)
    )
);

fn sum_expression(input: &[u8]) -> IResult<&[u8], Value> {
    let (mut rest, mut v) = term_value(input)?;
    while let Ok((nrest, (op, v2))) = alt((
        pair(
            alt((
                value(Operator::Plus, tag("+")),
                value(Operator::Minus, tag("-")),
            )),
            preceded(opt(spacelike2), term_value),
        ),
        pair(
            preceded(
                spacelike2,
                alt((
                    value(Operator::Plus, tag("+")),
                    value(Operator::Minus, tag("-")),
                )),
            ),
            preceded(spacelike2, term_value),
        ),
    ))(rest)
    {
        v = Value::BinOp(Box::new(v), false, op, false, Box::new(v2));
        rest = nrest;
    }
    Ok((rest, v))
}

fn term_value(input: &[u8]) -> IResult<&[u8], Value> {
    let (mut rest, mut v) = single_value(input)?;
    while let Ok((nrest, (s1, op, s2, v2))) = tuple((
        map(multispace0, |s: &[u8]| !s.is_empty()),
        alt((
            value(Operator::Multiply, tag(b"*")),
            value(Operator::Div, tag(b"/")),
        )),
        map(multispace0, |s: &[u8]| !s.is_empty()),
        map(opt(single_value), |v: Option<Value>| {
            v.unwrap_or(Value::Null)
        }),
    ))(rest)
    {
        rest = nrest;
        v = Value::BinOp(Box::new(v), s1, op, s2, Box::new(v2));
    }
    Ok((rest, v))
}

named!(pub single_value<&[u8], Value>,
       alt!(
           simple_value |
           delimited!(preceded!(tag("("), opt_spacelike),
                      alt!(
                          dictionary_inner |
                          map!(value_expression,
                               |v| Value::Paren(Box::new(v))) |
                          value!(Value::List(
                              vec![], ListSeparator::Space, false, false
                          ))
                      ),
                      terminated!(opt_spacelike, tag(")")))
               ));

fn simple_value(input: &[u8]) -> IResult<&[u8], Value> {
    let s_v = alt((
        bang,
        value(Value::True, tag("true")),
        value(Value::False, tag("false")),
        value(Value::HereSelector, tag("&")),
        unicode_range,
        bracket_list,
        number,
        variable,
        hex_color,
        value(Value::Null, tag("null")),
        // Really ugly special case ... sorry.
        value(Value::Literal("-null".into()), tag("-null")),
        unary_op,
        function_call,
        // And a bunch of string variants
        map(sass_string, literal_or_color),
        map(sass_string_dq, Value::Literal),
        map(sass_string_sq, Value::Literal),
    ));
    Ok(s_v(input)?)
}

named!(
    bang<Value>,
    map!(
        map_res!(
            preceded!(
                pair!(tag("!"), opt_spacelike),
                tag("important") // TODO Pretty much anythig goes, here?
            ),
            input_to_string
        ),
        Value::Bang
    )
);

named!(
    unicode_range<Value>,
    map!(
        map_res!(
            recognize!(do_parse!(
                call!(tag_no_case("U+"))
                    >> a: many_m_n!(
                        0,
                        6,
                        call!(one_of("0123456789ABCDEFabcdef"))
                    )
                    >> alt!(
                        preceded!(
                            tag("-"),
                            many_m_n!(
                                1,
                                6,
                                call!(one_of("0123456789ABCDEFabcdef"))
                            )
                        ) | many_m_n!(1, 6 - a.len(), call!(one_of("?")))
                            | value!(vec![])
                    )
                    >> ()
            )),
            input_to_string
        ),
        Value::UnicodeRange
    )
);

named!(
    bracket_list<Value>,
    map!(
        delimited!(tag("["), opt!(value_expression), tag("]")),
        |item| match item {
            Some(Value::List(list, sep, false, _)) => {
                Value::List(list, sep, true, false)
            }
            Some(single) => {
                Value::List(vec![single], ListSeparator::Space, true, false)
            }
            None => Value::List(vec![], ListSeparator::Space, true, false),
        }
    )
);

named!(
    number<Value>,
    map!(
        tuple!(
            opt!(alt((tag("-"), tag("+")))),
            alt!(
                map!(
                    pair!(decimal_integer, opt!(decimal_decimals)),
                    |(n, d)| (true, if let Some(d) = d { n + d } else { n })
                ) | map!(decimal_decimals, |dec| (false, dec))
            ),
            unit
        ),
        |(sign, (lead_zero, num), unit)| Value::Numeric(
            Number {
                value: if sign == Some(b"-") { -num } else { num },
                plus_sign: sign == Some(b"+"),
                lead_zero,
            },
            unit,
        )
    )
);

named!(
    decimal_integer<Rational>,
    map!(
        fold_many1!(
            // Note: We should use bytes directly, one_of returns a char.
            // Also, rustc 1.25 and earlier does not have isize::from(u8),
            // so use i8 for bytes.
            one_of("0123456789"),
            0,
            |r, d| r * 10 + isize::from(d as i8 - b'0' as i8)
        ),
        Rational::from_integer
    )
);

named!(
    decimal_decimals<Rational>,
    map!(
        preceded!(
            complete!(tag(".")),
            fold_many1!(one_of!("0123456789"), (0, 1), |(r, n), d| (
                r * 10 + isize::from(d as i8 - b'0' as i8),
                n * 10
            ))
        ),
        |(r, d)| Rational::new(r, d)
    )
);

named!(
    variable<Value>,
    map!(preceded!(tag("$"), name), Value::Variable)
);

named!(
    hex_color<Value>,
    preceded!(
        tag("#"),
        map!(
            alt!(
                tuple!(hexchar2, hexchar2, hexchar2, opt!(hexchar2))
                    | tuple!(hexchar, hexchar, hexchar, opt!(hexchar))
            ),
            |(r, g, b, a): (&[u8], &[u8], &[u8], Option<&[u8]>)| Value::Color(
                Rgba::from_rgba(
                    from_hex(&r),
                    from_hex(&g),
                    from_hex(&b),
                    a.map(|a| from_hex(&a)).unwrap_or(255),
                ),
                Some(format!(
                    "#{}{}{}{}",
                    from_utf8(&r).unwrap(),
                    from_utf8(&g).unwrap(),
                    from_utf8(&b).unwrap(),
                    a.and_then(|a| from_utf8(&a).ok()).unwrap_or(""),
                )),
            )
        )
    )
);

named!(
    unary_op<Value>,
    map!(
        pair!(
            terminated!(
                alt((
                    value(Operator::Plus, tag("+")),
                    value(Operator::Minus, tag("-")),
                    value(Operator::Not, terminated(tag("not"), spacelike2)),
                )),
                opt_spacelike
            ),
            single_value
        ),
        |(op, v)| Value::UnaryOp(op, Box::new(v))
    )
);

named!(
    pub function_call<Value>,
    map!(
        pair!(sass_string, call_args),
        |(name, args)| Value::Call(name, args)
    )
);

fn literal_or_color(s: SassString) -> Value {
    if let Some(val) = s.single_raw() {
        if let Some(rgba) = Rgba::from_name(val) {
            return Value::Color(rgba, Some(val.to_string()));
        }
    }
    Value::Literal(s)
}

named!(hexchar<&[u8]>, recognize!(one_of("0123456789ABCDEFabcdef")));

named!(hexchar2<&[u8]>, recognize!(pair(hexchar, hexchar)));

fn from_hex(v: &[u8]) -> u8 {
    let i = u8::from_str_radix(from_utf8(v).unwrap(), 16).unwrap();
    if v.len() > 1 {
        i
    } else {
        i * 0x11
    }
}

named!(pub dictionary<Value>,
       delimited!(preceded!(tag("("), opt_spacelike),
                  dictionary_inner,
                  terminated!(opt_spacelike, tag(")"))));

named!(
    dictionary_inner<Value>,
    map!(
        terminated!(
            separated_nonempty_list!(
                delimited!(opt_spacelike, tag(","), opt_spacelike),
                pair!(
                    simple_value,
                    preceded!(
                        delimited!(opt_spacelike, tag(":"), opt_spacelike),
                        space_list
                    )
                )
            ),
            opt!(delimited!(opt_spacelike, tag(","), opt_spacelike))
        ),
        |items| Value::Map(items.into_iter().collect())
    )
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::sass::CallArgs;
    use crate::sass::Value::*;
    use crate::value::Unit;
    use crate::variablescope::GlobalScope;
    use num_rational::Rational;

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
        check_expr(
            "+4;",
            Numeric(
                Number {
                    value: Rational::new(4, 1),
                    plus_sign: true,
                    lead_zero: true,
                },
                Unit::None,
            ),
        )
    }

    #[test]
    fn simple_number_dec() {
        check_expr("4.34;", number(434, 100))
    }
    #[test]
    fn simple_number_onlydec() {
        check_expr(
            ".34;",
            Numeric(
                Number {
                    value: Rational::new(34, 100),
                    plus_sign: false,
                    lead_zero: false,
                },
                Unit::None,
            ),
        )
    }
    #[test]
    fn simple_number_onlydec_neg() {
        check_expr(
            "-.34;",
            Numeric(
                Number {
                    value: Rational::new(-34, 100),
                    plus_sign: false,
                    lead_zero: false,
                },
                Unit::None,
            ),
        )
    }
    #[test]
    fn simple_number_onlydec_pos() {
        check_expr(
            "+.34;",
            Numeric(
                Number {
                    value: Rational::new(34, 100), // actually 17/50
                    plus_sign: true,
                    lead_zero: false,
                },
                Unit::None,
            ),
        )
    }

    fn number(nom: isize, denom: isize) -> Value {
        Numeric(Number::from(Rational::new(nom, denom)), Unit::None)
    }

    #[test]
    fn simple_value_literal() {
        check_expr("rad;", Literal("rad".into()))
    }

    #[test]
    fn simple_value_literal_color() {
        check_expr(
            "red;",
            Color(Rgba::from_rgb(255, 0, 0), Some("red".into())),
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
            Call(
                "foo".into(),
                CallArgs::new(vec![(None, Value::scalar(17))]),
            ),
        )
    }

    #[test]
    fn multi_expression() {
        check_expr(
            "15/10 2 3;",
            List(
                vec![
                    BinOp(
                        Box::new(Value::scalar(15)),
                        false,
                        Operator::Div,
                        false,
                        Box::new(Value::scalar(10)),
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
            BinOp(
                Box::new(BinOp(
                    Box::new(Value::scalar(15)),
                    false,
                    Operator::Div,
                    false,
                    Box::new(Value::scalar(5)),
                )),
                false,
                Operator::Div,
                false,
                Box::new(Value::scalar(3)),
            ),
        )
    }

    #[test]
    fn color_short() {
        check_expr(
            "#AbC;",
            Color(Rgba::from_rgb(0xaa, 0xbb, 0xcc), Some("#AbC".into())),
        )
    }

    #[test]
    fn color_long() {
        check_expr(
            "#AaBbCc;",
            Color(Rgba::from_rgb(0xaa, 0xbb, 0xcc), Some("#AaBbCc".into())),
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
            ]
            .into_iter()
            .collect()),
        )
    }

    fn check_expr(expr: &str, value: Value) {
        assert_eq!(value_expression(expr.as_bytes()), Ok((&b";"[..], value)))
    }

    #[test]
    fn parse_extended_literal() {
        let t = value_expression_eof(b"http://#{\")\"}.com/");
        if let &Ok((rest, ref result)) = &t {
            assert_eq!(
                (
                    format!(
                        "{}",
                        result.evaluate(&GlobalScope::new()).unwrap()
                    ),
                    rest
                ),
                ("http://).com/".to_string(), &b""[..])
            );
        } else {
            assert_eq!(format!("{:?}", t), "Done")
        }
    }
    #[test]
    fn parse_extended_literal_in_arg() {
        let t = value_expression_eof(b"url(http://#{\")\"}.com/)");
        if let &Ok((rest, ref result)) = &t {
            assert_eq!(
                (
                    format!(
                        "{}",
                        result.evaluate(&GlobalScope::new()).unwrap()
                    ),
                    rest
                ),
                ("url(http://).com/)".to_string(), &b""[..])
            );
        } else {
            assert_eq!(format!("{:?}", t), "Done")
        }
    }
    #[test]
    fn parse_extended_literal_in_arg_2() {
        let t = value_expression_eof(b"url(//#{\")\"}.com/)");
        if let &Ok((rest, ref result)) = &t {
            assert_eq!(
                (
                    format!(
                        "{}",
                        result.evaluate(&GlobalScope::new()).unwrap()
                    ),
                    rest
                ),
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
