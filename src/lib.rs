#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate num_rational;

use nom::{alphanumeric, multispace};
use nom::IResult::*;
use std::io::{self, Write};
use std::str::from_utf8;

mod colors;
mod selectors;
mod spacelike;
mod valueexpression;
mod variablescope;
use selectors::{Selector, selector};
use spacelike::spacelike;
use valueexpression::{Value, value_expression, single_expression, space_list};
use variablescope::Scope;

pub fn compile_scss(input: &[u8]) -> Result<Vec<u8>, ()> {
    match sassfile(input) {
        Done(b"", items) => {
            let mut globals = Scope::new();
            let mut result = Vec::new();
            let mut separate = false;
            for item in items {
                match item {
                    SassItem::Rule(rule) => {
                        if separate {
                            write!(result, "\n").unwrap();
                        } else {
                            separate = true;
                        }
                        rule.write(&mut result, &globals, None, 0).unwrap();
                    }
                    SassItem::VariableDeclaration { name, val } => {
                        globals.define(&name, &val);
                    }
                    SassItem::MixinDeclaration(m) => globals.define_mixin(&m),
                    SassItem::MixinCall { name, args } => {
                        writeln!(result, "mixin {}({:?})", name, args).unwrap();
                    }
                    SassItem::Comment(c) => {
                        if separate {
                            separate = false;
                            write!(result, "\n").unwrap();
                        }
                        writeln!(result, "/*{}*/", c).unwrap();
                    }
                    SassItem::Property(_) => {
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
            println!("Failed to parse entire input: `{}` remains.", t);
            Err(())
        }
        Incomplete(x) => {
            println!("Incomplete: {:?}", x);
            Err(())
        }
        Error(x) => {
            println!("Error: {}", x);
            Err(())
        }
    }
}

named!(sassfile<&[u8], Vec<SassItem> >,
       many0!(alt!(chain!(spacelike, || SassItem::None) |
                   chain!(d: variable_declaration,
                          || SassItem::VariableDeclaration{
                              name: d.0.to_string(),
                              val: d.1.clone(),
                          }) |
                   chain!(d: mixin_declaration,
                          || SassItem::MixinDeclaration(d)) |
                   chain!(r: rule, || SassItem::Rule(r)) |
                   chain!(c: comment, || {
                       SassItem::Comment(from_utf8(c).unwrap().into())
                   }
                   ))));

#[derive(Clone, Debug, PartialEq, Eq)]
enum SassItem {
    None,
    Comment(String),
    Property(Property),
    Rule(Rule),
    VariableDeclaration { name: String, val: Value },
    MixinDeclaration(MixinDeclaration),
    MixinCall {
        name: String,
        args: Option<Vec<(Option<String>, Value)>>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rule {
    selectors: Vec<Selector>,
    body: Vec<SassItem>,
}

impl Rule {
    fn write(&self,
             out: &mut Write,
             scope: &Scope,
             parent: Option<&Vec<Selector>>,
             indent: usize)
             -> io::Result<()> {
        let selectors = if let Some(parent) = parent {
            let mut result = Vec::new();
            for p in parent {
                for s in &self.selectors {
                    result.push(p.join(s));
                }
            }
            result
        } else {
            self.selectors.clone()
        };
        let mut direct = Vec::new();
        let mut sub = Vec::new();
        try!(handle_body(&mut direct, &mut sub, &mut Scope::sub(scope),
                         &selectors, &self.body, indent));
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
}

fn handle_body(direct: &mut Vec<u8>,
               sub: &mut Vec<u8>,
               scope: &mut Scope,
               selectors: &Vec<Selector>,
               body: &Vec<SassItem>,
               indent: usize)
               -> io::Result<()> {
    for b in body {
        match b {
            &SassItem::Comment(ref c) => {
                try!(do_indent(direct, indent + 2));
                try!(writeln!(direct, "/*{}*/", c));
            }
            &SassItem::Property(ref p) => {
                try!(p.write(direct, &scope, indent + 2));
            }
            &SassItem::Rule(ref r) => {
                try!(r.write(sub, &scope, Some(&selectors), indent));
            }
            &SassItem::VariableDeclaration { ref name, ref val } => {
                scope.define(&name, &val);
            }
            &SassItem::MixinDeclaration(ref m) => {
                scope.define_mixin(m);
            }
            &SassItem::MixinCall { ref name, ref args } => {
                if let Some(mixin) = scope.get_mixin(name) {
                    let mut argscope = Scope::sub(&scope);
                    for (fi, &(ref name, ref default)) in
                        mixin.args.iter().enumerate() {
                        args.as_ref()
                            .and_then(|ref a| {
                                a.iter()
                                    .find(|&&(ref k, ref _v)| {
                                        k.as_ref() == Some(name)
                                    })
                                    .or_else(|| a.get(fi))
                                    .map(|&(ref _k, ref v)| v)
                            })
                            .or_else(|| default.as_ref())
                            .map(|value| argscope.define(&name, &value));
                    }
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

named!(rule<&[u8], Rule>,
       chain!(spacelike? ~
              selectors: separated_nonempty_list!(chain!(tag!(",") ~ spacelike?,
                                                         || ()),
                                                  selector) ~
              spacelike? ~
              tag!("{") ~
              body: many0!(body_item) ~
              tag!("}"),
              || Rule {
                  selectors: selectors,
                  body: body,
              }));

named!(body_item<SassItem>,
       alt_complete!(
           chain!(spacelike, || SassItem::None) |
           chain!(d: mixin_declaration,
                  || SassItem::MixinDeclaration(d)) |
           chain!(d: variable_declaration,
                  || SassItem::VariableDeclaration{
                      name: d.0.to_string(),
                      val: d.1.clone(),
                  }) |
           chain!(r: rule, || SassItem::Rule(r)) |
           chain!(p: property, || SassItem::Property(p)) |
           chain!(m: mixin_call,
                  || SassItem::MixinCall {
                      name: m.0.clone(),
                      args: m.1.clone(),
                  }) |
           chain!(c: comment,
                  || SassItem::Comment(from_utf8(c).unwrap().into()))
               ));

named!(mixin_call<&[u8], (String, Option<Vec<(Option<String>, Value)>>)>,
       chain!(tag!("@include") ~ spacelike ~
              name: alphanumeric ~ // spacelike? ~
              args: opt!(chain!(tag!("(") ~
                                a: separated_nonempty_list!(
                                    chain!(tag!(",") ~ spacelike?, || ()),
                                    alt_complete!(
                                        chain!(tag!("$") ~
                                               name: alphanumeric ~
                                               multispace? ~
                                               tag!(":") ~
                                               multispace? ~
                                               val: space_list ~
                                               multispace?,
                                               || (Some(from_utf8(name)
                                                        .unwrap().into()),
                                                   val)) |
                                        chain!(e: single_expression,
                                               || (None, e))
                                            )) ~
                                tag!(")") ~
                                spacelike?,
                                || a)) ~
              tag!(";"),
              || (from_utf8(name).unwrap().to_string(), args)
              ));

#[test]
fn test_mixin_call_noargs() {
    assert_eq!(mixin_call(b"@include foo;\n"),
               Done(&b"\n"[..],
                    ("foo".to_string(), None)));
}

#[test]
fn test_mixin_call_pos_args() {
    assert_eq!(mixin_call(b"@include foo(bar, baz);\n"),
               Done(&b"\n"[..],
                    ("foo".to_string(),
                     Some(vec![(None, Value::Literal("bar".to_string())),
                               (None, Value::Literal("baz".to_string()))]))));
}

#[test]
fn test_mixin_call_named_args() {
    assert_eq!(mixin_call(b"@include foo($x: bar, $y: baz);\n"),
               Done(&b"\n"[..],
                    ("foo".to_string(),
                     Some(vec![(Some("x".into()), Value::Literal("bar".into())),
                               (Some("y".into()), Value::Literal("baz".into()))
                               ]))));
}

named!(mixin_declaration<&[u8], MixinDeclaration>,
       chain!(tag!("@mixin") ~ spacelike ~
              name: alphanumeric ~ spacelike? ~
              tag!("(") ~ spacelike? ~
              args: separated_list!(
                  chain!(tag!(",") ~ spacelike?, || ()),
                  chain!(tag!("$") ~ name: alphanumeric ~
                         d: opt!(chain!(tag!(":") ~ spacelike? ~
                                        d: value_expression ~ spacelike?,
                                        || d)),
                         || (from_utf8(name).unwrap().to_string(), d))) ~
              tag!(")") ~ spacelike? ~
              tag!("{") ~ spacelike? ~
              body: many0!(body_item) ~
              tag!("}"),
              || MixinDeclaration{
                  name: from_utf8(name).unwrap().into(),
                  args: args,
                  body: body,
              }));

#[test]
fn test_mixin_declaration_empty() {
    assert_eq!(mixin_declaration(b"@mixin foo() {}\n"),
               Done(&b"\n"[..], MixinDeclaration {
                   name: "foo".into(),
                   args: vec![],
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
                   args: vec![("x".into(), None)],
                   body: vec![SassItem::Property(Property {
                       name: "foo-bar".into(),
                       value: Value::MultiSpace(
                           vec![Value::Literal("baz".into()),
                                Value::Variable("x".into())]),
                   })],
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
                   args: vec![("a".into(), None),
                              ("b".into(),
                               Some(Value::Literal("flug".into())))],
                   body: vec![
                       SassItem::Property(Property {
                           name: "foo-bar".into(),
                           value: Value::Literal("baz".into()),
                       }),
                       SassItem::Rule(Rule {
                           selectors: vec![
                               selector(b"foo").unwrap().1,
                               selector(b"bar").unwrap().1,
                               ],
                           body: vec![
                               SassItem::None,
                               SassItem::Property(Property {
                                   name: "property".into(),
                                   value: Value::Variable("b".into())
                               })],
                       }),
                       SassItem::None,
                       ]}))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MixinDeclaration {
    name: String,
    args: Vec<(String, Option<Value>)>,
    body: Vec<SassItem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Property {
    name: String,
    value: Value,
}

impl Property {
    fn write(&self,
             out: &mut Write,
             scope: &Scope,
             indent: usize)
             -> io::Result<()> {
        try!(do_indent(out, indent));
        write!(out, "{}: {};\n", self.name, scope.evaluate(&self.value))
    }
}

fn do_indent(out: &mut Write, steps: usize) -> io::Result<()> {
    for _i in 0..steps {
        try!(write!(out, " "));
    }
    Ok(())
}

named!(property<&[u8], Property>,
       chain!(multispace? ~
              name: take_while!(is_property_name_char) ~
              multispace? ~
              tag!(":") ~
              multispace? ~
              val: value_expression ~
              multispace? ~
              tag!(";") ~
              spacelike?,
              || Property {
                  name: from_utf8(name).unwrap().into(),
                  value: val,
              }));

fn is_property_name_char(chr: u8) -> bool {
    use nom::is_alphanumeric;
    is_alphanumeric(chr) || chr == b'-'
}

#[test]
fn test_simple_property() {
    assert_eq!(property(b"color: red;\n"),
               Done(&b""[..], Property {
                   name: "color".to_string(),
                   value: Value::Literal("red".to_string()),
               }));
}
#[test]
fn test_property_2() {
    assert_eq!(property(b"background-position: 90% 50%;\n"),
               Done(&b""[..], Property {
                   name: "background-position".to_string(),
                   value: Value::MultiSpace(vec![percentage(90),
                                                 percentage(50)]),
               }));
}

#[cfg(test)]
fn percentage(v: isize) -> Value {
    use num_rational::Rational;
    use valueexpression::Unit;
    Value::Numeric(Rational::from_integer(v), Unit::Percent, false)
}

named!(variable_declaration<&[u8], (&str, Value)>,
       chain!(tag!("$") ~
              name: alphanumeric ~
              multispace? ~
              tag!(":") ~
              multispace? ~
              val: value_expression ~
              multispace? ~
              tag!(";") ~
              multispace?,
              || (from_utf8(name).unwrap(), val)));

#[test]
fn test_simple_variable_declaration() {
    assert_eq!(variable_declaration(b"$foo: bar;\n"),
               Done(&b""[..], ("foo", Value::Literal("bar".into()))))
}

named!(comment,
       delimited!(tag!("/*"), is_not!("*"), tag!("*/")));
