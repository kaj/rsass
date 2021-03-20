//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_456.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "body {\
            \n  -webkit-filter: invert(100%);\
            \n}\
            \n"
        )
        .unwrap(),
        "body {\
        \n  -webkit-filter: invert(100%);\
        \n}\
        \n"
    );
}
