//! Tests auto-converted from "sass-spec/spec/core_functions/color/lch/no_alpha.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_alpha")
}

#[test]
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch($channels: 1% 2 3deg));\n"),
        "a {\
         \n  value: lch(1% 2 3deg);\
         \n  space: lch;\
         \n  channels: 1% 2 3deg / 1;\
         \n}\n"
    );
}
mod none {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn chroma() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% none 3deg));\n"),
            "a {\
         \n  value: lch(1% none 3deg);\
         \n  space: lch;\
         \n  channels: 1% none 3deg / 1;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 2 none));\n"),
            "a {\
         \n  value: lch(1% 2 none);\
         \n  space: lch;\
         \n  channels: 1% 2 none / 1;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(none 2 3deg));\n"),
            "a {\
         \n  value: lch(none 2 3deg);\
         \n  space: lch;\
         \n  channels: none 2 3deg / 1;\
         \n}\n"
        );
    }
}
mod percent {
    use super::runner;

    mod chroma {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 101% 3deg));\n"),
                "a {\
         \n  value: lch(1% 151.5 3deg);\
         \n  space: lch;\
         \n  channels: 1% 151.5 3deg / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% -1% 3deg));\n"),
                "a {\
         \n  value: lch(1% 0 3deg);\
         \n  space: lch;\
         \n  channels: 1% 0 3deg / 1;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn in_range() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 2% 3deg));\n"),
            "a {\
         \n  value: lch(1% 3 3deg);\
         \n  space: lch;\
         \n  channels: 1% 3 3deg / 1;\
         \n}\n"
        );
    }
    mod lightness {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(110% 2 3deg));\n"),
                "a {\
         \n  value: lch(100% 2 3deg);\
         \n  space: lch;\
         \n  channels: 100% 2 3deg / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(-1% 2 3deg));\n"),
                "a {\
         \n  value: lch(0% 2 3deg);\
         \n  space: lch;\
         \n  channels: 0% 2 3deg / 1;\
         \n}\n"
            );
        }
    }
}
mod unitless {
    use super::runner;

    mod chroma {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 0.5 3deg));\n"),
                "a {\
         \n  value: lch(1% 0.5 3deg);\
         \n  space: lch;\
         \n  channels: 1% 0.5 3deg / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% -0.1 3deg));\n"),
                "a {\
         \n  value: lch(1% 0 3deg);\
         \n  space: lch;\
         \n  channels: 1% 0 3deg / 1;\
         \n}\n"
            );
        }
        mod degenerate {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn nan() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% calc(NaN) 3deg));\n"),
                    "a {\
         \n  value: lch(1% 0 3deg);\
         \n  space: lch;\
         \n  channels: 1% 0 3deg / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% calc(-infinity) 3deg));\n"),
                    "a {\
         \n  value: lch(1% 0 3deg);\
         \n  space: lch;\
         \n  channels: 1% 0 3deg / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn positive_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% calc(infinity) 3deg));\n"),
                    "a {\
         \n  value: lch(1% calc(infinity) 3deg);\
         \n  space: lch;\
         \n  channels: 1% calc(infinity) 3deg / 1;\
         \n}\n"
                );
            }
        }
    }
    mod hue {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 2 361deg));\n"),
                "a {\
         \n  value: lch(1% 2 1deg);\
         \n  space: lch;\
         \n  channels: 1% 2 1deg / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 2 -1deg));\n"),
                "a {\
         \n  value: lch(1% 2 359deg);\
         \n  space: lch;\
         \n  channels: 1% 2 359deg / 1;\
         \n}\n"
            );
        }
        mod degenerate {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn nan() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 2 calc(NaN)));\n"),
                    "a {\
         \n  value: lch(1% 2 calc(NaN * 1deg));\
         \n  space: lch;\
         \n  channels: 1% 2 calc(NaN * 1deg) / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 2 calc(-infinity)));\n"),
                    "a {\
         \n  value: lch(1% 2 calc(NaN * 1deg));\
         \n  space: lch;\
         \n  channels: 1% 2 calc(NaN * 1deg) / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn positive_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1% 2 calc(infinity)));\n"),
                    "a {\
         \n  value: lch(1% 2 calc(NaN * 1deg));\
         \n  space: lch;\
         \n  channels: 1% 2 calc(NaN * 1deg) / 1;\
         \n}\n"
                );
            }
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn in_range() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(1 2 3deg));\n"),
            "a {\
         \n  value: lch(1% 2 3deg);\
         \n  space: lch;\
         \n  channels: 1% 2 3deg / 1;\
         \n}\n"
        );
    }
    mod lightness {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(101 2 3deg));\n"),
                "a {\
         \n  value: lch(100% 2 3deg);\
         \n  space: lch;\
         \n  channels: 100% 2 3deg / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(-1 2 3deg));\n"),
                "a {\
         \n  value: lch(0% 2 3deg);\
         \n  space: lch;\
         \n  channels: 0% 2 3deg / 1;\
         \n}\n"
            );
        }
        mod degenerate {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn nan() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(calc(NaN) 2 3deg));\n"),
                    "a {\
         \n  value: lch(0% 2 3deg);\
         \n  space: lch;\
         \n  channels: 0% 2 3deg / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(calc(-infinity) 2 3deg));\n"),
                    "a {\
         \n  value: lch(0% 2 3deg);\
         \n  space: lch;\
         \n  channels: 0% 2 3deg / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn positive_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lch(calc(infinity) 2 3deg));\n"),
                    "a {\
         \n  value: lch(100% 2 3deg);\
         \n  space: lch;\
         \n  channels: 100% 2 3deg / 1;\
         \n}\n"
                );
            }
        }
    }
}
