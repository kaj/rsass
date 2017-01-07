use nom::{is_alphanumeric, multispace};
use std::str::from_utf8;

named!(pub spacelike<&[u8], Vec<&[u8]> >,
       many1!(alt!(multispace |
                   chain!(tag!("//") ~ c: is_not!("\n"), || c))));

named!(pub name<String>,
       chain!(n: take_while1!(is_name_char),
              || from_utf8(n).unwrap().into()));

fn is_name_char(c: u8) -> bool {
    is_alphanumeric(c) || c == b'_' || c == b'-'
}
