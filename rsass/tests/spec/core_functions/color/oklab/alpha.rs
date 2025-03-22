//! Tests auto-converted from "sass-spec/spec/core_functions/color/oklab/alpha.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

mod degenerate {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn nan() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2 -3 / calc(NaN)));\n"),
            "a {\
         \n  value: oklab(1% 2 -3 / 0);\
         \n  space: oklab;\
         \n  channels: 1% 2 -3 / 0;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn negative_infinity() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2 -3 / calc(-infinity)));\n"),
            "a {\
         \n  value: oklab(1% 2 -3 / 0);\
         \n  space: oklab;\
         \n  channels: 1% 2 -3 / 0;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn positive_infinity() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2 -3 / calc(infinity)));\n"),
            "a {\
         \n  value: oklab(1% 2 -3);\
         \n  space: oklab;\
         \n  channels: 1% 2 -3 / 1;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab($channels: 1% 2 3 / 0.4));\n"),
        "a {\
         \n  value: oklab(1% 2 3 / 0.4);\
         \n  space: oklab;\
         \n  channels: 1% 2 3 / 0.4;\
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
             \n@include utils.inspect(oklab(1% 2 3 / none));\n"),
                "a {\
         \n  value: oklab(1% 2 3 / none);\
         \n  space: oklab;\
         \n  channels: 1% 2 3 / none;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn b() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2 none / 0.4));\n"),
                "a {\
         \n  value: oklab(1% 2 none / 0.4);\
         \n  space: oklab;\
         \n  channels: 1% 2 none / 0.4;\
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
                runner().ok("@use \'sass:list\';\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(list.slash(1% 2 3, none)));\n"),
                "a {\
         \n  value: oklab(1% 2 3 / none);\
         \n  space: oklab;\
         \n  channels: 1% 2 3 / none;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn b() {
            assert_eq!(
                runner().ok("@use \'sass:list\';\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(list.slash(1% 2 none, 0.4)));\n"),
                "a {\
         \n  value: oklab(1% 2 none / 0.4);\
         \n  space: oklab;\
         \n  channels: 1% 2 none / 0.4;\
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
             \n@include utils.inspect(oklab(1% 2 3 / 1));\n"),
        "a {\
         \n  value: oklab(1% 2 3);\
         \n  space: oklab;\
         \n  channels: 1% 2 3 / 1;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn partial() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2 3 / 0.4));\n"),
        "a {\
         \n  value: oklab(1% 2 3 / 0.4);\
         \n  space: oklab;\
         \n  channels: 1% 2 3 / 0.4;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn percent() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2 3 / 40%));\n"),
        "a {\
         \n  value: oklab(1% 2 3 / 0.4);\
         \n  space: oklab;\
         \n  channels: 1% 2 3 / 0.4;\
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
             \n@include utils.inspect(oklab(from #aaa l a b / 25%));\n"),
            "a {\
         \n  value: oklab(from #aaa l a b/25%);\
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
             \n@include utils.inspect(oklab(list.slash(from #aaa l a b, 25%)));\n"
        ),
        "a {\
         \n  value: oklab(from #aaa l a b / 25%);\
         \n  type: string;\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn slash_list() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(list.slash(1% 2 3, 0.4)));\n"),
        "a {\
         \n  value: oklab(1% 2 3 / 0.4);\
         \n  space: oklab;\
         \n  channels: 1% 2 3 / 0.4;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn transparent() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(0 255 127 / 0));\n"),
        "a {\
         \n  value: oklab(0% 255 127 / 0);\
         \n  space: oklab;\
         \n  channels: 0% 255 127 / 0;\
         \n}\n"
    );
}
