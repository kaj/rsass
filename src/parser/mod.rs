pub mod formalargs;
pub mod selectors;
mod strings;
mod unit;
mod util;
pub mod value;

use self::formalargs::{call_args, formal_args};
use self::selectors::selectors;
use self::strings::{name, sass_string, sass_string_dq, sass_string_sq};
use self::util::{comment2, ignore_space, opt_spacelike, spacelike};
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
use nom::types::CompleteByteSlice as Input;
use nom::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::{from_utf8, Utf8Error};

/// Parse a scss value.
///
/// Returns a single value (or an error).
pub fn parse_value_data(data: &[u8]) -> Result<Value, Error> {
    let (rest, result) = value_expression(Input(data))?;
    assert!(rest.is_empty());
    Ok(result)
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
    match sassfile(Input(data)) {
        Ok((Input(b""), items)) => Ok(items),
        Ok((rest, _styles)) => Err((data.len() - rest.len(), None)),
        Err(Err::Incomplete(_needed)) => Err((data.len(), None)),
        Err(Err::Error(Context::Code(rest, ek))) => {
            Err((data.len() - rest.len(), Some(ek)))
        }
        Err(Err::Error(Context::List(list))) => {
            if let Some((rest, ek)) = list.first() {
                Err((data.len() - rest.len(), Some(ek.clone())))
            } else {
                Err((0, None))
            }
        }
        Err(Err::Failure(Context::Code(rest, ek))) => {
            Err((data.len() - rest.len(), Some(ek)))
        }
        Err(Err::Failure(Context::List(list))) => {
            if let Some((rest, ek)) = list.first() {
                Err((data.len() - rest.len(), Some(ek.clone())))
            } else {
                Err((0, None))
            }
        }
    }
}

named!(
    sassfile<Input, Vec<Item>>,
    preceded!(
        opt!(tag!("\u{feff}".as_bytes())),
        map!(
            many_till!(
                preceded!(opt_spacelike, top_level_item),
                preceded!(opt_spacelike, eof!())
            ),
            |(v, _eof)| v
        )
    )
);

named!(
    top_level_item<Input, Item>,
    switch!(
        alt!(
            tag!("$") |
            tag!("/*") |
            tag!("@each") |
            tag!("@for") |
            tag!("@function") |
            tag!("@if") |
            tag!("@import") |
            tag!("@include") |
            tag!("@mixin") |
            tag!("@while") |
            tag!("@") |
            tag!("")
        ),
        Input(b"$") => call!(variable_declaration2) |
        Input(b"/*") => map!(
            map_res!(comment2, input_to_string),
            Item::Comment
        ) |
        Input(b"@each") => call!(each_loop2) |
        Input(b"@for") => call!(for_loop2) |
        Input(b"@function") => call!(function_declaration2) |
        Input(b"@if") => call!(if_statement2) |
        Input(b"@import") => call!(import2) |
        Input(b"@include") => call!(mixin_call2) |
        Input(b"@mixin") => call!(mixin_declaration2) |
        Input(b"@while") => call!(while_loop2) |
        Input(b"@") => call!(at_rule2) |
        Input(b"") => call!(rule)
    )
);

named!(
    rule<Input, Item>,
    map!(
        pair!(
            rule_start,
            body_block2
        ),
        |(selectors, body)| Item::Rule(selectors, body)
    )
);

named!(
    rule_start<Input, Selectors>,
    terminated!(
        selectors,
        terminated!(opt!(is_a!(", \t\n")), tag!("{"))
    )
);

