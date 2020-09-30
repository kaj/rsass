pub mod formalargs;
pub mod selectors;
mod strings;
mod unit;
mod util;
pub mod value;

use self::formalargs::{call_args, formal_args};
use self::selectors::selectors;
use self::strings::{
    name, sass_string, sass_string_dq, sass_string_sq, special_url,
};
use self::util::{
    comment2, ignore_comments, ignore_space, opt_spacelike, spacelike,
};
use self::value::{
    dictionary, function_call, single_value, value_expression,
};
use crate::error::{ErrPos, Error};
use crate::functions::SassFunction;
#[cfg(test)]
use crate::sass::{CallArgs, FormalArgs};
use crate::sass::{Item, Value};
use crate::selectors::Selectors;
use crate::value::ListSeparator;
#[cfg(test)]
use crate::value::{Number, Rgba, Unit};
use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag};
use nom::character::complete::one_of;
use nom::combinator::{all_consuming, map, map_res, opt, peek, value};
use nom::error::ErrorKind;
use nom::multi::{many0, many_till, separated_list, separated_nonempty_list};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::{Err, IResult};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::{from_utf8, Utf8Error};

/// Parse a scss value.
///
/// Returns a single value (or an error).
pub fn parse_value_data(data: &[u8]) -> Result<Value, Error> {
    let (rest, result) = all_consuming(value_expression)(data)?;
    assert!(rest.is_empty());
    Ok(result)
}

#[test]
fn test_parse_value_data_1() -> Result<(), Error> {
    let v = parse_value_data(b"17em")?;
    assert_eq!(Value::Numeric(17.into(), Unit::Em), v);
    Ok(())
}

#[test]
fn test_parse_value_data_2() -> Result<(), Error> {
    let v = parse_value_data(b"17em;");
    assert!(v.is_err());
    Ok(())
}

/// Parse a scss file.
///
/// Returns a vec of the top level items of the file (or an error message).
pub fn parse_scss_file(file: &Path) -> Result<Vec<Item>, Error> {
    let mut f = File::open(file).map_err(|e| Error::Input(file.into(), e))?;
    let mut data = vec![];
    f.read_to_end(&mut data)
        .map_err(|e| Error::Input(file.into(), e))?;
    parse_scss_data(&data).map_err(|(pos, kind)| Error::ParseError {
        file: file.to_string_lossy().into(),
        pos: ErrPos::pos_of(pos, &data),
        kind,
    })
}

/// Parse scss data from a buffer.
///
/// Returns a vec of the top level items of the file (or an error message).
pub fn parse_scss_data(
    data: &[u8],
) -> Result<Vec<Item>, (usize, Option<ErrorKind>)> {
    match sassfile(data) {
        Ok((b"", items)) => Ok(items),
        Ok((rest, _styles)) => Err((data.len() - rest.len(), None)),
        Err(Err::Error((rest, err))) => {
            Err((data.len() - rest.len(), Some(err)))
        }
        Err(Err::Incomplete(_needed)) => Err((data.len(), None)),
        Err(Err::Failure((rest, err))) => {
            Err((data.len() - rest.len(), Some(err)))
        }
    }
}

