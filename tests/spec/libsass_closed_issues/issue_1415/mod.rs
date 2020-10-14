//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1415"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1415/direct.hrx"
#[test]
fn direct() {
    assert_eq!(
        rsass(
            "@if & {\
            \n  foo {\
            \n    foo: bar;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1415/mixin.hrx"
#[test]
fn mixin() {
    assert_eq!(
        rsass(
            "@mixin prepend-foo {\
            \n  $parent: &;\
            \n\
            \n  @if $parent {\
            \n    .foo & {\
            \n      @content;\
            \n    }\
            \n  } @else {\
            \n    .foo {\
            \n      @content;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \n@include prepend-foo {\
            \n  bar {\
            \n    color: red;\
            \n  }\
            \n}\
            \n\
            \nbar {\
            \n  @include prepend-foo {\
            \n    baz {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo bar {\
        \n  color: red;\
        \n}\
        \n.foo bar baz {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1415/variable.hrx"
#[test]
fn variable() {
    assert_eq!(
        rsass(
            "$parent: &;\
            \n\
            \n@if $parent {\
            \n  foo {\
            \n    foo: bar;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}
