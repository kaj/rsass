use nom::bytes::complete::{is_not, tag};
use nom::character::complete::multispace1;
use nom::combinator::opt;
use nom::sequence::terminated;
use nom::*;

named!(pub spacelike<()>,
       fold_many1!(alt!(ignore_space |ignore_lcomment),
                   (),
                   |(), ()| ()));
named!(pub spacelike2<()>,
       terminated!(spacelike,
                   ignore_comments));

named!(
    pub opt_spacelike<&[u8], ()>,
    map!(many0!(alt!(ignore_space | ignore_lcomment)), |_| ())
    /*
    fold_many0!(alt!(ignore_space | ignore_lcomment),
                (),
                |(), ()| ())
     */
);

named!(
    pub ignore_comments<&[u8], ()>,
    map!(many0!(alt!(ignore_space |
                     ignore_lcomment |
                     map!(comment, |_| ()))),
         |_| ())
    /*
    fold_many0!(alt!(ignore_space |
                     ignore_lcomment |
                     map!(comment, |_| ())),
                (),
                |(), ()| ())
     */
);

named!(pub comment<&[u8], &[u8]>,
       preceded!(tag("/*"), comment2)
);
named!(pub comment2<&[u8]>,
       terminated!(
           recognize!(many0!(alt!(
               value!((), is_not!("*")) |
               preceded!(tag("*"), not!(tag("/")))
           ))),
           tag("*/")));

named!(pub ignore_space<&[u8], ()>, map!(multispace1, |_: &[u8]|()));

named!(
    ignore_lcomment<&[u8], ()>,
    map!(terminated(tag("//"), opt(is_not("\n"))), |_| ())
);

#[cfg(test)]
mod test {
    use super::comment;

    #[test]
    fn comment_simple() {
        assert_eq!(
            comment(b"/* hello */\n"),
            Ok((&b"\n"[..], &b" hello "[..])),
        )
    }

    #[test]
    fn comment_with_stars() {
        assert_eq!(
            comment(b"/**** hello ****/\n"),
            Ok((&b"\n"[..], &b"*** hello ***"[..])),
        )
    }

    #[test]
    fn comment_with_stars2() {
        assert_eq!(
            comment(b"/* / * / * / * hello * \\ * \\ * \\ */\n"),
            Ok((&b"\n"[..], &b" / * / * / * hello * \\ * \\ * \\ "[..])),
        )
    }
}
