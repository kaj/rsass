//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_108.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$a: red;\r\
            \n\r\
            \n@mixin f($a: $a) {\r\
            \n  color: $a;\r\
            \n}\r\
            \n\r\
            \nh1 {\r\
            \n  @include f;\r\
            \n}\r\
            \n\r\
            \nh2 {\r\
            \n  @include f(blue);\r\
            \n}"
        )
        .unwrap(),
        "h1 {\
        \n  color: red;\
        \n}\
        \nh2 {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
