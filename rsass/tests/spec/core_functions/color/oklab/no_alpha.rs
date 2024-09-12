//! Tests auto-converted from "sass-spec/spec/core_functions/color/oklab/no_alpha.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_alpha")
}

#[test]
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab($channels: 1% 2 3));\n"),
        "a {\
         \n  value: oklab(1% 2 3);\
         \n  space: oklab;\
         \n  channels: 1% 2 3 / 1;\
         \n}\n"
    );
}
mod none {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% none 3));\n"),
            "a {\
         \n  value: oklab(1% none 3);\
         \n  space: oklab;\
         \n  channels: 1% none 3 / 1;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2 none));\n"),
            "a {\
         \n  value: oklab(1% 2 none);\
         \n  space: oklab;\
         \n  channels: 1% 2 none / 1;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(none 2 3));\n"),
            "a {\
         \n  value: oklab(none 2 3);\
         \n  space: oklab;\
         \n  channels: none 2 3 / 1;\
         \n}\n"
        );
    }
}
mod percent {
    #[allow(unused)]
    use super::runner;

    mod ab {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 101% 150%));\n"),
                "a {\
         \n  value: oklab(1% 0.404 0.6);\
         \n  space: oklab;\
         \n  channels: 1% 0.404 0.6 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% -101% -150%));\n"),
                "a {\
         \n  value: oklab(1% -0.404 -0.6);\
         \n  space: oklab;\
         \n  channels: 1% -0.404 -0.6 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn in_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2% -3%));\n"),
                "a {\
         \n  value: oklab(1% 0.008 -0.012);\
         \n  space: oklab;\
         \n  channels: 1% 0.008 -0.012 / 1;\
         \n}\n"
            );
        }
    }
    mod lightness {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(110% 2 3));\n"),
                "a {\
         \n  value: oklab(100% 2 3);\
         \n  space: oklab;\
         \n  channels: 100% 2 3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(-1% 2 3));\n"),
                "a {\
         \n  value: oklab(0% 2 3);\
         \n  space: oklab;\
         \n  channels: 0% 2 3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn in_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2% -3%));\n"),
                "a {\
         \n  value: oklab(1% 0.008 -0.012);\
         \n  space: oklab;\
         \n  channels: 1% 0.008 -0.012 / 1;\
         \n}\n"
            );
        }
    }
}
#[test]
#[ignore] // unexepected error
fn relative_color() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(from #aaa l a b));\n"),
        "a {\
         \n  value: oklab(from #aaa l a b);\
         \n  type: string;\
         \n}\n"
    );
}
mod unitless {
    #[allow(unused)]
    use super::runner;

    mod a {
        #[allow(unused)]
        use super::runner;

        mod degenerate {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn nan() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% calc(NaN) -300));\n"),
                    "a {\
         \n  value: oklab(1% calc(NaN) -300);\
         \n  space: oklab;\
         \n  channels: 1% calc(NaN) -300 / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% calc(-infinity) -300));\n"),
                    "a {\
         \n  value: oklab(1% calc(-infinity) -300);\
         \n  space: oklab;\
         \n  channels: 1% calc(-infinity) -300 / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn positive_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% calc(infinity) -300));\n"),
                    "a {\
         \n  value: oklab(1% calc(infinity) -300);\
         \n  space: oklab;\
         \n  channels: 1% calc(infinity) -300 / 1;\
         \n}\n"
                );
            }
        }
    }
    mod ab {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 126 300));\n"),
                "a {\
         \n  value: oklab(1% 126 300);\
         \n  space: oklab;\
         \n  channels: 1% 126 300 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% -126 -300));\n"),
                "a {\
         \n  value: oklab(1% -126 -300);\
         \n  space: oklab;\
         \n  channels: 1% -126 -300 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn in_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2 -3));\n"),
                "a {\
         \n  value: oklab(1% 2 -3);\
         \n  space: oklab;\
         \n  channels: 1% 2 -3 / 1;\
         \n}\n"
            );
        }
    }
    mod b {
        #[allow(unused)]
        use super::runner;

        mod degenerate {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn nan() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2 calc(NaN)));\n"),
                    "a {\
         \n  value: oklab(1% 2 calc(NaN));\
         \n  space: oklab;\
         \n  channels: 1% 2 calc(NaN) / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2 calc(-infinity)));\n"),
                    "a {\
         \n  value: oklab(1% 2 calc(-infinity));\
         \n  space: oklab;\
         \n  channels: 1% 2 calc(-infinity) / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn positive_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2 calc(infinity)));\n"),
                    "a {\
         \n  value: oklab(1% 2 calc(infinity));\
         \n  space: oklab;\
         \n  channels: 1% 2 calc(infinity) / 1;\
         \n}\n"
                );
            }
        }
    }
    mod lightness {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1.1 2 3));\n"),
                "a {\
         \n  value: oklab(100% 2 3);\
         \n  space: oklab;\
         \n  channels: 100% 2 3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(-0.1 2 3));\n"),
                "a {\
         \n  value: oklab(0% 2 3);\
         \n  space: oklab;\
         \n  channels: 0% 2 3 / 1;\
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
             \n@include utils.inspect(oklab(calc(NaN) 2 -3));\n"),
                    "a {\
         \n  value: oklab(0% 2 -3);\
         \n  space: oklab;\
         \n  channels: 0% 2 -3 / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(calc(-infinity) 2 -3));\n"),
                    "a {\
         \n  value: oklab(0% 2 -3);\
         \n  space: oklab;\
         \n  channels: 0% 2 -3 / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn positive_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(calc(infinity) 2 -3));\n"),
                    "a {\
         \n  value: oklab(100% 2 -3);\
         \n  space: oklab;\
         \n  channels: 100% 2 -3 / 1;\
         \n}\n"
                );
            }
        }
        #[test]
        #[ignore] // unexepected error
        fn in_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(0.1 2 -3));\n"),
                "a {\
         \n  value: oklab(10% 2 -3);\
         \n  space: oklab;\
         \n  channels: 10% 2 -3 / 1;\
         \n}\n"
            );
        }
    }
}
