//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2147/rhs.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rhs")
}

#[test]
fn test() {
    assert_eq!(
        runner().err("$map: 1 + (a:b,c:d);\r\n"),
        "Error: (a: b, c: d) isn\'t a valid CSS value.\
         \n  ,\
         \n1 | $map: 1 + (a:b,c:d);\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
