//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2123"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_2123/test-01.hrx"
#[test]
fn test_01() {
    assert_eq!(
        rsass(".class{background-image:url(//foo.bar/favicon.ico)}").unwrap(),
        ".class {\
        \n  background-image: url(//foo.bar/favicon.ico);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2123/test-02.hrx"
#[test]
fn test_02() {
    assert_eq!(
        rsass(
            "@font-face { font-family:\'foo\';src:url(//foo.bar) format(\'woff2\'); }"
        )
        .unwrap(),
        "@font-face {\
        \n  font-family: \"foo\";\
        \n  src: url(//foo.bar) format(\"woff2\");\
        \n}\
        \n"
    );
}
