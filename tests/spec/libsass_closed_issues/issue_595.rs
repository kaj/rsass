//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_595.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n    color: red;\
            \n};\
            \n"
        )
        .unwrap(),
        "a {\
        \n  color: red;\
        \n}\
        \n"
    );
}
