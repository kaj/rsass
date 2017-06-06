pub mod util;
pub mod selectors;
pub mod formalargs;
pub mod unit;

use self::formalargs::{call_args, formal_args};
use self::selectors::selectors;

use SassItem;
use error::Error;

#[cfg(test)]
use formalargs::{CallArgs, FormalArgs};

use functions::SassFunction;
use nom::IResult;
#[cfg(test)]
use num_rational::Rational;
use parser::util::{comment, ignore_space, name, opt_spacelike, spacelike};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::from_utf8;
#[cfg(test)]
use unit::Unit;

use value::{ListSeparator, Quotes, Value, function_call, interpolation,
            quoted_string, singlequoted_string};
use value::{single_value, value_expression};

/// Parse a scss file.
///
/// Returns a vec of the top level items of the file (or an error message).
pub fn parse_scss_file(file: &Path) -> Result<Vec<SassItem>, Error> {
    let mut f = File::open(file).map_err(|e| Error::Input(file.into(), e))?;
    let mut data = vec![];
    f.read_to_end(&mut data).map_err(|e| Error::Input(file.into(), e))?;
    parse_scss_data(&data)
}

/// Parse scss data from a buffer.
///
/// Returns a vec of the top level items of the file (or an error message).
pub fn parse_scss_data(data: &[u8]) -> Result<Vec<SassItem>, Error> {
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

named!(sassfile<&[u8], Vec<SassItem> >,
       many0!(alt!(value!(SassItem::None, spacelike) |
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
                        |c| SassItem::Comment(from_utf8(c).unwrap().into()))
                   )));

named!(rule<SassItem>,
       do_parse!(opt_spacelike >>
                 selectors: selectors >>
                 opt!(is_a!(", \t\n")) >>
                 body: body_block >>
                 (SassItem::Rule(selectors, body))));

named!(body_item<SassItem>,
       alt_complete!(
           value!(SassItem::None, spacelike) |
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
           value!(SassItem::None,
                  delimited!(opt_spacelike, tag!(";"), opt_spacelike)) |
           map!(comment, |c| SassItem::Comment(from_utf8(c).unwrap().into()))
               ));

named!(import<SassItem>,
       map!(delimited!(tag!("@import "), value_expression, tag!(";")),
            |v| SassItem::Import(v)));

named!(mixin_call<SassItem>,
       do_parse!(tag!("@include") >> spacelike >>
                 name: name >> opt_spacelike >>
                 args: opt!(call_args) >> opt_spacelike >>
                 body: opt!(body_block) >> opt_spacelike >>
                 opt!(tag!(";")) >>
                 (SassItem::MixinCall {
                     name: name,
                     args: args.unwrap_or_default(),
                     body: body.unwrap_or_default(),
                 })));

named!(at_rule<SassItem>,
       do_parse!(tag!("@") >>
                 name: name >> ignore_space >>
                 args: many0!(preceded!(opt!(ignore_space), alt!(
                     function_call |
                     quoted_string |
                     singlequoted_string |
                     interpolation |
                     map!(is_not!("\"'{};"),
                          |s| Value::Literal(from_utf8(s)
                                             .unwrap()
                                             .trim_right()
                                             .into(),
                                             Quotes::None))
                         ))) >>
                 body: opt!(body_block) >>
                 opt!(tag!(";")) >>
                 (SassItem::AtRule {
                     name: name,
                     args: if args.len() == 1 {
                         args.into_iter().next().unwrap()
                     } else {
                         Value::List(args, ListSeparator::Space)
                     },
                     body: body,
                 })));

named!(if_statement<SassItem>,
       preceded!(tag!("@"), if_statement_inner));

named!(if_statement_inner<SassItem>,
       do_parse!(tag!("if") >> spacelike >>
                 cond: value_expression >> opt_spacelike >>
                 body: body_block >>
                 else_body: opt!(complete!(preceded!(
                     delimited!(opt_spacelike, tag!("@else"), opt_spacelike),
                     alt_complete!(
                         body_block |
                         map!(if_statement_inner, |s| vec![s]))))) >>
                 (SassItem::IfStatement(cond,
                                        body,
                                        else_body.unwrap_or_default()))));

