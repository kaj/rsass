//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1890.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".wrap {\
            \n  @media (min-width: 480px) { \
            \n    display: block;\
            \n    @at-root (without: media){ \
            \n      .box { \
            \n        display: inline-block;\
            \n      }\
            \n    } \
            \n  }\
            \n}"
        )
        .unwrap(),
        "@media (min-width: 480px) {\
        \n  .wrap {\
        \n    display: block;\
        \n  }\
        \n}\
        \n.wrap .box {\
        \n  display: inline-block;\
        \n}\
        \n"
    );
}
