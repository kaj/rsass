//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2156/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:string\";\
             \n@error string.unquote(\"\\\"foo\\\" and \\\"bar\\\"\");\n"
        ),
        "Error: \"foo\" and \"bar\"\
         \n  ,\
         \n2 | @error string.unquote(\"\\\"foo\\\" and \\\"bar\\\"\");\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
    );
}
