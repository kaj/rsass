use parser::strings::{sass_string, sass_string_dq, sass_string_sq};
use parser::util::{opt_spacelike, spacelike2};
use selectors::{Selector, SelectorPart, Selectors};
use std::str::from_utf8;

named!(pub selectors<Selectors>,
       map!(separated_nonempty_list!(
           complete!(do_parse!(tag!(",") >> opt!(is_a!(", \t\n")) >> ())),
           selector),
            Selectors));

named!(pub selector<Selector>,
       map!(many1!(selector_part),
            |s: Vec<SelectorPart>| {
                let mut s = s;
                if s.last() == Some(&SelectorPart::Descendant) {
                    s.pop();
                }
                Selector(s)
            }));

named!(selector_part<&[u8], SelectorPart>,
       alt_complete!(
           map!(sass_string, SelectorPart::Simple) |
           value!(SelectorPart::Simple("*".into()), tag!("*")) |
           do_parse!(tag!("::") >>
                     name: sass_string >>
                     arg: opt!(delimited!(tag!("("), selectors,
                                          tag!(")"))) >>
                     (SelectorPart::PseudoElement {
                         name,
                         arg,
                     })) |
           do_parse!(tag!(":") >>
                     name: sass_string >>
                     arg: opt!(delimited!(tag!("("), selectors,
                                          tag!(")"))) >>
                     (SelectorPart::Pseudo {
                         name,
                         arg,
                     })) |
           do_parse!(tag!("[") >> opt_spacelike >>
                     name: sass_string >> opt_spacelike >>
                     op: alt_complete!(tag!("*=") | tag!("|=") | tag!("=")) >>
                     opt_spacelike >>
                     val: alt_complete!(sass_string_dq | sass_string_sq |
                                        sass_string) >>
                     opt_spacelike >>
                     tag!("]") >>
                     (SelectorPart::Attribute {
                         name,
                         op: from_utf8(op).unwrap().into(),
                         val,
                     })) |
           do_parse!(tag!("[") >> opt_spacelike >>
                     name: sass_string >> opt_spacelike >>
                     tag!("]") >>
                     (SelectorPart::Attribute {
                         name,
                         op: "".to_string(),
                         val: "".into(),
                     })) |
           value!(SelectorPart::BackRef, tag!("&")) |
           delimited!(opt_spacelike,
                      alt!(value!(SelectorPart::RelOp(b'>'), tag!(">")) |
                           value!(SelectorPart::RelOp(b'+'), tag!("+")) |
                           value!(SelectorPart::RelOp(b'~'), tag!("~")) |
                           value!(SelectorPart::RelOp(b'\\'), tag!("\\"))),
                      opt_spacelike) |
           value!(SelectorPart::Descendant, spacelike2)
           ));

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::*;
    use sass::{SassString, StringPart};
    use value::Quotes;

    #[test]
    fn simple_selector() {
        assert_eq!(
            selector(b"foo "),
            Done(
                &b""[..],
                Selector(vec![SelectorPart::Simple("foo".into())])
            )
        )
    }
    #[test]
    fn escaped_simple_selector() {
        assert_eq!(
            selector(b"\\E9m "),
            Done(
                &b""[..],
                Selector(vec![SelectorPart::Simple("\\E9m".into())])
            )
        )
    }

    #[test]
    fn selector2() {
        assert_eq!(
            selector(b"foo bar "),
            Done(
                &b""[..],
                Selector(vec![
                    SelectorPart::Simple("foo".into()),
                    SelectorPart::Descendant,
                    SelectorPart::Simple("bar".into()),
                ])
            )
        )
    }

    #[test]
    fn child_selector() {
        assert_eq!(
            selector(b"foo > bar "),
            Done(
                &b""[..],
                Selector(vec![
                    SelectorPart::Simple("foo".into()),
                    SelectorPart::RelOp(b'>'),
                    SelectorPart::Simple("bar".into()),
                ])
            )
        )
    }

    #[test]
    fn foo1_selector() {
        assert_eq!(
            selector(b"[data-icon='test-1'] "),
            Done(
                &b""[..],
                Selector(vec![SelectorPart::Attribute {
                    name: "data-icon".into(),
                    op: "=".into(),
                    val: SassString::new(
                        vec![StringPart::Raw("test-1".into())],
                        Quotes::Single,
                    ),
                }])
            )
        )
    }

    #[test]
    fn pseudo_selector() {
        assert_eq!(
            selector(b":before "),
            Done(
                &b""[..],
                Selector(vec![SelectorPart::Pseudo {
                    name: "before".into(),
                    arg: None,
                }])
            )
        )
    }
    #[test]
    fn pseudo_on_simple_selector() {
        assert_eq!(
            selector(b"figure:before "),
            Done(
                &b""[..],
                Selector(vec![
                    SelectorPart::Simple("figure".into()),
                    SelectorPart::Pseudo {
                        name: "before".into(),
                        arg: None,
                    },
                ])
            )
        )
    }

    #[test]
    fn selectors_simple() {
        let foo = Selector(vec![SelectorPart::Simple("foo".into())]);
        let bar = Selector(vec![SelectorPart::Simple("bar".into())]);
        assert_eq!(
            selectors(b"foo, bar "),
            Done(&b""[..], Selectors(vec![foo, bar]))
        )
    }

}
