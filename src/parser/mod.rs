#[cfg(test)]
macro_rules! check_parse {
    ($parser:ident, $value:expr) => {{
        use crate::parser::{code_span, ParseError};
        ParseError::check($parser(code_span($value)))
            .unwrap_or_else(|e| panic!("{}", e))
    }};
}

mod error;
pub mod formalargs;
mod pos;
pub mod selectors;
mod strings;
mod unit;
mod util;
pub mod value;

pub use error::ParseError;
pub use pos::{SourceName, SourcePos};

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
use crate::functions::SassFunction;
#[cfg(test)]
use crate::sass::{CallArgs, FormalArgs};
use crate::sass::{Item, UseAs, Value};
use crate::selectors::Selectors;
use crate::value::ListSeparator;
#[cfg(test)]
use crate::value::{Number, Rgba, Unit};
use crate::Error;
use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag};
use nom::character::complete::one_of;
use nom::combinator::{
    all_consuming, map, map_res, opt, peek, value, verify,
};
use nom::multi::{
    fold_many0, many0, many_till, separated_list0, separated_list1,
};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;
use nom_locate::{position, LocatedSpan};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::{from_utf8, Utf8Error};

pub type Span<'a> = LocatedSpan<&'a [u8], &'a SourceName>;

pub fn code_span(value: &[u8]) -> Span {
    use lazy_static::lazy_static;
    lazy_static! {
        static ref SOURCE: SourceName = SourceName::root("(rsass)");
    }
    Span::new_extra(value, &SOURCE)
}

