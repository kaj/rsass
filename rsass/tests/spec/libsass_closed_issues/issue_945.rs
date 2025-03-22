//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_945.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_945")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            ".a {\
             \n  b: c;\
             \n  d:\
             \n}\n"
        ),
        "Error: Expected expression.\
         \n  ,\
         \n3 |   d:\
         \n  |     ^\
         \n  \'\
         \n  input.scss 3:5  root stylesheet",
    );
}
