//! Tests auto-converted from "sass-spec/spec/directives/function/whitespace.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("whitespace")
}

mod before_name {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@function\
             \n  a() {}\n"),
            ""
        );
    }
}
mod nested_at_rule {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@function a() {@debug \
             \n  b;\
             \n  @return null}\n"),
            ""
        );
    }
}
