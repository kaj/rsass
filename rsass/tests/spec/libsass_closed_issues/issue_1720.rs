//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1720.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1720")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            ".test {\
             \n    a: a#{b//}c;\
             \n}\n"
        ),
        "Error: expected \"}\".\
         \n  ,\
         \n3 | }\
         \n  |  ^\
         \n  \'\
         \n  input.scss 3:2  root stylesheet",
    );
}
