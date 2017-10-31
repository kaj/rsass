pub mod util;
pub mod selectors;
pub mod formalargs;
pub mod strings;
pub mod unit;
pub mod value;

use self::formalargs::{call_args, formal_args};
use self::selectors::selectors;
use self::strings::{sass_string, sass_string_dq, sass_string_sq};
use self::value::{function_call, single_value, value_expression};
use error::Error;

use functions::SassFunction;
use nom::IResult;
#[cfg(test)]
use num_rational::Rational;
use parser::util::{comment, ignore_space, name, opt_spacelike, spacelike};

#[cfg(test)]
use sass::{CallArgs, FormalArgs};

use sass::{Item, Value};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::from_utf8;

use value::ListSeparator;
#[cfg(test)]
use value::Unit;

/// Parse a scss value.
///
/// Returns a single value (or an error).
pub fn parse_value_data(data: &[u8]) -> Result<Value, Error> {
    let expression = format!("{};", String::from_utf8(Vec::from(data))?);
    let value = value_expression(expression.as_bytes()).to_result()?;
    Ok(value)
}

/// Parse a scss file.
///
/// Returns a vec of the top level items of the file (or an error message).
pub fn parse_scss_file(file: &Path) -> Result<Vec<Item>, Error> {
    let mut f = File::open(file).map_err(|e| Error::Input(file.into(), e))?;
    let mut data = vec![];
    f.read_to_end(&mut data).map_err(|e| Error::Input(file.into(), e))?;
    parse_scss_data(&data)
}

/// Parse scss data from a buffer.
///
/// Returns a vec of the top level items of the file (or an error message).
pub fn parse_scss_data(data: &[u8]) -> Result<Vec<Item>, Error> {
    match sassfile(data) {
        IResult::Done(b"", items) => Ok(items),
        IResult::Done(rest, _styles) => {
            let t = from_utf8(rest)
                .map(|s| s.to_string())
                .unwrap_or_else(|_| format!("{:?}", rest));
            Err(Error::S(format!("Failed to parse entire input: `{}` remains.",
                                 t)))
        }
        IResult::Incomplete(x) => Err(Error::S(format!("Incomplete: {:?}", x))),
        IResult::Error(x) => Err(Error::S(format!("Error: {}", x))),
    }
}

named!(sassfile<&[u8], Vec<Item> >,
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
                   map!(comment,
                        |c| Item::Comment(from_utf8(c).unwrap().into()))
                   )));

named!(rule<Item>,
       do_parse!(opt_spacelike >>
                 selectors: selectors >>
                 opt!(is_a!(", \t\n")) >>
                 body: body_block >>
                 (Item::Rule(selectors, body))));

named!(body_item<Item>,
       alt_complete!(
           value!(Item::None, spacelike) |
           mixin_declaration |
           variable_declaration |
           rule |
           namespace_rule |
           property |
           each_loop |
           for_loop |
           while_loop |
           function_declaration |
           mixin_call |
           import |
           if_statement |
           return_stmt |
           content_stmt |
           at_rule |
           value!(Item::None,
                  delimited!(opt_spacelike, tag!(";"), opt_spacelike)) |
           map!(comment, |c| Item::Comment(from_utf8(c).unwrap().into()))
               ));

named!(import<Item>,
       map!(delimited!(tag!("@import "), value_expression, tag!(";")),
            Item::Import));

named!(mixin_call<Item>,
       do_parse!(tag!("@include") >> spacelike >>
                 name: name >> opt_spacelike >>
                 args: opt!(call_args) >> opt_spacelike >>
                 body: opt!(body_block) >> opt_spacelike >>
                 opt!(tag!(";")) >>
                 (Item::MixinCall {
                     name: name,
                     args: args.unwrap_or_default(),
                     body: body.unwrap_or_default(),
                 })));

named!(at_rule<Item>,
       do_parse!(tag!("@") >>
                 name: name >>
                 args: many0!(preceded!(ignore_space, alt!(
                     terminated!(alt!(function_call |
                                     map!(sass_string, Value::Literal) |
                                     map!(sass_string_dq, Value::Literal) |
                                     map!(sass_string_sq, Value::Literal)),
                                 peek!(one_of!(" \n\t{;"))) |
                     map!(is_not!("\"'{};"),
                          |s| Value::Literal(from_utf8(s)
                                              .unwrap()
                                              .trim_right()
                                              .into()))
                         ))) >>
                 opt!(ignore_space) >>
                 body: opt!(body_block) >>
                 opt!(tag!(";")) >>
                 (Item::AtRule {
                     name: name,
                     args: if args.len() == 1 {
                         args.into_iter().next().unwrap()
                     } else {
                         Value::List(args, ListSeparator::Space, false)
                     },
                     body: body,
                 })));

named!(if_statement<Item>,
       preceded!(tag!("@"), if_statement_inner));

