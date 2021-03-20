//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243/at-rule.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@foo bar\
            \n"
        )
        .unwrap(),
        "@foo bar;\
        \n"
    );
}
