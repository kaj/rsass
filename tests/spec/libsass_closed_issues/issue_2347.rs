//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2347.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%baz2 {\r\
            \n    display: flex;\r\
            \n}\r\
            \n%baz3 {\r\
            \n    display: flex;\r\
            \n}\r\
            \n\r\
            \ncustom2, [custom2], .custom2 {\r\
            \n    @extend %baz2\r\
            \n}\r\
            \n\r\
            \n[custom3], custom3, .custom3 {\r\
            \n    @extend %baz3\r\
            \n}"
        )
        .unwrap(),
        "custom2, [custom2], .custom2 {\
        \n  display: flex;\
        \n}\
        \n[custom3], custom3, .custom3 {\
        \n  display: flex;\
        \n}\
        \n"
    );
}
