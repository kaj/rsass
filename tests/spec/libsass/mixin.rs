//! Tests auto-converted from "sass-spec/spec/libsass/mixin.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "// @charset \"UTF-8\";\
            \n\
            \n@mixin background-image-retina($file, $type, $width, $height) {\
            \n  background-image: unquote(image-url(\"#{$file}.#{$type}\", true));\
            \n  hey: length(a b c d);\
            \n  ho: unquote(\"hello\");\
            \n  hee: unquote(unit(10fudge));\
            \n\
            \n  @media (-webkit-min-device-pixel-ratio: 2), (-moz-min-device-pixel-ratio: 2) {\
            \n    & {\
            \n      background-image: image-url(\"#{$file}@2x.#{$type}\");\
            \n      -webkit-background-size: $width $height;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \n$x: foo;\
            \n$y: bar;\
            \n\
            \ndiv {\
            \n  @include background-image-retina(test, png, 100px, 300px);\
            \n  fudge: walnut;\
            \n}\
            \n\
            \nspan {\
            \n  blah: \"#{$x} #{$y}\";\
            \n  bleh: image-url(\"#{$x} #{$y}\");\
            \n}\
            \n\
            \n\
            \n@mixin foo($x, $y) {\
            \n  foo-x: $x;\
            \n  foo-y: $y;\
            \n}\
            \n\
            \ndiv {\
            \n  @include foo(1, 2);\
            \n  @include foo($y: 2, $x: 1);\
            \n}\
            \n\
            \n@mixin bar($x, $y: default) {\
            \n  bar-x: $x;\
            \n  bar-y: $y;\
            \n}\
            \n\
            \ndiv {\
            \n  @include bar(1, 2);\
            \n  @include bar(1);\
            \n  @include bar($x: n1, $y: n2);\
            \n  @include bar($x: n1);\
            \n  blah: unquote(\"hello\");\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  background-image: image-url(\"test.png\", true);\
        \n  hey: 4;\
        \n  ho: hello;\
        \n  hee: fudge;\
        \n  fudge: walnut;\
        \n}\
        \n@media (-webkit-min-device-pixel-ratio: 2), (-moz-min-device-pixel-ratio: 2) {\
        \n  div {\
        \n    background-image: image-url(\"test@2x.png\");\
        \n    -webkit-background-size: 100px 300px;\
        \n  }\
        \n}\
        \nspan {\
        \n  blah: \"foo bar\";\
        \n  bleh: image-url(\"foo bar\");\
        \n}\
        \ndiv {\
        \n  foo-x: 1;\
        \n  foo-y: 2;\
        \n  foo-x: 1;\
        \n  foo-y: 2;\
        \n}\
        \ndiv {\
        \n  bar-x: 1;\
        \n  bar-y: 2;\
        \n  bar-x: 1;\
        \n  bar-y: default;\
        \n  bar-x: n1;\
        \n  bar-y: n2;\
        \n  bar-x: n1;\
        \n  bar-y: default;\
        \n  blah: hello;\
        \n}\
        \n"
    );
}
