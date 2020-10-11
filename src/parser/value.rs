use super::formalargs::call_args;
use super::strings::{
    name, sass_string_dq, sass_string_ext, sass_string_sq,
    special_function_minmax, special_function_misc, special_url,
};
use super::unit::unit;
use super::util::{opt_spacelike, spacelike2};
use super::{input_to_string, sass_string};
use crate::ordermap::OrderMap;
use crate::sass::{SassString, Value};
use crate::value::{ListSeparator, Number, Operator, Rgba};
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{
    alphanumeric1, multispace0, multispace1, one_of,
};
use nom::combinator::{map, map_opt, map_res, not, opt, peek, value};
use nom::multi::{
    fold_many0, fold_many1, many0, many_m_n, separated_nonempty_list,
};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;
use num_bigint::BigInt;
use num_rational::{Ratio, Rational};
use num_traits::{One, Zero};

pub fn value_expression(input: &[u8]) -> IResult<&[u8], Value> {
    let (input, result) = separated_nonempty_list(
        preceded(tag(","), opt_spacelike),
        space_list,
    )(input)?;
    let (input, trail) =
        many0(delimited(opt_spacelike, tag(","), opt_spacelike))(input)?;
    Ok((
        input,
        if result.len() == 1 && trail.is_empty() {
            result.into_iter().next().unwrap()
        } else {
            Value::List(result, ListSeparator::Comma, false)
        },
    ))
}

pub fn space_list(input: &[u8]) -> IResult<&[u8], Value> {
    let (input, first) = se_or_ext_string(input)?;
    let (input, list) = fold_many0(
        pair(multispace0, se_or_ext_string),
        vec![first],
        |mut list: Vec<Value>, (s, item)| {
            match (list.last_mut(), s, &item) {
                (
                    Some(Value::Literal(ref mut s1)),
                    b"",
                    Value::Literal(ref s2),
                ) if s1.is_unquoted() && s2.is_unquoted() => {
                    s1.append(s2);
                }
                _ => {
                    list.push(item);
                }
            }
            list
        },
    )(input)?;
    Ok((
        input,
        if list.len() == 1 {
            list.into_iter().next().unwrap()
        } else {
            Value::List(list, ListSeparator::Space, false)
        },
    ))
}

fn se_or_ext_string(input: &[u8]) -> IResult<&[u8], Value> {
    alt((single_expression, map(sass_string_ext, Value::Literal)))(input)
}

fn single_expression(input: &[u8]) -> IResult<&[u8], Value> {
    let (input, a) = logic_expression(input)?;
    fold_many0(
        pair(
            delimited(
                multispace0,
                alt((
                    value(Operator::And, tag("and")),
                    value(Operator::Or, tag("or")),
                )),
                multispace1,
            ),
            single_expression,
        ),
        a,
        |a, (op, b)| Value::BinOp(Box::new(a), false, op, false, Box::new(b)),
    )(input)
}

fn logic_expression(input: &[u8]) -> IResult<&[u8], Value> {
    let (input, a) = sum_expression(input)?;
    fold_many0(
        pair(
            delimited(
                multispace0,
                alt((
                    value(Operator::Equal, tag("==")),
                    value(Operator::NotEqual, tag("!=")),
                    value(Operator::GreaterE, tag(">=")),
                    value(Operator::Greater, tag(">")),
                    value(Operator::LesserE, tag("<=")),
                    value(Operator::Lesser, tag("<")),
                )),
                multispace0,
            ),
            sum_expression,
        ),
        a,
        |a, (op, b)| Value::BinOp(Box::new(a), false, op, false, Box::new(b)),
    )(input)
}

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
                    value(Operator::Minus, terminated(tag("-"), spacelike2)),
                )),
            ),
            preceded(opt(spacelike2), term_value),
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
            value(Operator::Div, terminated(tag(b"/"), peek(not(tag(b"/"))))),
            value(Operator::Modulo, tag(b"%")),
        )),
        map(multispace0, |s: &[u8]| !s.is_empty()),
        single_value,
    ))(rest)
    {
        rest = nrest;
        v = Value::BinOp(Box::new(v), s1, op, s2, Box::new(v2));
    }
    Ok((rest, v))
}

