use super::input_to_string;
use nom::multispace;
use nom::types::CompleteByteSlice as Input;

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
           verify!(
               recognize!(
                   fold_many0!(
                       verify!(take_char, is_name_char),
                       (),
                       |_, _| ()
                   )
               ),
               |n| n != Input(b"") && n != Input(b"-")),
           input_to_string
       )
);

named!(
    pub take_char<Input, char>,
    alt!(
        map_opt!(take!(1), single_char) |
        map_opt!(take!(2), single_char) |
        map_opt!(take!(3), single_char) |
        map_opt!(take!(4), single_char) |
        map_opt!(take!(5), single_char)
    )
);

fn single_char(data: Input) -> Option<char> {
    use std::str::from_utf8;
    from_utf8(&data).ok().and_then(|s| s.chars().next())
}

pub fn is_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-'
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