named!(
    body_item<Input, Item>,
    switch!(
        alt!(
            tag!("$") |
            tag!("/*") |
            tag!(";") |
            tag!("@at-root") |
            tag!("@content") |
            tag!("@each") |
            tag!("@for") |
            tag!("@function") |
            tag!("@if") |
            tag!("@import") |
            tag!("@include") |
            tag!("@mixin") |
            tag!("@return") |
            tag!("@while") |
            tag!("@") |
            tag!("")
        ),
        Input(b"$") => call!(variable_declaration2) |
        Input(b"/*") => map!(map_res!(comment2, input_to_string),
                             Item::Comment) |
        Input(b";") => value!(Item::None) |
        Input(b"@at-root") => call!(at_root2) |
        Input(b"@content") => call!(content_stmt2) |
        Input(b"@each") => call!(each_loop2) |
        Input(b"@for") => call!(for_loop2) |
        Input(b"@function") => call!(function_declaration2) |
        Input(b"@if") => call!(if_statement2) |
        Input(b"@import") => call!(import2) |
        Input(b"@include") => call!(mixin_call2) |
        Input(b"@mixin") => call!(mixin_declaration2) |
        Input(b"@return") => call!(return_stmt2) |
        Input(b"@while") => call!(while_loop2) |
        Input(b"@") => call!(at_rule2) |
        Input(b"") => switch!(
            opt!(rule_start),
            Some(selectors) => map!(
                body_block2,
                |body| Item::Rule(selectors, body)
            ) |
            None => call!(property_or_namespace_rule)
        )
    )
);

named!(
    import<Input, Item>,
    preceded!(tag!("@import"), import2));

named!(
    import2<Input, Item>,
    map!(
        delimited!(tag!(" "), value_expression, tag!(";")),
        Item::Import
    )
);

named!(
    at_root<Input, Item>,
    preceded!(tag!("@at-root"), at_root2));

named!(
    at_root2<Input, Item>,
    preceded!(
        opt_spacelike,
        map!(
            pair!(
                map!(opt!(selectors), |s| s
                    .unwrap_or_else(Selectors::root)),
                body_block
            ),
            |(selectors, body)| Item::AtRoot { selectors, body }
        )
    )
);

named!(
    mixin_call<Input, Item>,
    preceded!(tag!("@include"), mixin_call2));

named!(
    mixin_call2<Input, Item>,
    do_parse!(
        spacelike
            >> name: name
            >> opt_spacelike
            >> args: opt!(call_args)
            >> opt_spacelike
            >> body: opt!(body_block)
            >> opt_spacelike
            >> opt!(complete!(tag!(";")))
            >> (Item::MixinCall {
                name,
                args: args.unwrap_or_default(),
                body: body.unwrap_or_default(),
            })
    )
);

named!(
    at_rule<Input, Item>,
    preceded!(tag!("@"), at_rule2));

named!(
    at_rule2<Input, Item>,
    do_parse!(
        name: name
            >> args: many0!(preceded!(
                opt!(ignore_space),
                alt!(
                    terminated!(
                        alt!(
                            function_call
                                | dictionary
                                | map!(sass_string, Value::Literal)
                                | map!(sass_string_dq, Value::Literal)
                                | map!(sass_string_sq, Value::Literal)
                        ),
                        peek!(one_of!(" \n\t{;"))
                    ) | map!(
                        map_res!(is_not!("\"'{};"), input_to_str),
                        |s| Value::Literal(s.trim_end().into(),
                    ))
                )
            ))
            >> opt!(ignore_space)
            >> body: alt!(
                map!(body_block, Some) |
                value!(None, eof!()) |
                value!(None, tag!(";"))
            )
            >> (Item::AtRule {
                name,
                args: if args.len() == 1 {
                    args.into_iter().next().unwrap()
                } else {
                    Value::List(args, ListSeparator::Space, false, false)
                },
                body,
            })
    )
);

named!(
    if_statement<Input, Item>,
    preceded!(tag!("@if"), if_statement2));

named!(
    if_statement_inner<Input, Item>,
    preceded!(tag!("if"), if_statement2));

named!(
    if_statement2<Input, Item>,
    do_parse!(
        spacelike
            >> cond: value_expression
            >> opt_spacelike
            >> body: body_block
            >> else_body:
                opt!(complete!(preceded!(
                    delimited!(opt_spacelike, tag!("@else"), opt_spacelike),
                    alt_complete!(
                        body_block | map!(if_statement_inner, |s| vec![s])
                    )
                )))
            >> (Item::IfStatement(cond, body, else_body.unwrap_or_default()))
    )
);

