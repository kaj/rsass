//! Tests auto-converted from "sass-spec/spec/non_conformant/operations"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/operations/division.hrx"
mod division {
    #[allow(unused)]
    use super::rsass;
    mod slash {
        #[allow(unused)]
        use super::rsass;
        mod with_string {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn slash_minus_string() {
                assert_eq!(
                    rsass(
                        "a {b: 1 / 2 - foo()}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: 0.5-foo();\
        \n}\
        \n"
                );
            }
            #[test]
            fn slash_plus_string() {
                assert_eq!(
                    rsass(
                        "a {b: 1 / 2 + foo()}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: 0.5foo();\
        \n}\
        \n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn slash_slash_string() {
                assert_eq!(
                    rsass(
                        "a {b: 1 / 2 / foo()}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: 1/2/foo();\
        \n}\
        \n"
                );
            }
            #[test]
            fn string_minus_slash() {
                assert_eq!(
                    rsass(
                        "a {b: foo() - 1 / 2}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: foo()-0.5;\
        \n}\
        \n"
                );
            }
            #[test]
            fn string_plus_slash() {
                assert_eq!(
                    rsass(
                        "a {b: foo() + 1 / 2}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: foo()0.5;\
        \n}\
        \n"
                );
            }
        }
    }
}
