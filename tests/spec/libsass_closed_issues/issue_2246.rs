//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2246.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo($option: \'foo\') {\
            \n     // Create a unique, random placeholder to store styles\
            \n    $placeholder : $option + random(9999);\
            \n\
            \n    // Store the styles in the placeholder\
            \n    @at-root %#{$placeholder} {\
            \n        content: \'foo\';\
            \n    }\
            \n\
            \n    @at-root {\
            \n        .bar {\
            \n            @extend %#{$placeholder};\
            \n        }\
            \n    }\
            \n}\
            \n\
            \n@mixin bar($option) {\
            \n    @include foo($option);\
            \n}\
            \n\
            \n.foo {\
            \n    @include bar(\'baz\');\
            \n}"
        )
        .unwrap(),
        ".bar {\
        \n  content: \"foo\";\
        \n}\
        \n"
    );
}
