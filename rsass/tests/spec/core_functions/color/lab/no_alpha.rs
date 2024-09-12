//! Tests auto-converted from "sass-spec/spec/core_functions/color/lab/no_alpha.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_alpha")
}

#[test]
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab($channels: 1% 2 3));\n"),
        "a {\
         \n  value: lab(1% 2 3);\
         \n  space: lab;\
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
             \n@include utils.inspect(lab(1% none 3));\n"),
            "a {\
         \n  value: lab(1% none 3);\
         \n  space: lab;\
         \n  channels: 1% none 3 / 1;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 none));\n"),
            "a {\
         \n  value: lab(1% 2 none);\
         \n  space: lab;\
         \n  channels: 1% 2 none / 1;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(none 2 3));\n"),
            "a {\
         \n  value: lab(none 2 3);\
         \n  space: lab;\
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
             \n@include utils.inspect(lab(1% 101% 150%));\n"),
                "a {\
         \n  value: lab(1% 126.25 187.5);\
         \n  space: lab;\
         \n  channels: 1% 126.25 187.5 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% -101% -150%));\n"),
                "a {\
         \n  value: lab(1% -126.25 -187.5);\
         \n  space: lab;\
         \n  channels: 1% -126.25 -187.5 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn in_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2% -3%));\n"),
                "a {\
         \n  value: lab(1% 2.5 -3.75);\
         \n  space: lab;\
         \n  channels: 1% 2.5 -3.75 / 1;\
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
             \n@include utils.inspect(lab(110% 2 3));\n"),
                "a {\
         \n  value: lab(100% 2 3);\
         \n  space: lab;\
         \n  channels: 100% 2 3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(-1% 2 3));\n"),
                "a {\
         \n  value: lab(0% 2 3);\
         \n  space: lab;\
         \n  channels: 0% 2 3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn in_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2% -3%));\n"),
                "a {\
         \n  value: lab(1% 2.5 -3.75);\
         \n  space: lab;\
         \n  channels: 1% 2.5 -3.75 / 1;\
         \n}\n"
            );
        }
    }
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
             \n@include utils.inspect(lab(1% calc(NaN) -3));\n"),
                    "a {\
         \n  value: lab(1% calc(NaN) -3);\
         \n  space: lab;\
         \n  channels: 1% calc(NaN) -3 / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% calc(-infinity) -3));\n"),
                    "a {\
         \n  value: lab(1% calc(-infinity) -3);\
         \n  space: lab;\
         \n  channels: 1% calc(-infinity) -3 / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn positive_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% calc(infinity) -3));\n"),
                    "a {\
         \n  value: lab(1% calc(infinity) -3);\
         \n  space: lab;\
         \n  channels: 1% calc(infinity) -3 / 1;\
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
             \n@include utils.inspect(lab(1% 126 300));\n"),
                "a {\
         \n  value: lab(1% 126 300);\
         \n  space: lab;\
         \n  channels: 1% 126 300 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% -126 -300));\n"),
                "a {\
         \n  value: lab(1% -126 -300);\
         \n  space: lab;\
         \n  channels: 1% -126 -300 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn in_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 -3));\n"),
                "a {\
         \n  value: lab(1% 2 -3);\
         \n  space: lab;\
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
             \n@include utils.inspect(lab(1% 2 calc(NaN)));\n"),
                    "a {\
         \n  value: lab(1% 2 calc(NaN));\
         \n  space: lab;\
         \n  channels: 1% 2 calc(NaN) / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 calc(-infinity)));\n"),
                    "a {\
         \n  value: lab(1% 2 calc(-infinity));\
         \n  space: lab;\
         \n  channels: 1% 2 calc(-infinity) / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn positive_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 calc(infinity)));\n"),
                    "a {\
         \n  value: lab(1% 2 calc(infinity));\
         \n  space: lab;\
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
             \n@include utils.inspect(lab(101 2 3));\n"),
                "a {\
         \n  value: lab(100% 2 3);\
         \n  space: lab;\
         \n  channels: 100% 2 3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(-1 2 3));\n"),
                "a {\
         \n  value: lab(0% 2 3);\
         \n  space: lab;\
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
             \n@include utils.inspect(lab(calc(NaN) 2 -3));\n"),
                    "a {\
         \n  value: lab(0% 2 -3);\
         \n  space: lab;\
         \n  channels: 0% 2 -3 / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(calc(-infinity) 2 -3));\n"),
                    "a {\
         \n  value: lab(0% 2 -3);\
         \n  space: lab;\
         \n  channels: 0% 2 -3 / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn positive_infinity() {
                assert_eq!(
                    runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(calc(infinity) 2 -3));\n"),
                    "a {\
         \n  value: lab(100% 2 -3);\
         \n  space: lab;\
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
             \n@include utils.inspect(lab(10 2 -3));\n"),
                "a {\
         \n  value: lab(10% 2 -3);\
         \n  space: lab;\
         \n  channels: 10% 2 -3 / 1;\
         \n}\n"
            );
        }
    }
}
