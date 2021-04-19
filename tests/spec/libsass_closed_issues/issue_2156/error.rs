//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2156/error.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@error unquote(\"\\\"foo\\\" and \\\"bar\\\"\");\r\
             \n"
        )
        .unwrap_err(),
        "Error: \"foo\" and \"bar\"\
         \n  ,\
         \n1 | @error unquote(\"\\\"foo\\\" and \\\"bar\\\"\");\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
    );
}
