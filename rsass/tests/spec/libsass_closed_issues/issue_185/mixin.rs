//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_185/mixin.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@function shift($list) {\
             \n  @if list.length($list) == 1 { @return (); }\n\
             \n  $new: ();\
             \n  @for $i from 2 through list.length($list) {\
             \n    $new: list.append($new, list.nth($list, $i));\
             \n  }\
             \n  @return $new;\
             \n}\n\
             \n@mixin media($medias...) {\
             \n  @if list.length($medias) == 0 {\
             \n    @content;\
             \n  } @else {\
             \n    @media #{list.nth($medias, 1)} {\
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
