//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1732/valid/mixin-def.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin a {\
            \n  b: c;\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}
