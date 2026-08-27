//! Tests auto-converted from "sass-spec/spec/core_functions/color/lab/alpha.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

mod degenerate {
    use super::runner;

    #[test]
    fn nan() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 -3 / calc(NaN)));\n"),
            "a {\
         \n  value: lab(1% 2 -3 / 0);\
         \n  space: lab;\
         \n  channels: 1% 2 -3 / 0;\
         \n}\n"
        );
    }
    #[test]
    fn negative_infinity() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 -3 / calc(-infinity)));\n"),
            "a {\
         \n  value: lab(1% 2 -3 / 0);\
         \n  space: lab;\
         \n  channels: 1% 2 -3 / 0;\
         \n}\n"
        );
    }
    #[test]
    fn positive_infinity() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 -3 / calc(infinity)));\n"),
            "a {\
         \n  value: lab(1% 2 -3);\
         \n  space: lab;\
         \n  channels: 1% 2 -3 / 1;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab($channels: 1% 2 3 / 0.4));\n"),
        "a {\
         \n  value: lab(1% 2 3 / 0.4);\
         \n  space: lab;\
         \n  channels: 1% 2 3 / 0.4;\
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
             \n@include utils.inspect(lab(1% 2 3 / none));\n"),
                "a {\
         \n  value: lab(1% 2 3 / none);\
         \n  space: lab;\
         \n  channels: 1% 2 3 / none;\
         \n}\n"
            );
        }
        #[test]
        fn b() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 none / 0.4));\n"),
                "a {\
         \n  value: lab(1% 2 none / 0.4);\
         \n  space: lab;\
         \n  channels: 1% 2 none / 0.4;\
         \n}\n"
            );
        }
        #[test]
        fn b_and_alpha() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 none / none));\n"),
                "a {\
         \n  value: lab(1% 2 none / none);\
         \n  space: lab;\
         \n  channels: 1% 2 none / none;\
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
             \n@include utils.inspect(lab(list.slash(1% 2 3, none)));\n"),
                "a {\
         \n  value: lab(1% 2 3 / none);\
         \n  space: lab;\
         \n  channels: 1% 2 3 / none;\
         \n}\n"
            );
        }
        #[test]
        fn b() {
            assert_eq!(
                runner().ok("@use \'sass:list\';\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(list.slash(1% 2 none, 0.4)));\n"),
                "a {\
         \n  value: lab(1% 2 none / 0.4);\
         \n  space: lab;\
         \n  channels: 1% 2 none / 0.4;\
         \n}\n"
            );
        }
        #[test]
        fn b_and_alpha() {
            assert_eq!(
                runner().ok("@use \'sass:list\';\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(list.slash(1% 2 none, none)));\n"),
                "a {\
         \n  value: lab(1% 2 none / none);\
         \n  space: lab;\
         \n  channels: 1% 2 none / none;\
         \n}\n"
            );
        }
    }
}
#[test]
fn opaque() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 3 / 1));\n"),
        "a {\
         \n  value: lab(1% 2 3);\
         \n  space: lab;\
         \n  channels: 1% 2 3 / 1;\
         \n}\n"
    );
}
#[test]
fn partial() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 3 / 0.4));\n"),
        "a {\
         \n  value: lab(1% 2 3 / 0.4);\
         \n  space: lab;\
         \n  channels: 1% 2 3 / 0.4;\
         \n}\n"
    );
}
#[test]
fn percent() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 3 / 40%));\n"),
        "a {\
         \n  value: lab(1% 2 3 / 0.4);\
         \n  space: lab;\
         \n  channels: 1% 2 3 / 0.4;\
         \n}\n"
    );
}
#[test]
fn slash_list() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(list.slash(1% 2 3, 0.4)));\n"),
        "a {\
         \n  value: lab(1% 2 3 / 0.4);\
         \n  space: lab;\
         \n  channels: 1% 2 3 / 0.4;\
         \n}\n"
    );
}
#[test]
fn transparent() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(0 255 127 / 0));\n"),
        "a {\
         \n  value: lab(0% 255 127 / 0);\
         \n  space: lab;\
         \n  channels: 0% 255 127 / 0;\
         \n}\n"
    );
}
