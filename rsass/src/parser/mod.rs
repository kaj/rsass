pub(crate) mod css;
mod css_function;
mod error;
pub mod formalargs;
mod imports;
mod media;
pub mod selectors;
mod span;
pub(crate) mod strings;
mod unit;
pub(crate) mod util;
pub mod value;

pub(crate) use self::strings::name;
pub use error::ParseError;
pub(crate) use span::DebugBytes;
pub(crate) use span::{position, Span};

use self::formalargs::{call_args, formal_args};
use self::selectors::selectors;
use self::strings::{
    custom_value, sass_string, sass_string_dq, sass_string_sq,
};
use self::util::{
    comment2, ignore_comments, ignore_space, opt_spacelike, semi_or_end,
    spacelike,
};
use self::value::{
    dictionary, function_call_or_string, single_value, value_expression,
};
use crate::input::{SourceFile, SourceName, SourcePos};
use crate::sass::parser::{variable_declaration2, variable_declaration_mod};
use crate::sass::{Callable, FormalArgs, Item, Name, Selectors, Value};
use crate::value::ListSeparator;
#[cfg(test)]
use crate::value::{Numeric, Unit};
use crate::Error;
use imports::{forward2, import2, use2};
use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag};
use nom::character::complete::one_of;
use nom::combinator::{
    all_consuming, into, map, map_res, opt, peek, value, verify,
};
use nom::multi::{many0, many_till, separated_list0, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::IResult;
use std::str::{from_utf8, Utf8Error};

/// A Parsing Result; ok gives a span for the rest of the data and a parsed T.
pub(crate) type PResult<'a, T> = IResult<Span<'a>, T>;

pub(crate) fn code_span(value: &[u8]) -> SourcePos {
    SourceFile::scss_bytes(value, SourceName::root("(rsass)")).into()
}

pub(crate) fn input_span(value: impl Into<Vec<u8>>) -> SourcePos {
    SourceFile::scss_bytes(value, SourceName::root("-")).into()
}

/// Parse a scss value.
///
/// Returns a single value (or an error).
pub fn parse_value_data(data: &[u8]) -> Result<Value, Error> {
    let data = code_span(data);
    let value = all_consuming(value_expression)(data.borrow());
    Ok(ParseError::check(value)?)
}

#[test]
fn test_parse_value_data_1() -> Result<(), Error> {
    let v = parse_value_data(b"17em")?;
    assert_eq!(Value::Numeric(Numeric::new(17, Unit::Em)), v);
    Ok(())
}

#[test]
fn test_parse_value_data_2() -> Result<(), Error> {
    let v = parse_value_data(b"17em;");
    assert!(v.is_err());
    Ok(())
}

pub(crate) fn sassfile(input: Span) -> PResult<Vec<Item>> {
    preceded(
        opt(tag("\u{feff}".as_bytes())),
        map(
            many_till(
                preceded(opt_spacelike, top_level_item),
                all_consuming(opt_spacelike),
            ),
            |(v, _eof)| v,
        ),
    )(input)
}

fn top_level_item(input: Span) -> PResult<Item> {
    let (rest, tag) = alt((tag("$"), tag("/*"), tag("@"), tag("")))(input)?;
    match tag.fragment() {
        b"$" => into(variable_declaration2)(rest),
        b"/*" => comment_item(rest),
        b"@" => at_rule2(input),
        b"" => alt((into(variable_declaration_mod), rule))(input),
        _ => unreachable!(),
    }
}

fn comment_item(input: Span) -> PResult<Item> {
    map(comment2, Item::Comment)(input)
}

fn rule(input: Span) -> PResult<Item> {
    map(pair(rule_start, body_block2), |(selectors, body)| {
        Item::Rule(selectors, body)
    })(input)
}

fn rule_start(input: Span) -> PResult<Selectors> {
    terminated(selectors, terminated(opt(is_a(", \t\r\n")), tag("{")))(input)
}

fn body_item(input: Span) -> PResult<Item> {
    let (rest, tag) =
        alt((tag("$"), tag("/*"), tag(";"), tag("@"), tag("--"), tag("")))(
            input,
        )?;
    match tag.fragment() {
        b"$" => into(variable_declaration2)(rest),
        b"/*" => comment_item(rest),
        b";" => Ok((rest, Item::None)),
        b"@" => at_rule2(input),
        b"--" => {
            let result = custom_property(rest);
            if result.is_err() {
                // Note use of `input` rather than `rest` here.
                if let Ok((rest, rule)) = rule(input) {
                    return Ok((rest, rule));
                }
            }
            result
        }
        b"" => match rule_start(rest) {
            Ok((rest, selectors)) => {
                let (rest, body) = body_block2(rest)?;
                Ok((rest, Item::Rule(selectors, body)))
            }
            Err(_) => property_or_namespace_rule(rest),
        },
        _ => unreachable!(),
    }
}

/// What follows the `@at-root` tag.
fn at_root2(input: Span) -> PResult<Item> {
    preceded(
        opt_spacelike,
        map(
            pair(
                map(opt(selectors), |s| s.unwrap_or_else(Selectors::root)),
                body_block,
            ),
            |(selectors, body)| Item::AtRoot(selectors, body),
        ),
    )(input)
}

/// What follows the `@include` tag.
fn mixin_call<'a>(start: Span, input: Span<'a>) -> PResult<'a, Item> {
    let (rest, n1) = terminated(name, opt_spacelike)(input)?;
    let (rest, n2) = opt(preceded(tag("."), name))(rest)?;
    let name = n2.map(|n2| format!("{n1}.{n2}")).unwrap_or(n1);
    let (rest, _) = opt_spacelike(rest)?;
    let (rest0, args) = terminated(opt(call_args), opt_spacelike)(rest)?;
    let (rest, t) = alt((tag("using"), tag("{"), tag("")))(rest0)?;
    let (end, body) = match t.fragment() {
        b"using" => {
            let (end, args) = preceded(opt_spacelike, formal_args)(rest)?;
            let (rest, body) = preceded(opt_spacelike, body_block)(end)?;
            let decl = rest0.up_to(&end).to_owned();
            (rest, Some(Callable::new(args, body, decl)))
        }
        b"{" => {
            let (rest, body) = body_block(rest0)?;
            let decl = rest0.up_to(&rest).to_owned();
            (rest, Some(Callable::no_args(body, decl)))
        }
        _ => {
            let (rest, _) = opt(tag(";"))(rest)?;
            (rest, None)
        }
    };
    let pos = start.up_to(&rest).to_owned();
    Ok((
        end,
        Item::MixinCall(name, args.unwrap_or_default(), body, pos),
    ))
}

