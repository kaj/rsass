use nom::{is_alphanumeric, multispace};
use std::str::from_utf8;

named!(pub spacelike<()>,
       fold_many1!(alt_complete!(ignore_space |ignore_lcomment),
                   (),
                   |(), ()| ()));
named!(pub spacelike2<()>,
       terminated!(spacelike,
                   ignore_comments));

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
       map!(verify!(take_while1!(is_name_char),
                    |n| n != b"-"),
            |n| from_utf8(n).unwrap().into()));

pub fn is_name_char(c: u8) -> bool {
    is_alphanumeric(c) || c == b'_' || c == b'-'
}

named!(pub comment,
       delimited!(tag!("/*"),
                  recognize!(many0!(alt_complete!(
                      is_not!("*") |
                      preceded!(tag!("*"), not!(tag!("/")))))),
                  tag!("*/")));

named!(pub ignore_space<()>, map!(multispace, |_|()));
named!(
    ignore_lcomment<()>,
    do_parse!(tag!("//") >> opt!(is_not!("\n")) >> ())
);

#[cfg(test)]
mod test {
    use super::comment;
    use nom::IResult::*;

    #[test]
    fn comment_simple() {
        assert_eq!(comment(b"/* hello */\n"), Done(&b"\n"[..], &b" hello "[..]))
    }

    #[test]
    fn comment_with_stars() {
        assert_eq!(
            comment(b"/**** hello ****/\n"),
            Done(&b"\n"[..], &b"*** hello ***"[..])
        )
    }

    #[test]
    fn comment_with_stars2() {
        assert_eq!(
            comment(b"/* / * / * / * hello * \\ * \\ * \\ */\n"),
            Done(&b"\n"[..], &b" / * / * / * hello * \\ * \\ * \\ "[..])
        )
    }
}