fn sassfile(input: &[u8]) -> IResult<&[u8], Vec<Item>> {
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

fn top_level_item(input: &[u8]) -> IResult<&[u8], Item> {
    let (input, tag) = alt((
        tag("$"),
        tag("/*"),
        tag("@each"),
        tag("@error"),
        tag("@for"),
        tag("@function"),
        tag("@if"),
        tag("@import"),
        tag("@include"),
        tag("@mixin"),
        tag("@warn"),
        tag("@while"),
        tag("@"),
        tag(""),
    ))(input)?;
    match tag {
        b"$" => variable_declaration2(input),
        b"/*" => comment_item(input),
        b"@each" => each_loop2(input),
        b"@error" => error2(input),
        b"@for" => for_loop2(input),
        b"@function" => function_declaration2(input),
        b"@if" => if_statement2(input),
        b"@import" => import2(input),
        b"@include" => mixin_call2(input),
        b"@mixin" => mixin_declaration2(input),
        b"@warn" => warn2(input),
        b"@while" => while_loop2(input),
        b"@" => at_rule2(input),
        b"" => rule(input),
        _ => unreachable!(),
    }
}

fn comment_item(input: &[u8]) -> IResult<&[u8], Item> {
    let (rest, comment) = map_res(comment2, input_to_string)(input)?;
    let comment = comment
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .replace('\u{c}', "\n");
    Ok((rest, Item::Comment(comment)))
}

fn rule(input: &[u8]) -> IResult<&[u8], Item> {
    map(pair(rule_start, body_block2), |(selectors, body)| {
        Item::Rule(selectors, body)
    })(input)
}

fn rule_start(input: &[u8]) -> IResult<&[u8], Selectors> {
    terminated(selectors, terminated(opt(is_a(", \t\r\n")), tag("{")))(input)
}

fn body_item(input: &[u8]) -> IResult<&[u8], Item> {
    let (input, tag) = alt((
        tag("$"),
        tag("/*"),
        tag(";"),
        tag("@at-root"),
        tag("@content"),
        tag("@each"),
        tag("@error"),
        tag("@for"),
        tag("@function"),
        tag("@if"),
        tag("@import"),
        tag("@include"),
        tag("@mixin"),
        tag("@return"),
        tag("@warn"),
        tag("@while"),
        tag("@"),
        tag(""),
    ))(input)?;
    match tag {
        b"$" => variable_declaration2(input),
        b"/*" => comment_item(input),
        b";" => Ok((input, Item::None)),
        b"@at-root" => at_root2(input),
        b"@content" => content_stmt2(input),
        b"@error" => error2(input),
        b"@each" => each_loop2(input),
        b"@for" => for_loop2(input),
        b"@function" => function_declaration2(input),
        b"@if" => if_statement2(input),
        b"@import" => import2(input),
        b"@include" => mixin_call2(input),
        b"@mixin" => mixin_declaration2(input),
        b"@return" => return_stmt2(input),
        b"@warn" => warn2(input),
        b"@while" => while_loop2(input),
        b"@" => at_rule2(input),
        b"" => {
            let (input, selectors) = opt(rule_start)(input)?;
            match selectors {
                Some(selectors) => map(body_block2, |body| {
                    Item::Rule(selectors.clone(), body)
                })(input),
                None => property_or_namespace_rule(input),
            }
        }
        _ => unreachable!(),
    }
}

/// What follows the `@import` tag.
fn import2(input: &[u8]) -> IResult<&[u8], Item> {
    map(
        delimited(
            tag(" "),
            pair(
                separated_list(
                    preceded(tag(","), ignore_comments),
                    alt((
                        sass_string_dq,
                        sass_string_sq,
                        special_url,
                        sass_string,
                    )),
                ),
                opt(media_args),
            ),
            preceded(opt(ignore_space), tag(";")),
        ),
        |(import, args)| Item::Import(import, args.unwrap_or(Value::Null)),
    )(input)
}

/// What follows the `@at-root` tag.
fn at_root2(input: &[u8]) -> IResult<&[u8], Item> {
    preceded(
        opt_spacelike,
        map(
            pair(
                map(opt(selectors), |s| s.unwrap_or_else(Selectors::root)),
                body_block,
            ),
            |(selectors, body)| Item::AtRoot { selectors, body },
        ),
    )(input)
}

#[cfg(test)] // TODO: Or remove this?
fn mixin_call(input: &[u8]) -> IResult<&[u8], Item> {
    preceded(tag("@include"), mixin_call2)(input)
}

/// What follows the `@include` tag.
fn mixin_call2(input: &[u8]) -> IResult<&[u8], Item> {
    let (input, name) = delimited(spacelike, name, opt_spacelike)(input)?;
    let (input, args) = terminated(opt(call_args), opt_spacelike)(input)?;
    let (input, body) = terminated(
        opt(body_block),
        terminated(opt_spacelike, opt(tag(";"))),
    )(input)?;
    Ok((
        input,
        Item::MixinCall {
            name,
            args: args.unwrap_or_default(),
            body: body.unwrap_or_default(),
        },
    ))
}

/// What follows an `@` sign (unless specifically handled).
fn at_rule2(input: &[u8]) -> IResult<&[u8], Item> {
    let (input, name) = name(input)?;
    let (input, args) = opt(media_args)(input)?;
    let (input, body) = preceded(
        opt(ignore_space),
        alt((
            map(body_block, Some),
            value(None, all_consuming(tag(""))),
            value(None, tag(";")),
        )),
    )(input)?;
    Ok((
        input,
        Item::AtRule {
            name,
            args: args.unwrap_or(Value::Null),
            body,
        },
    ))
}

fn media_args(input: &[u8]) -> IResult<&[u8], Value> {
    let (input, args) = separated_list(
        preceded(tag(","), opt_spacelike),
        map(
            many0(preceded(
                opt(ignore_space),
                alt((
                    terminated(
                        alt((
                            function_call,
                            dictionary,
                            map(
                                delimited(tag("("), media_args, tag(")")),
                                |v| Value::Paren(Box::new(v), true),
                            ),
                            map(sass_string, Value::Literal),
                            map(sass_string_dq, Value::Literal),
                            map(sass_string_sq, Value::Literal),
                        )),
                        peek(one_of(") \r\n\t{,;")),
                    ),
                    map(map_res(is_not("#()\"'{};, "), input_to_str), |s| {
                        Value::Literal(s.trim_end().into())
                    }),
                )),
            )),
            |args| {
                if args.len() == 1 {
                    args.into_iter().next().unwrap()
                } else {
                    Value::List(args, ListSeparator::Space, false)
                }
            },
        ),
    )(input)?;
    Ok((
        input,
        if args.len() == 1 {
            args.into_iter().next().unwrap()
        } else {
            Value::List(args, ListSeparator::Comma, false)
        },
    ))
}

#[test]
fn test_media_args_1() {
    let (rest, _) =
        media_args(b"#{$media} and ($key + \"-foo\": $value + 5);").unwrap();
    assert_eq!(from_utf8(rest).unwrap(), ";");
}
#[test]
fn test_media_args_2() {
    let (rest, _) = media_args(
        b"print and (foo: 1 2 3), (bar: 3px hux(muz)), not screen;",
    )
    .unwrap();
    assert_eq!(from_utf8(rest).unwrap(), ";");
}

#[cfg(test)] // TODO: Or remove this?
fn if_statement(input: &[u8]) -> IResult<&[u8], Item> {
    preceded(tag("@if"), if_statement2)(input)
}

fn if_statement_inner(input: &[u8]) -> IResult<&[u8], Item> {
    preceded(tag("if"), if_statement2)(input)
}

fn if_statement2(input: &[u8]) -> IResult<&[u8], Item> {
    let (input, cond) =
        delimited(spacelike, value_expression, opt_spacelike)(input)?;
    let (input, body) = body_block(input)?;
    let (input, else_body) = opt(preceded(
        delimited(opt_spacelike, tag("@else"), opt_spacelike),
        alt((body_block, map(if_statement_inner, |s| vec![s]))),
    ))(input)?;
    Ok((
        input,
        Item::IfStatement(cond, body, else_body.unwrap_or_default()),
    ))
}

/// The part of an each look that follows the `@each`.
fn each_loop2(input: &[u8]) -> IResult<&[u8], Item> {
    let (input, names) = preceded(
        spacelike,
        separated_nonempty_list(
            delimited(opt_spacelike, tag(","), opt_spacelike),
            preceded(tag("$"), name),
        ),
    )(input)?;
    let (input, values) = delimited(
        delimited(spacelike, tag("in"), spacelike),
        value_expression,
        spacelike,
    )(input)?;
    let (input, body) = body_block(input)?;
    Ok((input, Item::Each(names, values, body)))
}

/// A for loop after the initial `@for`.
fn for_loop2(input: &[u8]) -> IResult<&[u8], Item> {
    let (input, name) =
        delimited(preceded(spacelike, tag("$")), name, spacelike)(input)?;
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
            name,
            from: Box::new(from),
            to: Box::new(to),
            inclusive,
            body,
        },
    ))
}

