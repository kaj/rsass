//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1709.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin transition( $prefix_properties, $transitions... ) {\
            \n\
            \n    @if not str-index( inspect( $transitions ), \',\') {\
            \n        $transitions: ( $transitions );\
            \n    }\
            \n\
            \n    @each $prefix in -webkit-, -moz-, -ms-, -o-, \'\' {\
            \n\
            \n        $prefixed: \'\';\
            \n\
            \n        @each $transition in $transitions {\
            \n\
            \n            @if $prefix_properties and \'\' != $prefix {\
            \n                $prefixed: #{$prefix}$transition,$transition;\
            \n            } @else {\
            \n                $prefixed: $transition;\
            \n            }\
            \n\
            \n\
            \n        }\
            \n\
            \n        #{$prefix}transition: $prefixed;\
            \n    }\
            \n}\
            \n\
            \n.my-element {\
            \n    @include transition( true, transform 0.25s linear );\
            \n}\
            \n"
        )
        .unwrap(),
        ".my-element {\
        \n  -webkit-transition: -webkit- transform 0.25s linear, transform 0.25s linear;\
        \n  -moz-transition: -moz- transform 0.25s linear, transform 0.25s linear;\
        \n  -ms-transition: -ms- transform 0.25s linear, transform 0.25s linear;\
        \n  -o-transition: -o- transform 0.25s linear, transform 0.25s linear;\
        \n  transition: transform 0.25s linear;\
        \n}\
        \n"
    );
}
