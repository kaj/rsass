//! Tests auto-converted from "sass-spec/spec/parser/selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn empty_placeholder() {
        assert_eq!(
            runner().err(
                "% {\
             \n  a: b;\
             \n}\n"
            ),
            "Error: Expected identifier.\
         \n  ,\
         \n1 | %{\
         \n  |  ^\
         \n  \'\
         \n  input.scss 1:2  root stylesheet",
        );
    }
}
#[test]
fn escaped_backslash() {
    assert_eq!(
        runner().ok("// Regression test for sass/dart-sass#1855.\
             \n\\\\{\
             \n  b: c;\
             \n}\n"),
        "\\\\ {\
         \n  b: c;\
         \n}\n"
    );
}
