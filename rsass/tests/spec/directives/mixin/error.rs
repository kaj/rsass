//! Tests auto-converted from "sass-spec/spec/directives/mixin/error.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod duplicate_parameter {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn non_rest() {
        assert_eq!(
            runner().err("@mixin a($b, $b) {}\n"),
            "Error: Duplicate parameter.\
         \n  ,\
         \n1 | @mixin a($b, $b) {}\
         \n  |          ^^ first\
         \n  |              == second\
         \n  \'\
         \n  input.scss 1:10  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn rest() {
        assert_eq!(
            runner().err("@mixin a($b, $b...) {}\n"),
            "Error: Duplicate parameter.\
         \n  ,\
         \n1 | @mixin a($b, $b...) {}\
         \n  |          ^^ first\
         \n  |              ===== second\
         \n  \'\
         \n  input.scss 1:10  root stylesheet",
        );
    }
}
