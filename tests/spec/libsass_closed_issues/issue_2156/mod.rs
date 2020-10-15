//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2156"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_2156/debug.hrx"
#[test]
fn debug() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/libsass-closed-issues/issue_2156/error.hrx"

// Ignoring "error", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_2156/warn.hrx"
#[test]
fn warn() {
    assert_eq!(
        rsass(
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