/// When we know that `input0` starts with an `@` sign.
fn at_rule2(input0: Span) -> PResult<Item> {
    let (input, name) =
        delimited(tag("@"), sass_string, opt_spacelike)(input0)?;
    match name.single_raw().unwrap_or("") {
        "at-root" => at_root2(input),
        "charset" => charset2(input),
        "content" => content_stmt2(input),
        "debug" => map(expression_argument, Item::Debug)(input),
        "each" => each_loop2(input),
        "error" => {
            let (end, v) = value_expression(input)?;
            let (rest, _) = opt(tag(";"))(end)?;
            let pos = input0.up_to(&end).to_owned();
            Ok((rest, Item::Error(v, pos)))
        }
        "extend" => map(
            delimited(
                opt_spacelike,
                selectors,
                preceded(opt_spacelike, tag(";")),
            ),
            Item::Extend,
        )(input),
        "for" => for_loop2(input),
        "forward" => forward2(input0, input),
        "function" => function_declaration2(input),
        "if" => if_statement2(input),
        "import" => import2(input),
        "include" => mixin_call(input0, input),
        "media" => media::rule(input0, input),
        "mixin" => mixin_declaration2(input),
        "return" => return_stmt2(input0, input),
        "use" => use2(input0, input),
        "warn" => map(expression_argument, Item::Warn)(input),
        "while" => while_loop2(input),
        _ => unknown_atrule(name, input0, input),
    }
}
fn unknown_atrule<'a>(
    name: SassString,
    start: Span,
    input: Span<'a>,
) -> PResult<'a, Item> {
    let (input, args) =
        terminated(opt(unknown_rule_args), opt(ignore_space))(input)?;
    fn x_args(value: Value) -> Value {
        match value {
            Value::Variable(name, _pos) => {
                Value::Literal(SassString::from(format!("${name}")))
            }
            Value::Map(map) => Value::Map(
                map.into_iter()
                    .map(|(k, v)| (x_args(k), x_args(v)))
                    .collect(),
            ),
            value => value,
        }
    }
    let (rest, body) = if input.first() == Some(&b'{') {
        map(body_block, Some)(input)?
    } else {
        value(None, semi_or_end)(input)?
    };
    Ok((
        rest,
        Item::AtRule {
            name,
            args: args.map_or(Value::Null, x_args),
            body,
            pos: start.up_to(&input).to_owned(),
        },
    ))
}

