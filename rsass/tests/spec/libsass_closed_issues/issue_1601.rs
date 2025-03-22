//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1601.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1601")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            ".code.ruby > {\r\
             \n    &.ruby {\r\
             \n        color: green;\r\
             \n    }\r\
             \n}"
        ),
        "Error: Selector \".code.ruby >\" can\'t be used as a parent in a compound selector.\
         \n  ,\
         \n1 | .code.ruby > {\
         \n  | ^^^^^^^^^^^^ outer selector\
         \n2 |     &.ruby {\
         \n  |     = parent selector\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
