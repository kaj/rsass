use super::input_to_string;
use nom::types::CompleteByteSlice as Input;
use nom::{is_alphanumeric, multispace};

named!(pub spacelike<Input, ()>,
       fold_many1!(alt_complete!(ignore_space |ignore_lcomment),
                   (),
                   |(), ()| ()));
named!(pub spacelike2<Input, ()>,
       terminated!(spacelike,
                   ignore_comments));

named!(pub opt_spacelike<Input, ()>,
       fold_many0!(alt_complete!(ignore_space | ignore_lcomment),
                   (),
                   |(), ()| ()));

named!(pub ignore_comments<Input, ()>,
       fold_many0!(alt_complete!(ignore_space |
                                 ignore_lcomment |
                                 map!(comment, |_| ())),
                   (),
                   |(), ()| ()));

named!(pub name<Input, String>,
       map_res!(
           verify!(take_while1!(is_name_char), |n| n != Input(b"-")),
           input_to_string
       )
);

pub fn is_name_char(c: u8) -> bool {
    is_alphanumeric(c) || c == b'_' || c == b'-'
}

named!(pub comment<Input, Input>,
       delimited!(tag!("/*"),
                  recognize!(many0!(alt_complete!(
                      value!((), is_not!("*")) |
                      preceded!(tag!("*"), not!(tag!("/")))
                  ))),
                  tag!("*/")));

named!(pub ignore_space<Input, ()>, map!(multispace, |_|()));
named!(
    ignore_lcomment<Input, ()>,
    do_parse!(tag!("//") >> opt!(is_not!("\n")) >> ())
);

#[cfg(test)]
mod test {
    use super::comment;
    use nom::types::CompleteByteSlice as Input;

    #[test]
    fn comment_simple() {
        assert_eq!(
            comment(Input(b"/* hello */\n")),
            Ok((Input(b"\n"), Input(b" hello ")))
        )
    }

    #[test]
    fn comment_with_stars() {
        assert_eq!(
            comment(Input(b"/**** hello ****/\n")),
            Ok((Input(b"\n"), Input(b"*** hello ***")))
        )
    }

    #[test]
    fn comment_with_stars2() {
        assert_eq!(
            comment(Input(b"/* / * / * / * hello * \\ * \\ * \\ */\n")),
            Ok((Input(b"\n"), Input(b" / * / * / * hello * \\ * \\ * \\ ")))
        )
    }
}
