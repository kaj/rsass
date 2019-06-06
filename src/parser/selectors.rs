use super::input_to_string;
use super::strings::{sass_string, sass_string_dq, sass_string_sq};
use super::util::{opt_spacelike, spacelike2};
use crate::selectors::{Selector, SelectorPart, Selectors};
use nom::types::CompleteByteSlice as Input;
use nom::*;

named!(pub selectors<Input, Selectors>,
       map!(separated_nonempty_list!(
           complete!(do_parse!(tag!(",") >> opt!(is_a!(", \t\n")) >> ())),
           selector),
            Selectors::new));

named!(pub selector<Input, Selector>,
       map!(many1!(selector_part),
            |s: Vec<SelectorPart>| {
                let mut s = s;
                if s.last() == Some(&SelectorPart::Descendant) {
                    s.pop();
                }
                Selector(s)
            }));

named!(selector_part<Input, SelectorPart>,
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
                     op: map_res!(
                         alt_complete!(tag!("*=") | tag!("|=") | tag!("=")),
                         input_to_string
                     ) >>
                     opt_spacelike >>
                     val: alt_complete!(sass_string_dq | sass_string_sq |
                                        sass_string) >>
                     opt_spacelike >>
                     modifier: opt!(terminated!(
                         one_of!("ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                  abcdefghijklmnopqrstuvwxyz"),
                         opt_spacelike)) >>
                     tag!("]") >>
                     (SelectorPart::Attribute {
                         name,
                         op,
                         val,
                         modifier
                     })) |
           do_parse!(tag!("[") >> opt_spacelike >>
                     name: sass_string >> opt_spacelike >>
                     tag!("]") >>
                     (SelectorPart::Attribute {
                         name,
                         op: "".to_string(),
                         val: "".into(),
                         modifier: None,
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
    use crate::sass::{SassString, StringPart};
    use crate::value::Quotes;

    #[test]
    fn simple_selector() {
        assert_eq!(
            selector(Input(b"foo ")),
            Ok((
                Input(b""),
                Selector(vec![SelectorPart::Simple("foo".into())])
            ))
        )
    }
    #[test]
    fn escaped_simple_selector() {
        assert_eq!(
            selector(Input(b"\\E9m ")),
            Ok((
                Input(b""),
                Selector(vec![SelectorPart::Simple("ém".into())])
            ))
        )
    }

    #[test]
    fn selector2() {
        assert_eq!(
            selector(Input(b"foo bar ")),
            Ok((
                Input(b""),
                Selector(vec![
                    SelectorPart::Simple("foo".into()),
                    SelectorPart::Descendant,
                    SelectorPart::Simple("bar".into()),
                ])
            ))
        )
    }

    #[test]
    fn child_selector() {
        assert_eq!(
            selector(Input(b"foo > bar ")),
            Ok((
                Input(b""),
                Selector(vec![
                    SelectorPart::Simple("foo".into()),
                    SelectorPart::RelOp(b'>'),
                    SelectorPart::Simple("bar".into()),
                ])
            ))
        )
    }

    #[test]
    fn foo1_selector() {
        assert_eq!(
            selector(Input(b"[data-icon='test-1'] ")),
            Ok((
                Input(b""),
                Selector(vec![SelectorPart::Attribute {
                    name: "data-icon".into(),
                    op: "=".into(),
                    val: SassString::new(
                        vec![StringPart::Raw("test-1".into())],
                        Quotes::Single,
                    ),
                    modifier: None,
                }])
            ))
        )
    }

    #[test]
    fn pseudo_selector() {
        assert_eq!(
            selector(Input(b":before ")),
            Ok((
                Input(b""),
                Selector(vec![SelectorPart::Pseudo {
                    name: "before".into(),
                    arg: None,
                }])
            ))
        )
    }
    #[test]
    fn pseudo_on_simple_selector() {
        assert_eq!(
            selector(Input(b"figure:before ")),
            Ok((
                Input(b""),
                Selector(vec![
                    SelectorPart::Simple("figure".into()),
                    SelectorPart::Pseudo {
                        name: "before".into(),
                        arg: None,
                    },
                ])
            ))
        )
    }

    #[test]
    fn selectors_simple() {
        assert_eq!(
            selectors(Input(b"foo, bar ")),
            Ok((
                Input(b""),
                Selectors::new(vec![
                    Selector(vec![SelectorPart::Simple("foo".into())]),
                    Selector(vec![SelectorPart::Simple("bar".into())]),
                ])
            ))
        )
    }

}
