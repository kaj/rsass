//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2444.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  @at-root (with: rule) {\
            \n    b: c;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
