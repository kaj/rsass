//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/four_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("four_args")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    mod percent {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn above_max() {
            assert_eq!(
                runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(0, 0%, 0%, 250%)}\n"),
                "a {\
         \n  b: red;\
         \n}\n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(0, 0%, 0%, 100%)}\n"),
                "a {\
         \n  b: red;\
         \n}\n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(0, 0%, 0%, 0%)}\n"),
                "a {\
         \n  b: rgba(255, 0, 0, 0);\
         \n}\n"
            );
        }
        #[test]
        fn negative() {
            assert_eq!(
                runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(0, 0%, 0%, -10%)}\n"),
                "a {\
         \n  b: rgba(255, 0, 0, 0);\
         \n}\n"
            );
        }
        #[test]
        fn positive() {
            assert_eq!(
                runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(0, 0%, 0%, 45.6%)}\n"),
                "a {\
         \n  b: rgba(255, 0, 0, 0.456);\
         \n}\n"
            );
        }
    }
    mod unitless {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn above_max() {
            assert_eq!(
                runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(0, 0%, 0%, 250)}\n"),
                "a {\
         \n  b: red;\
         \n}\n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(0, 0%, 0%, 1)}\n"),
                "a {\
         \n  b: red;\
         \n}\n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(0, 0%, 0%, 0)}\n"),
                "a {\
         \n  b: rgba(255, 0, 0, 0);\
         \n}\n"
            );
        }
        #[test]
        fn negative() {
            assert_eq!(
                runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(0, 0%, 0%, -10)}\n"),
                "a {\
         \n  b: rgba(255, 0, 0, 0);\
         \n}\n"
            );
        }
        #[test]
        fn positive() {
            assert_eq!(
                runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(0, 0%, 0%, 0.456)}\n"),
                "a {\
         \n  b: rgba(255, 0, 0, 0.456);\
         \n}\n"
            );
        }
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "@use \'sass:color\';\
             \na {b: color.hwb($hue: 180, $whiteness: 30%, $blackness: 40%, $alpha: 0.4)}\n"
        ),
        "a {\
         \n  b: rgba(77, 153, 153, 0.4);\
         \n}\n"
    );
}
