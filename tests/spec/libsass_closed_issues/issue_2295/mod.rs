//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2295"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_2295/basic.hrx"
#[test]
#[ignore] // wrong result
fn basic() {
    assert_eq!(
        rsass(
            ".my-scope {\r\
            \n  @import \'include.scss\';\r\
            \n}"
        )
        .unwrap(),
        ".my-scope .foo {\
        \n  display: none;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2295/original.hrx"
#[test]
#[ignore] // wrong result
fn original() {
    assert_eq!(
        rsass(
            "$include-foo: true !default;\r\
            \n.my-scope {\r\
            \n  .bar {  display: none; }\r\
            \n  @if ($include-foo) {\r\
            \n    .foo { display: none; }\r\
            \n  }\r\
            \n  @import \'input-bug\';\r\
            \n}"
        )
        .unwrap(),
        ".my-scope .bar {\
        \n  display: none;\
        \n}\
        \n.my-scope .foo {\
        \n  display: none;\
        \n}\
        \n.my-scope .bar {\
        \n  display: none;\
        \n}\
        \n.my-scope .foo {\
        \n  display: none;\
        \n}\
        \n"
    );
}
