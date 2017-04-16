//! Sass reimplemented in rust with nom.
//!
//! The "r" in the name might stand for the Rust programming language,
//! or for my name Rasmus.
//!
//! # Example
//!
//! ```
//! use rsass::{OutputStyle, compile_scss_file};
//!
//! let file = "tests/basic/14_imports/a.scss".as_ref();
//! let css = compile_scss_file(file, OutputStyle::Compressed).unwrap();
//!
//! assert_eq!(css, b"div span{moo:goo}\n")
//! ```
//!
//! # Sass language and implemetation status
//!
//! The sass language [is defined in its reference
//! doc](http://sass-lang.com/documentation/file.SASS_REFERENCE.html).
//! This implementation is incomplete but getting there, if slowly.
//!
//! Progress: ![648](http://progressed.io/bar/648?scale=3294&suffix=+)
//! of 3294 tests passed (or 693 of 6049 when claiming to be libsass).
//!
//! If you want a working rust library for sass right now, you will
//! probably be better of with [sass-rs](https://crates.io/crates/sass-rs)
//! which is a rust wrapper around libsass.
//! Another alternative is [sassers](https://crates.io/crates/sassers)
//! which is another early stage pure rust implementation.
//! That said, this implementation has reached a version where I find it
//! usable for my personal projects, and the number of working tests are
//! improving.
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate num_rational;
extern crate num_traits;
extern crate rand;

use nom::IResult;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::from_utf8;

mod colors;
mod error;
mod formalargs;
mod operator;
mod functions;
mod selectors;
mod parseutil;
mod valueexpression;
mod variablescope;
mod output_style;
mod unit;

