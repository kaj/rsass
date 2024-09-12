//! Tests auto-converted from "sass-spec/spec/libsass/parent-selector/missing.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("missing")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:map\";\
             \n$tablet-portrait:                 768px;\
             \n$tablet-landscape:                980px;\
             \n$desk-normal:                     1120px;\
             \n$desk-big:                        1280px;\
             \n$grid-breakpoints-immobile: (\
             \n        \'tablet-portrait\':   \'(min-width: \' + $tablet-portrait + \') and (max-width: \' + $tablet-landscape + \')\',\
             \n        \'tablet-landscape\':  \'(min-width: \' +  $tablet-landscape + \') and (max-width: \' + $desk-normal + \')\',\
             \n        \'desk-normal\':       \'(min-width: \' +  $desk-normal + \') and (max-width: \' + $desk-big + \')\',\
             \n        \'desk-big\':          \'(min-width: \' +  $desk-big + \')\'\
             \n);\
             \n@mixin grid-media-query($media-query, $breakpointDefinitions) {\
             \n  $breakpoint-found: false;\n\
             \n  @each $breakpoint, $breakpointvalue in $breakpointDefinitions{\
             \n    $name: $breakpoint;\
             \n    $declaration: $breakpointvalue;\n\
             \n    @if $media-query == $name and $declaration{\
             \n      $breakpoint-found: true;\n\
             \n      @media only screen and #{$declaration} {\
             \n        @content;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \n@each $name in map.keys($grid-breakpoints-immobile) {\
             \n  @include grid-media-query($name, $grid-breakpoints-immobile) {\
             \n    body.immobile & {\
             \n      margin-bottom: 0;\
             \n    }\
             \n  }\
             \n}\n"
        ),
        "Error: Top-level selectors may not contain the parent selector \"&\".\
         \n   ,\
         \n31 |     body.immobile & {\
         \n   |                   ^\
         \n   \'\
         \n  input.scss 31:19  @content\
         \n  input.scss 23:9   grid-media-query()\
         \n  input.scss 30:3   root stylesheet",
    );
}
