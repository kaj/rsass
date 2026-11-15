//! Tests auto-converted from "sass-spec/spec/values/numbers/units/syntax.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("syntax")
}

mod exponent_like {
    use super::runner;

    #[test]
    fn after_exponent() {
        assert_eq!(
            runner().ok("a {b: 1e1e1}\n"),
            "a {\
         \n  b: 10\\65 1;\
         \n}\n"
        );
    }
    #[test]
    fn positive() {
        assert_eq!(
            runner().ok("a {b: 1\\65 1}\n"),
            "a {\
         \n  b: 1\\65 1;\
         \n}\n"
        );
    }
}
