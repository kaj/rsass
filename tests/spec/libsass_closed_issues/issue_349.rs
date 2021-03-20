//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_349.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  blah: not true;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  blah: false;\
        \n}\
        \n"
    );
}
