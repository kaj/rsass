//! Tests auto-converted from "sass-spec/spec/css/custom_properties/syntax.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("syntax")
}

mod double_dash {
    use super::runner;

    #[test]
    fn declare() {
        assert_eq!(
            runner().ok("a {--: b}\n"),
            "a {\
         \n  --: b;\
         \n}\n"
        );
    }
    #[test]
    fn test_use() {
        assert_eq!(
            runner().ok("a {b: var(--)}\n"),
            "a {\
         \n  b: var(--);\
         \n}\n"
        );
    }
}
mod initial_digit {
    use super::runner;

    #[test]
    fn declare() {
        assert_eq!(
            runner().ok("a {--1: b}\n"),
            "a {\
         \n  --1: b;\
         \n}\n"
        );
    }
    #[test]
    fn test_use() {
        assert_eq!(
            runner().ok("a {b: var(--1)}\n"),
            "a {\
         \n  b: var(--1);\
         \n}\n"
        );
    }
}
mod triple_dash {
    use super::runner;

    #[test]
    fn declare() {
        assert_eq!(
            runner().ok("a {---: b}\n"),
            "a {\
         \n  ---: b;\
         \n}\n"
        );
    }
    #[test]
    fn test_use() {
        assert_eq!(
            runner().ok("a {b: var(---)}\n"),
            "a {\
         \n  b: var(---);\
         \n}\n"
        );
    }
}
