//! Tests auto-converted from "sass-spec/spec/core_functions/color/change_color/no_rgb_hsl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_rgb_hsl")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner()
                .ok("a {b: change-color(rgba(red, 0.5), $alpha: 0.72)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.72);\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner()
                .ok("a {b: change-color(rgba(red, 0.5), $alpha: 0.36)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.36);\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: change-color(rgba(red, 0.5), $alpha: 1)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: change-color(rgba(red, 0.5), $alpha: 0)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0);\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: change-color($color: red)}\n"),
        "a {\
         \n  b: red;\
         \n}\n"
    );
}
#[test]
fn positional() {
    assert_eq!(
        runner().ok("a {b: change-color(red)}\n"),
        "a {\
         \n  b: red;\
         \n}\n"
    );
}
