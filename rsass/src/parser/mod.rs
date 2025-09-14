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
    dictionary, function_call_or_string_rulearg, single_value,
    value_expression,
};
use crate::input::{SourceFile, SourceName, SourcePos};
use crate::sass::parser::{
    src_range, variable_declaration2, variable_declaration_mod,
};
use crate::sass::{
    Callable, FormalArgs, Item, ItemBody, Name, Selectors, SrcValue, Value,
};
use crate::value::ListSeparator;
#[cfg(test)]
use crate::value::{Numeric, Unit};
use crate::Error;
use imports::{forward2, import2, use2};
use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag};
use nom::character::complete::{char, one_of};
use nom::combinator::{
    all_consuming, into, map, map_res, not, opt, peek, value, verify,
};
use nom::error::context;
use nom::multi::{many0, many_till, separated_list0, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::{IResult, Parser as _};
use nom_language::error::VerboseError;
use std::str::{from_utf8, Utf8Error};

/// A Parsing Result; ok gives a span for the rest of the data and a parsed T.
pub(crate) type PResult<'a, T> = IResult<Span<'a>, T, VerboseError<Span<'a>>>;

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
    let value = all_consuming(value_expression).parse(data.borrow());
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

pub(crate) fn sassfile(input: Span) -> PResult<ItemBody> {
    preceded(
        opt(tag("\u{feff}".as_bytes())),
        into(map(
            many_till(
                preceded(opt_spacelike, top_level_item),
                all_consuming(opt_spacelike),
            ),
            |(v, _eof)| v,
        )),
    )
    .parse(input)
}

fn top_level_item(input: Span) -> PResult<Item> {
    let (rest, tag) =
        alt((tag("$"), tag("/*"), tag("@"), tag(""))).parse(input)?;
    match tag.fragment() {
        b"$" => into(variable_declaration2).parse(rest),
        b"/*" => comment_item(rest),
        b"@" => at_rule2(input),
        b"" => alt((into(variable_declaration_mod), rule)).parse(input),
        _ => unreachable!(),
    }
}

fn comment_item(input: Span) -> PResult<Item> {
    map(comment2, Item::Comment).parse(input)
}

fn rule(input: Span) -> PResult<Item> {
    map(pair(rule_start, body_block2), |(selectors, body)| {
        Item::Rule(selectors, body)
    })
    .parse(input)
}

fn rule_start(input: Span) -> PResult<Selectors> {
    terminated(selectors, terminated(opt(is_a(", \t\r\n")), tag("{")))
        .parse(input)
}

fn body_item(input: Span) -> PResult<Item> {
    let (rest, tag) =
        alt((tag("$"), tag("/*"), tag(";"), tag("@"), tag("--"), tag("")))
            .parse(input)?;
    match tag.fragment() {
        b"$" => into(variable_declaration2).parse(rest),
        b"/*" => comment_item(rest),
        b";" => Ok((rest, Item::None)),
        b"@" => at_rule2(input),
        b"--" => {
            let result = custom_property(input);
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
    )
    .parse(input)
}

/// What follows the `@include` tag.
fn mixin_call<'a>(start: Span, input: Span<'a>) -> PResult<'a, Item> {
    let (rest, n1) = terminated(name, opt_spacelike).parse(input)?;
    let (rest, n2) = opt(preceded(tag("."), name)).parse(rest)?;
    let name = n2.map(|n2| format!("{n1}.{n2}")).unwrap_or(n1);
    let (rest, _) = opt_spacelike(rest)?;
    let (rest0, args) =
        terminated(opt(call_args), ignore_comments).parse(rest)?;
    let (rest, t) = alt((tag("using"), tag("{"), tag(""))).parse(rest0)?;
    let (end, body) = match t.fragment() {
        b"using" => {
            let (end, args) =
                preceded(ignore_comments, formal_args).parse(rest)?;
            let (rest, body) =
                preceded(ignore_comments, body_block).parse(end)?;
            let decl = rest0.up_to(&end).to_owned();
            (rest, Some(Callable::new(args, body, decl)))
        }
        b"{" => {
            let (rest, body) = body_block(rest0)?;
            let decl = rest0.up_to(&rest).to_owned();
            (rest, Some(Callable::no_args(body, decl)))
        }
        _ => map(semi_or_end, |_| None).parse(rest)?,
    };
    let pos = start.up_to(&rest).to_owned();
    Ok((
        end,
        Item::MixinCall(name, args.unwrap_or_default(), body, pos),
    ))
}

