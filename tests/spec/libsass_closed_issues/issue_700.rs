//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_700.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".selector {\
            \n  color: invert(transparent);\
            \n}"
        )
        .unwrap(),
        ".selector {\
        \n  color: rgba(255, 255, 255, 0);\
        \n}\
        \n"
    );
}