pub fn single_value(input: &[u8]) -> IResult<&[u8], Value> {
    if let Ok((input0, _p)) = preceded(tag("("), opt_spacelike)(input) {
        if let Ok((input, first_key)) = sum_expression(input0) {
            let (input, value) = if let Ok((mut input, first_val)) =
                preceded(colon, space_list)(input)
            {
                let mut items = OrderMap::new();
                items.insert(first_key, first_val);
                while let Ok((rest, (key, val))) = pair(
                    preceded(comma, sum_expression),
                    preceded(colon, space_list),
                )(input)
                {
                    items.insert(key, val);
                    input = rest;
                }
                let (input, _) = opt(comma)(input)?;
                (input, Value::Map(items))
            } else {
                (input, Value::Paren(Box::new(first_key), false))
            };
            if let Ok((input, _)) = end_paren(input) {
                return Ok((input, value));
            }
        }
        terminated(fallback_in_paren, end_paren)(input0)
    } else {
        simple_value(input)
    }
}

fn comma(input: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(opt_spacelike, tag(","), opt_spacelike)(input)
}

fn colon(input: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(opt_spacelike, tag(":"), opt_spacelike)(input)
}

fn fallback_in_paren(input: &[u8]) -> IResult<&[u8], Value> {
    alt((
        map(value_expression, |v| Value::Paren(Box::new(v), false)),
        value(Value::List(vec![], ListSeparator::Space, false), tag("")),
    ))(input)
}

fn end_paren(input: &[u8]) -> IResult<&[u8], &[u8]> {
    preceded(opt_spacelike, tag(")"))(input)
}

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
        map(special_url, Value::Literal),
        special_function,
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

fn bang(input: &[u8]) -> IResult<&[u8], Value> {
    map(
        map_res(
            preceded(
                pair(tag("!"), opt_spacelike),
                tag("important"), // TODO Pretty much anythig goes, here?
            ),
            input_to_string,
        ),
        Value::Bang,
    )(input)
}

fn unicode_range(input: &[u8]) -> IResult<&[u8], Value> {
    let (rest, _) = tag_no_case("U+")(input)?;
    let (rest, a) = many_m_n(0, 6, one_of("0123456789ABCDEFabcdef"))(rest)?;
    let (rest, _) = opt(alt((
        preceded(tag("-"), many_m_n(1, 6, one_of("0123456789ABCDEFabcdef"))),
        many_m_n(1, 6 - a.len(), one_of("?")),
    )))(rest)?;
    let length = input.len() - rest.len();
    let matched = &input[0..length];
    Ok((
        rest,
        // The unwrap should be ok, as only ascii is matched.
        Value::UnicodeRange(input_to_string(matched).unwrap()),
    ))
}

fn bracket_list(input: &[u8]) -> IResult<&[u8], Value> {
    let (input, content) =
        delimited(tag("["), opt(value_expression), tag("]"))(input)?;
    Ok((
        input,
        match content {
            Some(Value::List(list, sep, false)) => {
                Value::List(list, sep, true)
            }
            Some(single) => {
                Value::List(vec![single], ListSeparator::Space, true)
            }
            None => Value::List(vec![], ListSeparator::Space, true),
        },
    ))
}

fn sign_prefix(input: &[u8]) -> IResult<&[u8], Option<&[u8]>> {
    opt(alt((tag("-"), tag("+"))))(input)
}

enum AnyRatio {
    Machine(Ratio<isize>),
    Big(Ratio<BigInt>),
}

