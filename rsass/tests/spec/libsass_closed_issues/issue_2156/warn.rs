//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2156/warn.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("warn")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \n@warn \"\\\"foo\\\"\" + \"\";\r\
             \n@warn \"\" + \"\\\"foo\\\"\";\r\
             \n@warn \"\" + \"\\\"foo\";\r\
             \n@warn \"\\\"foo\\\"\" + \"bar\";\r\
             \n@warn string.unquote(\"\\\"foo\\\" and \\\"bar\\\"\");\r\n"),
        ""
    );
}