/// When we know that `input0` starts with an `@` sign.
fn at_rule2(input0: Span) -> PResult<Item> {
    let (input, name) = delimited(
        tag("@"),
        context("Expected identifier.", sass_string),
        ignore_comments,
    )
    .parse(input0)?;
    match name.single_raw().unwrap_or("") {
        "at-root" => at_root2(input),
        "charset" => charset2(input),
        "content" => content_stmt2(input),
        "debug" => map(expression_argument, Item::Debug).parse(input),
        "each" => each_loop2(input),
        "error" => {
            let (end, v) = value_expression(input)?;
            let (rest, _) = opt(tag(";")).parse(end)?;
            let pos = input0.up_to(&end).to_owned();
            Ok((rest, Item::Error(v, pos)))
        }
        "extend" => map(
            delimited(opt_spacelike, selectors, semi_or_end),
            Item::Extend,
        )
        .parse(input),
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
        "warn" => map(expression_argument, Item::Warn).parse(input),
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
        terminated(opt(unknown_rule_args), opt(ignore_space)).parse(input)?;
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
        map(body_block, Some).parse(input)?
    } else {
        value(None, semi_or_end).parse(input)?
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
    terminated(value_expression, opt(tag(";"))).parse(input)
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
    )
    .parse(input)
}

/// Arguments to an unkown at rule.
fn unknown_rule_args(input: Span) -> PResult<Value> {
    let (input, args) = separated_list0(
        preceded(tag(","), opt_spacelike),
        map(
            many0(preceded(
                opt_spacelike,
                alt((
                    terminated(
                        alt((
                            function_call_or_string_rulearg,
                            dictionary,
                            map(
                                delimited(tag("("), media::args, tag(")")),
                                |v| Value::Paren(Box::new(v), true),
                            ),
                            map(sass_string_dq, Value::Literal),
                            map(sass_string_sq, Value::Literal),
                        )),
                        terminated(
                            alt((
                                value((), all_consuming(tag(""))),
                                value((), peek(one_of(") \r\n\t{,;/"))),
                            )),
                            opt_spacelike,
                        ),
                    ),
                    map(map_res(is_not("\"'{};#"), input_to_str), |s| {
                        Value::Literal(s.trim_end().into())
                    }),
                )),
            )),
            |args| list_or_single(args, ListSeparator::Space),
        ),
    )
    .parse(input)?;
    Ok((input, list_or_single(args, ListSeparator::Comma)))
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
    )
    .parse(input)
}

fn if_statement2(input: Span) -> PResult<Item> {
    let (input, cond) =
        terminated(value_expression, opt_spacelike).parse(input)?;
    let (input, body) = body_block(input)?;
    let (input2, word) = opt(delimited(
        preceded(opt_spacelike, tag("@")),
        name,
        opt_spacelike,
    ))
    .parse(input)?;
    match word.as_ref().map(AsRef::as_ref) {
        Some("else") => {
            let (input2, else_body) =
                alt((map(if_statement_inner, ItemBody::single), body_block))
                    .parse(input2)?;
            Ok((input2, Item::IfStatement(cond, body, else_body)))
        }
        Some("elseif") => {
            let (input2, else_body) = if_statement2(input2)?;
            Ok((
                input2,
                Item::IfStatement(cond, body, ItemBody::single(else_body)),
            ))
        }
        _ => Ok((input, Item::IfStatement(cond, body, ItemBody::empty()))),
    }
}

/// The part of an each look that follows the `@each`.
fn each_loop2(input: Span) -> PResult<Item> {
    let (input, names) = separated_list1(
        delimited(opt_spacelike, tag(","), opt_spacelike),
        map(preceded(tag("$"), name), Name::from),
    )
    .parse(input)?;
    let (input, values) = delimited(
        delimited(spacelike, tag("in"), spacelike),
        value_expression,
        opt_spacelike,
    )
    .parse(input)?;
    let (input, body) = body_block(input)?;
    Ok((input, Item::Each(names, values, body)))
}

/// A for loop after the initial `@for`.
fn for_loop2(input: Span) -> PResult<Item> {
    let (input, name) =
        delimited(tag("$"), name, ignore_comments).parse(input)?;
    let (input, range) = src_range(input)?;
    let (input, body) = body_block(input)?;
    Ok((input, Item::For(name.into(), Box::new(range), body)))
}

