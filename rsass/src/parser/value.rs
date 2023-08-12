use super::css_function::css_function;
use super::formalargs::call_args;
use super::strings::{
    name, sass_string_dq, sass_string_ext, sass_string_sq,
    special_function_misc, special_url, var_name,
};
use super::unit::unit;
use super::util::{ignore_comments, opt_spacelike, spacelike2};
use super::{
    input_to_string, list_or_single, position, sass_string, PResult, Span,
};
use crate::sass::{BinOp, SassString, Value};
use crate::value::{ListSeparator, Number, Numeric, Operator, Rgba};
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{
    alphanumeric1, multispace0, multispace1, one_of,
};
use nom::combinator::{
    into, map, map_res, not, opt, peek, recognize, value, verify,
};
use nom::multi::{fold_many0, fold_many1, many0, many_m_n, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use num_traits::Zero;
use std::str::from_utf8;

pub fn value_expression(input: Span) -> PResult<Value> {
    let (input, result) = separated_list1(
        preceded(tag(","), ignore_comments),
        terminated(space_list, ignore_comments),
    )(input)?;
    let (input, trail) =
        many0(delimited(opt_spacelike, tag(","), opt_spacelike))(input)?;
    Ok((
        input,
        if result.len() == 1 && trail.is_empty() {
            result.into_iter().next().unwrap()
        } else {
            Value::List(result, Some(ListSeparator::Comma), false)
        },
    ))
}

pub fn space_list(input: Span) -> PResult<Value> {
    let (input, first) = se_or_ext_string(input)?;
    let (input, list) = fold_many0(
        pair(recognize(ignore_comments), se_or_ext_string),
        move || vec![first.clone()],
        |mut list: Vec<Value>, (s, item)| {
            match (list.last_mut(), s.fragment(), &item) {
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
    Ok((input, list_or_single(list, ListSeparator::Space)))
}

pub fn simple_space_list(input: Span) -> PResult<Value> {
    let (input, first) = single_expression(input)?;
    let (input, list) = fold_many0(
        preceded(spacelike2, single_expression),
        move || vec![first.clone()],
        |mut list, item| {
            list.push(item);
            list
        },
    )(input)?;
    Ok((input, list_or_single(list, ListSeparator::Space)))
}

fn se_or_ext_string(input: Span) -> PResult<Value> {
    alt((single_expression, map(sass_string_ext, Value::Literal)))(input)
}

fn single_expression(input: Span) -> PResult<Value> {
    let (input1, a) = logic_expression(input)?;
    fold_many0(
        tuple((
            delimited(
                multispace0,
                alt((
                    value(Operator::And, tag("and")),
                    value(Operator::Or, tag("or")),
                )),
                multispace1,
            ),
            single_expression,
            position,
        )),
        move || a.clone(),
        |a, (op, b, end)| {
            let pos = input.up_to(&end).to_owned();
            BinOp::new(a, false, op, false, b, pos).into()
        },
    )(input1)
}

fn logic_expression(input: Span) -> PResult<Value> {
    let (input1, a) = sum_expression(input)?;
    fold_many0(
        tuple((
            delimited(multispace0, relational_operator, multispace0),
            sum_expression,
            position,
        )),
        move || a.clone(),
        |a, (op, b, end)| {
            let pos = input.up_to(&end).to_owned();
            BinOp::new(a, true, op, true, b, pos).into()
        },
    )(input1)
}

fn relational_operator(input: Span) -> PResult<Operator> {
    alt((
        value(Operator::Equal, tag("==")),
        value(Operator::NotEqual, tag("!=")),
        value(Operator::GreaterE, tag(">=")),
        value(Operator::Greater, tag(">")),
        value(Operator::LesserE, tag("<=")),
        value(Operator::Lesser, tag("<")),
    ))(input)
}

fn sum_expression(input: Span) -> PResult<Value> {
    any_additive_expr(term_value, input)
}

pub fn any_additive_expr<F>(term: F, input: Span) -> PResult<Value>
where
    F: Fn(Span) -> PResult<Value>,
{
    let (rest, v) = term(input)?;
    fold_many0(
        tuple((
            verify(
                tuple((
                    ignore_comments,
                    alt((
                        value(Operator::Plus, tag("+")),
                        value(Operator::Minus, tag("-")),
                    )),
                    ignore_comments,
                )),
                |(s1, op, s2)| op == &Operator::Plus || !*s1 || *s2,
            ),
            term,
            position,
        )),
        move || v.clone(),
        |v, ((s1, op, s2), v2, end)| {
            let pos = input.up_to(&end).to_owned();
            BinOp::new(v, s1, op, s2, v2, pos).into()
        },
    )(rest)
}

fn term_value(input: Span) -> PResult<Value> {
    any_product(single_value, input)
}

pub fn any_product<F>(factor: F, input: Span) -> PResult<Value>
where
    F: Fn(Span) -> PResult<Value>,
{
    let (rest, v) = factor(input)?;
    fold_many0(
        tuple((
            ignore_comments,
            alt((
                value(Operator::Multiply, tag("*")),
                value(
                    Operator::Div,
                    terminated(tag("/"), peek(not(tag("/")))),
                ),
                value(Operator::Modulo, tag("%")),
            )),
            ignore_comments,
            factor,
            position,
        )),
        move || v.clone(),
        |v1, (s1, op, s2, v2, end)| {
            let pos = input.up_to(&end).to_owned();
            BinOp::new(v1, s1, op, s2, v2, pos).into()
        },
    )(rest)
}

pub fn single_value(input: Span) -> PResult<Value> {
    if let Ok((input0, _p)) = preceded(tag("("), opt_spacelike)(input) {
        if let Ok((input, first_key)) = simple_space_list(input0) {
            let (input, value) = if let Ok((mut input, first_val)) =
                preceded(colon, space_list)(input)
            {
                let mut items = vec![(first_key, first_val)];
                while let Ok((rest, (key, val))) = pair(
                    preceded(comma, simple_space_list),
                    preceded(colon, space_list),
                )(input)
                {
                    items.push((key, val));
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

fn comma(input: Span) -> PResult<Span> {
    delimited(opt_spacelike, tag(","), opt_spacelike)(input)
}

fn colon(input: Span) -> PResult<Span> {
    delimited(opt_spacelike, tag(":"), opt_spacelike)(input)
}

fn fallback_in_paren(input: Span) -> PResult<Value> {
    alt((
        map(value_expression, |v| Value::Paren(Box::new(v), false)),
        value(Value::List(vec![], None, false), tag("")),
    ))(input)
}

fn end_paren(input: Span) -> PResult<Span> {
    preceded(opt_spacelike, tag(")"))(input)
}

fn simple_value(input: Span) -> PResult<Value> {
    match input.first() {
        Some(b'!') => bang(input),
        Some(b'&') => value(Value::HereSelector, tag("&"))(input),
        Some(b'"') => map(sass_string_dq, Value::Literal)(input),
        Some(b'\'') => map(sass_string_sq, Value::Literal)(input),
        Some(b'[') => bracket_list(input),
        _ => alt((
            value(Value::True, tag("true")),
            value(Value::False, tag("false")),
            unicode_range,
            into(numeric),
            variable,
            hex_color,
            value(Value::Null, tag("null")),
            map(special_url, Value::Literal),
            special_function,
            // Really ugly special case ... sorry.
            value(Value::Literal("-null".into()), tag("-null")),
            map(var_name, Value::Literal),
            unary_op,
            function_call_or_string,
        ))(input),
    }
}

fn bang(input: Span) -> PResult<Value> {
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

fn unicode_range(input: Span) -> PResult<Value> {
    let (rest, _) = tag_no_case("U+")(input)?;
    let (rest, a) = many_m_n(0, 6, one_of("0123456789ABCDEFabcdef"))(rest)?;
    let (rest, _) = alt((
        preceded(tag("-"), many_m_n(1, 6, one_of("0123456789ABCDEFabcdef"))),
        many_m_n(0, 6 - a.len(), one_of("?")),
    ))(rest)?;
    let length = input.fragment().len() - rest.fragment().len();
    let matched = &input.fragment()[0..length];
    Ok((
        rest,
        // The unwrap should be ok, as only ascii is matched.
        Value::UnicodeRange(from_utf8(matched).unwrap().to_string()),
    ))
}

pub fn bracket_list(input: Span) -> PResult<Value> {
    let (input, content) =
        delimited(tag("["), opt(value_expression), tag("]"))(input)?;
    Ok((
        input,
        match content {
            Some(Value::List(list, sep, false)) => {
                Value::List(list, sep, true)
            }
            Some(single) => Value::List(vec![single], None, true),
            None => Value::List(vec![], None, true),
        },
    ))
}

fn sign_prefix(input: Span) -> PResult<Option<&[u8]>> {
    opt(alt((tag("-"), tag("+"))))(input)
        .map(|(r, s)| (r, s.map(|s| s.fragment())))
}

pub fn numeric(input: Span) -> PResult<Numeric> {
    map(pair(number, unit), |(number, unit)| {
        Numeric::new(number, unit)
    })(input)
}

pub fn number(input: Span) -> PResult<Number> {
    map(
        tuple((
            sign_prefix,
            alt((
                map(pair(decimal_integer, decimal_decimals), |(n, d)| n + d),
                decimal_decimals,
                decimal_integer,
            )),
            opt(preceded(
                alt((tag("e"), tag("E"))),
                tuple((sign_prefix, decimal_i32)),
            )),
        )),
        |(sign, num, exp)| {
            let value = if sign == Some(b"-") {
                // Only f64-based Number can represent negative zero.
                if num.is_zero() {
                    (-0.0).into()
                } else {
                    -num
                }
            } else {
                num
            };
            if let Some((e_sign, e_val)) = exp {
                let e_val = if e_sign == Some(b"-") { -e_val } else { e_val };
                value * Number::from(10f64.powi(e_val))
            } else {
                value
            }
        },
    )(input)
}

pub fn decimal_integer(input: Span) -> PResult<Number> {
    fold_many1(
        // Note: We should use bytes directly, one_of returns a char.
        one_of("0123456789"),
        || Number::from(0),
        |r, d| (r * 10) + Number::from(i64::from(d as u8 - b'0')),
    )(input)
}
pub fn decimal_i32(input: Span) -> PResult<i32> {
    fold_many1(
        // Note: We should use bytes directly, one_of returns a char.
        one_of("0123456789"),
        || 0,
        |r, d| (r * 10) + i32::from(d as u8 - b'0'),
    )(input)
}

pub fn decimal_decimals(input: Span) -> PResult<Number> {
    map(
        preceded(
            tag("."),
            fold_many1(
                one_of("0123456789"),
                || (Number::from(0), Number::from(1)),
                |(r, n), d| {
                    (
                        (r * 10) + Number::from(i64::from(d as u8 - b'0')),
                        n * 10,
                    )
                },
            ),
        ),
        |(r, d)| r / d,
    )(input)
}

pub fn variable(input: Span) -> PResult<Value> {
    let (rest, (modules, name)) = pair(
        many0(terminated(name, tag("."))),
        preceded(tag("$"), name),
    )(input)?;
    let name = if modules.is_empty() {
        name
    } else {
        format!("{}.{}", modules.join("."), name)
    };
    let pos = input.up_to(&rest).to_owned();
    Ok((rest, Value::Variable(name.into(), pos)))
}

fn hex_color(input: Span) -> PResult<Value> {
    let (rest, (r, g, b, a)) = delimited(
        tag("#"),
        alt((
            tuple((hexchar2, hexchar2, hexchar2, opt(hexchar2))),
            tuple((hexchar1, hexchar1, hexchar1, opt(hexchar1))),
        )),
        peek(map(not(alphanumeric1), |_| ())),
    )(input)?;

    if let Some(a) = a {
        let rgba = Rgba::from_rgba(r, g, b, a);
        Ok((rest, Value::Color(rgba, None)))
    } else {
        let rgba = Rgba::from_rgb(r, g, b);
        let length = input.fragment().len() - rest.fragment().len();
        // Unwrap should be ok as only ascii is matched.
        let raw =
            from_utf8(&input.fragment()[0..length]).unwrap().to_string();
        Ok((rest, Value::Color(rgba, Some(raw))))
    }
}

pub fn unary_op(input: Span) -> PResult<Value> {
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

pub fn special_function(input: Span) -> PResult<Value> {
    // Either a nice semantic css function or a fallback with interpolation.
    alt((css_function, map(special_function_misc, Value::Literal)))(input)
}

pub fn function_call_or_string(input: Span) -> PResult<Value> {
    let (rest, name) = sass_string(input)?;
    /* TODO: true, false and null should end up here, but can't as long as '.' is a normal part of a string.
    if let Some(special) = name.single_raw().and_then(|s| match dbg!(s) {
        "true" => return Some(Value::True),
        "false" => return Some(Value::False),
        "null" => return Some(Value::Null),
        _ => None,
    }) {
        return Ok((rest, special));
    }
     */
    if let Ok((rest, args)) = call_args(rest) {
        let pos = input.up_to(&rest).to_owned();
        return Ok((rest, Value::Call(name, args, pos)));
    }
    Ok((rest, literal_or_color(name)))
}

fn literal_or_color(s: SassString) -> Value {
    if let Some(val) = s.single_raw() {
        if let Some(rgba) = Rgba::from_name(val) {
            return Value::Color(rgba, Some(val.to_string()));
        }
    }
    Value::Literal(s)
}

fn hexchar1(input: Span) -> PResult<u8> {
    map(hexchar_raw, |one| one * 0x11)(input)
}
fn hexchar2(input: Span) -> PResult<u8> {
    map(pair(hexchar_raw, hexchar_raw), |(hi, lo)| hi * 0x10 + lo)(input)
}
fn hexchar_raw(input: Span) -> PResult<u8> {
    map(one_of("0123456789ABCDEFabcdef"), |ch| {
        ch.to_digit(16).unwrap() as u8
    })(input)
}

pub fn dictionary(input: Span) -> PResult<Value> {
    delimited(
        preceded(tag("("), opt_spacelike),
        dictionary_inner,
        terminated(opt_spacelike, tag(")")),
    )(input)
}

pub fn dictionary_inner(input: Span) -> PResult<Value> {
    let (input, items) = terminated(
        separated_list1(
            delimited(opt_spacelike, tag(","), opt_spacelike),
            pair(
                sum_expression,
                preceded(
                    delimited(ignore_comments, tag(":"), opt_spacelike),
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
    use super::super::{code_span, parse_value_data};
    use super::*;
    use crate::sass::CallArgs;
    use crate::sass::Value::{Color, List, Literal, Map, Paren};
    use crate::value::Rational;
    use crate::ScopeRef;

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
        check_expr("+4;", Value::scalar(4))
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
        check_expr("+.34;", number(34, 100))
    }

    fn number(nom: i64, denom: i64) -> Value {
        Value::scalar(Rational::new(nom, denom))
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
        match value_expression(code_span(b"$red;").borrow())
            .map(|(_, value)| value)
            .unwrap()
        {
            Value::Variable(name, _) => assert_eq!(name, "red".into()),
            val => panic!("Unexpected value {:?}", val),
        }
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
                    Some(ListSeparator::Space),
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
                    Some(ListSeparator::Comma),
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
                Some(ListSeparator::Comma),
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
                    Some(ListSeparator::Comma),
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
                Some(ListSeparator::Comma),
                false,
            ),
        )
    }

    #[test]
    fn call_no_args() {
        assert_eq!(
            parse_call(code_span(b"foo();").borrow()),
            Ok(("foo".into(), CallArgs::default(), ";".as_bytes())),
        );
    }

    #[test]
    fn call_one_arg() {
        assert_eq!(
            parse_call(code_span(b"foo(17);").borrow()),
            Ok((
                "foo".into(),
                CallArgs::new(vec![(None, Value::scalar(17))], false)
                    .unwrap(),
                ";".as_bytes(),
            )),
        );
    }

    // test helper
    fn parse_call(
        expr: Span,
    ) -> Result<(SassString, CallArgs, &[u8]), String> {
        let (rest, value) =
            value_expression(expr).map_err(|e| e.to_string())?;
        if let Value::Call(name, args, _) = value {
            Ok((name, args, rest.fragment()))
        } else {
            Err(format!("Not a call parse result: {:?} {:?}", value, rest))
        }
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
                Some(ListSeparator::Space),
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
                Some(ListSeparator::Comma),
                true,
            ),
        )
    }

    #[test]
    fn parse_bracket_empty_array() {
        check_expr("[];", List(vec![], None, true))
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
        assert_eq!(
            value_expression(code_span(expr.as_bytes()).borrow())
                .map(|(rest, value)| (rest.fragment(), value))
                .unwrap(),
            (&b";"[..], value),
        )
    }

    #[test]
    fn parse_extended_literal() -> Result<(), crate::Error> {
        assert_eq!(
            parse_value_data(b"http://#{\")\"}.com/")?
                .evaluate(ScopeRef::new_global(Default::default()))?
                .format(Default::default())
                .to_string(),
            "http://).com/".to_string(),
        );
        Ok(())
    }
    #[test]
    fn parse_extended_literal_in_arg() -> Result<(), crate::Error> {
        assert_eq!(
            parse_value_data(b"url(http://#{\")\"}.com/)")?
                .evaluate(ScopeRef::new_global(Default::default()))?
                .format(Default::default())
                .to_string(),
            "url(http://).com/)".to_string(),
        );
        Ok(())
    }
    #[test]
    fn parse_extended_literal_in_arg_2() -> Result<(), crate::Error> {
        assert_eq!(
            parse_value_data(b"url(//#{\")\"}.com/)")?
                .evaluate(ScopeRef::new_global(Default::default()))?
                .format(Default::default())
                .to_string(),
            "url(//).com/)".to_string(),
        );
        Ok(())
    }
}