named!(each_loop<SassItem>,
       do_parse!(tag!("@each") >> spacelike >> tag!("$") >>
                 name: name >> spacelike >> tag!("in") >> spacelike >>
                 values: value_expression >> opt_spacelike >>
                 body: body_block >>
                 (SassItem::Each(name, values, body))));

named!(for_loop<SassItem>,
       do_parse!(tag!("@for") >> spacelike >> tag!("$") >>
                 name: name >> spacelike >>
                 tag!("from") >> spacelike >>
                 from: single_value >> spacelike >>
                 inclusive: alt!(value!(true, tag!("through")) |
                                 value!(false, tag!("to"))) >> spacelike >>
                 to: single_value >> opt_spacelike >>
                 body: body_block >>
                 (SassItem::For {
                     name: name,
                     from: Box::new(from),
                     to: Box::new(to),
                     inclusive: inclusive,
                     body: body
                 })));

named!(while_loop<SassItem>,
       do_parse!(tag!("@while") >> spacelike >>
                 cond: value_expression >> spacelike >>
                 body: body_block >>
                 (SassItem::While(cond, body))));

named!(mixin_declaration<&[u8], SassItem>,
       do_parse!(tag!("@mixin") >> spacelike >>
                 name: name >> opt_spacelike >>
                 args: opt!(formal_args) >> opt_spacelike >>
                 body: body_block >>
                 (SassItem::MixinDeclaration{
                     name: name,
                     args: args.unwrap_or_default(),
                     body: body,
                 })));

named!(function_declaration<SassItem>,
       do_parse!(tag!("@function") >> spacelike >>
                 name: name >> opt_spacelike >>
                 args: formal_args >> opt_spacelike >>
                 body: body_block >>
                 (SassItem::FunctionDeclaration {
                     name: name,
                     func: SassFunction::new(args, body),
                 })));

named!(return_stmt<SassItem>,
       do_parse!(tag!("@return") >> spacelike >>
                 v: value_expression >> opt_spacelike >>
                 opt!(tag!(";")) >>
                 (SassItem::Return(v))));

named!(content_stmt<SassItem>,
       do_parse!(tag!("@content") >> opt_spacelike >>
                 opt!(tag!(";")) >>
                 (SassItem::Content)));

named!(property<&[u8], SassItem>,
       do_parse!(opt_spacelike >>
                 name: name >> opt_spacelike >>
                 tag!(":") >> opt_spacelike >>
                 val: value_expression >>
                 imp: opt_important >> opt_spacelike >>
                 opt!(tag!(";")) >> opt_spacelike >>
                 (SassItem::Property(name, val, imp))));

named!(namespace_rule<SassItem>,
       do_parse!(opt_spacelike >>
                 n1: name >> opt_spacelike >>
                 tag!(":") >> opt_spacelike >>
                 value: opt!(value_expression) >> opt_spacelike >>
                 body: body_block >>
                 (SassItem::NamespaceRule(n1,
                                          value.unwrap_or(Value::Null),
                                          body))));

named!(opt_important<bool>,
       map!(opt!(do_parse!(opt_spacelike >> tag!("!") >>
                           opt_spacelike >> tag!("important") >>
                           ())),
            |o: Option<()>| o.is_some()));

named!(body_block<Vec<SassItem>>,
       delimited!(preceded!(tag!("{"), opt_spacelike),
                  many0!(body_item),
                  tag!("}")));

