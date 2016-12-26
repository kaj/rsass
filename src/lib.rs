#[macro_use]
extern crate nom;

use nom::{alphanumeric, multispace};
use nom::IResult::*;
use std::str::from_utf8;
use std::io::{self, Write};

mod selectors;
mod spacelike;
mod valueexpression;
mod variablescope;
use spacelike::spacelike;
use selectors::{Selector, selector};
use variablescope::Scope;
use valueexpression::{Value, value_expression};

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
                   chain!(r: rule, || SassItem::Rule(r)) |
                   chain!(c: comment, || {
                       SassItem::Comment(from_utf8(c).unwrap().into())
                   }
                   ))));

enum SassItem {
    None,
    Comment(String),
    Property(Property),
    Rule(Rule),
    VariableDeclaration { name: String, val: Value },
}

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
        let mut scope = Scope::sub(scope);
        let mut direct = Vec::new();
        let mut sub = Vec::new();

        for b in &self.body {
            match b {
                &SassItem::Comment(ref c) => {
                    try!(do_indent(&mut direct, indent + 2));
                    try!(writeln!(&mut direct, "/*{}*/", c));
                }
                &SassItem::Property(ref p) => {
                    try!(p.write(&mut direct, &scope, indent + 2));
                }
                &SassItem::Rule(ref r) => {
                    try!(r.write(&mut sub, &scope, Some(&selectors), indent));
                }
                &SassItem::VariableDeclaration { ref name, ref val } => {
                    scope.define(&name, &val);
                }
                &SassItem::None => (),
            }
        }
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


named!(rule<&[u8], Rule>,
       chain!(spacelike? ~
              selectors: separated_nonempty_list!(chain!(tag!(",") ~ spacelike?,
                                                         || ()),
                                                  selector) ~
              spacelike? ~
              tag!("{") ~
              body: many0!(alt!(
                  chain!(spacelike, || SassItem::None) |
                  chain!(d: variable_declaration,
                         || SassItem::VariableDeclaration{
                             name: d.0.to_string(),
                             val: d.1.clone(),
                         }) |
                  chain!(r: rule, || SassItem::Rule(r)) |
                  chain!(p: property, || SassItem::Property(p)) |
                  chain!(c: comment, || {
                      SassItem::Comment(from_utf8(c).unwrap().into())
                  })
                      )) ~
              tag!("}"),
              || Rule {
                  selectors: selectors,
                  body: body,
              }));

#[derive(Clone, Debug, PartialEq, Eq)]
struct Property {
    name: String,
    value: Value,
}

impl Property {
    fn write(&self, out: &mut Write, scope: &Scope, indent: usize) -> io::Result<()> {
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
                   value: Value::Multi(vec![Value::Literal("90%".to_string()),
                                            Value::Literal("50%".to_string())]),
               }));
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