/// A single `SrcValue`.
///
/// That is, a single sass value with source position.
pub fn single_value_p(input: Span) -> PResult<SrcValue> {
    let (rest, value) = single_value(input)?;
    let pos = input.up_to(&rest).to_owned();
    Ok((rest, SrcValue::new(value, pos)))
}

fn while_loop2(input: Span) -> PResult<Item> {
    let (input, cond) =
        terminated(value_expression, opt_spacelike).parse(input)?;
    let (input, body) = body_block(input)?;
    Ok((input, Item::While(cond, body)))
}

fn mixin_declaration2(input: Span) -> PResult<Item> {
    let (rest, (name, args)) = pair(
        terminated(name, ignore_comments),
        alt((value(None, peek(not(char('(')))), map(formal_args, Some))),
    )
    .parse(input)?;
    let (end, body) = preceded(ignore_comments, body_block).parse(rest)?;
    let args = args.unwrap_or_else(FormalArgs::none);
    let decl = input.up_to(&rest).to_owned();
    Ok((
        end,
        Item::MixinDeclaration(name, Callable { args, body, decl }),
    ))
}

fn function_declaration2(input: Span) -> PResult<Item> {
    let (end, name) = terminated(name, ignore_comments).parse(input)?;
    let (end, args) = formal_args(end)?;
    let (rest, body) = preceded(ignore_comments, body_block).parse(end)?;
    let decl = input.up_to(&end).to_owned();
    Ok((
        rest,
        Item::FunctionDeclaration(name, Callable { args, body, decl }),
    ))
}

fn return_stmt2<'a>(start: Span, input: Span<'a>) -> PResult<'a, Item> {
    let (input, v) =
        terminated(value_expression, ignore_comments).parse(input)?;
    let pos = start.up_to(&input).to_owned();
    let (input, _) = opt(tag(";")).parse(input)?;
    Ok((input, Item::Return(v, pos)))
}

fn content_stmt2(input: Span) -> PResult<Item> {
    let (rest, args) =
        terminated(opt(call_args), opt(tag(";"))).parse(input)?;
    let pos = input.up_to(&rest).to_owned();
    Ok((rest, Item::Content(args.unwrap_or_default(), pos)))
}

fn custom_property(input: Span) -> PResult<Item> {
    let (rest, name) = terminated(sass_string, char(':')).parse(input)?;
    let (rest, value) = terminated(custom_value, semi_or_end).parse(rest)?;
    Ok((rest, Item::CustomProperty(name, value)))
}

fn property_or_namespace_rule(input: Span) -> PResult<Item> {
    let (start_val, (star, mut name)) = terminated(
        (opt(tag("*")), sass_string),
        (ignore_comments, char(':'), ignore_comments),
    )
    .parse(input)?;
    if star.is_some() {
        name.prepend("*");
    }

    let (input, val) = alt((
        map(peek(char('{')), |_| None),
        map(context("Expected expression.", value_expression), Some),
    ))
    .parse(start_val)?;

    let pos = start_val.up_to(&input);

    let (input, body) = preceded(
        ignore_comments,
        alt((map(semi_or_end, |_| None), map(body_block, Some))),
    )
    .parse(input)?;
    Ok((input, ns_or_prop_item(name, val, body, pos.to_owned())))
}

use crate::sass::SassString;
fn ns_or_prop_item(
    name: SassString,
    value: Option<Value>,
    body: Option<ItemBody>,
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

fn body_block(input: Span) -> PResult<ItemBody> {
    preceded(char('{'), body_block2).parse(input)
}

fn body_block2(input: Span) -> PResult<ItemBody> {
    let (input, (v, _end)) = preceded(
        opt_spacelike,
        many_till(
            terminated(body_item, opt_spacelike),
            terminated(terminated(tag("}"), opt_spacelike), opt(tag(";"))),
        ),
    )
    .parse(input)?;
    Ok((input, v.into()))
}

pub(crate) fn input_to_str(s: Span<'_>) -> Result<&str, Utf8Error> {
    from_utf8(s.fragment())
}

fn input_to_string(s: Span) -> Result<String, Utf8Error> {
    from_utf8(s.fragment()).map(String::from)
}

fn list_or_single(list: Vec<Value>, sep: ListSeparator) -> Value {
    if list.len() == 1 {
        list.into_iter().next().unwrap()
    } else {
        Value::List(list, Some(sep), false)
    }
}
