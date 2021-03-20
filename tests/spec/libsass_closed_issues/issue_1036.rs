//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1036.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin all-vip() {\
            \n  test: vip;\
            \n}\
            \n@mixin gold() {\
            \n  test: gold;\
            \n}\
            \n@mixin platinum() {\
            \n  test: platinum;\
            \n}\
            \n\
            \n@mixin icons-sprite($icon-name){\
            \n    @if $icon-name == \'all-vip\' {\
            \n        @include all-vip();\
            \n    }\
            \n    @else if $icon-name == \'gold\' {\
            \n        @include gold();\
            \n    }\
            \n    @else if $icon-name == \'platinum\' {\
            \n        @include platinum();\
            \n    }\
            \n}\
            \n\
            \ndiv {\
            \n  @include icons-sprite(\"platinum\");\
            \n  @include icons-sprite(\"all-vip\");\
            \n  @include icons-sprite(\"gold\");\
            \n}\
            \ndiv {\
            \n  @include icons-sprite(platinum);\
            \n  @include icons-sprite(all-vip);\
            \n  @include icons-sprite(gold);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  test: platinum;\
        \n  test: vip;\
        \n  test: gold;\
        \n}\
        \ndiv {\
        \n  test: platinum;\
        \n  test: vip;\
        \n}\
        \n"
    );
}
