//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1720.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
