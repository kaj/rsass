//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2371.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("#{a==b}\n"),
        "Error: expected \"{\".\
         \n  ,\
         \n1 | #{a==b}\
         \n  |        ^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet",
    );
}
