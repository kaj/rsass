//! Tests auto-converted from "sass-spec/spec/css/functions/not_special.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("not_special")
}

mod prefixed {
    use super::runner;

    mod lowercase {
        use super::runner;

        #[test]
        fn and() {
            assert_eq!(
                runner().ok("a {b: -c-and(1 + 1)}\n"),
                "a {\
         \n  b: -c-and(2);\
         \n}\n"
            );
        }
        #[test]
        fn not() {
            assert_eq!(
                runner().ok("a {b: -c-not(1 + 1)}\n"),
                "a {\
         \n  b: -c-not(2);\
         \n}\n"
            );
        }
        #[test]
        fn or() {
            assert_eq!(
                runner().ok("a {b: -c-or(1 + 1)}\n"),
                "a {\
         \n  b: -c-or(2);\
         \n}\n"
            );
        }
        #[test]
        fn test_type() {
            assert_eq!(
                runner().ok("a {b: -c-type(1 + 1)}\n"),
                "a {\
         \n  b: -c-type(2);\
         \n}\n"
            );
        }
    }
    mod uppercase {
        use super::runner;

        #[test]
        fn and() {
            assert_eq!(
                runner().ok("a {b: -C-AND(1 + 1)}\n"),
                "a {\
         \n  b: -C-AND(2);\
         \n}\n"
            );
        }
        #[test]
        fn not() {
            assert_eq!(
                runner().ok("a {b: -C-NOT(1 + 1)}\n"),
                "a {\
         \n  b: -C-NOT(2);\
         \n}\n"
            );
        }
        #[test]
        fn or() {
            assert_eq!(
                runner().ok("a {b: -C-OR(1 + 1)}\n"),
                "a {\
         \n  b: -C-OR(2);\
         \n}\n"
            );
        }
        #[test]
        fn test_type() {
            assert_eq!(
                runner().ok("a {b: -C-TYPE(1 + 1)}\n"),
                "a {\
         \n  b: -C-TYPE(2);\
         \n}\n"
            );
        }
    }
}
