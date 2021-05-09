//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2023/pseudo-selector-id.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "#4d {\r\
             \n    color: red;\r\
             \n}\r\n"
        ),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | #4d{\
         \n  |  ^\
         \n  \'\
         \n  input.scss 1:2  root stylesheet",
    );
}
