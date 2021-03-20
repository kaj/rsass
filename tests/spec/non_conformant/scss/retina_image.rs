//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/retina-image.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
        )
        .unwrap(),
        ""
    );
}
