//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2156/debug.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@debug \"\\\"foo\\\"\" + \"\";\r\
            \n@debug \"\" + \"\\\"foo\\\"\";\r\
            \n@debug \"\" + \"\\\"foo\";\r\
            \n@debug \"\\\"foo\\\"\" + \"bar\";\r\
            \n@debug unquote(\"\\\"foo\\\" and \\\"bar\\\"\");\r\
            \n"
        )
        .unwrap(),
        ""
    );
}
