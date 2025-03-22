//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/no_channels.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_channels")
}

mod alpha {
    use super::runner;

    #[test]
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(rgba(red, 0.5), $alpha: 1)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(rgba(red, 0.5), $alpha: 2)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(rgba(red, 0.5), $alpha: -2)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(rgba(red, 0.5), $alpha: -1)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(rgba(red, 0.5), $alpha: 0.14)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.64);\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(rgba(red, 0.5), $alpha: -0.14)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.36);\
         \n}\n"
        );
    }
    #[test]
    fn max_remaining() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(rgba(red, 0.5), $alpha: 0.5)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    fn min_remaining() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(rgba(red, 0.5), $alpha: -0.5)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(rgba(red, 0.5), $alpha: 0)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.5);\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.adjust($color: red)}\n"),
        "a {\
         \n  b: red;\
         \n}\n"
    );
}
#[test]
fn positional() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red)}\n"),
        "a {\
         \n  b: red;\
         \n}\n"
    );
}
