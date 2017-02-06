//! Sass reimplemented in rust with nom (very early stage).
//!
//! The "r" in the name might stand for the Rust programming language,
//! or for my name Rasmus.
//!
//! # Example
//!
//! ```
//! use rsass::{OutputStyle, compile_scss_file};
//!
//! let sass = "tests/basic/14_imports/a.scss".as_ref();
//! let css = compile_scss_file(sass, OutputStyle::Compressed);
//!
//! assert_eq!(css, Ok("div span{moo:goo}\n".into()))
//! ```

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate num_rational;
extern crate num_traits;
extern crate rand;

use nom::IResult::*;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::from_utf8;

mod colors;
mod formalargs;
mod operator;
mod functions;
mod selectors;
mod parseutil;
mod valueexpression;
mod variablescope;
mod output_style;
mod identify_last_iterator;

use formalargs::{CallArgs, FormalArgs, call_args, formal_args};
pub use output_style::OutputStyle;
use parseutil::{comment, name, opt_spacelike, spacelike};
use selectors::{Selector, selector};
use valueexpression::{Value, value_expression};
use variablescope::{ScopeImpl, Scope};

/// Parse scss data and write css in the given style.
///
/// # Example
///
/// ```
/// use rsass::{OutputStyle, compile_scss};
///
/// assert_eq!(compile_scss(b"foo {\n\
///                             bar {\n\
///                               baz:value;\n\
///                             }\n\
///                           }", OutputStyle::Compressed),
///            Ok("foo bar{baz:value}\n".into()))
/// ```
pub fn compile_scss(input: &[u8],
                    style: OutputStyle)
                    -> Result<Vec<u8>, String> {
    let file_context = FileContext::new();
    let items = parse_scss_data(input)?;
    style.write_root(&items, file_context)
}

/// Parse a file of scss data and write css in the given style.
///
/// Any `@import` directives will be handled relative to the directory
/// part of `file`.
///
/// # Example
///
/// ```
/// use rsass::{OutputStyle, compile_scss_file};
///
/// assert_eq!(compile_scss_file("tests/basic/14_imports/a.scss".as_ref(),
///                              OutputStyle::Compressed),
///            Ok("div span{moo:goo}\n".into()))
/// ```
pub fn compile_scss_file(file: &Path,
                         style: OutputStyle)
                         -> Result<Vec<u8>, String> {
    let file_context = FileContext::new();
    let (sub_context, file) = file_context.file(file);
    let items = parse_scss_file(&file)?;
    style.write_root(&items, sub_context)
}

#[derive(Clone, Debug)]
pub struct FileContext {
    path: PathBuf,
}

impl FileContext {
    fn new() -> Self {
        FileContext { path: PathBuf::new() }
    }
    fn file(&self, file: &Path) -> (Self, PathBuf) {
        let t = self.path.join(file);
        if let Some(dir) = t.parent() {
            (FileContext { path: PathBuf::from(dir) }, t.clone())
        } else {
            (FileContext::new(), t.clone())
        }
    }
}


pub fn parse_scss_file(file: &Path) -> Result<Vec<SassItem>, String> {
    let mut f = File::open(file)
        .map_err(|e| format!("Failed to open {:?}: {}", file, e))?;
    let mut data = vec![];
    f.read_to_end(&mut data)
        .map_err(|e| format!("Failed to read {:?}: {}", file, e))?;
    parse_scss_data(&data)
}

fn parse_scss_data(data: &[u8]) -> Result<Vec<SassItem>, String> {
    match sassfile(&data) {
        Done(b"", items) => Ok(items),
        Done(rest, _styles) => {
            let t = from_utf8(&rest)
                .map(|s| s.to_string())
                .unwrap_or_else(|_| format!("{:?}", rest));
            Err(format!("Failed to parse entire input: `{}` remains.", t))
        }
        Incomplete(x) => Err(format!("Incomplete: {:?}", x)),
        Error(x) => Err(format!("Error: {}", x)),
    }
}

named!(sassfile<&[u8], Vec<SassItem> >,
       many0!(alt!(value!(SassItem::None, spacelike) |
                   import |
                   variable_declaration |
                   map!(mixin_declaration, |d| SassItem::MixinDeclaration(d)) |
                   mixin_call |
                   if_statement |
                   rule |
                   map!(comment,
                        |c| SassItem::Comment(from_utf8(c).unwrap().into()))
                   )));

