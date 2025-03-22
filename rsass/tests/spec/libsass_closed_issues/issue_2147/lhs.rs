//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2147/lhs.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lhs")
}

#[test]
fn test() {
    assert_eq!(
        runner().err("$map: (a:b,c:d) + 1;\r\n"),
        "Error: (a: b, c: d) isn\'t a valid CSS value.\
         \n  ,\
         \n1 | $map: (a:b,c:d) + 1;\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