named!(if_statement_inner<Item>,
       do_parse!(tag!("if") >> spacelike >>
                 cond: value_expression >> opt_spacelike >>
                 body: body_block >>
                 else_body: opt!(complete!(preceded!(
                     delimited!(opt_spacelike, tag!("@else"), opt_spacelike),
                     alt_complete!(
                         body_block |
                         map!(if_statement_inner, |s| vec![s]))))) >>
                 (Item::IfStatement(cond,
                                        body,
                                        else_body.unwrap_or_default()))));

named!(each_loop<Item>,
       do_parse!(tag!("@each") >> spacelike >> tag!("$") >>
                 name: name >> spacelike >> tag!("in") >> spacelike >>
                 values: value_expression >> opt_spacelike >>
                 body: body_block >>
                 (Item::Each(name, values, body))));

named!(for_loop<Item>,
       do_parse!(tag!("@for") >> spacelike >> tag!("$") >>
                 name: name >> spacelike >>
                 tag!("from") >> spacelike >>
                 from: single_value >> spacelike >>
                 inclusive: alt!(value!(true, tag!("through")) |
                                 value!(false, tag!("to"))) >> spacelike >>
                 to: single_value >> opt_spacelike >>
                 body: body_block >>
                 (Item::For {
                     name: name,
                     from: Box::new(from),
                     to: Box::new(to),
                     inclusive: inclusive,
                     body: body
                 })));

named!(while_loop<Item>,
       do_parse!(tag!("@while") >> spacelike >>
                 cond: value_expression >> spacelike >>
                 body: body_block >>
                 (Item::While(cond, body))));

named!(mixin_declaration<&[u8], Item>,
       do_parse!(tag!("@mixin") >> spacelike >>
                 name: name >> opt_spacelike >>
                 args: opt!(formal_args) >> opt_spacelike >>
                 body: body_block >>
                 (Item::MixinDeclaration{
                     name: name,
                     args: args.unwrap_or_default(),
                     body: body,
                 })));

named!(function_declaration<Item>,
       do_parse!(tag!("@function") >> spacelike >>
                 name: name >> opt_spacelike >>
                 args: formal_args >> opt_spacelike >>
                 body: body_block >>
                 (Item::FunctionDeclaration {
                     name: name,
                     func: SassFunction::new(args, body),
                 })));

named!(return_stmt<Item>,
       do_parse!(tag!("@return") >> spacelike >>
                 v: value_expression >> opt_spacelike >>
                 opt!(tag!(";")) >>
                 (Item::Return(v))));

named!(content_stmt<Item>,
       do_parse!(tag!("@content") >> opt_spacelike >>
                 opt!(tag!(";")) >>
                 (Item::Content)));

named!(property<&[u8], Item>,
       do_parse!(opt_spacelike >>
                 name: name >> opt_spacelike >>
                 tag!(":") >> opt_spacelike >>
                 val: value_expression >>
                 imp: opt_important >> opt_spacelike >>
                 opt!(tag!(";")) >> opt_spacelike >>
                 (Item::Property(name, val, imp))));

named!(namespace_rule<Item>,
       do_parse!(opt_spacelike >>
                 n1: name >> opt_spacelike >>
                 tag!(":") >> opt_spacelike >>
                 value: opt!(value_expression) >> opt_spacelike >>
                 body: body_block >>
                 (Item::NamespaceRule(n1,
                                          value.unwrap_or(Value::Null),
                                          body))));

named!(opt_important<bool>,
       map!(opt!(do_parse!(opt_spacelike >> tag!("!") >>
                           opt_spacelike >> tag!("important") >>
                           ())),
            |o: Option<()>| o.is_some()));

named!(body_block<Vec<Item>>,
       delimited!(preceded!(tag!("{"), opt_spacelike),
                  many0!(body_item),
                  tag!("}")));

named!(variable_declaration<Item>,
       do_parse!(tag!("$") >>
                 name: name >> opt_spacelike >>
                 tag!(":") >> opt_spacelike >>
                 val: value_expression >> opt_spacelike >>
                 default: opt!(tag!("!default")) >> opt_spacelike >>
                 global: opt!(tag!("!global")) >> opt_spacelike >>
                 tag!(";") >> opt_spacelike >>
                 (Item::VariableDeclaration {
                     name: name,
                     val: val,
                     default: default.is_some(),
                     global: global.is_some(),
                 })));

#[cfg(test)]
fn percentage(v: isize) -> Value {
    Value::Numeric(Rational::from_integer(v), Unit::Percent, false, false)
}

#[cfg(test)]
fn string(v: &str) -> Value {
    Value::Literal(v.into())
}

#[test]
fn if_with_no_else() {
    assert_eq!(if_statement(b"@if true { p { color: black; } }\n"),
               IResult::Done(&b"\n"[..],
                    Item::IfStatement(
                        Value::True,
                        vec![Item::Rule(
                            selectors(b"p").unwrap().1,
                            vec![Item::Property("color".into(),
                                                    Value::black(),
                                                    false)]),
                             Item::None],
                        vec![])))
}

