//! Tests auto-converted from "sass-spec/spec/core_functions/color/oklch/no_alpha.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_alpha")
}

#[test]
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch($channels: 1% 0.2 3deg));\n"),
        "a {\
         \n  value: oklch(1% 0.2 3deg);\
         \n  space: oklch;\
         \n  channels: 1% 0.2 3deg / 1;\
         \n}\n"
    );
}
mod none {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn chroma() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% none 3deg));\n"),
            "a {\
         \n  value: oklch(1% none 3deg);\
         \n  space: oklch;\
         \n  channels: 1% none 3deg / 1;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% 0.2 none));\n"),
            "a {\
         \n  value: oklch(1% 0.2 none);\
         \n  space: oklch;\
         \n  channels: 1% 0.2 none / 1;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(none 0.2 3deg));\n"),
            "a {\
         \n  value: oklch(none 0.2 3deg);\
         \n  space: oklch;\
         \n  channels: none 0.2 3deg / 1;\
         \n}\n"
        );
    }
}
mod percent {
    #[allow(unused)]
    use super::runner;

    mod chroma {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% 101% 3deg));\n"),
                "a {\
         \n  value: oklch(1% 0.404 3deg);\
         \n  space: oklch;\
         \n  channels: 1% 0.404 3deg / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% -1% 3deg));\n"),
                "a {\
         \n  value: oklch(1% 0 3deg);\
         \n  space: oklch;\
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
             \n@include utils.inspect(oklch(1% 2% 3deg));\n"),
            "a {\
         \n  value: oklch(1% 0.008 3deg);\
         \n  space: oklch;\
         \n  channels: 1% 0.008 3deg / 1;\
         \n}\n"
        );
    }
    mod lightness {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(110% 0.2 3deg));\n"),
                "a {\
         \n  value: oklch(100% 0.2 3deg);\
         \n  space: oklch;\
         \n  channels: 100% 0.2 3deg / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(-1% 0.2 3deg));\n"),
                "a {\
         \n  value: oklch(0% 0.2 3deg);\
         \n  space: oklch;\
         \n  channels: 0% 0.2 3deg / 1;\
         \n}\n"
            );
        }
    }
}
mod unitless {
    #[allow(unused)]
    use super::runner;

    mod chroma {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% 0.5 3deg));\n"),
                "a {\
         \n  value: oklch(1% 0.5 3deg);\
         \n  space: oklch;\
         \n  channels: 1% 0.5 3deg / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% -0.1 3deg));\n"),
                "a {\
         \n  value: oklch(1% 0 3deg);\
         \n  space: oklch;\
         \n  channels: 1% 0 3deg / 1;\
         \n}\n"
            );
        }
        mod degenerate {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn nan() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% calc(NaN) 3deg));\n"),
                    "a {\
         \n  value: oklch(1% 0 3deg);\
         \n  space: oklch;\
         \n  channels: 1% 0 3deg / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% calc(-infinity) 3deg));\n"),
                    "a {\
         \n  value: oklch(1% 0 3deg);\
         \n  space: oklch;\
         \n  channels: 1% 0 3deg / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn positive_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% calc(infinity) 3deg));\n"),
                    "a {\
         \n  value: oklch(1% calc(infinity) 3deg);\
         \n  space: oklch;\
         \n  channels: 1% calc(infinity) 3deg / 1;\
         \n}\n"
                );
            }
        }
    }
    mod hue {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% 0.2 361deg));\n"),
                "a {\
         \n  value: oklch(1% 0.2 1deg);\
         \n  space: oklch;\
         \n  channels: 1% 0.2 1deg / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% 0.2 -1deg));\n"),
                "a {\
         \n  value: oklch(1% 0.2 359deg);\
         \n  space: oklch;\
         \n  channels: 1% 0.2 359deg / 1;\
         \n}\n"
            );
        }
        mod degenerate {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn nan() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% 2 calc(NaN)));\n"),
                    "a {\
         \n  value: oklch(1% 2 calc(NaN * 1deg));\
         \n  space: oklch;\
         \n  channels: 1% 2 calc(NaN * 1deg) / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% 2 calc(-infinity)));\n"),
                    "a {\
         \n  value: oklch(1% 2 calc(NaN * 1deg));\
         \n  space: oklch;\
         \n  channels: 1% 2 calc(NaN * 1deg) / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn positive_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% 2 calc(infinity)));\n"),
                    "a {\
         \n  value: oklch(1% 2 calc(NaN * 1deg));\
         \n  space: oklch;\
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
             \n@include utils.inspect(oklch(0.1 0.2 3deg));\n"),
            "a {\
         \n  value: oklch(10% 0.2 3deg);\
         \n  space: oklch;\
         \n  channels: 10% 0.2 3deg / 1;\
         \n}\n"
        );
    }
    mod lightness {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1.1 0.2 3deg));\n"),
                "a {\
         \n  value: oklch(100% 0.2 3deg);\
         \n  space: oklch;\
         \n  channels: 100% 0.2 3deg / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(-0.1 0.2 3deg));\n"),
                "a {\
         \n  value: oklch(0% 0.2 3deg);\
         \n  space: oklch;\
         \n  channels: 0% 0.2 3deg / 1;\
         \n}\n"
            );
        }
        mod degenerate {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn nan() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(calc(NaN) 0.2 3deg));\n"),
                    "a {\
         \n  value: oklch(0% 0.2 3deg);\
         \n  space: oklch;\
         \n  channels: 0% 0.2 3deg / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(calc(-infinity) 0.2 3deg));\n"),
                    "a {\
         \n  value: oklch(0% 0.2 3deg);\
         \n  space: oklch;\
         \n  channels: 0% 0.2 3deg / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn positive_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(calc(infinity) 0.2 3deg));\n"),
                    "a {\
         \n  value: oklch(100% 0.2 3deg);\
         \n  space: oklch;\
         \n  channels: 100% 0.2 3deg / 1;\
         \n}\n"
                );
            }
        }
    }
}