fn warn2(input: &[u8]) -> IResult<&[u8], Item> {
    let (input, arg) =
        delimited(spacelike, value_expression, opt(tag(";")))(input)?;
    Ok((input, Item::Warn(arg)))
}

fn error2(input: &[u8]) -> IResult<&[u8], Item> {
    let (input, arg) =
        delimited(spacelike, value_expression, opt(tag(";")))(input)?;
    Ok((input, Item::Error(arg)))
}

fn while_loop2(input: &[u8]) -> IResult<&[u8], Item> {
    let (input, cond) =
        delimited(spacelike, value_expression, spacelike)(input)?;
    let (input, body) = body_block(input)?;
    Ok((input, Item::While(cond, body)))
}

#[cfg(test)] // TODO: Or remove this?
fn mixin_declaration(input: &[u8]) -> IResult<&[u8], Item> {
    preceded(tag("@mixin"), mixin_declaration2)(input)
}

fn mixin_declaration2(input: &[u8]) -> IResult<&[u8], Item> {
    let (input, name) = delimited(spacelike, name, opt_spacelike)(input)?;
    let (input, args) = terminated(opt(formal_args), opt_spacelike)(input)?;
    let (input, body) = body_block(input)?;
    Ok((
        input,
        Item::MixinDeclaration {
            name,
            args: args.unwrap_or_default(),
            body,
        },
    ))
}

