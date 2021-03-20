//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243/import.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"http://foo\"\
            \n"
        )
        .unwrap(),
        "@import \"http://foo\";\
        \n"
    );
}