fn number(input: &[u8]) -> IResult<&[u8], Value> {
    map(
        tuple((
            sign_prefix,
            alt((
                map(pair(decimal_integer, decimal_decimals), |(n, d)| {
                    (true, AnyRatio::Machine(n + d))
                }),
                map(
                    pair(decimal_integer_big, decimal_decimals_big),
                    |(n, d)| (true, AnyRatio::Big(n + d)),
                ),
                map(decimal_decimals, |dec| (false, AnyRatio::Machine(dec))),
                map(decimal_decimals_big, |dec| (false, AnyRatio::Big(dec))),
                map(decimal_integer, |int| (true, AnyRatio::Machine(int))),
                map(decimal_integer_big, |int| (true, AnyRatio::Big(int))),
            )),
            unit,
        )),
        |(sign, (lead_zero, num), unit)| match num {
            AnyRatio::Machine(num) => Value::Numeric(
                Number {
                    value: if sign == Some(b"-") { -num } else { num },
                    plus_sign: sign == Some(b"+"),
                    lead_zero,
                },
                unit,
            ),
            AnyRatio::Big(num) => Value::NumericBig(
                Number {
                    value: if sign == Some(b"-") { -num } else { num },
                    plus_sign: sign == Some(b"+"),
                    lead_zero,
                },
                unit,
            ),
        },
    )(input)
}

pub fn decimal_integer(input: &[u8]) -> IResult<&[u8], Rational> {
    map_opt(
        fold_many1(
            // Note: We should use bytes directly, one_of returns a char.
            one_of("0123456789"),
            Some(0isize),
            |r, d| {
                r?.checked_mul(10)?.checked_add(isize::from(d as u8 - b'0'))
            },
        ),
        |opt_int| opt_int.map(Rational::from_integer),
    )(input)
}

pub fn decimal_integer_big(input: &[u8]) -> IResult<&[u8], Ratio<BigInt>> {
    map(
        fold_many1(
            // Note: We should use bytes directly, one_of returns a char.
            one_of("0123456789"),
            BigInt::zero(),
            |r, d| r * 10 + BigInt::from(d as u8 - b'0'),
        ),
        Ratio::from_integer,
    )(input)
}

pub fn decimal_decimals(input: &[u8]) -> IResult<&[u8], Rational> {
    map_opt(
        preceded(
            tag("."),
            fold_many1(
                one_of("0123456789"),
                Some((0isize, 1isize)),
                |opt_pair, d| {
                    let (r, n) = opt_pair?;
                    Some((
                        r.checked_mul(10)?
                            .checked_add(isize::from(d as i8 - b'0' as i8))?,
                        n.checked_mul(10)?,
                    ))
                },
            ),
        ),
        |opt_pair| opt_pair.map(|(r, d)| Rational::new(r, d)),
    )(input)
}

pub fn decimal_decimals_big(input: &[u8]) -> IResult<&[u8], Ratio<BigInt>> {
    map(
        preceded(
            tag("."),
            fold_many1(
                one_of("0123456789"),
                (BigInt::zero(), BigInt::one()),
                |(r, n), d| {
                    (r * 10 + BigInt::from(d as i8 - b'0' as i8), n * 10)
                },
            ),
        ),
        |(r, d)| Ratio::new(r, d),
    )(input)
}

pub fn variable(input: &[u8]) -> IResult<&[u8], Value> {
    map(preceded(tag("$"), name), Value::Variable)(input)
}

fn hex_color(input: &[u8]) -> IResult<&[u8], Value> {
    let (rest, rgba) = delimited(
        tag("#"),
        map(
            alt((
                tuple((hexchar2, hexchar2, hexchar2, opt(hexchar2))),
                tuple((hexchar1, hexchar1, hexchar1, opt(hexchar1))),
            )),
            |(r, g, b, a): (u8, u8, u8, Option<u8>)| {
                Rgba::from_rgba(r, g, b, a.unwrap_or(255))
            },
        ),
        peek(map(not(alphanumeric1), |_| ())),
    )(input)?;
    let length = input.len() - rest.len();
    // Unwrap should be ok as only ascii is matched.
    let raw = input_to_string(&input[0..length]).unwrap();
    Ok((rest, Value::Color(rgba, Some(raw))))
}

pub fn unary_op(input: &[u8]) -> IResult<&[u8], Value> {
    map(
        pair(
            terminated(
                alt((
                    value(Operator::Plus, tag("+")),
                    value(Operator::Minus, tag("-")),
                    value(Operator::Div, terminated(tag("/"), spacelike2)),
                    value(Operator::Not, terminated(tag("not"), spacelike2)),
                )),
                opt_spacelike,
            ),
            single_value,
        ),
        |(op, v)| Value::UnaryOp(op, Box::new(v)),
    )(input)
}

