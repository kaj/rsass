//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/one_arg/alpha.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

mod bounds {
    use super::runner;

    mod alpha {
        use super::runner;

        mod percent {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn above() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(rgb(0 0 0 / 250%));\n"),
                    "a {\
         \n  value: rgb(0, 0, 0);\
         \n  space: rgb;\
         \n  channels: 0 0 0 / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn below() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(rgb(0 0 0 / -10%));\n"),
                    "a {\
         \n  value: rgba(0, 0, 0, 0);\
         \n  space: rgb;\
         \n  channels: 0 0 0 / 0;\
         \n}\n"
                );
            }
        }
        mod unitless {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn above() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(rgb(0 0 0 / 1.1));\n"),
                    "a {\
         \n  value: rgb(0, 0, 0);\
         \n  space: rgb;\
         \n  channels: 0 0 0 / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn below() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(rgb(0 0 0 / -0.1));\n"),
                    "a {\
         \n  value: rgba(0, 0, 0, 0);\
         \n  space: rgb;\
         \n  channels: 0 0 0 / 0;\
         \n}\n"
                );
            }
        }
    }
    mod blue {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(rgb(0 0 999));\n"),
                "a {\
         \n  value: rgb(0, 0, 255);\
         \n  space: rgb;\
         \n  channels: 0 0 255 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(rgb(0 0 -500));\n"),
                "a {\
         \n  value: rgb(0, 0, 0);\
         \n  space: rgb;\
         \n  channels: 0 0 0 / 1;\
         \n}\n"
            );
        }
    }
    mod green {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(rgb(0 500 0));\n"),
                "a {\
         \n  value: rgb(0, 255, 0);\
         \n  space: rgb;\
         \n  channels: 0 255 0 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(rgb(0 -100 0));\n"),
                "a {\
         \n  value: rgb(0, 0, 0);\
         \n  space: rgb;\
         \n  channels: 0 0 0 / 1;\
         \n}\n"
            );
        }
    }
    mod red {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(rgb(256 0 0));\n"),
                "a {\
         \n  value: rgb(255, 0, 0);\
         \n  space: rgb;\
         \n  channels: 255 0 0 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(rgb(-1 0 0));\n"),
                "a {\
         \n  value: rgb(0, 0, 0);\
         \n  space: rgb;\
         \n  channels: 0 0 0 / 1;\
         \n}\n"
            );
        }
    }
}
mod in_gamut {
    use super::runner;

    #[test]
    fn named() {
        assert_eq!(
            runner().ok("a {b: rgb($channels: 0 255 127 / 0.3)}\n"),
            "a {\
         \n  b: rgba(0, 255, 127, 0.3);\
         \n}\n"
        );
    }
    #[test]
    fn opaque() {
        assert_eq!(
            runner().ok("a {b: rgb(190 173 237 / 1)}\n"),
            "a {\
         \n  b: rgb(190, 173, 237);\
         \n}\n"
        );
    }
    #[test]
    fn parenthesized() {
        assert_eq!(
        runner().ok(
            "// Extra parens shouldn\'t cause the slash to be forced into division.\
             \na {b: rgb(0 255 127 / 0.3)}\n"
        ),
        "a {\
         \n  b: rgba(0, 255, 127, 0.3);\
         \n}\n"
    );
    }
    #[test]
    fn partial() {
        assert_eq!(
            runner().ok("a {b: rgb(18 52 86 / 0.5)}\n"),
            "a {\
         \n  b: rgba(18, 52, 86, 0.5);\
         \n}\n"
        );
    }
    #[test]
    fn percent() {
        assert_eq!(
            runner().ok("a {b: rgb(18 52 86 / 50%)}\n"),
            "a {\
         \n  b: rgba(18, 52, 86, 0.5);\
         \n}\n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            runner().ok("a {b: rgb(0 255 127 / 0)}\n"),
            "a {\
         \n  b: rgba(0, 255, 127, 0);\
         \n}\n"
        );
    }
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn slash() {
        assert_eq!(
            runner().ok("a {b: rgb(0 255 127 / none)}\n"),
            "a {\
         \n  b: rgb(0 255 127 / none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn slash_list() {
        assert_eq!(
            runner().ok("@use \'sass:list\';\
             \na {b: rgb(list.slash(0 255 127, none))}\n"),
            "a {\
         \n  b: rgb(0 255 127 / none);\
         \n}\n"
        );
    }
}
#[test]
fn slash_list() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: rgb(list.slash(0 255 127, 0))}\n"),
        "a {\
         \n  b: rgba(0, 255, 127, 0);\
         \n}\n"
    );
}
