//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2106/test.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err("@import \"../does-not-exist\";\n"),
        "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n1 | @import \"../does-not-exist\";\
         \n  |         ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet",
    );
}
