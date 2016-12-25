use spacelike::spacelike;

named!(pub selector<&[u8], Vec<u8> >,
       chain!(v: many1!(terminated!(selector_part,
                                    opt!(spacelike))),
              || v.join(&b' ')
       ));

named!(selector_part,
       alt!(take_while1!(is_foo_char) | tag!(">") | tag!("+") | tag!("~")));

fn is_foo_char(chr: u8) -> bool {
    use nom::is_alphanumeric;
    is_alphanumeric(chr) || chr == b'_'
}

#[cfg(test)]
mod test {
    use nom::IResult::*;
    use selectors::selector;

    #[test]
    fn simple_selector() {
        assert_eq!(selector(b"foo "),
                   Done(&b""[..], b"foo"[..].into()));
    }

    #[test]
    fn selector2() {
        assert_eq!(selector(b"foo bar "),
                   Done(&b""[..], b"foo bar"[..].into()));
    }

    #[test]
    fn child_selector() {
        assert_eq!(selector(b"foo > bar "),
                   Done(&b""[..], b"foo > bar"[..].into()));
    }
}