named!(
    each_loop<Input, Item>,
    preceded!(tag!("@each"), each_loop2));

named!(
    each_loop2<Input, Item>,
    map!(
        tuple!(
            preceded!(
                spacelike,
                separated_nonempty_list!(
                    complete!(delimited!(
                        opt_spacelike,
                        tag!(","),
                        opt_spacelike
                    )),
                    preceded!(tag!("$"), name)
                )
            ),
            delimited!(
                delimited!(spacelike, tag!("in"), spacelike),
                value_expression,
                spacelike
            ),
            body_block
        ),
        |(names, values, body)| Item::Each(names, values, body)
    )
);

named!(
    for_loop<Input, Item>,
    preceded!(tag!("@for"), for_loop2));

named!(
    for_loop2<Input, Item>,
    do_parse!(
        spacelike
            >> tag!("$")
            >> name: name
            >> spacelike
            >> tag!("from")
            >> spacelike
            >> from: single_value
            >> spacelike
            >> inclusive:
                alt!(
                    value!(true, tag!("through")) | value!(false, tag!("to"))
                )
            >> spacelike
            >> to: single_value
            >> opt_spacelike
            >> body: body_block
            >> (Item::For {
                name,
                from: Box::new(from),
                to: Box::new(to),
                inclusive,
                body,
            })
    )
);

named!(
    while_loop<Input, Item>,
    preceded!(tag!("@while"), while_loop2));

named!(
    while_loop2<Input, Item>,
    do_parse!(
        spacelike
            >> cond: value_expression
            >> spacelike
            >> body: body_block
            >> (Item::While(cond, body))
    )
);

named!(mixin_declaration<Input, Item>,
       preceded!(tag!("@mixin"), mixin_declaration2));

named!(mixin_declaration2<Input, Item>,
       do_parse!(spacelike >>
                 name: name >> opt_spacelike >>
                 args: opt!(formal_args) >> opt_spacelike >>
                 body: body_block >>
                 (Item::MixinDeclaration{
                     name,
                     args: args.unwrap_or_default(),
                     body,
                 })));

named!(
    function_declaration2<Input, Item>,
    do_parse!(
        spacelike
            >> name: name
            >> opt_spacelike
            >> args: formal_args
            >> opt_spacelike
            >> body: body_block
            >> (Item::FunctionDeclaration {
                name,
                func: SassFunction::new(args, body),
            })
    )
);

named!(
    return_stmt2<Input, Item>,
    do_parse!(
        spacelike
            >> v: value_expression
            >> opt_spacelike
            >> opt!(tag!(";"))
            >> (Item::Return(v))
    )
);

named!(
    content_stmt2<Input, Item>,
    do_parse!(
        opt_spacelike
            >> opt!(tag!(";"))
            >> (Item::Content)
    )
);

named!(
    property_or_namespace_rule<Input, Item>,
    do_parse!(
        name: terminated!(sass_string,
                          delimited!(opt_spacelike, tag!(":"), opt_spacelike)) >>
        val: opt!(terminated!(value_expression, opt_spacelike)) >>
        body: terminated!(
            switch!(
                alt!(tag!("{") | cond_reduce!(val.is_some(), alt!(tag!(";") | tag!("")))),
                Input(b"{") => map!(body_block2, Some) |
                //Input(b";") => value!(None) |
                //Input(b"") => value!(None) |
                _ => value!(None)
                //None => return_error!(call!())
            ),
            opt_spacelike
        ) >> (ns_or_prop_item(name, val, body))
    )
);

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

named!(
    body_block<Input, Vec<Item>>,
    preceded!(tag!("{"), body_block2));