/// Every sass file is a sequence of sass items.
/// Scoping items contains further sequences of items.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SassItem {
    None,
    Comment(String),
    Import(Value),
    Property(String, Value),
    Rule(Vec<Selector>, Vec<SassItem>),
    VariableDeclaration {
        name: String,
        val: Value,
        global: bool,
    },
    MixinDeclaration(MixinDeclaration),
    MixinCall { name: String, args: CallArgs },
    IfStatement(Value, Vec<SassItem>, Vec<SassItem>),
}

named!(rule<SassItem>,
       do_parse!(opt_spacelike >>
                 selectors: separated_nonempty_list!(
                     do_parse!(tag!(",") >> opt!(is_a!(", \t\n")) >> ()),
                     selector) >>
                 opt!(is_a!(", \t\n")) >>
                 tag!("{") >>
                 body: many0!(body_item) >>
                 tag!("}") >>
                 (SassItem::Rule(selectors, body))));

named!(body_item<SassItem>,
       alt_complete!(
           value!(SassItem::None, spacelike) |
           map!(mixin_declaration,
                |d| SassItem::MixinDeclaration(d)) |
           variable_declaration |
           rule |
           property |
           mixin_call |
           import |
           if_statement |
           map!(comment, |c| SassItem::Comment(from_utf8(c).unwrap().into()))
               ));

named!(import<SassItem>,
       map!(delimited!(tag!("@import "), value_expression, tag!(";")),
            |v| SassItem::Import(v)));

named!(mixin_call<SassItem>,
       do_parse!(tag!("@include") >> spacelike >>
                 name: name >> opt_spacelike >>
                 args: opt!(call_args) >> opt_spacelike >>
                 tag!(";") >>
                 (SassItem::MixinCall {
                     name: name,
                     args: args.unwrap_or_default(),
                 })));

named!(if_statement<SassItem>,
       preceded!(tag!("@"), if_statement_inner));

named!(if_statement_inner<SassItem>,
       do_parse!(tag!("if") >> spacelike >>
                 cond: value_expression >> opt_spacelike >>
                 body: delimited!(preceded!(tag!("{"), opt_spacelike),
                                  many0!(body_item),
                                  tag!("}")) >>
                 else_body: opt!(complete!(preceded!(
                     delimited!(opt_spacelike, tag!("@else"), opt_spacelike),
                     alt_complete!(
                         delimited!(preceded!(tag!("{"), opt_spacelike),
                                    many0!(body_item),
                                    preceded!(tag!("}"), opt_spacelike)) |
                         map!(if_statement_inner, |s| vec![s]))))) >>
                 (SassItem::IfStatement(cond,
                                        body,
                                        else_body.unwrap_or_default()))));

#[test]
fn if_with_no_else() {
    use operator::Operator;

    assert_eq!(if_statement(b"@if 1 == 1 { p { color: black; } }\n"),
               Done(&b"\n"[..],
                    SassItem::IfStatement(
                        Value::BinOp(Box::new(Value::scalar(1)),
                                     Operator::Equal,
                                     Box::new(Value::scalar(1))),
                        vec![SassItem::Rule(
                            vec![selector(b"p").unwrap().1],
                            vec![SassItem::None,
                                 SassItem::Property("color".into(),
                                                    Value::black())]),
                             SassItem::None],
                        vec![])))
}

#[test]
fn test_mixin_call_noargs() {
    assert_eq!(mixin_call(b"@include foo;\n"),
               Done(&b"\n"[..],
                    SassItem::MixinCall {
                        name: "foo".to_string(),
                        args: CallArgs::new(vec![]),
                    }))
}

#[test]
fn test_mixin_call_pos_args() {
    assert_eq!(mixin_call(b"@include foo(bar, baz);\n"),
               Done(&b"\n"[..],
                    SassItem::MixinCall {
                        name: "foo".to_string(),
                        args: CallArgs::new(
                            vec![(None, string("bar")),
                                 (None, string("baz"))]),
                    }))
}

#[test]
fn test_mixin_call_named_args() {
    assert_eq!(mixin_call(b"@include foo($x: bar, $y: baz);\n"),
               Done(&b"\n"[..],
                    SassItem::MixinCall {
                        name: "foo".to_string(),
                        args: CallArgs::new(
                            vec![(Some("x".into()), string("bar")),
                                 (Some("y".into()), string("baz"))]),
                    }))
}

