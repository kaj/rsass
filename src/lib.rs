#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate num_rational;
extern crate num_traits;

use nom::IResult::*;
use std::io::Write;
use std::str::from_utf8;

mod colors;
mod formalargs;
mod operator;
mod sassfunction;
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

pub fn compile_scss(input: &[u8],
                    style: OutputStyle)
                    -> Result<Vec<u8>, String> {
    match sassfile(input) {
        Done(b"", items) => {
            let mut globals = ScopeImpl::new();
            let mut result = Vec::new();
            let mut separate = false;
            for item in items {
                match item {
                    SassItem::Rule(s, b) => {
                        if separate {
                            style.do_indent(&mut result, 0).unwrap();
                        } else {
                            separate = true;
                        }
                        style.write_rule(&s,
                                        &b,
                                        &mut result,
                                        &mut globals,
                                        None,
                                        0)
                            .unwrap();
                    }
                    SassItem::VariableDeclaration { name, val, global } => {
                        globals.define(&name, &val, global);
                    }
                    SassItem::MixinDeclaration(m) => globals.define_mixin(&m),
                    SassItem::MixinCall { name, args } => {
                        if separate {
                            style.do_indent(&mut result, 0).unwrap();
                        } else {
                            separate = true;
                        }
                        if let Some(mixin) = globals.get_mixin(&name) {
                            let mut direct = vec![];
                            style.handle_body(&mut direct,
                                             &mut result,
                                             &mut mixin.argscope(&mut globals,
                                                                 &args),
                                             &vec![Selector::root()],
                                             &mixin.body,
                                             0)
                                .unwrap();
                            assert_eq!(direct, &[]);
                        } else {
                            panic!(format!("Unknown mixin {}({:?})",
                                           name,
                                           args))
                        }
                    }
                    SassItem::Comment(c) => {
                        if !style.is_compressed() {
                            if separate {
                                style.do_indent(&mut result, 0).unwrap();
                            } else {
                                separate = true;
                            }
                            write!(result, "/*{}*/", c).unwrap();
                        }
                    }
                    SassItem::Property(_, _) => {
                        panic!("Global property not allowed");
                    }
                    SassItem::IfStatement(cond, do_if, do_else) => {
                        if separate {
                            style.do_indent(&mut result, 0).unwrap();
                        } else {
                            separate = true;
                        }
                        if globals.evaluate(&cond).is_true() {
                            let mut direct = vec![];
                            style.handle_body(&mut direct,
                                             &mut result,
                                             &mut ScopeImpl::sub(&mut globals),
                                             &vec![Selector::root()],
                                             &do_if,
                                             0)
                                .unwrap();
                            assert_eq!(direct, &[]);
                        } else {
                            let mut direct = vec![];
                            style.handle_body(&mut direct,
                                             &mut result,
                                             &mut ScopeImpl::sub(&mut globals),
                                             &vec![Selector::root()],
                                             &do_else,
                                             0)
                                .unwrap();
                            assert_eq!(direct, &[]);
                        }
                    }
                    SassItem::None => (),
                }
            }
            if result != b"" && result[result.len() - 1] != b'\n' {
                write!(result, "\n").unwrap();
            }
            Ok(result)
        }
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
                   variable_declaration |
                   map!(mixin_declaration, |d| SassItem::MixinDeclaration(d)) |
                   mixin_call |
                   if_statement |
                   rule |
                   map!(comment,
                        |c| SassItem::Comment(from_utf8(c).unwrap().into()))
                   )));

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SassItem {
    None,
    Comment(String),
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
           if_statement |
           map!(comment, |c| SassItem::Comment(from_utf8(c).unwrap().into()))
               ));

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
    Value::Literal(v.into(), false)
}
