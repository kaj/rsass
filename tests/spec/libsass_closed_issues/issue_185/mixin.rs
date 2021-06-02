//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_185/mixin.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            "@function shift($list) {\
             \n  @if length($list) == 1 { @return (); }\n\
             \n  $new: ();\
             \n  @for $i from 2 through length($list) {\
             \n    $new: append($new, nth($list, $i));\
             \n  }\
             \n  @return $new;\
             \n}\n\
             \n@mixin media($medias...) {\
             \n  @if length($medias) == 0 {\
             \n    @content;\
             \n  } @else {\
             \n    @media #{nth($medias, 1)} {\
             \n      @include media(shift($medias)...) {\
             \n        @content;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n.foo {\
             \n  @include media(\'only screen\', \'(color)\', \'(orientation: portrait)\') {\
             \n    content: bar;\
             \n  }\
             \n}\n\
             \n@include media(\'all\', \'(min-width: 42em)\') {\
             \n  .foo {\
             \n    content: bar;\
             \n  }\
             \n}\n"
        ),
        "@media only screen and (color) and (orientation: portrait) {\
         \n  .foo {\
         \n    content: bar;\
         \n  }\
         \n}\
         \n@media (min-width: 42em) {\
         \n  .foo {\
         \n    content: bar;\
         \n  }\
         \n}\n"
    );
}
