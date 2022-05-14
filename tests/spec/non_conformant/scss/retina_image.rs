//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/retina-image.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("retina-image")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@mixin retina-image($filename, $background-size, $extension: png, $retina-filename: null, $asset-pipeline: false) {\
             \n  @if $asset-pipeline {\
             \n    background-image: image_url($filename + \".\" + $extension);\
             \n  }\
             \n  @else {\
             \n    background-image: url($filename + \".\" + $extension);\
             \n  }\
             \n  @include hidpi {\
             \n    @if $asset-pipeline {\
             \n      @if $retina-filename {\
             \n        background-image: image_url($retina-filename + \".\" + $extension);\
             \n      }\
             \n      @else {\
             \n        background-image: image_url($filename + \"@2x\" + \".\" + $extension);\
             \n      }\
             \n    }\
             \n    @else {\
             \n      @if $retina-filename {\
             \n        background-image: url($retina-filename + \".\" + $extension);\
             \n      }\
             \n      @else {\
             \n        background-image: url($filename + \"@2x\" + \".\" + $extension);\
             \n      }\
             \n    }\
             \n    background-size: $background-size;\
             \n  }\
             \n}"
        ),
        ""
    );
}