fn function_declaration2(input: &[u8]) -> IResult<&[u8], Item> {
    let (input, name) = delimited(spacelike, name, opt_spacelike)(input)?;
    let (input, args) = terminated(formal_args, opt_spacelike)(input)?;
    let (input, body) = body_block(input)?;
    Ok((
        input,
        Item::FunctionDeclaration {
            name,
            func: SassFunction::new(args, body),
        },
    ))
}

fn return_stmt2(input: &[u8]) -> IResult<&[u8], Item> {
    let (input, v) =
        delimited(opt_spacelike, value_expression, opt_spacelike)(input)?;
    let (input, _) = opt(tag(";"))(input)?;
    Ok((input, Item::Return(v)))
}

/// The "rest" of an `@content` statement is just an optional terminator
fn content_stmt2(input: &[u8]) -> IResult<&[u8], Item> {
    let (input, _) = opt_spacelike(input)?;
    let (input, _) = opt(tag(";"))(input)?;
    Ok((input, Item::Content))
}

fn property_or_namespace_rule(input: &[u8]) -> IResult<&[u8], Item> {
    let (input, name) = terminated(
        sass_string,
        delimited(opt_spacelike, tag(":"), opt_spacelike),
    )(input)?;

    let (input, val) =
        opt(terminated(value_expression, opt_spacelike))(input)?;

    let (input, next) = if val.is_some() {
        alt((tag("{"), tag(";"), tag("")))(input)?
    } else {
        tag("{")(input)?
    };

    let (input, body) = match next {
        b"{" => map(body_block2, |b| Some(b))(input)?,
        b";" => (input, None),
        b"" => (input, None),
        _ => (input, None), // error?
    };
    let (input, _) = opt_spacelike(input)?;

    Ok((input, ns_or_prop_item(name, val, body)))
}

use crate::sass::SassString;
fn ns_or_prop_item(
    name: SassString,
    value: Option<Value>,
    body: Option<Vec<Item>>,
) -> Item {
    if let Some(body) = body {
        Item::NamespaceRule(name, value.unwrap_or(Value::Null), body)
    } else if let Some(value) = value {
        Item::Property(name, value)
    } else {
        unreachable!()
    }
}

fn body_block(input: &[u8]) -> IResult<&[u8], Vec<Item>> {
    preceded(tag("{"), body_block2)(input)
}

fn body_block2(input: &[u8]) -> IResult<&[u8], Vec<Item>> {
    let (input, (v, _end)) = preceded(
        opt_spacelike,
        many_till(
            terminated(body_item, opt_spacelike),
            terminated(tag("}"), opt(tag(";"))),
        ),
    )(input)?;
    Ok((input, v))
}

#[cfg(test)] // TODO: Or remove this?
fn variable_declaration(input: &[u8]) -> IResult<&[u8], Item> {
    preceded(tag("$"), variable_declaration2)(input)
}

fn variable_declaration2(input: &[u8]) -> IResult<&[u8], Item> {
    let (input, name) = terminated(
        name,
        delimited(opt_spacelike, tag(":"), opt_spacelike),
    )(input)?;
    let (input, val) = terminated(value_expression, opt_spacelike)(input)?;
    let (input, default) = terminated(
        map(opt(tag("!default")), |d| d.is_some()),
        opt_spacelike,
    )(input)?;
    let (input, global) = terminated(
        map(opt(tag("!global")), |g| g.is_some()),
        opt_spacelike,
    )(input)?;
    let (input, _) = terminated(tag(";"), opt_spacelike)(input)?;
    Ok((
        input,
        Item::VariableDeclaration {
            name,
            val,
            default,
            global,
        },
    ))
}

fn input_to_str(s: &[u8]) -> Result<&str, Utf8Error> {
    from_utf8(&s)
}

fn input_to_string(s: &[u8]) -> Result<String, Utf8Error> {
    from_utf8(&s).map(String::from)
}

#[cfg(test)]
fn percentage(v: isize) -> Value {
    Value::Numeric(Number::from(v), Unit::Percent)
}

#[cfg(test)]
fn string(v: &str) -> Value {
    Value::Literal(v.into())
}

#[test]
fn if_with_no_else() {
    assert_eq!(
        if_statement(b"@if true { p { color: black; } }\n"),
        Ok((
            &b"\n"[..],
            Item::IfStatement(
                Value::True,
                vec![Item::Rule(
                    selectors(b"p").unwrap().1,
                    vec![Item::Property("color".into(), Value::black())],
                )],
                vec![]
            )
        ))
    )
}

#[test]
fn test_mixin_call_noargs() {
    assert_eq!(
        mixin_call(b"@include foo;\n"),
        Ok((
            &b"\n"[..],
            Item::MixinCall {
                name: "foo".to_string(),
                args: CallArgs::new(vec![]),
                body: vec![],
            }
        ))
    )
}

