//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_86.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".color-functions {\r\
            \n    $color: red;\r\
            \n    hue: hue($color);\r\
            \n    hue-type: type-of(hue($color));\r\
            \n    hue-unit: unit(hue($color));\r\
            \n    hue-comparable: comparable(hue($color), hue($color));\r\
            \n\ttest-1: comparable(lightness(red), 1%);\r\
            \n\ttest-2: comparable(saturation(red), 1%);\r\
            \n}"
        )
        .unwrap(),
        ".color-functions {\
        \n  hue: 0deg;\
        \n  hue-type: number;\
        \n  hue-unit: \"deg\";\
        \n  hue-comparable: true;\
        \n  test-1: true;\
        \n  test-2: true;\
        \n}\
        \n"
    );
}
