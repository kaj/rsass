//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1901.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a, b {\
            \n    &:not(c) {\
            \n        d: e;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "a:not(c), b:not(c) {\
        \n  d: e;\
        \n}\
        \n"
    );
}