named!(variable_declaration<SassItem>,
       do_parse!(tag!("$") >>
                 name: name >> opt_spacelike >>
                 tag!(":") >> opt_spacelike >>
                 val: value_expression >> opt_spacelike >>
                 default: opt!(tag!("!default")) >> opt_spacelike >>
                 global: opt!(tag!("!global")) >> opt_spacelike >>
                 tag!(";") >> opt_spacelike >>
                 (SassItem::VariableDeclaration {
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
    Value::Literal(v.into(), Quotes::None)
}

#[test]
fn if_with_no_else() {
    assert_eq!(if_statement(b"@if true { p { color: black; } }\n"),
               IResult::Done(&b"\n"[..],
                    SassItem::IfStatement(
                        Value::True,
                        vec![SassItem::Rule(
                            selectors(b"p").unwrap().1,
                            vec![SassItem::Property("color".into(),
                                                    Value::black(),
                                                    false)]),
                             SassItem::None],
                        vec![])))
}

#[test]
fn test_mixin_call_noargs() {
    assert_eq!(mixin_call(b"@include foo;\n"),
               IResult::Done(&b"\n"[..],
                    SassItem::MixinCall {
                        name: "foo".to_string(),
                        args: CallArgs::new(vec![]),
                        body: vec![],
                    }))
}

#[test]
fn test_mixin_call_pos_args() {
    assert_eq!(mixin_call(b"@include foo(bar, baz);\n"),
               IResult::Done(&b"\n"[..],
                    SassItem::MixinCall {
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
                    SassItem::MixinCall {
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
               IResult::Done(&b"\n"[..], SassItem::MixinDeclaration {
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
               IResult::Done(&b"\n"[..], SassItem::MixinDeclaration {
                   name: "foo".into(),
                   args: FormalArgs::new(vec![("x".into(), Value::Null)],
                                         false),
                   body: vec![SassItem::Property(
                       "foo-bar".into(),
                       Value::List(
                           vec![string("baz"),
                                Value::Variable("x".into())],
                           ListSeparator::Space),
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
               IResult::Done(&b"\n"[..], SassItem::MixinDeclaration {
                   name: "bar".into(),
                   args: FormalArgs::new(
                       vec![("a".into(), Value::Null),
                            ("b".into(), string("flug"))],
                       false),
                   body: vec![
                       SassItem::Property("foo-bar".into(),
                                          string("baz"),
                                          false),
                       SassItem::Rule(
                           selectors(b"foo, bar").unwrap().1,
                           vec![SassItem::Property(
                                    "property".into(),
                                    Value::Variable("b".into()),
                                    false)]),
                       SassItem::None,
                       ]}))
}

#[test]
fn test_simple_property() {
    let one = Rational::from_integer(1);
    fn r(v: u8) -> Rational {
        Rational::from_integer(v as isize)
    }
    assert_eq!(property(b"color: red;\n"),
               IResult::Done(&b""[..], SassItem::Property(
                   "color".to_string(),
                   Value::Color(r(255), r(0), r(0), one, Some("red".into())),
                   false)))
}
#[test]
fn test_property_2() {
    assert_eq!(property(b"background-position: 90% 50%;\n"),
               IResult::Done(&b""[..], SassItem::Property(
                   "background-position".to_string(),
                   Value::List(vec![percentage(90), percentage(50)],
                               ListSeparator::Space),
                   false)))
}

#[test]
fn test_variable_declaration_simple() {
    assert_eq!(variable_declaration(b"$foo: bar;\n"),
               IResult::Done(&b""[..],
                    SassItem::VariableDeclaration {
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
                    SassItem::VariableDeclaration {
                        name: "y".into(),
                        val: Value::List(
                            vec![string("some"), string("value")],
                            ListSeparator::Space),
                        default: false,
                        global: true,
                    }))
}

#[test]
fn test_variable_declaration_default() {
    assert_eq!(variable_declaration(b"$y: some value !default;\n"),
               IResult::Done(&b""[..],
                    SassItem::VariableDeclaration {
                        name: "y".into(),
                        val: Value::List(
                            vec![string("some"), string("value")],
                            ListSeparator::Space),
                        default: true,
                        global: false,
                    }))
}
