//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_453.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n    --a: 2px;\
            \n    top: var(--a);\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  --a: 2px;\
        \n  top: var(--a);\
        \n}\
        \n"
    );
}