/// Parse a scss value.
///
/// Returns a single value (or an error).
pub fn parse_value_data(data: &[u8]) -> Result<Value, Error> {
    let data = code_span(data);
    Ok(ParseError::check(all_consuming(value_expression)(data))?)
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

/// Parse a scss file by path.
///
/// Returns a vec of the top level items of the file (or an error message).
pub fn parse_scss_path(path: &Path) -> Result<Vec<Item>, Error> {
    do_parse_scss_path(path, &SourceName::root(path.display()))
}

pub fn parse_imported_scss_path(
    path: &Path,
    from: SourcePos,
) -> Result<Vec<Item>, Error> {
    let source = SourceName::imported(path.display(), from);
    do_parse_scss_path(path, &source)
}

fn do_parse_scss_path(
    path: &Path,
    source: &SourceName,
) -> Result<Vec<Item>, Error> {
    let mut f = File::open(path)
        .map_err(|e| Error::Input(source.name().to_string(), e))?;
    do_parse_scss_file(&mut f, source)
}

/// Parse a scss file.
///
/// Returns a vec of the top level items of the file (or an error message).
pub fn parse_scss_file<T: Read>(
    file: &mut T,
    path: &str,
) -> Result<Vec<Item>, Error> {
    do_parse_scss_file(file, &SourceName::root(path))
}

pub fn parse_imported_scss_file<T: Read>(
    file: &mut T,
    path: &str,
    from: SourcePos,
) -> Result<Vec<Item>, Error> {
    let source = SourceName::imported(path, from);
    do_parse_scss_file(file, &source)
}

fn do_parse_scss_file<T: Read>(
    file: &mut T,
    source: &SourceName,
) -> Result<Vec<Item>, Error> {
    let mut data = vec![];
    file.read_to_end(&mut data)
        .map_err(|e| Error::Input(source.name().to_string(), e))?;
    let data = Span::new_extra(&data, &source);
    Ok(ParseError::check(sassfile(data))?)
}

/// Parse scss data from a buffer.
///
/// Returns a vec of the top level items of the file (or an error message).
pub fn parse_scss_data(data: &[u8]) -> Result<Vec<Item>, ParseError> {
    ParseError::check(sassfile(code_span(data)))
}

fn sassfile(input: Span) -> IResult<Span, Vec<Item>> {
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

fn top_level_item(input: Span) -> IResult<Span, Item> {
    let (input, tag) = alt((tag("$"), tag("/*"), tag("@"), tag("")))(input)?;
    match *tag.fragment() {
        b"$" => variable_declaration2(input),
        b"/*" => comment_item(input),
        b"@" => at_rule2(input),
        b"" => rule(input),
        _ => unreachable!(),
    }
}

fn comment_item(input: Span) -> IResult<Span, Item> {
    map(comment2, Item::Comment)(input)
}

fn rule(input: Span) -> IResult<Span, Item> {
    map(pair(rule_start, body_block2), |(selectors, body)| {
        Item::Rule(selectors, body)
    })(input)
}

fn rule_start(input: Span) -> IResult<Span, Selectors> {
    terminated(selectors, terminated(opt(is_a(", \t\r\n")), tag("{")))(input)
}

fn body_item(input: Span) -> IResult<Span, Item> {
    let (input, tag) = alt((
        tag("$"),
        tag("/*"),
        tag(";"),
        tag("@at-root"),
        tag("@content"),
        tag("@return"),
        tag("@"),
        tag(""),
    ))(input)?;
    match *tag.fragment() {
        b"$" => variable_declaration2(input),
        b"/*" => comment_item(input),
        b";" => Ok((input, Item::None)),
        b"@at-root" => at_root2(input),
        b"@content" => content_stmt2(input),
        b"@return" => return_stmt2(input),
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
fn import2(input: Span) -> IResult<Span, Item> {
    map(
        terminated(
            tuple((
                position,
                separated_list0(
                    preceded(tag(","), ignore_comments),
                    alt((
                        sass_string_dq,
                        sass_string_sq,
                        special_url,
                        sass_string,
                    )),
                ),
                opt(media_args),
            )),
            preceded(
                opt(ignore_space),
                alt((tag(";"), all_consuming(tag("")))),
            ),
        ),
        |(position, import, args)| {
            Item::Import(import, args.unwrap_or(Value::Null), position.into())
        },
    )(input)
}

/// What follows the `@at-root` tag.
fn at_root2(input: Span) -> IResult<Span, Item> {
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
fn mixin_call(input: Span) -> IResult<Span, Item> {
    preceded(tag("@include "), mixin_call2)(input)
}

/// What follows the `@include` tag.
fn mixin_call2(input: Span) -> IResult<Span, Item> {
    let (input, name) = terminated(name, opt_spacelike)(input)?;
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

/// What follows an `@` sign
fn at_rule2(input: Span) -> IResult<Span, Item> {
    let (input, name) = terminated(name, opt_spacelike)(input)?;
    match name.as_ref() {
        "debug" => map(expression_argument, Item::Debug)(input),
        "each" => each_loop2(input),
        "error" => map(expression_argument, Item::Error)(input),
        "charset" => charset2(input),
        "for" => for_loop2(input),
        "function" => function_declaration2(input),
        "import" => import2(input),
        "include" => mixin_call2(input),
        "mixin" => mixin_declaration2(input),
        "if" => if_statement2(input),
        "use" => use2(input),
        "warn" => map(expression_argument, Item::Warn)(input),
        "while" => while_loop2(input),
        _ => {
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
    }
}

fn use2(input: Span) -> IResult<Span, Item> {
    map(
        terminated(
            pair(
                terminated(
                    alt((sass_string_dq, sass_string_sq, sass_string)),
                    opt_spacelike,
                ),
                opt(delimited(
                    terminated(tag("as"), opt_spacelike),
                    alt((
                        map(sass_string, UseAs::Name),
                        value(UseAs::Star, tag("*")),
                    )),
                    opt_spacelike,
                )),
            ),
            tag(";"),
        ),
        |(s, n)| Item::Use(s, n.unwrap_or(UseAs::KeepName)),
    )(input)
}

fn expression_argument(input: Span) -> IResult<Span, Value> {
    terminated(value_expression, opt(tag(";")))(input)
}

fn charset2(input: Span) -> IResult<Span, Item> {
    use nom::combinator::map_opt;
    map_opt(
        terminated(
            alt((sass_string_dq, sass_string_sq, sass_string)),
            preceded(
                opt(ignore_space),
                alt((tag(";"), all_consuming(tag("")))),
            ),
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

fn media_args(input: Span) -> IResult<Span, Value> {
    let (input, args) = separated_list0(
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
                        alt((
                            value((), all_consuming(tag(""))),
                            value((), peek(one_of(") \r\n\t{,;"))),
                        )),
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
    check_parse!(media_args, b"#{$media} and ($key + \"-foo\": $value + 5)");
}
#[test]
fn test_media_args_2() {
    check_parse!(
        media_args,
        b"print and (foo: 1 2 3), (bar: 3px hux(muz)), not screen"
    );
}

#[cfg(test)] // TODO: Or remove this?
fn if_statement(input: Span) -> IResult<Span, Item> {
    preceded(tag("@if "), if_statement2)(input)
}

fn if_statement_inner(input: Span) -> IResult<Span, Item> {
    preceded(
        terminated(verify(name, |n: &String| n == "if"), opt_spacelike),
        if_statement2,
    )(input)
}

fn if_statement2(input: Span) -> IResult<Span, Item> {
    let (input, cond) = terminated(value_expression, opt_spacelike)(input)?;
    let (input, body) = body_block(input)?;
    let (input2, word) = opt(delimited(
        preceded(opt_spacelike, tag("@")),
        name,
        opt_spacelike,
    ))(input)?;
    match word.as_ref().map(|w| w.as_ref()) {
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
fn each_loop2(input: Span) -> IResult<Span, Item> {
    let (input, names) = separated_list1(
        delimited(opt_spacelike, tag(","), opt_spacelike),
        preceded(tag("$"), name),
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
fn for_loop2(input: Span) -> IResult<Span, Item> {
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
            name,
            from: Box::new(from),
            to: Box::new(to),
            inclusive,
            body,
        },
    ))
}

fn while_loop2(input: Span) -> IResult<Span, Item> {
    let (input, cond) = terminated(value_expression, opt_spacelike)(input)?;
    let (input, body) = body_block(input)?;
    Ok((input, Item::While(cond, body)))
}

#[cfg(test)] // TODO: Or remove this?
fn mixin_declaration(input: Span) -> IResult<Span, Item> {
    preceded(tag("@mixin "), mixin_declaration2)(input)
}

fn mixin_declaration2(input: Span) -> IResult<Span, Item> {
    let (input, name) = terminated(name, opt_spacelike)(input)?;
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

fn function_declaration2(input: Span) -> IResult<Span, Item> {
    let (input, name) = terminated(name, opt_spacelike)(input)?;
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

fn return_stmt2(input: Span) -> IResult<Span, Item> {
    let (input, v) =
        delimited(opt_spacelike, value_expression, opt_spacelike)(input)?;
    let (input, _) = opt(tag(";"))(input)?;
    Ok((input, Item::Return(v)))
}

/// The "rest" of an `@content` statement is just an optional terminator
fn content_stmt2(input: Span) -> IResult<Span, Item> {
    let (input, _) = opt_spacelike(input)?;
    let (input, _) = opt(tag(";"))(input)?;
    Ok((input, Item::Content))
}

fn property_or_namespace_rule(input: Span) -> IResult<Span, Item> {
    let (input, name) = terminated(
        sass_string,
        delimited(ignore_comments, tag(":"), ignore_comments),
    )(input)?;

    let (input, val) =
        opt(terminated(value_expression, opt_spacelike))(input)?;

    let (input, next) = if val.is_some() {
        alt((tag("{"), tag(";"), tag("")))(input)?
    } else {
        tag("{")(input)?
    };

    let (input, body) = match *next.fragment() {
        b"{" => map(body_block2, Some)(input)?,
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

fn body_block(input: Span) -> IResult<Span, Vec<Item>> {
    preceded(tag("{"), body_block2)(input)
}

fn body_block2(input: Span) -> IResult<Span, Vec<Item>> {
    let (input, (v, _end)) = preceded(
        opt_spacelike,
        many_till(
            terminated(body_item, opt_spacelike),
            terminated(terminated(tag("}"), opt_spacelike), opt(tag(";"))),
        ),
    )(input)?;
    Ok((input, v))
}

#[cfg(test)] // TODO: Or remove this?
fn variable_declaration(input: Span) -> IResult<Span, Item> {
    preceded(tag("$"), variable_declaration2)(input)
}

fn variable_declaration2(input: Span) -> IResult<Span, Item> {
    let (input, name) = terminated(
        name,
        delimited(opt_spacelike, tag(":"), opt_spacelike),
    )(input)?;
    let (input, val) = terminated(value_expression, opt_spacelike)(input)?;
    let (input, (default, global)) = fold_many0(
        terminated(
            alt((
                map(tag("!default"), |_| (true, false)),
                map(tag("!global"), |_| (false, true)),
            )),
            opt_spacelike,
        ),
        (false, false),
        |(default, global), (d, g)| (default || d, global || g),
    )(input)?;
    let (input, _) = alt((
        peek(tag("}")), // semicolon optional for last thing in block
        all_consuming(tag("")), // ... or last thing in file
        terminated(tag(";"), opt_spacelike), // or end with semicolon
    ))(input)?;
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

fn input_to_str(s: Span) -> Result<&str, Utf8Error> {
    from_utf8(s.fragment())
}

fn input_to_string(s: Span) -> Result<String, Utf8Error> {
    from_utf8(s.fragment()).map(String::from)
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
        check_parse!(if_statement, b"@if true { p { color: black; } }\n"),
        Item::IfStatement(
            Value::True,
            vec![Item::Rule(
                selectors(code_span(b"p")).unwrap().1,
                vec![Item::Property("color".into(), Value::black())],
            )],
            vec![],
        ),
    )
}

#[test]
fn test_mixin_call_noargs() {
    assert_eq!(
        check_parse!(mixin_call, b"@include foo;"),
        Item::MixinCall {
            name: "foo".to_string(),
            args: CallArgs::new(vec![]),
            body: vec![],
        },
    )
}

#[test]
fn test_mixin_call_pos_args() {
    assert_eq!(
        check_parse!(mixin_call, b"@include foo(bar, baz);"),
        Item::MixinCall {
            name: "foo".to_string(),
            args: CallArgs::new(vec![
                (None, string("bar")),
                (None, string("baz")),
            ]),
            body: vec![],
        },
    )
}

#[test]
fn test_mixin_call_named_args() {
    assert_eq!(
        check_parse!(mixin_call, b"@include foo($x: bar, $y: baz);"),
        Item::MixinCall {
            name: "foo".to_string(),
            args: CallArgs::new(vec![
                (Some("x".into()), string("bar")),
                (Some("y".into()), string("baz")),
            ]),
            body: vec![],
        },
    )
}

#[test]
fn test_mixin_declaration_empty() {
    assert_eq!(
        check_parse!(mixin_declaration, b"@mixin foo() {}\n"),
        Item::MixinDeclaration {
            name: "foo".into(),
            args: FormalArgs::default(),
            body: vec![],
        },
    )
}

#[test]
fn test_mixin_declaration() {
    assert_eq!(
        check_parse!(
            mixin_declaration,
            b"@mixin foo($x) {\n  foo-bar: baz $x;\n}\n"
        ),
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
        },
    )
}

#[test]
fn test_mixin_declaration_default_and_subrules() {
    assert_eq!(
        check_parse!(
            mixin_declaration,
            b"@mixin bar($a, $b: flug) {\
              \n  foo-bar: baz;\
              \n  foo, bar {\
              \n    property: $b;\
              \n  }\
              \n}\n"
        ),
        Item::MixinDeclaration {
            name: "bar".into(),
            args: FormalArgs::new(
                vec![("a".into(), Value::Null), ("b".into(), string("flug"))],
                false,
            ),
            body: vec![
                Item::Property("foo-bar".into(), string("baz")),
                Item::Rule(
                    selectors(code_span(b"foo, bar")).unwrap().1,
                    vec![Item::Property(
                        "property".into(),
                        Value::Variable("b".into()),
                    )],
                ),
            ],
        },
    )
}

#[test]
fn test_simple_property() {
    assert_eq!(
        check_parse!(property_or_namespace_rule, b"color: red;\n"),
        Item::Property(
            "color".into(),
            Value::Color(Rgba::from_rgb(255, 0, 0), Some("red".into())),
        ),
    )
}

#[test]
fn test_property_2() {
    assert_eq!(
        check_parse!(
            property_or_namespace_rule,
            b"background-position: 90% 50%;\n"
        ),
        Item::Property(
            "background-position".into(),
            Value::List(
                vec![percentage(90), percentage(50)],
                ListSeparator::Space,
                false,
            ),
        ),
    )
}

#[test]
fn test_variable_declaration_simple() {
    assert_eq!(
        check_parse!(variable_declaration, b"$foo: bar;\n"),
        Item::VariableDeclaration {
            name: "foo".into(),
            val: string("bar"),
            default: false,
            global: false,
        },
    )
}

#[test]
fn test_variable_declaration_global() {
    assert_eq!(
        check_parse!(variable_declaration, b"$y: some value !global;\n"),
        Item::VariableDeclaration {
            name: "y".into(),
            val: Value::List(
                vec![string("some"), string("value")],
                ListSeparator::Space,
                false,
            ),
            default: false,
            global: true,
        },
    )
}

#[test]
fn test_variable_declaration_default() {
    assert_eq!(
        check_parse!(variable_declaration, b"$y: some value !default;\n"),
        Item::VariableDeclaration {
            name: "y".into(),
            val: Value::List(
                vec![string("some"), string("value")],
                ListSeparator::Space,
                false,
            ),
            default: true,
            global: false,
        },
    )
}
