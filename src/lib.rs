#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate num_rational;

use nom::IResult::*;
use nom::multispace;
use std::io::{self, Write};
use std::str::from_utf8;

mod colors;
mod formalargs;
mod sassfunction;
mod selectors;
mod parseutil;
mod valueexpression;
mod variablescope;
use formalargs::{CallArgs, FormalArgs, call_args, formal_args};
use parseutil::{comment, name, opt_spacelike, spacelike};
use selectors::{Selector, selector};
use valueexpression::{Value, value_expression};
use variablescope::{ScopeImpl, Scope};

pub fn compile_scss(input: &[u8]) -> Result<Vec<u8>, String> {
    match sassfile(input) {
        Done(b"", items) => {
            let mut globals = ScopeImpl::new();
            let mut result = Vec::new();
            let mut separate = false;
            for item in items {
                match item {
                    SassItem::Rule(s, b) => {
                        if separate {
                            write!(result, "\n").unwrap();
                        } else {
                            separate = true;
                        }
                        write_rule(&s, &b, &mut result, &mut globals, None, 0)
                            .unwrap();
                    }
                    SassItem::VariableDeclaration { name, val, global } => {
                        globals.define(&name, &val, global);
                    }
                    SassItem::MixinDeclaration(m) => globals.define_mixin(&m),
                    SassItem::MixinCall { name, args } => {
                        if separate {
                            write!(result, "\n").unwrap();
                        } else {
                            separate = true;
                        }
                        if let Some(mixin) = globals.get_mixin(&name) {
                            let mut direct = vec![];
                            handle_body(
                                &mut direct,
                                &mut result,
                                &mut mixin.argscope(&mut globals, &args),
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
                        if separate {
                            separate = false;
                            write!(result, "\n").unwrap();
                        }
                        writeln!(result, "/*{}*/", c).unwrap();
                    }
                    SassItem::Property(_, _) => {
                        panic!("Global property not allowed");
                    }
                    SassItem::None => (),
                }
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
       many0!(alt!(chain!(spacelike, || SassItem::None) |
                   chain!(d: variable_declaration,
                          || SassItem::VariableDeclaration{
                              name: d.0.clone(),
                              val: d.1.clone(),
                              global: d.2,
                          }) |
                   chain!(d: mixin_declaration,
                          || SassItem::MixinDeclaration(d)) |
                   chain!(m: mixin_call,
                          || SassItem::MixinCall {
                              name: m.0.clone(),
                              args: m.1.clone(),
                          }) |
                   rule |
                   chain!(c: comment, || {
                       SassItem::Comment(from_utf8(c).unwrap().into())
                   }
                   ))));

#[derive(Clone, Debug, PartialEq, Eq)]
enum SassItem {
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
}

fn write_rule(selectors: &[Selector],
              body: &[SassItem],
              out: &mut Write,
              scope: &mut Scope,
              parent: Option<&[Selector]>,
              indent: usize)
              -> io::Result<()> {
    let selectors = if let Some(parent) = parent {
        let mut result = Vec::new();
        for p in parent {
            for s in selectors {
                result.push(p.join(s));
            }
        }
        result
    } else {
        selectors.into()
    };
    let mut direct = Vec::new();
    let mut sub = Vec::new();
    try!(handle_body(&mut direct, &mut sub, &mut ScopeImpl::sub(scope),
                     &selectors, &body, indent));
    if !direct.is_empty() {
        try!(write!(out, "{} {{\n",
                    selectors.iter()
                    .map(|s| format!("{}", s))
                    .collect::<Vec<_> >()
                    .join(", ")));
        try!(out.write(&direct));
        try!(write!(out, "}}\n"));
    }
    try!(out.write(&sub));
    Ok(())
}

fn handle_body(direct: &mut Vec<u8>,
               sub: &mut Vec<u8>,
               scope: &mut Scope,
               selectors: &[Selector],
               body: &[SassItem],
               indent: usize)
               -> io::Result<()> {
    for b in body {
        match b {
            &SassItem::Comment(ref c) => {
                try!(do_indent(direct, indent + 2));
                try!(writeln!(direct, "/*{}*/", c));
            }
            &SassItem::Property(ref name, ref value) => {
                try!(do_indent(direct, indent + 2));
                try!(write!(direct, "{}: {};\n", name, scope.evaluate(value)));
            }
            &SassItem::Rule(ref s, ref b) => {
                try!(write_rule(s, b, sub, scope, Some(&selectors), indent));
            }
            &SassItem::VariableDeclaration { ref name, ref val, global } => {
                scope.define(&name, &val, global);
            }
            &SassItem::MixinDeclaration(ref m) => {
                scope.define_mixin(m);
            }
            &SassItem::MixinCall { ref name, ref args } => {
                if let Some(mixin) = scope.get_mixin(name) {
                    let mut argscope = mixin.argscope(scope, &args);
                    try!(handle_body(direct, sub, &mut argscope, selectors,
                                     &mixin.body, indent));
                } else {
                    writeln!(direct,
                             "/* Unknown mixin {}({:?}) */",
                             name,
                             args)
                        .unwrap();
                }
            }
            &SassItem::None => (),
        }
    }
    Ok(())
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
           chain!(spacelike, || SassItem::None) |
           chain!(d: mixin_declaration,
                  || SassItem::MixinDeclaration(d)) |
           chain!(d: variable_declaration,
                  || SassItem::VariableDeclaration{
                      name: d.0.to_string(),
                      val: d.1.clone(),
                      global: d.2,
                  }) |
           rule |
           property |
           chain!(m: mixin_call,
                  || SassItem::MixinCall {
                      name: m.0.clone(),
                      args: m.1.clone(),
                  }) |
           chain!(c: comment,
                  || SassItem::Comment(from_utf8(c).unwrap().into()))
               ));

named!(mixin_call<&[u8], (String, CallArgs)>,
       chain!(tag!("@include") ~ spacelike ~
              name: name ~ spacelike? ~
              args: call_args? ~
              spacelike? ~
              tag!(";"),
              || (name, args.unwrap_or_default())
              ));

#[test]
fn test_mixin_call_noargs() {
    assert_eq!(mixin_call(b"@include foo;\n"),
               Done(&b"\n"[..],
                    ("foo".to_string(), CallArgs::new(vec![]))))
}

#[test]
fn test_mixin_call_pos_args() {
    assert_eq!(mixin_call(b"@include foo(bar, baz);\n"),
               Done(&b"\n"[..],
                    ("foo".to_string(),
                     CallArgs::new(
                         vec![(None, Value::Literal("bar".to_string())),
                              (None, Value::Literal("baz".to_string()))]))))
}

#[test]
fn test_mixin_call_named_args() {
    assert_eq!(mixin_call(b"@include foo($x: bar, $y: baz);\n"),
               Done(&b"\n"[..],
                    ("foo".to_string(),
                     CallArgs::new(
                         vec![(Some("x".into()), Value::Literal("bar".into())),
                              (Some("y".into()), Value::Literal("baz".into()))
                              ]))))
}

named!(mixin_declaration<&[u8], MixinDeclaration>,
       chain!(tag!("@mixin") ~ spacelike ~
              name: name ~ spacelike? ~
              args: formal_args ~ spacelike? ~
              tag!("{") ~ spacelike? ~
              body: many0!(body_item) ~
              tag!("}"),
              || MixinDeclaration{
                  name: name,
                  args: args,
                  body: body,
              }));

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
                   args: FormalArgs::new(vec![("x".into(), None)]),
                   body: vec![SassItem::Property(
                       "foo-bar".into(),
                       Value::MultiSpace(
                           vec![Value::Literal("baz".into()),
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
                       vec![("a".into(), None),
                            ("b".into(),
                             Some(Value::Literal("flug".into())))]),
                   body: vec![
                       SassItem::Property("foo-bar".into(),
                                          Value::Literal("baz".into())),
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


fn do_indent(out: &mut Write, steps: usize) -> io::Result<()> {
    for _i in 0..steps {
        try!(write!(out, " "));
    }
    Ok(())
}

named!(property<&[u8], SassItem>,
       chain!(multispace? ~
              name: name ~
              multispace? ~
              tag!(":") ~
              multispace? ~
              val: value_expression ~
              multispace? ~
              tag!(";") ~
              spacelike?,
              || SassItem::Property(name, val)));

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

named!(variable_declaration<&[u8], (String, Value, bool)>,
       chain!(tag!("$") ~
              name: name ~
              multispace? ~
              tag!(":") ~
              multispace? ~
              val: value_expression ~
              multispace? ~
              global: opt!(tag!("!global")) ~
              multispace? ~
              tag!(";") ~
              multispace?,
              || (name, val, global.is_some())));

#[test]
fn test_variable_declaration_simple() {
    assert_eq!(variable_declaration(b"$foo: bar;\n"),
               Done(&b""[..],
                    ("foo".into(), Value::Literal("bar".into()), false)))
}

#[test]
fn test_variable_declaration_global() {
    assert_eq!(variable_declaration(b"$y: some value !global;\n"),
               Done(&b""[..], ("y".into(),
                               Value::MultiSpace(
                                   vec![Value::Literal("some".into()),
                                        Value::Literal("value".into())]),
                               true)))
}
