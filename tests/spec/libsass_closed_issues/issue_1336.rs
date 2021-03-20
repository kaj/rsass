//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1336.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@debug null;\
            \n"
        )
        .unwrap(),
        ""
    );
}
