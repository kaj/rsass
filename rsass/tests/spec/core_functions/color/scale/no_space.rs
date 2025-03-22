//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/no_space.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_space")
}

mod alpha {
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(rgba(red, 0.5), $alpha: 14%)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.57);\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(rgba(red, 0.3), $alpha: -36%)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.192);\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(rgba(red, 0.5), $alpha: 100%)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(rgba(red, 0.5), $alpha: -100%)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(rgba(red, 0.5), $alpha: 0%)}\n"),
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
             \na {b: color.scale($color: red)}\n"),
        "a {\
         \n  b: red;\
         \n}\n"
    );
}
#[test]
fn positional() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.scale(red)}\n"),
        "a {\
         \n  b: red;\
         \n}\n"
    );
}
