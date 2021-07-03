//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2156/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().err("@error unquote(\"\\\"foo\\\" and \\\"bar\\\"\");\r\n"),
        "Error: \"foo\" and \"bar\"\
         \n  ,\
         \n1 | @error unquote(\"\\\"foo\\\" and \\\"bar\\\"\");\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
