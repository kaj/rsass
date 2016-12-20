#[macro_use]
extern crate nom;

use nom::{alphanumeric, multispace};
use nom::IResult::*;
use std::str::from_utf8;
use std::io::{self, Write};

pub fn compile_scss(input: &[u8]) -> Result<Vec<u8>, ()> {
    match sassfile(input) {
        Done(b"", styles) =>  {
            let mut result = Vec::new();
            for rule in styles {
                rule.write(&mut result, None, 0).unwrap();
            }
            Ok(result)
        },
        Done(rest, _styles) => {
            println!("Failed to parse entire input: {:?} remains.", rest);
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

named!(sassfile<&[u8], Vec<Rule> >,
       chain!(multispace? ~
              rules: many0!(rule),
              || rules));

struct Rule {
    selector: String,
    properties: Vec<Property>,
    subrules: Vec<Rule>,
}

impl Rule {
    fn write(&self, out: &mut Write, parent: Option<&str>, indent: usize) -> io::Result<()> {
        let selector = if let Some(parent) = parent {
            format!("{} {}", parent, self.selector)
        } else {
            self.selector.clone()
        };
        if !self.properties.is_empty() {
            try!(write!(out, "{} {{\n", selector));
            for ref p in &self.properties {
                try!(p.write(out, indent + 2));
            }
            try!(write!(out, "}}\n"));
        }
        for ref r in &self.subrules {
            try!(r.write(out, Some(&selector), indent));
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
    value: String,
}

impl Property {
    fn write(&self, out: &mut Write, indent: usize) -> io::Result<()> {
        for _i in 0..indent {
            try!(write!(out, " "));
        }
        write!(out, "{}: {};\n", self.name, self.value)
    }
}

named!(property<&[u8], Property>,
       chain!(multispace? ~
              name: alphanumeric ~
              multispace? ~
              tag!(":") ~
              multispace? ~
              value: is_not!(";") ~
              multispace? ~
              tag!(";") ~
              multispace?,
              || Property {
                  name: from_utf8(name).unwrap().into(),
                  value: from_utf8(value).unwrap().into(),
              }));
