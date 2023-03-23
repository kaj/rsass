//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2023/id-selector-id.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("id-selector-id")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "#2b {\r\
             \n    color: red;\r\
             \n}\r\n"
        ),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | #2b {\
         \n  |  ^\
         \n  \'\
         \n  input.scss 1:2  root stylesheet",
    );
}
