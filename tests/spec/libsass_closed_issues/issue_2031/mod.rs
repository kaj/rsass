//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2031"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_2031/extended-not.hrx"
#[test]
#[ignore] // wrong result
fn extended_not() {
    assert_eq!(
        rsass(
            ":not(.foo) {\r\
            \n  content: test;\r\
            \n}\r\
            \n.bar, .baz {\r\
            \n  @extend .foo;\r\
            \n}\r\
            \n\r\
            \ntest {\r\
            \n  content: selector-extend(\":not(.foo)\", \".foo\", \".bar\");\r\
            \n}"
        )
        .unwrap(),
        ":not(.foo):not(.bar):not(.baz) {\
        \n  content: test;\
        \n}\
        \ntest {\
        \n  content: :not(.foo):not(.bar);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2031/wrapped-not.hrx"
#[test]
fn wrapped_not() {
    assert_eq!(
        rsass(
            ":not(.asd, .qwe) {\r\
            \n  content: test;\r\
            \n}"
        )
        .unwrap(),
        ":not(.asd, .qwe) {\
        \n  content: test;\
        \n}\
        \n"
    );
}
