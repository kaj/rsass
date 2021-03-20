//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243/debug.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@debug(\"foo\")\
            \n"
        )
        .unwrap(),
        ""
    );
}