pub use error::Error;
use formalargs::{CallArgs, FormalArgs, call_args, formal_args};
use functions::SassFunction;
pub use output_style::OutputStyle;
use parseutil::{comment, name, opt_spacelike, spacelike};
use selectors::{Selector, selector};
use valueexpression::{Value, single_value, value_expression};
#[cfg(test)]
use valueexpression::ListSeparator;

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
///                           }", OutputStyle::Compressed).unwrap(),
///            b"foo bar{baz:value}\n")
/// ```
pub fn compile_scss(input: &[u8],
                    style: OutputStyle)
                    -> Result<Vec<u8>, Error> {
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
///                              OutputStyle::Compressed).unwrap(),
///            b"div span{moo:goo}\n")
/// ```
pub fn compile_scss_file(file: &Path,
                         style: OutputStyle)
                         -> Result<Vec<u8>, Error> {
    let file_context = FileContext::new();
    let (sub_context, file) = file_context.file(file);
    let items = parse_scss_file(&file)?;
    style.write_root(&items, sub_context)
}

/// A file context specifies where to find files to load.
///
/// When opening an included file, an extended file context is
/// created, to find further included files relative to the file they
/// are inlcuded from.
#[derive(Clone, Debug)]
pub struct FileContext {
    path: PathBuf,
}

impl FileContext {
    /// Create a new FileContext.
    ///
    /// Files will be resolved from the current working directory.
    pub fn new() -> Self {
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
    fn find_file(&self, name: &Path) -> Option<(Self, PathBuf)> {
        // TODO Check docs what expansions should be tried!
        let parent = name.parent();
        if let Some(name) = name.file_name().and_then(|n| n.to_str()) {
            for name in &[name,
                          &format!("{}.scss", name),
                          &format!("_{}.scss", name)] {
                let full =
                    parent.map(|p| p.join(name)).unwrap_or_else(|| name.into());
                let (c, p) = self.file(&full);
                if p.is_file() {
                    return Some((c, p));
                }
            }
        }
        None
    }
}

/// Parse a scss file.
///
/// Returns a vec of the top level items of the file (or an error message).
pub fn parse_scss_file(file: &Path) -> Result<Vec<SassItem>, Error> {
    let mut f = File::open(file).map_err(|e| Error::Input(file.into(), e))?;
    let mut data = vec![];
    f.read_to_end(&mut data).map_err(|e| Error::Input(file.into(), e))?;
    parse_scss_data(&data)
}

fn parse_scss_data(data: &[u8]) -> Result<Vec<SassItem>, Error> {
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

/// Every sass file is a sequence of sass items.
/// Scoping items contains further sequences of items.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SassItem {
    None,
    Comment(String),
    Import(Value),
    Property(String, Value, bool),
    Rule(Vec<Selector>, Vec<SassItem>),
    NamespaceRule(String, Value, Vec<SassItem>),
    Return(Value),
    VariableDeclaration {
        name: String,
        val: Value,
        default: bool,
        global: bool,
    },
    AtRule(String, Vec<SassItem>),
    MixinDeclaration {
        name: String,
        args: FormalArgs,
        body: Vec<SassItem>,
    },
    FunctionDeclaration { name: String, func: SassFunction },
    MixinCall { name: String, args: CallArgs },
    IfStatement(Value, Vec<SassItem>, Vec<SassItem>),
    /// The value may be or evaluate to a list.
    Each(String, Value, Vec<SassItem>),
    For {
        name: String,
        from: Value,
        to: Value,
        inclusive: bool,
        body: Vec<SassItem>,
    },
    While(Value, Vec<SassItem>),
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
                 tag!(";") >>
                 (SassItem::MixinCall {
                     name: name,
                     args: args.unwrap_or_default(),
                 })));

named!(at_rule<SassItem>,
       do_parse!(tag!("@") >> // opt_spacelike >>
                 query: is_not!("{}") >>
                 tag!("{") >>
                 body: many0!(body_item) >>
                 tag!("}") >>
                 (SassItem::AtRule(from_utf8(query).unwrap().into(),
                                   body))));

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

named!(each_loop<SassItem>,
       do_parse!(tag!("@each") >> spacelike >> tag!("$") >>
                 name: name >> spacelike >> tag!("in") >> spacelike >>
                 values: value_expression >> opt_spacelike >>
                 body: delimited!(preceded!(tag!("{"), opt_spacelike),
                                  many0!(body_item),
                                  tag!("}")) >>
                 (SassItem::Each(name, values, body))));

named!(for_loop<SassItem>,
       do_parse!(tag!("@for") >> spacelike >> tag!("$") >>
                 name: name >> spacelike >>
                 tag!("from") >> spacelike >>
                 from: single_value >> spacelike >>
                 inclusive: alt!(value!(true, tag!("through")) |
                                 value!(false, tag!("to"))) >> spacelike >>
                 to: single_value >> spacelike >>
                 body: delimited!(preceded!(tag!("{"), opt_spacelike),
                                  many0!(body_item),
                                  tag!("}")) >>
                 (SassItem::For {
                     name: name,
                     from: from,
                     to: to,
                     inclusive: inclusive,
                     body: body
                 })));

named!(while_loop<SassItem>,
       do_parse!(tag!("@while") >> spacelike >>
                 cond: value_expression >> spacelike >>
                 body: delimited!(preceded!(tag!("{"), opt_spacelike),
                                  many0!(body_item),
                                  tag!("}")) >>
                 (SassItem::While(cond, body))));

#[test]
fn if_with_no_else() {
    use operator::Operator;

    assert_eq!(if_statement(b"@if 1 == 1 { p { color: black; } }\n"),
               IResult::Done(&b"\n"[..],
                    SassItem::IfStatement(
                        Value::BinOp(Box::new(Value::scalar(1)),
                                     Operator::Equal,
                                     Box::new(Value::scalar(1))),
                        vec![SassItem::Rule(
                            vec![selector(b"p").unwrap().1],
                            vec![SassItem::None,
                                 SassItem::Property("color".into(),
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
                    }))
}

named!(mixin_declaration<&[u8], SassItem>,
       do_parse!(tag!("@mixin") >> spacelike >>
                 name: name >> opt_spacelike >>
                 args: opt!(formal_args) >> opt_spacelike >>
                 tag!("{") >> opt_spacelike >>
                 body: many0!(body_item) >>
                 tag!("}") >>
                 (SassItem::MixinDeclaration{
                     name: name,
                     args: args.unwrap_or_default(),
                     body: body,
                 })));

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
                           vec![selector(b"foo").unwrap().1,
                                selector(b"bar").unwrap().1],
                           vec![SassItem::None,
                                SassItem::Property(
                                    "property".into(),
                                    Value::Variable("b".into()),
                                    false)]),
                       SassItem::None,
                       ]}))
}

named!(function_declaration<SassItem>,
       do_parse!(tag!("@function") >> spacelike >>
                 name: name >> opt_spacelike >>
                 args: formal_args >> opt_spacelike >>
                 tag!("{") >> opt_spacelike >>
                 body: many0!(body_item) >>
                 tag!("}") >>
                 (SassItem::FunctionDeclaration {
                     name: name,
                     func: SassFunction::new(args, body),
                 })));

named!(return_stmt<SassItem>,
       do_parse!(tag!("@return") >> spacelike >>
                 v: value_expression >> opt_spacelike >>
                 opt!(tag!(";")) >>
                 (SassItem::Return(v))));

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
                 tag!("{") >>
                 body: many0!(body_item) >> opt_spacelike >>
                 tag!("}") >>
                 (SassItem::NamespaceRule(n1,
                                          value.unwrap_or(Value::Null),
                                          body))));

named!(opt_important<bool>,
       map!(opt!(do_parse!(opt_spacelike >> tag!("!") >>
                           opt_spacelike >> tag!("important") >>
                           ())),
            |o: Option<()>| o.is_some()));
#[test]
fn test_simple_property() {
    use num_rational::Rational;
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

#[cfg(test)]
fn percentage(v: isize) -> Value {
    use num_rational::Rational;
    use unit::Unit;
    Value::Numeric(Rational::from_integer(v), Unit::Percent, false)
}

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

#[cfg(test)]
fn string(v: &str) -> Value {
    use valueexpression::Quotes;
    Value::Literal(v.into(), Quotes::None)
}
