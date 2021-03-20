//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2233.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@media all and (min-width: 100px) {\
            \n  @import \"foo\"\
            \n}\
            \n"
        )
        .unwrap(),
        "@media all and (min-width: 100px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n"
    );
}
