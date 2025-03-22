//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2143.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2143")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("$map:;"),
        "Error: Expected expression.\
         \n  ,\
         \n1 | $map:;\
         \n  |      ^\
         \n  \'\
         \n  input.scss 1:6  root stylesheet",
    );
}
