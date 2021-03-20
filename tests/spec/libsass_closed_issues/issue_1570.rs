//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1570.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n    font: 12px/normal serif;\
            \n}\
            \n\
            \nb {\
            \n    font: normal 12px/normal serif;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  font: 12px/normal serif;\
        \n}\
        \nb {\
        \n  font: normal 12px/normal serif;\
        \n}\
        \n"
    );
}
