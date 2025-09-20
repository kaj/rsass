//! Tests auto-converted from "sass-spec/spec/css/functions/not_special.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("not_special")
}

mod prefixed {
    use super::runner;

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
