#[cfg(test)]
macro_rules! check_parse {
    ($parser:ident, $value:expr) => {{
        use crate::parser::{code_span, ParseError};
        ParseError::check($parser(code_span($value)))
            .unwrap_or_else(|e| panic!("{}", e))
    }};
}

pub(crate) mod css;
mod css_function;
mod error;
pub mod formalargs;
mod imports;
mod pos;
pub mod selectors;
pub(crate) mod strings;
mod unit;
pub(crate) mod util;
pub mod value;

pub use error::ParseError;
pub use pos::{SourceName, SourcePos};

use self::formalargs::{call_args, formal_args};
use self::selectors::selectors;
use self::strings::{
    custom_value, name, sass_string, sass_string_dq, sass_string_sq,
};
use self::util::{
    comment2, ignore_comments, ignore_space, opt_spacelike, semi_or_end,
    spacelike,
};
use self::value::{
    dictionary, function_call, single_value, value_expression,
};
use crate::sass::{FormalArgs, Item, Name, Selectors, Value};
use crate::value::ListSeparator;
#[cfg(test)]
use crate::value::{Numeric, Rgba, Unit};
use crate::Error;
use imports::{forward2, import2, use2};
use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag};
use nom::character::complete::one_of;
use nom::combinator::{
    all_consuming, map, map_res, opt, peek, value, verify,
};
use nom::multi::{
    fold_many0, many0, many_till, separated_list0, separated_list1,
};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::IResult;
use nom_locate::LocatedSpan;
use std::convert::TryFrom;
use std::io::Read;
use std::str::{from_utf8, Utf8Error};

pub type Span<'a> = LocatedSpan<&'a [u8], &'a SourceName>;
/// A Parsing Result; ok gives a span for the rest of the data and a parsed T.
type PResult<'a, T> = IResult<Span<'a>, T>;

/// A supported input format.
///
/// Rsass handles the scss format and raw css.
/// TODO: In the future, the sass format may also be supported.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SourceFormat {
    /// The scss format is the main input format.
    Scss,
    /// The css format
    Css,
}

impl TryFrom<&str> for SourceFormat {
    type Error = Error;
    fn try_from(name: &str) -> Result<SourceFormat, Error> {
        if name.ends_with(".scss") {
            Ok(SourceFormat::Scss)
        } else if name.ends_with(".css") {
            Ok(SourceFormat::Css)
        } else {
            Err(Error::error(format!("Unknown source format: {:?}", name)))
        }
    }
}

/// The full data of a source file.
///
/// This type contains both the contents (as a `Vec<u8>`) of a file
/// and information on from where (and why) it was loaded.
/// You can create a SourceFile with the [`SourceFile::read`]
/// constructor, but normally you will get one from a
/// [`FileContext`][crate::FileContext].
pub struct SourceFile {
    data: Vec<u8>,
    source: SourceName,
    format: SourceFormat,
}

impl SourceFile {
    /// Create a SourceFile from something readable and a name.
    pub fn read<T: Read>(
        file: &mut T,
        source: SourceName,
    ) -> Result<Self, Error> {
        let format = SourceFormat::try_from(source.name())?;
        let mut data = vec![];
        file.read_to_end(&mut data)
            .map_err(|e| Error::Input(source.name().to_string(), e))?;
        Ok(SourceFile {
            data,
            source,
            format,
        })
    }

    /// Parse this scss source.
    pub fn parse(&self) -> Result<Parsed, Error> {
        let data = Span::new_extra(&self.data, &self.source);
        match self.format {
            SourceFormat::Scss => {
                Ok(Parsed::Scss(ParseError::check(sassfile(data))?))
            }
            SourceFormat::Css => {
                Ok(Parsed::Css(ParseError::check(css::file(data))?))
            }
        }
    }
    pub(crate) fn path(&self) -> &str {
        self.source.name()
    }
}
/// Parsed source that is either css or sass data.
#[derive(Clone, Debug)]
pub enum Parsed {
    /// Raw css data.
    Css(Vec<crate::css::Item>),
    /// Sass (scss) data.
    Scss(Vec<crate::sass::Item>),
}

