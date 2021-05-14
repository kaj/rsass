//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale_color/no_rgb_hsl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("a {b: scale-color(rgba(red, 0.5), $alpha: 14%)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.57);\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("a {b: scale-color(rgba(red, 0.3), $alpha: -36%)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.192);\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: scale-color(rgba(red, 0.5), $alpha: 100%)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner()
                .ok("a {b: scale-color(rgba(red, 0.5), $alpha: -100%)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("a {b: scale-color(rgba(red, 0.5), $alpha: 0%)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.5);\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: scale-color($color: red)}\n"),
        "a {\
         \n  b: red;\
         \n}\n"
    );
}
#[test]
fn positional() {
    assert_eq!(
        runner().ok("a {b: scale-color(red)}\n"),
        "a {\
         \n  b: red;\
         \n}\n"
    );
}
