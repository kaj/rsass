//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1167.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: 3s + 101ms;\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  b: 3.101s;\
        \n}\
        \n"
    );
}
