//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1822.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1822")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            ".btn {\
             \n    .open& {\
             \n        color: #000;\
             \n    }\
             \n}\n"
        ),
        "Error: \"&\" may only used at the beginning of a compound selector.\
         \n  ,\
         \n2 |     .open& {\
         \n  |          ^\
         \n  \'\
         \n  input.scss 2:10  root stylesheet",
    );
}
