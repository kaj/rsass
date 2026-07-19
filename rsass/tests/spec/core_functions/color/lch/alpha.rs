//! Tests auto-converted from "sass-spec/spec/core_functions/color/lch/alpha.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

mod degenerate {
    use super::runner;

    #[test]
    fn nan() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 2 3deg / calc(NaN)));\n"),
            "a {\
         \n  value: lch(1% 2 3deg / 0);\
         \n  space: lch;\
         \n  channels: 1% 2 3deg / 0;\
         \n}\n"
        );
    }
    #[test]
    fn negative_infinity() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 2 3deg / calc(-infinity)));\n"),
            "a {\
         \n  value: lch(1% 2 3deg / 0);\
         \n  space: lch;\
         \n  channels: 1% 2 3deg / 0;\
         \n}\n"
        );
    }
    #[test]
    fn positive_infinity() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 2 3deg / calc(infinity)));\n"),
            "a {\
         \n  value: lch(1% 2 3deg);\
         \n  space: lch;\
         \n  channels: 1% 2 3deg / 1;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch($channels: 1% 2 3deg / 0.4));\n"),
        "a {\
         \n  value: lch(1% 2 3deg / 0.4);\
         \n  space: lch;\
         \n  channels: 1% 2 3deg / 0.4;\
         \n}\n"
    );
}
mod none {
    use super::runner;

    mod slash {
        use super::runner;

        #[test]
        fn alpha() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 2 3deg / none));\n"),
                "a {\
         \n  value: lch(1% 2 3deg / none);\
         \n  space: lch;\
         \n  channels: 1% 2 3deg / none;\
         \n}\n"
            );
        }
        #[test]
        fn hue() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 2 none / 0.4));\n"),
                "a {\
         \n  value: lch(1% 2 none / 0.4);\
         \n  space: lch;\
         \n  channels: 1% 2 none / 0.4;\
         \n}\n"
            );
        }
    }
    mod slash_list {
        use super::runner;

        #[test]
        fn alpha() {
            assert_eq!(
                runner().ok("@use \'sass:list\';\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(list.slash(1% 2 3deg, none)));\n"),
                "a {\
         \n  value: lch(1% 2 3deg / none);\
         \n  space: lch;\
         \n  channels: 1% 2 3deg / none;\
         \n}\n"
            );
        }
        #[test]
        fn hue() {
            assert_eq!(
                runner().ok("@use \'sass:list\';\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(list.slash(1% 2 none, 0.4)));\n"),
                "a {\
         \n  value: lch(1% 2 none / 0.4);\
         \n  space: lch;\
         \n  channels: 1% 2 none / 0.4;\
         \n}\n"
            );
        }
    }
}
#[test]
fn opaque() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 2 3deg / 1));\n"),
        "a {\
         \n  value: lch(1% 2 3deg);\
         \n  space: lch;\
         \n  channels: 1% 2 3deg / 1;\
         \n}\n"
    );
}
#[test]
fn partial() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 2 3deg / 0.4));\n"),
        "a {\
         \n  value: lch(1% 2 3deg / 0.4);\
         \n  space: lch;\
         \n  channels: 1% 2 3deg / 0.4;\
         \n}\n"
    );
}
#[test]
fn percent() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 2 3deg / 40%));\n"),
        "a {\
         \n  value: lch(1% 2 3deg / 0.4);\
         \n  space: lch;\
         \n  channels: 1% 2 3deg / 0.4;\
         \n}\n"
    );
}
#[test]
fn slash_list() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(list.slash(1% 2 3deg, 0.4)));\n"),
        "a {\
         \n  value: lch(1% 2 3deg / 0.4);\
         \n  space: lch;\
         \n  channels: 1% 2 3deg / 0.4;\
         \n}\n"
    );
}
#[test]
fn transparent() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 2 3deg / 0));\n"),
        "a {\
         \n  value: lch(1% 2 3deg / 0);\
         \n  space: lch;\
         \n  channels: 1% 2 3deg / 0;\
         \n}\n"
    );
}