#[test]
fn test_mixin_call_pos_args() {
    assert_eq!(
        mixin_call(b"@include foo(bar, baz);\n"),
        Ok((
            &b"\n"[..],
            Item::MixinCall {
                name: "foo".to_string(),
                args: CallArgs::new(vec![
                    (None, string("bar")),
                    (None, string("baz")),
                ]),
                body: vec![],
            }
        ))
    )
}

#[test]
fn test_mixin_call_named_args() {
    assert_eq!(
        mixin_call(b"@include foo($x: bar, $y: baz);\n"),
        Ok((
            &b"\n"[..],
            Item::MixinCall {
                name: "foo".to_string(),
                args: CallArgs::new(vec![
                    (Some("x".into()), string("bar")),
                    (Some("y".into()), string("baz")),
                ]),
                body: vec![],
            }
        ))
    )
}

#[test]
fn test_mixin_declaration_empty() {
    assert_eq!(
        mixin_declaration(b"@mixin foo() {}\n"),
        Ok((
            &b"\n"[..],
            Item::MixinDeclaration {
                name: "foo".into(),
                args: FormalArgs::default(),
                body: vec![],
            }
        ))
    )
}

#[test]
fn test_mixin_declaration() {
    assert_eq!(
        mixin_declaration(b"@mixin foo($x) {\n  foo-bar: baz $x;\n}\n"),
        Ok((
            &b"\n"[..],
            Item::MixinDeclaration {
                name: "foo".into(),
                args: FormalArgs::new(vec![("x".into(), Value::Null)], false),
                body: vec![Item::Property(
                    "foo-bar".into(),
                    Value::List(
                        vec![string("baz"), Value::Variable("x".into())],
                        ListSeparator::Space,
                        false,
                    ),
                )],
            }
        ))
    )
}

#[test]
fn test_mixin_declaration_default_and_subrules() {
    assert_eq!(
        mixin_declaration(
            b"@mixin bar($a, $b: flug) {\n  \
                                   foo-bar: baz;\n  \
                                   foo, bar {\n    \
                                   property: $b;\n  \
                                   }\n\
                                   }\n"
        ),
        Ok((
            &b"\n"[..],
            Item::MixinDeclaration {
                name: "bar".into(),
                args: FormalArgs::new(
                    vec![
                        ("a".into(), Value::Null),
                        ("b".into(), string("flug")),
                    ],
                    false
                ),
                body: vec![
                    Item::Property("foo-bar".into(), string("baz")),
                    Item::Rule(
                        selectors(b"foo, bar").unwrap().1,
                        vec![Item::Property(
                            "property".into(),
                            Value::Variable("b".into()),
                        )],
                    ),
                ],
            }
        ))
    )
}

#[test]
fn test_simple_property() {
    assert_eq!(
        property_or_namespace_rule(b"color: red;\n"),
        Ok((
            &b""[..],
            Item::Property(
                "color".into(),
                Value::Color(Rgba::from_rgb(255, 0, 0), Some("red".into())),
            )
        ))
    )
}

#[test]
fn test_property_2() {
    assert_eq!(
        property_or_namespace_rule(b"background-position: 90% 50%;\n"),
        Ok((
            &b""[..],
            Item::Property(
                "background-position".into(),
                Value::List(
                    vec![percentage(90), percentage(50)],
                    ListSeparator::Space,
                    false,
                ),
            )
        ))
    )
}

#[test]
fn test_variable_declaration_simple() {
    assert_eq!(
        variable_declaration(b"$foo: bar;\n"),
        Ok((
            &b""[..],
            Item::VariableDeclaration {
                name: "foo".into(),
                val: string("bar"),
                default: false,
                global: false,
            }
        ))
    )
}

#[test]
fn test_variable_declaration_global() {
    assert_eq!(
        variable_declaration(b"$y: some value !global;\n"),
        Ok((
            &b""[..],
            Item::VariableDeclaration {
                name: "y".into(),
                val: Value::List(
                    vec![string("some"), string("value")],
                    ListSeparator::Space,
                    false,
                ),
                default: false,
                global: true,
            }
        ))
    )
}

#[test]
fn test_variable_declaration_default() {
    assert_eq!(
        variable_declaration(b"$y: some value !default;\n"),
        Ok((
            &b""[..],
            Item::VariableDeclaration {
                name: "y".into(),
                val: Value::List(
                    vec![string("some"), string("value")],
                    ListSeparator::Space,
                    false,
                ),
                default: true,
                global: false,
            }
        ))
    )
}
