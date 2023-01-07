//! Tests auto-converted from "sass-spec/spec/non_conformant/operations/division.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("division")
}

mod slash {
    #[allow(unused)]
    use super::runner;

    mod with_string {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn slash_minus_string() {
            assert_eq!(
                runner().ok("a {b: 1 / 2 - foo()}\n"),
                "a {\
         \n  b: 0.5-foo();\
         \n}\n"
            );
        }
        #[test]
        fn slash_plus_string() {
            assert_eq!(
                runner().ok("a {b: 1 / 2 + foo()}\n"),
                "a {\
         \n  b: 0.5foo();\
         \n}\n"
            );
        }
        #[test]
        fn slash_slash_string() {
            assert_eq!(
                runner().ok("a {b: 1 / 2 / foo()}\n"),
                "a {\
         \n  b: 1/2/foo();\
         \n}\n"
            );
        }
        #[test]
        fn string_minus_slash() {
            assert_eq!(
                runner().ok("a {b: foo() - 1 / 2}\n"),
                "a {\
         \n  b: foo()-0.5;\
         \n}\n"
            );
        }
        #[test]
        fn string_plus_slash() {
            assert_eq!(
                runner().ok("a {b: foo() + 1 / 2}\n"),
                "a {\
         \n  b: foo()0.5;\
         \n}\n"
            );
        }
    }
}