named!(mixin_declaration<&[u8], MixinDeclaration>,
       do_parse!(tag!("@mixin") >> spacelike >>
                 name: name >> opt_spacelike >>
                 args: formal_args >> opt_spacelike >>
                 tag!("{") >> opt_spacelike >>
                 body: many0!(body_item) >>
                 tag!("}") >>
                 (MixinDeclaration{
                     name: name,
                     args: args,
                     body: body,
                 })));

#[test]
fn test_mixin_declaration_empty() {
    assert_eq!(mixin_declaration(b"@mixin foo() {}\n"),
               Done(&b"\n"[..], MixinDeclaration {
                   name: "foo".into(),
                   args: FormalArgs::new(vec![]),
                   body: vec![],
               }))
}

#[test]
fn test_mixin_declaration() {
    assert_eq!(mixin_declaration(b"@mixin foo($x) {\n  \
                                   foo-bar: baz $x;\n\
                                   }\n"),
               Done(&b"\n"[..], MixinDeclaration {
                   name: "foo".into(),
                   args: FormalArgs::new(vec![("x".into(), Value::Null)]),
                   body: vec![SassItem::Property(
                       "foo-bar".into(),
                       Value::MultiSpace(
                           vec![string("baz"),
                                Value::Variable("x".into())]),
                       )],
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
               Done(&b"\n"[..], MixinDeclaration {
                   name: "bar".into(),
                   args: FormalArgs::new(
                       vec![("a".into(), Value::Null),
                            ("b".into(), string("flug"))]),
                   body: vec![
                       SassItem::Property("foo-bar".into(),
                                          string("baz")),
                       SassItem::Rule(
                           vec![selector(b"foo").unwrap().1,
                                selector(b"bar").unwrap().1],
                           vec![SassItem::None,
                                SassItem::Property(
                                    "property".into(),
                                    Value::Variable("b".into()))]),
                       SassItem::None,
                       ]}))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MixinDeclaration {
    name: String,
    args: FormalArgs,
    body: Vec<SassItem>,
}

impl MixinDeclaration {
    fn argscope<'a>(&self,
                    scope: &'a mut Scope,
                    args: &CallArgs)
                    -> ScopeImpl<'a> {
        self.args.eval(scope, args)
    }
}

named!(property<&[u8], SassItem>,
       do_parse!(opt_spacelike >>
                 name: name >> opt_spacelike >>
                 tag!(":") >> opt_spacelike >>
                 val: value_expression >> opt_spacelike >>
                 opt!(tag!(";")) >> opt_spacelike >>
                 (SassItem::Property(name, val))));

#[test]
fn test_simple_property() {
    use num_rational::Rational;
    let one = Rational::from_integer(1);
    assert_eq!(property(b"color: red;\n"),
               Done(&b""[..], SassItem::Property(
                   "color".to_string(),
                   Value::Color(255, 0, 0, one, Some("red".into())))))
}
#[test]
fn test_property_2() {
    assert_eq!(property(b"background-position: 90% 50%;\n"),
               Done(&b""[..], SassItem::Property(
                   "background-position".to_string(),
                   Value::MultiSpace(vec![percentage(90), percentage(50)]))))
}

#[cfg(test)]
fn percentage(v: isize) -> Value {
    use num_rational::Rational;
    use valueexpression::Unit;
    Value::Numeric(Rational::from_integer(v), Unit::Percent, false)
}

named!(variable_declaration<SassItem>,
       do_parse!(tag!("$") >>
                 name: name >> opt_spacelike >>
                 tag!(":") >> opt_spacelike >>
                 val: value_expression >> opt_spacelike >>
                 global: opt!(tag!("!global")) >> opt_spacelike >>
                 tag!(";") >> opt_spacelike >>
                 (SassItem::VariableDeclaration {
                     name: name,
                     val: val.clone(),
                     global: global.is_some(),
                 })));

#[test]
fn test_variable_declaration_simple() {
    assert_eq!(variable_declaration(b"$foo: bar;\n"),
               Done(&b""[..],
                    SassItem::VariableDeclaration {
                        name: "foo".into(),
                        val: string("bar"),
                        global: false,
                    }))
}

#[test]
fn test_variable_declaration_global() {
    assert_eq!(variable_declaration(b"$y: some value !global;\n"),
               Done(&b""[..],
                    SassItem::VariableDeclaration {
                        name: "y".into(),
                        val: Value::MultiSpace(
                            vec![string("some"), string("value")]),
                        global: true,
                    }))
}

#[cfg(test)]
fn string(v: &str) -> Value {
    use valueexpression::Quotes;
    Value::Literal(v.into(), Quotes::None)
}
