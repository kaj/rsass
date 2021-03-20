//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2462.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "b {\
            \n    color: lighten(Crimson, 10%);\
            \n}\
            \n"
        )
        .unwrap(),
        "b {\
        \n  color: #ed365b;\
        \n}\
        \n"
    );
}
