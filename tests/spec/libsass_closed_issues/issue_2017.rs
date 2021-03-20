//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2017.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\r\
            \n  bar: baz;\r\
            \n}\r\
            \n\r\
            \n@mixin link() {\r\
            \n  a:not(.btn) {\r\
            \n    color: red;\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \nfoo {\r\
            \n  @include link;\r\
            \n  @extend .btn;\r\
            \n  @include link;\r\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n}\
        \nfoo a:not(.btn):not(foo) {\
        \n  color: red;\
        \n}\
        \nfoo a:not(.btn):not(foo) {\
        \n  color: red;\
        \n}\
        \n"
    );
}
