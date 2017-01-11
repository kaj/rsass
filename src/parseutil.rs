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

named!(pub ignore_comments<()>,
       fold_many0!(alt_complete!(ignore_space |
                                 ignore_lcomment |
                                 map!(comment, |_| ())),
                   (),
                   |(), ()| ()));

named!(pub name<String>,
       map!(take_while1!(is_name_char), |n| from_utf8(n).unwrap().into()));

fn is_name_char(c: u8) -> bool {
    is_alphanumeric(c) || c == b'_' || c == b'-'
}

named!(pub comment,
       delimited!(tag!("/*"), is_not!("*"), tag!("*/")));

named!(ignore_space<()>, map!(multispace, |_|()));
named!(ignore_lcomment<()>, do_parse!(tag!("//") >> c: is_not!("\n") >> ()));
