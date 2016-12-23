#[macro_use]
extern crate nom;

use nom::{alphanumeric, multispace};
use nom::IResult::*;
use std::str::from_utf8;
use std::io::{self, Write};

mod variablescope;
mod valueexpression;
use variablescope::Scope;
use valueexpression::{Value, value_expression};

pub fn compile_scss(input: &[u8]) -> Result<Vec<u8>, ()> {
    match sassfile(input) {
        Done(b"", items) =>  {
            let mut globals = Scope::new();
            let mut result = Vec::new();
            let mut first = true;
            for item in items {
                match item {
                    SassItem::Rule(rule) => {
                        if first {
                            first = false;
                        } else {
                            write!(result, "\n").unwrap();
                        }
                        rule.write(&mut result, &globals, None, 0).unwrap();
                    }
                    SassItem::VariableDeclaration{name, val} => {
                        globals.define(&name, &val);
                    }
                    SassItem::None => ()
                }
            }
            Ok(result)
        },
        Done(rest, _styles) => {
            let t = from_utf8(&rest).map(|s| s.to_string()).unwrap_or_else(|_| format!("{:?}", rest));
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
       many0!(alt!(chain!(multispace, || SassItem::None) |
                   chain!(d: variable_declaration,
                          || SassItem::VariableDeclaration{
                              name: d.0.to_string(),
                              val: d.1.clone(),
                          }) |
                   chain!(r: rule, || SassItem::Rule(r))
                   )));

enum SassItem {
    None,
    Rule(Rule),
    VariableDeclaration { name: String, val: Value },
}

struct Rule {
    selector: String,
    properties: Vec<Property>,
    subrules: Vec<Rule>,
}

impl Rule {
    fn write(&self, out: &mut Write, scope: &Scope, parent: Option<&str>, indent: usize) -> io::Result<()> {
        let selector = if let Some(parent) = parent {
            format!("{} {}", parent, self.selector)
        } else {
            self.selector.clone()
        };
        if !self.properties.is_empty() {
            try!(write!(out, "{} {{\n", selector));
            for ref p in &self.properties {
                try!(p.write(out, scope, indent + 2));
            }
            try!(write!(out, "}}\n"));
        }
        for ref r in &self.subrules {
            try!(r.write(out, scope, Some(&selector), indent));
        }
        Ok(())
    }
}

named!(rule<&[u8], Rule>,
       chain!(multispace? ~
              selector: alphanumeric ~
              multispace? ~
              tag!("{") ~
              multispace? ~
              properties: many0!(property) ~
              subrules: many0!(rule) ~
              multispace? ~
              tag!("}"),
              || Rule {
                  selector: from_utf8(selector).unwrap().into(),
                  properties: properties,
                  subrules: subrules,
              }));

struct Property {
    name: String,
    value: Value,
}

impl Property {
    fn write(&self, out: &mut Write, scope: &Scope, indent: usize) -> io::Result<()> {
        for _i in 0..indent {
            try!(write!(out, " "));
        }
        write!(out, "{}: {};\n", self.name, scope.evaluate(&self.value))
    }
}

named!(property<&[u8], Property>,
       chain!(multispace? ~
              name: alphanumeric ~
              multispace? ~
              tag!(":") ~
              multispace? ~
              val: value_expression ~
              multispace? ~
              tag!(";") ~
              multispace?,
              || Property {
                  name: from_utf8(name).unwrap().into(),
                  value: val,
              }));

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