#[test]
fn test_mixin_call_noargs() {
    assert_eq!(mixin_call(b"@include foo;\n"),
               IResult::Done(&b"\n"[..],
                    Item::MixinCall {
                        name: "foo".to_string(),
                        args: CallArgs::new(vec![]),
                        body: vec![],
                    }))
}

#[test]
fn test_mixin_call_pos_args() {
    assert_eq!(mixin_call(b"@include foo(bar, baz);\n"),
               IResult::Done(&b"\n"[..],
                    Item::MixinCall {
                        name: "foo".to_string(),
                        args: CallArgs::new(
                            vec![(None, string("bar")),
                                 (None, string("baz"))]),
                        body: vec![],
                    }))
}

#[test]
fn test_mixin_call_named_args() {
    assert_eq!(mixin_call(b"@include foo($x: bar, $y: baz);\n"),
               IResult::Done(&b"\n"[..],
                    Item::MixinCall {
                        name: "foo".to_string(),
                        args: CallArgs::new(
                            vec![(Some("x".into()), string("bar")),
                                 (Some("y".into()), string("baz"))]),
                        body: vec![],
                    }))
}

#[test]
fn test_mixin_declaration_empty() {
    assert_eq!(mixin_declaration(b"@mixin foo() {}\n"),
               IResult::Done(&b"\n"[..], Item::MixinDeclaration {
                   name: "foo".into(),
                   args: FormalArgs::default(),
                   body: vec![],
               }))
}

#[test]
fn test_mixin_declaration() {
    assert_eq!(mixin_declaration(b"@mixin foo($x) {\n  \
                                   foo-bar: baz $x;\n\
                                   }\n"),
               IResult::Done(&b"\n"[..], Item::MixinDeclaration {
                   name: "foo".into(),
                   args: FormalArgs::new(vec![("x".into(), Value::Null)],
                                         false),
                   body: vec![Item::Property(
                       "foo-bar".into(),
                       Value::List(
                           vec![string("baz"),
                                Value::Variable("x".into())],
                           ListSeparator::Space,
                           false),
                       false)],
               }))
}

#[test]
fn test_mixin_declaration_default_and_subrules() {
    assert_eq!(mixin_declaration(b"@mixin bar($a, $b: flug) {\n  \
                                   foo-bar: baz;\n  \
                                   foo, bar {\n    \
                                   property: $b;\n  \
                                   }\n\
                                   }\n"),
               IResult::Done(&b"\n"[..], Item::MixinDeclaration {
                   name: "bar".into(),
                   args: FormalArgs::new(
                       vec![("a".into(), Value::Null),
                            ("b".into(), string("flug"))],
                       false),
                   body: vec![
                       Item::Property("foo-bar".into(),
                                          string("baz"),
                                          false),
                       Item::Rule(
                           selectors(b"foo, bar").unwrap().1,
                           vec![Item::Property(
                                    "property".into(),
                                    Value::Variable("b".into()),
                                    false)]),
                       Item::None,
                       ]}))
}

#[test]
fn test_simple_property() {
    let one = Rational::from_integer(1);
    fn r(v: u8) -> Rational {
        Rational::from_integer(v as isize)
    }
    assert_eq!(property(b"color: red;\n"),
               IResult::Done(&b""[..], Item::Property(
                   "color".to_string(),
                   Value::Color(r(255), r(0), r(0), one, Some("red".into())),
                   false)))
}
#[test]
fn test_property_2() {
    assert_eq!(property(b"background-position: 90% 50%;\n"),
               IResult::Done(&b""[..], Item::Property(
                   "background-position".to_string(),
                   Value::List(vec![percentage(90), percentage(50)],
                               ListSeparator::Space,
                               false),
                   false)))
}

#[test]
fn test_variable_declaration_simple() {
    assert_eq!(variable_declaration(b"$foo: bar;\n"),
               IResult::Done(&b""[..],
                    Item::VariableDeclaration {
                        name: "foo".into(),
                        val: string("bar"),
                        default: false,
                        global: false,
                    }))
}

#[test]
fn test_variable_declaration_global() {
    assert_eq!(variable_declaration(b"$y: some value !global;\n"),
               IResult::Done(&b""[..],
                    Item::VariableDeclaration {
                        name: "y".into(),
                        val: Value::List(
                            vec![string("some"), string("value")],
                            ListSeparator::Space,
                            false),
                        default: false,
                        global: true,
                    }))
}

#[test]
fn test_variable_declaration_default() {
    assert_eq!(variable_declaration(b"$y: some value !default;\n"),
               IResult::Done(&b""[..],
                    Item::VariableDeclaration {
                        name: "y".into(),
                        val: Value::List(
                            vec![string("some"), string("value")],
                            ListSeparator::Space,
                            false),
                        default: true,
                        global: false,
                    }))
}
