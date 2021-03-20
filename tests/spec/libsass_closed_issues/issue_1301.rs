//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1301.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$name: \"my-class\";\
            \n\
            \n.-#{$name} {\
            \n  content: \"test\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".-my-class {\
        \n  content: \"test\";\
        \n}\
        \n"
    );
}
