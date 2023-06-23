//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2246.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2246")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@mixin foo($option: \'foo\') {\
             \n     // Create a unique, random placeholder to store styles\
             \n    $placeholder : $option + random(9999);\n\
             \n    // Store the styles in the placeholder\
             \n    @at-root %#{$placeholder} {\
             \n        content: \'foo\';\
             \n    }\n\
             \n    @at-root {\
             \n        .bar {\
             \n            @extend %#{$placeholder};\
             \n        }\
             \n    }\
             \n}\n\
             \n@mixin bar($option) {\
             \n    @include foo($option);\
             \n}\n\
             \n.foo {\
             \n    @include bar(\'baz\');\
             \n}"),
        ".bar {\
         \n  content: \"foo\";\
         \n}\n"
    );
}
