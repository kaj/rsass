//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1093/assignment.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("assignment")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("$foo: #{};\n"),
        "Error: Expected expression.\
         \n  ,\
         \n1 | $foo: #{};\
         \n  |       ^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
