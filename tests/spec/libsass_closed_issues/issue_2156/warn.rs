//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2156/warn.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@warn \"\\\"foo\\\"\" + \"\";\r\
            \n@warn \"\" + \"\\\"foo\\\"\";\r\
            \n@warn \"\" + \"\\\"foo\";\r\
            \n@warn \"\\\"foo\\\"\" + \"bar\";\r\
            \n@warn unquote(\"\\\"foo\\\" and \\\"bar\\\"\");\r\
            \n"
        )
        .unwrap(),
        ""
    );
}
