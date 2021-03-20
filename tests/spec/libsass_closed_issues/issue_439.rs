//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_439.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin odd( $selector, $n) {\
            \n  $selector: \"& + \" + $selector + \" + \" + $selector;\
            \n  $placeholder: unique_id();\
            \n  %#{$placeholder} { @content; }\
            \n  #{$selector}:first-child {\
            \n    #{$selector} { @extend %#{$placeholder}; }\
            \n  }\
            \n}\
            \n\
            \nul > {\
            \n  @include odd( li, 5 ) { background: #ccc;  }\
            \n}\
            \n"
        )
        .unwrap(),
        "ul > + li + li:first-child + li + li {\
        \n  background: #ccc;\
        \n}\
        \n"
    );
}
