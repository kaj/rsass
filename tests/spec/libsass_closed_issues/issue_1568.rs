//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1568.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "body {\
            \n    font-weight: bold; // test\
            \n    font-size: 10px // test\
            \n}\
            \n"
        )
        .unwrap(),
        "body {\
        \n  font-weight: bold;\
        \n  font-size: 10px;\
        \n}\
        \n"
    );
}
