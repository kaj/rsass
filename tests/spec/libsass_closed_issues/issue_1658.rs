//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1658.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err("@else{}\n"),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n1 | @else{}\
         \n  | ^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
