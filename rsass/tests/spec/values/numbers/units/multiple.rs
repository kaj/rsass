//! Tests auto-converted from "sass-spec/spec/values/numbers/units/multiple.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("multiple")
}

mod division {
    use super::runner;

    mod by {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn multiple_denominators() {
            assert_eq!(
                runner().ok("a {\
             \n  b: calc(1 / (1 / 1px / 1rad));\
             \n}\n"),
                "a {\
         \n  b: calc(1px * 1rad);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn multiple_numerators() {
            assert_eq!(
                runner().ok("a {\
             \n  b: calc(1 / (1px * 1rad));\
             \n}\n"),
                "a {\
         \n  b: calc(1 / 1px / 1rad);\
         \n}\n"
            );
        }
    }
    mod cancels {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn both() {
            assert_eq!(
                runner().ok("$number: calc(1px * 1rad / 1ms / 1Hz);\
             \na {\
             \n  b: calc($number / (1px / 1ms));\
             \n}\n"),
                "a {\
         \n  b: calc(1rad / 1Hz);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn compatible() {
            assert_eq!(
                runner().ok("$number: calc(96px * 1rad / 1ms / 1Hz);\
             \na {\
             \n  b: calc($number / 1in);\
             \n}\n"),
                "a {\
         \n  b: calc(1rad / 1ms / 1Hz);\
         \n}\n"
            );
        }
        mod denominator {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn once() {
                assert_eq!(
                    runner().ok("$number: calc(1px * 1rad / 1ms / 1Hz);\
             \na {\
             \n  b: calc($number / (1 / 1ms));\
             \n}\n"),
                    "a {\
         \n  b: calc(1px * 1rad / 1Hz);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn twice() {
                assert_eq!(
                    runner().ok("$number: calc(1px * 1rad / 1ms / 1Hz);\
             \na {\
             \n  b: calc($number / (1 / 1ms / 1Hz));\
             \n}\n"),
                    "a {\
         \n  b: calc(1px * 1rad);\
         \n}\n"
                );
            }
        }
        mod numerator {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn once() {
                assert_eq!(
                    runner().ok("$number: calc(1px * 1rad / 1ms / 1Hz);\
             \na {\
             \n  b: calc($number / 1px);\
             \n}\n"),
                    "a {\
         \n  b: calc(1rad / 1ms / 1Hz);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn twice() {
                assert_eq!(
                    runner().ok("$number: calc(1px * 1rad / 1ms / 1Hz);\
             \na {\
             \n  b: calc($number / (1px * 1rad));\
             \n}\n"),
                    "a {\
         \n  b: calc(1 / 1ms / 1Hz);\
         \n}\n"
                );
            }
        }
        #[test]
        #[ignore] // unexepected error
        fn unknown() {
            assert_eq!(
        runner().ok(
            "// Units cancel even if they\'re totally unknown to Sass.\
             \n$number: calc(1foo * 1bar / 1baz / 1qux);\
             \na {\
             \n  b: calc($number / 1foo);\
             \n}\n"
        ),
        "a {\
         \n  b: calc(1bar / 1baz / 1qux);\
         \n}\n"
    );
        }
    }
}
#[test]
#[ignore] // unexepected error
fn multiple_denominators() {
    assert_eq!(
        runner().ok("a {\
             \n  b: calc(1 / 1px / 1rad);\
             \n}\n"),
        "a {\
         \n  b: calc(1 / 1px / 1rad);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn multiple_numerators() {
    assert_eq!(
        runner().ok("a {\
             \n  b: calc(1px * 1rad);\
             \n}\n"),
        "a {\
         \n  b: calc(1px * 1rad);\
         \n}\n"
    );
}
mod multiplication {
    use super::runner;

    mod cancels {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn both() {
            assert_eq!(
                runner().ok("$number: calc(1px * 1rad / 1ms / 1Hz);\
             \na {\
             \n  b: calc($number * (1ms / 1px));\
             \n}\n"),
                "a {\
         \n  b: calc(1rad / 1Hz);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn compatible() {
            assert_eq!(
                runner().ok("$number: calc(1px * 1rad / 1ms / 1Hz);\
             \na {\
             \n  b: calc($number * 1s);\
             \n}\n"),
                "a {\
         \n  b: calc(1000px * 1rad / 1Hz);\
         \n}\n"
            );
        }
        mod denominator {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn once() {
                assert_eq!(
                    runner().ok("$number: calc(1px * 1rad / 1ms / 1Hz);\
             \na {\
             \n  b: calc($number * 1ms);\
             \n}\n"),
                    "a {\
         \n  b: calc(1px * 1rad / 1Hz);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn twice() {
                assert_eq!(
                    runner().ok("$number: calc(1px * 1rad / 1ms / 1Hz);\
             \na {\
             \n  b: calc($number * (1ms * 1Hz));\
             \n}\n"),
                    "a {\
         \n  b: calc(1px * 1rad);\
         \n}\n"
                );
            }
        }
        mod numerator {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn once() {
                assert_eq!(
                    runner().ok("$number: calc(1px * 1rad / 1ms / 1Hz);\
             \na {\
             \n  b: calc($number * (1 / 1px));\
             \n}\n"),
                    "a {\
         \n  b: calc(1rad / 1ms / 1Hz);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn twice() {
                assert_eq!(
                    runner().ok("$number: calc(1px * 1rad / 1ms / 1Hz);\
             \na {\
             \n  b: calc($number * (1 / 1px / 1rad));\
             \n}\n"),
                    "a {\
         \n  b: calc(1 / 1ms / 1Hz);\
         \n}\n"
                );
            }
        }
        #[test]
        #[ignore] // unexepected error
        fn unknown() {
            assert_eq!(
        runner().ok(
            "// Units cancel even if they\'re totally unknown to Sass.\
             \n$number: calc(1foo * 1bar / 1baz / 1qux);\
             \na {\
             \n  b: calc($number * 1baz);\
             \n}\n"
        ),
        "a {\
         \n  b: calc(1foo * 1bar / 1qux);\
         \n}\n"
    );
        }
    }
}
