//! Tests auto-converted from "sass-spec/spec/non_conformant/operations/division.hrx"

mod slash {
    mod with_string {
        #[test]
        fn slash_minus_string() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
        fn slash_slash_string() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
