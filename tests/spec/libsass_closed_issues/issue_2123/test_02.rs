//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2123/test-02.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
