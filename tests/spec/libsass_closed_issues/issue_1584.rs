//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1584.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo($out: false) {\
            \n  @if $out {\
            \n    @at-root { @content; }\
            \n  }\
            \n}\
            \n\
            \n@mixin bar() {\
            \n  @at-root { @content; }\
            \n}\
            \n\
            \n@mixin baz($string) {\
            \n  @at-root .#{$string} { @content; }\
            \n}\
            \n\
            \n.test {\
            \n  @include foo(true) {\
            \n    .nest1 {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n  @include bar() {\
            \n    .nest2 {\
            \n      color: green;\
            \n    }\
            \n  }\
            \n  @include baz(\'nest3\') {\
            \n    color: blue;\
            \n  }\
            \n  @at-root {\
            \n    .nest4 {\
            \n      color: yellow;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".nest1 {\
        \n  color: red;\
        \n}\
        \n.nest2 {\
        \n  color: green;\
        \n}\
        \n.nest3 {\
        \n  color: blue;\
        \n}\
        \n.nest4 {\
        \n  color: yellow;\
        \n}\
        \n"
    );
}