named!(
    body_block2<Input, Vec<Item>>,
    preceded!(
        opt_spacelike,
        map!(
            many_till!(
                terminated!(body_item, opt_spacelike),
                terminated!(tag!("}"), opt!(tag!(";")))
            ),
            |(v, _end)| v
        )
    )
);

named!(
    variable_declaration<Input, Item>,
    preceded!(tag!("$"), variable_declaration2)
);

named!(
    variable_declaration2<Input, Item>,
    do_parse!(
        name: name
            >> opt_spacelike
            >> tag!(":")
            >> opt_spacelike
            >> val: value_expression
            >> opt_spacelike
            >> default: map!(opt!(tag!("!default")), |d| d.is_some())
            >> opt_spacelike
            >> global: map!(opt!(tag!("!global")), |g| g.is_some())
            >> opt_spacelike
            >> tag!(";")
            >> opt_spacelike
            >> (Item::VariableDeclaration {
                name,
                val,
                default,
                global,
            })
    )
);

fn input_to_str<'a>(s: Input<'a>) -> Result<&str, Utf8Error> {
    from_utf8(&s)
}

fn input_to_string(s: Input) -> Result<String, Utf8Error> {
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
        if_statement(Input(b"@if true { p { color: black; } }\n")),
        Ok((
            Input(b"\n"),
            Item::IfStatement(
                Value::True,
                vec![Item::Rule(
                    selectors(Input(b"p")).unwrap().1,
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
        mixin_call(Input(b"@include foo;\n")),
        Ok((
            Input(b"\n"),
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
        mixin_call(Input(b"@include foo(bar, baz);\n")),
        Ok((
            Input(b"\n"),
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
        mixin_call(Input(b"@include foo($x: bar, $y: baz);\n")),
        Ok((
            Input(b"\n"),
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
        mixin_declaration(Input(b"@mixin foo() {}\n")),
        Ok((
            Input(b"\n"),
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
        mixin_declaration(Input(
            b"@mixin foo($x) {\n  foo-bar: baz $x;\n}\n"
        )),
        Ok((
            Input(b"\n"),
            Item::MixinDeclaration {
                name: "foo".into(),
                args: FormalArgs::new(vec![("x".into(), Value::Null)], false),
                body: vec![Item::Property(
                    "foo-bar".into(),
                    Value::List(
                        vec![string("baz"), Value::Variable("x".into())],
                        ListSeparator::Space,
                        false,
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
        mixin_declaration(Input(
            b"@mixin bar($a, $b: flug) {\n  \
                                   foo-bar: baz;\n  \
                                   foo, bar {\n    \
                                   property: $b;\n  \
                                   }\n\
                                   }\n"
        )),
        Ok((
            Input(b"\n"),
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
                        selectors(Input(b"foo, bar")).unwrap().1,
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
        property_or_namespace_rule(Input(b"color: red;\n")),
        Ok((
            Input(b""),
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
        property_or_namespace_rule(Input(b"background-position: 90% 50%;\n")),
        Ok((
            Input(b""),
            Item::Property(
                "background-position".into(),
                Value::List(
                    vec![percentage(90), percentage(50)],
                    ListSeparator::Space,
                    false,
                    false
                ),
            )
        ))
    )
}

#[test]
fn test_variable_declaration_simple() {
    assert_eq!(
        variable_declaration(Input(b"$foo: bar;\n")),
        Ok((
            Input(b""),
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
        variable_declaration(Input(b"$y: some value !global;\n")),
        Ok((
            Input(b""),
            Item::VariableDeclaration {
                name: "y".into(),
                val: Value::List(
                    vec![string("some"), string("value")],
                    ListSeparator::Space,
                    false,
                    false
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
        variable_declaration(Input(b"$y: some value !default;\n")),
        Ok((
            Input(b""),
            Item::VariableDeclaration {
                name: "y".into(),
                val: Value::List(
                    vec![string("some"), string("value")],
                    ListSeparator::Space,
                    false,
                    false
                ),
                default: true,
                global: false,
            }
        ))
    )
}
