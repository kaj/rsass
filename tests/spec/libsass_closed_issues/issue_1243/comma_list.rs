//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1243/comma-list.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$a: 1, 2\
            \n"
        )
        .unwrap(),
        ""
    );
}
