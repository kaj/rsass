//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_859.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@media screen {\
            \n  .two {\
            \n    @at-root .one {\
            \n      background: blue;\
            \n      .three {\
            \n        color: red;\
            \n      }\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen {\
        \n  .one {\
        \n    background: blue;\
        \n  }\
        \n  .one .three {\
        \n    color: red;\
        \n  }\
        \n}\
        \n"
    );
}
