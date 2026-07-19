//! Tests auto-converted from "sass-spec/spec/core_functions/color/color/no_alpha.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_alpha")
}

#[test]
fn case() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(sRGB 0.1 none 0.3));\n"),
        "a {\
         \n  value: color(srgb 0.1 none 0.3);\
         \n  space: srgb;\
         \n  channels: 0.1 none 0.3 / 1;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color($description: srgb 0.1 0.2 0.3));\n"
        ),
        "a {\
         \n  value: color(srgb 0.1 0.2 0.3);\
         \n  space: srgb;\
         \n  channels: 0.1 0.2 0.3 / 1;\
         \n}\n"
    );
}
mod none {
    use super::runner;

    #[test]
    fn blue() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb 0.1 none 0.3));\n"),
            "a {\
         \n  value: color(srgb 0.1 none 0.3);\
         \n  space: srgb;\
         \n  channels: 0.1 none 0.3 / 1;\
         \n}\n"
        );
    }
    #[test]
    fn green() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb 0.1 none 0.3));\n"),
            "a {\
         \n  value: color(srgb 0.1 none 0.3);\
         \n  space: srgb;\
         \n  channels: 0.1 none 0.3 / 1;\
         \n}\n"
        );
    }
    #[test]
    fn red() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb none 0.2 0.3));\n"),
            "a {\
         \n  value: color(srgb none 0.2 0.3);\
         \n  space: srgb;\
         \n  channels: none 0.2 0.3 / 1;\
         \n}\n"
        );
    }
}
#[test]
fn relative_color() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(from #aaa srgb r g b));\n"),
        "a {\
         \n  value: color(from #aaa srgb r g b);\
         \n  type: string;\
         \n}\n"
    );
}
