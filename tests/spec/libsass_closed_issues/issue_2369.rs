//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2369.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@a(#{url(\\#{})}\n"),
        "Error: Expected expression.\
         \n  ,\
         \n1 | @a(#{url(\\#{})}\
         \n  |           ^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
    );
}
