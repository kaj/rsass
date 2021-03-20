//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1257.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  color: invert(red...);\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  color: aqua;\
        \n}\
        \n"
    );
}
