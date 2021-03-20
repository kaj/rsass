//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1886.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "body {\
            \n  background: url()\
            \n}"
        )
        .unwrap(),
        "body {\
        \n  background: url();\
        \n}\
        \n"
    );
}