fn special_function(input: &[u8]) -> IResult<&[u8], Value> {
    map(
        alt((special_function_misc, special_function_minmax)),
        Value::Literal,
    )(input)
}

pub fn function_call(input: &[u8]) -> IResult<&[u8], Value> {
    map(pair(sass_string, call_args), |(name, args)| {
        Value::Call(name, args)
    })(input)
}

fn literal_or_color(s: SassString) -> Value {
    if let Some(val) = s.single_raw() {
        if let Some(rgba) = Rgba::from_name(val) {
            return Value::Color(rgba, Some(val.to_string()));
        }
    }
    Value::Literal(s)
}

fn hexchar1(input: &[u8]) -> IResult<&[u8], u8> {
    map(hexchar_raw, |one| one * 0x11)(input)
}
fn hexchar2(input: &[u8]) -> IResult<&[u8], u8> {
    map(pair(hexchar_raw, hexchar_raw), |(hi, lo)| hi * 0x10 + lo)(input)
}
fn hexchar_raw(input: &[u8]) -> IResult<&[u8], u8> {
    map(one_of("0123456789ABCDEFabcdef"), |ch| {
        ch.to_digit(16).unwrap() as u8
    })(input)
}

pub fn dictionary(input: &[u8]) -> IResult<&[u8], Value> {
    delimited(
        preceded(tag("("), opt_spacelike),
        dictionary_inner,
        terminated(opt_spacelike, tag(")")),
    )(input)
}

pub fn dictionary_inner(input: &[u8]) -> IResult<&[u8], Value> {
    let (input, items) = terminated(
        separated_nonempty_list(
            delimited(opt_spacelike, tag(","), opt_spacelike),
            pair(
                sum_expression,
                preceded(
                    delimited(opt_spacelike, tag(":"), opt_spacelike),
                    space_list,
                ),
            ),
        ),
        opt(delimited(opt_spacelike, tag(","), opt_spacelike)),
    )(input)?;
    Ok((input, Value::Map(items.into_iter().collect())))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sass::CallArgs;
    use crate::sass::Value::*;
    use crate::value::Unit;
    use crate::variablescope::GlobalScope;
    use nom::combinator::all_consuming;
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
        check_expr("(rad);", Paren(Box::new(Literal("rad".into())), false))
    }

    #[test]
    fn paren_multi() {
        check_expr(
            "(rod bloe);",
            Paren(
                Box::new(List(
                    vec![Literal("rod".into()), Literal("bloe".into())],
                    ListSeparator::Space,
                    false,
                )),
                false,
            ),
        )
    }

    #[test]
    fn paren_multi_comma() {
        check_expr(
            "(rod, bloe);",
            Paren(
                Box::new(List(
                    vec![Literal("rod".into()), Literal("bloe".into())],
                    ListSeparator::Comma,
                    false,
                )),
                false,
            ),
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
            ),
        )
    }

    #[test]
    fn paren_multi_comma_trailing() {
        check_expr(
            "(rod, bloe, );",
            Paren(
                Box::new(List(
                    vec![Literal("rod".into()), Literal("bloe".into())],
                    ListSeparator::Comma,
                    false,
                )),
                false,
            ),
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
            ),
        )
    }

    #[test]
    fn parse_bracket_empty_array() {
        check_expr("[];", List(vec![], ListSeparator::Space, true))
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
                    result
                        .evaluate(&GlobalScope::new(Default::default()))
                        .unwrap()
                        .format(Default::default())
                        .to_string(),
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
                    result
                        .evaluate(&GlobalScope::new(Default::default()))
                        .unwrap()
                        .format(Default::default())
                        .to_string(),
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
                    result
                        .evaluate(&GlobalScope::new(Default::default()))
                        .unwrap()
                        .format(Default::default())
                        .to_string(),
                    rest
                ),
                ("url(//).com/)".to_string(), &b""[..])
            );
        } else {
            assert_eq!(format!("{:?}", t), "Done")
        }
    }

    fn value_expression_eof(input: &[u8]) -> IResult<&[u8], Value> {
        all_consuming(value_expression)(input)
    }
}