pub fn code_span(value: &[u8]) -> Span {
    use lazy_static::lazy_static;
    lazy_static! {
        static ref SOURCE: SourceName = SourceName::root("(rsass)");
    }
    Span::new_extra(value, &SOURCE)
}
pub fn input_span(value: &[u8]) -> Span {
    use lazy_static::lazy_static;
    lazy_static! {
        static ref SOURCE: SourceName = SourceName::root("-");
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
    assert_eq!(Value::Numeric(Numeric::new(17, Unit::Em)), v);
    Ok(())
}

#[test]
fn test_parse_value_data_2() -> Result<(), Error> {
    let v = parse_value_data(b"17em;");
    assert!(v.is_err());
    Ok(())
}

/// Parse scss data from a buffer.
///
/// Returns a vec of the top level items of the file (or an error message).
pub fn parse_scss_data(data: &[u8]) -> Result<Vec<Item>, ParseError> {
    ParseError::check(sassfile(code_span(data)))
}

fn sassfile(input: Span) -> PResult<Vec<Item>> {
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
    let (input, tag) = alt((tag("$"), tag("/*"), tag("@"), tag("")))(input)?;
    match *tag.fragment() {
        b"$" => variable_declaration2(input),
        b"/*" => comment_item(input),
        b"@" => at_rule2(input),
        b"" => rule(input),
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
    match *tag.fragment() {
        b"$" => variable_declaration2(rest),
        b"/*" => comment_item(rest),
        b";" => Ok((rest, Item::None)),
        b"@" => at_rule2(rest),
        b"--" => {
            let result = custom_property(rest);
            if result.is_err() {
                // Note use of `input` rather than `rest` here.
                if let Ok((rest, (selectors, body))) =
                    pair(rule_start, body_block2)(input)
                {
                    return Ok((rest, Item::Rule(selectors, body)));
                }
            }
            result
        }
        b"" => {
            let (input, selectors) = opt(rule_start)(rest)?;
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
fn mixin_call2(input: Span) -> PResult<Item> {
    let (rest, n1) = terminated(name, opt_spacelike)(input)?;
    let (rest, n2) = opt(preceded(tag("."), name))(rest)?;
    let name = n2.map(|n2| format!("{}.{}", n1, n2)).unwrap_or(n1);
    let (rest, _) = opt_spacelike(rest)?;
    let (rest, args) = terminated(opt(call_args), opt_spacelike)(rest)?;
    let (end, body) = terminated(
        opt(body_block),
        terminated(opt_spacelike, opt(tag(";"))),
    )(rest)?;
    let mut pos = SourcePos::from_to(input, rest);
    // Ok, this is cheating for the test suite ...
    pos.opt_back("@include ");
    Ok((
        end,
        Item::MixinCall(name, args.unwrap_or_default(), body, pos),
    ))
}

/// What follows an `@` sign
fn at_rule2(input0: Span) -> PResult<Item> {
    let (input, name) = terminated(name, opt_spacelike)(input0)?;
    match name.as_ref() {
        "at-root" => at_root2(input),
        "charset" => charset2(input),
        "content" => content_stmt2(input),
        "debug" => map(expression_argument, Item::Debug)(input),
        "each" => each_loop2(input),
        "error" => {
            let (end, v) = value_expression(input)?;
            let (rest, _) = opt(tag(";"))(end)?;
            let mut pos = SourcePos::from_to(input, end);
            // Ok, this is cheating for the test suite ...
            pos.opt_back("@error ");
            Ok((rest, Item::Error(v, pos)))
        }
        "for" => for_loop2(input),
        "forward" => forward2(input0),
        "function" => function_declaration2(input),
        "if" => if_statement2(input),
        "import" => import2(input),
        "include" => mixin_call2(input),
        "mixin" => mixin_declaration2(input),
        "return" => return_stmt2(input),
        "use" => use2(input0),
        "warn" => map(expression_argument, Item::Warn)(input),
        "while" => while_loop2(input),
        _ => {
            let (input, name) = sass_string(input0)?;
            let (input, args) = opt(media_args)(input)?;
            let (input, body) = preceded(
                opt(ignore_space),
                alt((map(body_block, Some), value(None, semi_or_end))),
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

fn media_args(input: Span) -> PResult<Value> {
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
fn if_statement(input: Span) -> PResult<Item> {
    preceded(tag("@if "), if_statement2)(input)
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
    let pos = SourcePos::from_to(input, rest);
    Ok((end, Item::MixinDeclaration(name, args, body, pos)))
}

fn function_declaration2(input: Span) -> PResult<Item> {
    let (end, name) = terminated(name, opt_spacelike)(input)?;
    let (end, args) = formal_args(end)?;
    let (rest, body) = preceded(opt_spacelike, body_block)(end)?;
    let pos = SourcePos::from_to(input, end);
    Ok((rest, Item::FunctionDeclaration(name, args, pos, body)))
}

fn return_stmt2(input: Span) -> PResult<Item> {
    let (input, v) =
        delimited(opt_spacelike, value_expression, opt_spacelike)(input)?;
    let (input, _) = opt(tag(";"))(input)?;
    Ok((input, Item::Return(v)))
}

/// The "rest" of an `@content` statement is just an optional terminator
fn content_stmt2(input: Span) -> PResult<Item> {
    let (rest, _) = opt_spacelike(input)?;
    let (rest, _) = opt(tag(";"))(rest)?;
    let pos = SourcePos::from_to(input, rest);
    Ok((rest, Item::Content(pos)))
}

fn custom_property(input: Span) -> PResult<Item> {
    let (rest, name) = terminated(opt(sass_string), tag(":"))(input)?;
    let mut name = name.unwrap_or_else(|| SassString::from(""));
    name.prepend("--");
    let (rest, value) =
        terminated(custom_value, alt((tag(";"), peek(tag("}")))))(rest)?;
    Ok((rest, Item::CustomProperty(name, value)))
}

fn property_or_namespace_rule(input: Span) -> PResult<Item> {
    let (input, name) = terminated(
        alt((
            map(preceded(tag("*"), sass_string), |mut s| {
                s.prepend("*");
                s
            }),
            sass_string,
        )),
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

#[cfg(test)] // TODO: Or remove this?
fn variable_declaration(input: Span) -> PResult<Item> {
    preceded(tag("$"), variable_declaration2)(input)
}

fn variable_declaration2(input: Span) -> PResult<Item> {
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
        || (false, false),
        |(default, global), (d, g)| (default || d, global || g),
    )(input)?;
    let (input, _) = semi_or_end(input)?;
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
fn percentage(v: i64) -> Value {
    Value::Numeric(Numeric::new(v, Unit::Percent))
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
                Some(ListSeparator::Space),
                false,
            ),
        ),
    )
}

#[test]
fn test_variable_declaration_simple() {
    assert_eq!(
        check_parse!(variable_declaration, b"$foo: bar;"),
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
        check_parse!(variable_declaration, b"$y: some value !global;"),
        Item::VariableDeclaration {
            name: "y".into(),
            val: Value::List(
                vec![string("some"), string("value")],
                Some(ListSeparator::Space),
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
        check_parse!(variable_declaration, b"$y: some value !default;"),
        Item::VariableDeclaration {
            name: "y".into(),
            val: Value::List(
                vec![string("some"), string("value")],
                Some(ListSeparator::Space),
                false,
            ),
            default: true,
            global: false,
        },
    )
}
