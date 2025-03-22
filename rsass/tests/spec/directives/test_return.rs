//! Tests auto-converted from "sass-spec/spec/directives/return.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("return")
}

mod before_value {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@function a() {@return \
             \n  b}\n"),
            ""
        );
    }
}