fn expression_argument(input: Span) -> PResult<Value> {
    terminated(value_expression, opt(tag(";")))(input)
}

fn charset2(input: Span) -> PResult<Item> {
    use nom::combinator::map_opt;
    map_opt(
        terminated(
            alt((sass_string_dq, sass_string_sq, sass_string)),
            semi_or_end,
        ),
        |s| {
            s.single_raw().and_then(|s| {
                if s.eq_ignore_ascii_case("UTF-8") {
                    Some(Item::None)
                } else {
                    None
                }
            })
        },
    )(input)
}

/// Arguments to an unkown at rule.
fn unknown_rule_args(input: Span) -> PResult<Value> {
    let (input, args) = separated_list0(
        preceded(tag(","), opt_spacelike),
        map(
            many0(preceded(
                opt(ignore_space),
                alt((
                    terminated(
                        alt((
                            function_call_or_string,
                            dictionary,
                            map(
                                delimited(tag("("), media::args, tag(")")),
                                |v| Value::Paren(Box::new(v), true),
                            ),
                            map(sass_string_dq, Value::Literal),
                            map(sass_string_sq, Value::Literal),
                        )),
                        alt((
                            value((), all_consuming(tag(""))),
                            value((), peek(one_of(") \r\n\t{,;"))),
                        )),
                    ),
                    map(map_res(is_not("\"'{};#"), input_to_str), |s| {
                        Value::Literal(s.trim_end().into())
                    }),
                )),
            )),
            |args| {
                if args.len() == 1 {
                    args.into_iter().next().unwrap()
                } else {
                    Value::List(args, Some(ListSeparator::Space), false)
                }
            },
        ),
    )(input)?;
    Ok((
        input,
        if args.len() == 1 {
            args.into_iter().next().unwrap()
        } else {
            Value::List(args, Some(ListSeparator::Comma), false)
        },
    ))
}

#[cfg(test)]
pub(crate) fn check_parse<T>(
    parser: impl Fn(Span) -> PResult<T>,
    value: &[u8],
) -> Result<T, ParseError> {
    ParseError::check(parser(code_span(value).borrow()))
}

fn if_statement_inner(input: Span) -> PResult<Item> {
    preceded(
        terminated(verify(name, |n: &String| n == "if"), opt_spacelike),
        if_statement2,
    )(input)
}

fn if_statement2(input: Span) -> PResult<Item> {
    let (input, cond) = terminated(value_expression, opt_spacelike)(input)?;
    let (input, body) = body_block(input)?;
    let (input2, word) = opt(delimited(
        preceded(opt_spacelike, tag("@")),
        name,
        opt_spacelike,
    ))(input)?;
    match word.as_ref().map(AsRef::as_ref) {
        Some("else") => {
            let (input2, else_body) = alt((
                body_block,
                map(if_statement_inner, |s| vec![s]),
            ))(input2)?;
            Ok((input2, Item::IfStatement(cond, body, else_body)))
        }
        Some("elseif") => {
            let (input2, else_body) = if_statement2(input2)?;
            Ok((input2, Item::IfStatement(cond, body, vec![else_body])))
        }
        _ => Ok((input, Item::IfStatement(cond, body, vec![]))),
    }
}

/// The part of an each look that follows the `@each`.
fn each_loop2(input: Span) -> PResult<Item> {
    let (input, names) = separated_list1(
        delimited(opt_spacelike, tag(","), opt_spacelike),
        map(preceded(tag("$"), name), Name::from),
    )(input)?;
    let (input, values) = delimited(
        delimited(spacelike, tag("in"), spacelike),
        value_expression,
        opt_spacelike,
    )(input)?;
    let (input, body) = body_block(input)?;
    Ok((input, Item::Each(names, values, body)))
}

/// A for loop after the initial `@for`.
fn for_loop2(input: Span) -> PResult<Item> {
    let (input, name) = delimited(tag("$"), name, spacelike)(input)?;
    let (input, from) = delimited(
        terminated(tag("from"), spacelike),
        single_value,
        spacelike,
    )(input)?;
    let (input, inclusive) = terminated(
        alt((value(true, tag("through")), value(false, tag("to")))),
        spacelike,
    )(input)?;
    let (input, to) = terminated(single_value, opt_spacelike)(input)?;
    let (input, body) = body_block(input)?;
    Ok((
        input,
        Item::For {
            name: name.into(),
            from: Box::new(from),
            to: Box::new(to),
            inclusive,
            body,
        },
    ))
}

