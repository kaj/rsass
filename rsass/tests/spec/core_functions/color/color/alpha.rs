//! Tests auto-converted from "sass-spec/spec/core_functions/color/color/alpha.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

#[test]
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        runner().ok(
            "@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color($description: srgb 0.1 0.2 0.3 / 0.4));\n"
        ),
        "a {\
         \n  value: color(srgb 0.1 0.2 0.3 / 0.4);\
         \n  space: srgb;\
         \n  channels: 0.1 0.2 0.3 / 0.4;\
         \n}\n"
    );
}
mod none {
    use super::runner;

    mod slash {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn alpha() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb 0.1 0.2 0.3 / none));\n"),
                "a {\
         \n  value: color(srgb 0.1 0.2 0.3 / none);\
         \n  space: srgb;\
         \n  channels: 0.1 0.2 0.3 / none;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn blue() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb 0.1 0.2 none / 0.4));\n"),
                "a {\
         \n  value: color(srgb 0.1 0.2 none / 0.4);\
         \n  space: srgb;\
         \n  channels: 0.1 0.2 none / 0.4;\
         \n}\n"
            );
        }
    }
    mod slash_list {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn alpha() {
            assert_eq!(
        runner().ok(
            "@use \'sass:list\';\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(list.slash(srgb 0.1 0.2 0.3, none)));\n"
        ),
        "a {\
         \n  value: color(srgb 0.1 0.2 0.3 / none);\
         \n  space: srgb;\
         \n  channels: 0.1 0.2 0.3 / none;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn blue() {
            assert_eq!(
        runner().ok(
            "@use \'sass:list\';\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(list.slash(srgb 0.1 0.2 none, 0.4)));\n"
        ),
        "a {\
         \n  value: color(srgb 0.1 0.2 none / 0.4);\
         \n  space: srgb;\
         \n  channels: 0.1 0.2 none / 0.4;\
         \n}\n"
    );
        }
    }
}
#[test]
#[ignore] // unexepected error
fn opaque() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb 0.1 0.2 0.3 / 1));\n"),
        "a {\
         \n  value: color(srgb 0.1 0.2 0.3);\
         \n  space: srgb;\
         \n  channels: 0.1 0.2 0.3 / 1;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn partial() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb 0.1 0.2 0.3 / 0.4));\n"),
        "a {\
         \n  value: color(srgb 0.1 0.2 0.3 / 0.4);\
         \n  space: srgb;\
         \n  channels: 0.1 0.2 0.3 / 0.4;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn percent() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb 0.1 0.2 0.3 / 40%));\n"),
        "a {\
         \n  value: color(srgb 0.1 0.2 0.3 / 0.4);\
         \n  space: srgb;\
         \n  channels: 0.1 0.2 0.3 / 0.4;\
         \n}\n"
    );
}
mod relative_color {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn slash() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(from #aaa srgb r g b / 25%));\n"),
            "a {\
         \n  value: color(from #aaa srgb r g b/25%);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn slash_list() {
        assert_eq!(
        runner().ok(
            "@use \'sass:list\';\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(list.slash(from #aaa srgb r g b, 25%)));\n"
        ),
        "a {\
         \n  value: color(from #aaa srgb r g b / 25%);\
         \n  type: string;\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn slash_list() {
    assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(list.slash(srgb 0.1 0.2 0.3, 0.4)));\n"
        ),
        "a {\
         \n  value: color(srgb 0.1 0.2 0.3 / 0.4);\
         \n  space: srgb;\
         \n  channels: 0.1 0.2 0.3 / 0.4;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn transparent() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb 0.1 0.2 0.3 / 0));\n"),
        "a {\
         \n  value: color(srgb 0.1 0.2 0.3 / 0);\
         \n  space: srgb;\
         \n  channels: 0.1 0.2 0.3 / 0;\
         \n}\n"
    );
}
