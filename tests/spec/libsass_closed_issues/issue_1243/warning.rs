//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243/warning.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@warning \"foo\"\
            \n"
        )
        .unwrap(),
        "@warning \"foo\";\
        \n"
    );
}
