//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1415/mixin.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
