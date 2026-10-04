//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/operator/sass_script.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("sass_script")
}

mod plus_string {
    use super::runner;

    #[test]
    fn lhs() {
        assert_eq!(
            runner().ok("a {b: calc(1px + 1%) + \"\"}\n"),
            "a {\
         \n  b: \"calc(1px + 1%)\";\
         \n}\n"
        );
    }
    #[test]
    fn rhs() {
        assert_eq!(
            runner().ok("a {b: \"\" + calc(1px + 1%)}\n"),
            "a {\
         \n  b: \"calc(1px + 1%)\";\
         \n}\n"
        );
    }
}
