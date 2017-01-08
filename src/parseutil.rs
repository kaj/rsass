use nom::{is_alphanumeric, multispace};
use std::str::from_utf8;

named!(pub spacelike<()>,
       fold_many1!(alt_complete!(ignore_space |ignore_lcomment),
                   (),
                   |(), ()| ()));
named!(pub opt_spacelike<()>,
       fold_many0!(alt_complete!(ignore_space | ignore_lcomment),
                   (),
                   |(), ()| ()));

named!(ignore_space<()>, chain!(multispace, ||()));
named!(ignore_lcomment<()>, chain!(tag!("//") ~ c: is_not!("\n"), || ()));

named!(pub ignore_comments<()>,
       fold_many0!(alt_complete!(chain!(multispace, || ()) |
                                 chain!(tag!("//") ~ c: is_not!("\n"), || ()) |
                                 chain!(comment, || ())),
                   (),
                   |(), ()| ()));

named!(pub name<String>,
       chain!(n: take_while1!(is_name_char),
              || from_utf8(n).unwrap().into()));

fn is_name_char(c: u8) -> bool {
    is_alphanumeric(c) || c == b'_' || c == b'-'
}

named!(pub comment,
       delimited!(tag!("/*"), is_not!("*"), tag!("*/")));