fn while_loop2(input: Span) -> PResult<Item> {
    let (input, cond) = terminated(value_expression, opt_spacelike)(input)?;
    let (input, body) = body_block(input)?;
    Ok((input, Item::While(cond, body)))
}

fn mixin_declaration2(input: Span) -> PResult<Item> {
    let (rest, name) = terminated(name, opt_spacelike)(input)?;
    let (rest, args) = opt(formal_args)(rest)?;
    let (end, body) = preceded(opt_spacelike, body_block)(rest)?;
    let args = args.unwrap_or_else(FormalArgs::none);
    let decl = input.up_to(&rest).to_owned();
    Ok((
        end,
        Item::MixinDeclaration(name, Callable { args, body, decl }),
    ))
}

fn function_declaration2(input: Span) -> PResult<Item> {
    let (end, name) = terminated(name, opt_spacelike)(input)?;
    let (end, args) = formal_args(end)?;
    let (rest, body) = preceded(opt_spacelike, body_block)(end)?;
    let decl = input.up_to(&end).to_owned();
    Ok((
        rest,
        Item::FunctionDeclaration(name, Callable { args, body, decl }),
    ))
}

fn return_stmt2<'a>(start: Span, input: Span<'a>) -> PResult<'a, Item> {
    let (input, v) =
        delimited(opt_spacelike, value_expression, opt_spacelike)(input)?;
    let pos = start.up_to(&input).to_owned();
    let (input, _) = opt(tag(";"))(input)?;
    Ok((input, Item::Return(v, pos)))
}

/// The "rest" of an `@content` statement is just an optional terminator
fn content_stmt2(input: Span) -> PResult<Item> {
    let (rest, _) = opt_spacelike(input)?;
    let (rest, args) = opt(call_args)(rest)?;
    let (rest, _) = opt(tag(";"))(rest)?;
    let pos = input.up_to(&rest).to_owned();
    Ok((rest, Item::Content(args.unwrap_or_default(), pos)))
}

fn custom_property(input: Span) -> PResult<Item> {
    let (rest, name) = terminated(opt(sass_string), tag(":"))(input)?;
    let mut name = name.unwrap_or_else(|| SassString::from(""));
    // The dashes was parsed before calling this method.
    name.prepend("--");
    let (rest, value) =
        terminated(custom_value, alt((tag(";"), peek(tag("}")))))(rest)?;
    Ok((rest, Item::CustomProperty(name, value)))
}

fn property_or_namespace_rule(input: Span) -> PResult<Item> {
    let (start_val, name) = terminated(
        alt((
            map(preceded(tag("*"), sass_string), |mut s| {
                s.prepend("*");
                s
            }),
            sass_string,
        )),
        delimited(ignore_comments, tag(":"), ignore_comments),
    )(input)?;

    let (input, val) = opt(value_expression)(start_val)?;
    let pos = start_val.up_to(&input).to_owned();
    let (input, _) = opt_spacelike(input)?;

    let (input, next) = if val.is_some() {
        alt((tag("{"), tag(";"), tag("")))(input)?
    } else {
        tag("{")(input)?
    };

    let (input, body) = match next.fragment() {
        b"{" => map(body_block2, Some)(input)?,
        b";" => (input, None),
        b"" => (input, None),
        _ => (input, None), // error?
    };
    let (input, _) = opt_spacelike(input)?;
    Ok((input, ns_or_prop_item(name, val, body, pos)))
}

use crate::sass::SassString;
fn ns_or_prop_item(
    name: SassString,
    value: Option<Value>,
    body: Option<Vec<Item>>,
    pos: SourcePos,
) -> Item {
    if let Some(body) = body {
        Item::NamespaceRule(name, value.unwrap_or(Value::Null), body)
    } else if let Some(value) = value {
        Item::Property(name, value, pos)
    } else {
        unreachable!()
    }
}

fn body_block(input: Span) -> PResult<Vec<Item>> {
    preceded(tag("{"), body_block2)(input)
}

fn body_block2(input: Span) -> PResult<Vec<Item>> {
    let (input, (v, _end)) = preceded(
        opt_spacelike,
        many_till(
            terminated(body_item, opt_spacelike),
            terminated(terminated(tag("}"), opt_spacelike), opt(tag(";"))),
        ),
    )(input)?;
    Ok((input, v))
}

fn input_to_str(s: Span) -> Result<&str, Utf8Error> {
    from_utf8(s.fragment())
}

fn input_to_string(s: Span) -> Result<String, Utf8Error> {
    from_utf8(s.fragment()).map(String::from)
}
