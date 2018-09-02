pub mod formalargs;
pub mod selectors;
mod strings;
mod unit;
mod util;
pub mod value;

use self::formalargs::{call_args, formal_args};
use self::selectors::selectors;
use self::strings::{
    input_to_str, input_to_string, sass_string, sass_string_dq,
    sass_string_sq,
};
use self::value::{
    dictionary, function_call, single_value, value_expression,
};
use error::Error;
use functions::SassFunction;
use nom::types::CompleteByteSlice as Input;
use nom::Err;
use parser::util::{comment, ignore_space, name, opt_spacelike, spacelike};
#[cfg(test)]
use sass::{CallArgs, FormalArgs};
use sass::{Item, Value};
use selectors::Selectors;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::from_utf8;
use value::ListSeparator;
#[cfg(test)]
use value::{Number, Rgba, Unit};

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
    parse_scss_data(&data)
}

/// Parse scss data from a buffer.
///
/// Returns a vec of the top level items of the file (or an error message).
pub fn parse_scss_data(data: &[u8]) -> Result<Vec<Item>, Error> {
    match sassfile(Input(data)) {
        Ok((Input(b""), items)) => Ok(items),
        Ok((rest, _styles)) => {
            let t = from_utf8(&rest)
                .map(|s| s.to_string())
                .unwrap_or_else(|_| format!("{:?}", rest));
            Err(Error::S(format!(
                "Failed to parse entire input: `{}` remains.",
                t
            )))
        }
        Err(Err::Incomplete(x)) => {
            Err(Error::S(format!("Incomplete: {:?}", x)))
        }
        Err(x) => Err(Error::S(format!("Error: {:?}", x))),
    }
}

named!(sassfile<Input, Vec<Item>>,
       many0!(alt!(value!(Item::None, spacelike) |
                   import |
                   variable_declaration |
                   mixin_declaration |
                   each_loop |
                   for_loop |
                   while_loop |
                   function_declaration |
                   mixin_call |
                   if_statement |
                   at_rule |
                   rule |
                   map!(map_res!(comment, input_to_string), Item::Comment)
                   )));

named!(
    rule<Input, Item>,
    do_parse!(
        opt_spacelike
            >> selectors: selectors
            >> opt!(is_a!(", \t\n"))
            >> body: body_block
            >> (Item::Rule(selectors, body))
    )
);

named!(
    body_item<Input, Item>,
    alt_complete!(
        value!(Item::None, spacelike)
            | mixin_declaration
            | variable_declaration
            | rule
            | namespace_rule
            | property
            | each_loop
            | for_loop
            | while_loop
            | function_declaration
            | mixin_call
            | import
            | at_root
            | if_statement
            | return_stmt
            | content_stmt
            | at_rule
            | value!(
                Item::None,
                delimited!(opt_spacelike, tag!(";"), opt_spacelike)
            )
            | map!(map_res!(comment, input_to_string), Item::Comment)
    )
);

named!(
    import<Input, Item>,
    map!(
        delimited!(tag!("@import "), value_expression, tag!(";")),
        Item::Import
    )
);

named!(
    at_root<Input, Item>,
    preceded!(
        terminated!(tag!("@at-root"), opt_spacelike),
        map!(
            pair!(
                map!(opt!(selectors), |s| s
                    .unwrap_or_else(|| Selectors::root())),
                body_block
            ),
            |(selectors, body)| Item::AtRoot { selectors, body }
        )
    )
);

named!(
    mixin_call<Input, Item>,
    do_parse!(
        tag!("@include")
            >> spacelike
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
    do_parse!(
        tag!("@")
            >> name: name
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
                        |s| Value::Literal(s.trim_right().into(),
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

named!(if_statement<Input, Item>, preceded!(tag!("@"), if_statement_inner));

named!(
    if_statement_inner<Input, Item>,
    do_parse!(
        tag!("if")
            >> spacelike
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
    map!(
        tuple!(
            preceded!(
                terminated!(tag!("@each"), spacelike),
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
    do_parse!(
        tag!("@for")
            >> spacelike
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
    do_parse!(
        tag!("@while")
            >> spacelike
            >> cond: value_expression
            >> spacelike
            >> body: body_block
            >> (Item::While(cond, body))
    )
);

named!(mixin_declaration<Input, Item>,
       do_parse!(tag!("@mixin") >> spacelike >>
                 name: name >> opt_spacelike >>
                 args: opt!(formal_args) >> opt_spacelike >>
                 body: body_block >>
                 (Item::MixinDeclaration{
                     name,
                     args: args.unwrap_or_default(),
                     body,
                 })));

named!(
    function_declaration<Input, Item>,
    do_parse!(
        tag!("@function")
            >> spacelike
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
    return_stmt<Input, Item>,
    do_parse!(
        tag!("@return")
            >> spacelike
            >> v: value_expression
            >> opt_spacelike
            >> opt!(tag!(";"))
            >> (Item::Return(v))
    )
);

named!(
    content_stmt<Input, Item>,
    do_parse!(
        tag!("@content")
            >> opt_spacelike
            >> opt!(tag!(";"))
            >> (Item::Content)
    )
);

named!(property<Input, Item>,
       do_parse!(opt_spacelike >>
                 name: sass_string >> opt_spacelike >>
                 tag!(":") >> opt_spacelike >>
                 val: value_expression >> opt_spacelike >>
                 opt!(tag!(";")) >> opt_spacelike >>
                 (Item::Property(name, val))));

named!(
    namespace_rule<Input, Item>,
    do_parse!(
        opt_spacelike
            >> n1: name
            >> opt_spacelike
            >> tag!(":")
            >> opt_spacelike
            >> value: opt!(value_expression)
            >> opt_spacelike
            >> body: body_block
            >> (Item::NamespaceRule(n1, value.unwrap_or(Value::Null), body))
    )
);

named!(
    body_block<Input, Vec<Item>>,
    delimited!(
        preceded!(tag!("{"), opt_spacelike),
        many0!(body_item),
        tag!("}")
    )
);

named!(
    variable_declaration<Input, Item>,
    do_parse!(
        tag!("$")
            >> name: name
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

#[cfg(test)]
fn percentage(v: isize) -> Value {
    Value::Numeric(Number::from_integer(v), Unit::Percent)
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
                vec![
                    Item::Rule(
                        selectors(Input(b"p")).unwrap().1,
                        vec![Item::Property("color".into(), Value::black())],
                    ),
                    Item::None,
                ],
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
                    Item::None,
                ],
            }
        ))
    )
}

#[test]
fn test_simple_property() {
    assert_eq!(
        property(Input(b"color: red;\n")),
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
        property(Input(b"background-position: 90% 50%;\n")),
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
