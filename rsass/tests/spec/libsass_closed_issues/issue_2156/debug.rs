//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2156/debug.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("debug")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \n@debug \"\\\"foo\\\"\" + \"\";\r\
             \n@debug \"\" + \"\\\"foo\\\"\";\r\
             \n@debug \"\" + \"\\\"foo\";\r\
             \n@debug \"\\\"foo\\\"\" + \"bar\";\r\
             \n@debug string.unquote(\"\\\"foo\\\" and \\\"bar\\\"\");\r\n"),
        ""
    );
}
