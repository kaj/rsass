//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_86.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_86")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \"sass:math\";\
             \n@use \"sass:meta\";\
             \n.color-functions {\r\
             \n    $color: red;\r\
             \n    hue: color.hue($color);\r\
             \n    hue-type: meta.type-of(color.hue($color));\r\
             \n    hue-unit: math.unit(color.hue($color));\r\
             \n    hue-comparable: math.compatible(color.hue($color), color.hue($color));\r\
             \n\ttest-1: math.compatible(color.lightness(red), 1%);\r\
             \n\ttest-2: math.compatible(color.saturation(red), 1%);\r\
             \n}"
        ),
        ".color-functions {\
         \n  hue: 0deg;\
         \n  hue-type: number;\
         \n  hue-unit: \"deg\";\
         \n  hue-comparable: true;\
         \n  test-1: true;\
         \n  test-2: true;\
         \n}\n"
    );
}
