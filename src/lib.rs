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
//! Progress: ![1146](http://progressed.io/bar/114?scale=331&suffix=6)
//! of 3310 tests passed
//! (or 1215 of 6065 when claiming to be libsass).
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
mod functions;
mod selectors;
mod parseutil;
mod value;
mod variablescope;
mod output_style;
mod unit;

pub use error::Error;
use formalargs::{CallArgs, FormalArgs, call_args, formal_args};
pub use functions::SassFunction;
pub use num_rational::Rational;
pub use output_style::OutputStyle;
use parseutil::{comment, ignore_space, name, opt_spacelike, spacelike};
use selectors::{Selectors, selectors};
pub use unit::Unit;
pub use value::{ListSeparator, Quotes, Value, function_call, interpolation,
                quoted_string, singlequoted_string};
use value::{single_value, value_expression};
pub use variablescope::{GlobalScope, Scope};

/// Parse scss data from a buffer and write css in the given style.
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
    style.write_root(&items, &mut GlobalScope::new(), file_context)
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
    style.write_root(&items, &mut GlobalScope::new(), sub_context)
}

/// A file context specifies where to find files to load.
///
/// When opening an included file, an extended file context is
/// created, to find further included files relative to the file they
/// are inlcuded from.
///
/// # Example
/// ```
/// use rsass::FileContext;
///
/// let base = FileContext::new();
/// let (base, file1) = base.file("some/dir/file.scss".as_ref());
/// // base is now a relative to file1, usefull to open files
/// // by paths mentioned in file1.
/// let (base, file2) = base.file("some/other.scss".as_ref());
/// assert_eq!(file1.to_string_lossy(), "some/dir/file.scss");
/// assert_eq!(file2.to_string_lossy(), "some/dir/some/other.scss");
/// ```
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
    /// Get a file from this context.
    ///
    /// Get a path and a FileContext from this FileContext and a path.
    pub fn file(&self, file: &Path) -> (Self, PathBuf) {
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

/// Every sass file is a sequence of sass items.
/// Scoping items contains further sequences of items.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SassItem {
    Import(Value),
    VariableDeclaration {
        name: String,
        val: Value,
        default: bool,
        global: bool,
    },
    AtRule {
        name: String,
        args: Value,
        body: Option<Vec<SassItem>>,
    },

    MixinDeclaration {
        name: String,
        args: FormalArgs,
        body: Vec<SassItem>,
    },
    MixinCall { name: String, args: CallArgs, body: Vec<SassItem> },
    Content,

    FunctionDeclaration { name: String, func: SassFunction },
    Return(Value),

    IfStatement(Value, Vec<SassItem>, Vec<SassItem>),
    /// The value may be or evaluate to a list.
    Each(String, Value, Vec<SassItem>),
    For {
        name: String,
        from: Box<Value>,
        to: Box<Value>,
        inclusive: bool,
        body: Vec<SassItem>,
    },
    While(Value, Vec<SassItem>),

    Rule(Selectors, Vec<SassItem>),
    NamespaceRule(String, Value, Vec<SassItem>),
    Property(String, Value, bool),
    Comment(String),
    None,
}

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

/// Tests from `sass_spec/spec/css/unknown_directive`
#[cfg(test)]
mod css_unknown_directive {
    use super::check;
    // Unknown directives should support almost any sequence of valid tokens,
    // including interpolation.

    #[test]
    fn t01_characters_are_passed_through_unaltered() {
        check("@asdf .~@#$%^&*()_-+=[]|:<>,.?/;\n",
              "@asdf .~@#$%^&*()_-+=[]|:<>,.?/;\n")
    }
    #[test]
    fn t02_strings_are_tokenized_as_strings() {
        check("@asdf \"f'o\" 'b\"r' url(baz) url(\"qux\");\n",
              "@asdf \"f'o\" 'b\"r' url(baz) url(\"qux\");\n")
    }
    #[test]
    fn t03_comments_are_preserved() {
        check("@asdf foo //\n      bar;\n", "@asdf foo //\n      bar;\n")
    }
    #[test]
    fn t04_comments_are_preserved() {
        check("@asdf foo /* bar */ baz;", "@asdf foo /* bar */ baz;\n")
    }
    #[test]
    fn t05_interpolation_plain() {
        check("@asdf #{1 + 2};\n", "@asdf 3;\n")
    }
    #[test]
    fn t06_interpolation_in_string() {
        check("@asdf \"foo #{\"bar\"} baz\";\n", "@asdf \"foo bar baz\";\n")
    }
    #[test]
    #[ignore] // TODO The single quotes should not be converted to double.
    fn t07_interpolation_in_string() {
        check("@asdf 'foo #{'bar'} baz';\n", "@asdf 'foo bar baz';\n")
    }
    #[test]
    fn t08_interpolation_in_url() {
        check("@asdf url(http://#{\")\"}.com/);\n",
              "@asdf url(http://).com/);\n")
    }
    #[test]
    fn t09_interpolation_in_url() {
        check("@asdf url(\"http://#{\")\"}.com/\");\n",
              "@asdf url(\"http://).com/\");\n")
    }
}

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

#[cfg(test)]
fn percentage(v: isize) -> Value {
    Value::Numeric(Rational::from_integer(v), Unit::Percent, false, false)
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
    Value::Literal(v.into(), Quotes::None)
}

#[cfg(test)]
fn check(input: &str, expected: &str) {
    assert_eq!(compile_scss(input.as_bytes(), OutputStyle::Normal)
                   .and_then(|s| Ok(String::from_utf8(s)?))
                   .map_err(|e| format!("{:?}", e)),
               Ok(expected.to_string()));
}
