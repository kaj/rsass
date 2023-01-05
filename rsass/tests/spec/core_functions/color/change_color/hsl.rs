//! Tests auto-converted from "sass-spec/spec/core_functions/color/change_color/hsl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hsl")
}

#[test]
fn all() {
    assert_eq!(
        runner().ok(
            "a {b: change-color(black, $hue: 12, $saturation: 24%, $lightness: 48%)}\n"
        ),
        "a {\
         \n  b: #98695d;\
         \n}\n"
    );
}
#[test]
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "a {b: change-color(black, $hue: 12, $saturation: 24%, $lightness: 48%, $alpha: 0.7)}\n"
        ),
        "a {\
         \n  b: rgba(152, 105, 93, 0.7);\
         \n}\n"
    );
}
#[test]
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "a {b: change-color(rgba(black, 0.7), $hue: 12, $saturation: 24%, $lightness: 48%)}\n"
        ),
        "a {\
         \n  b: rgba(152, 105, 93, 0.7);\
         \n}\n"
    );
}
mod hue {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn above_max() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $hue: 540)}\n"),
            "a {\
         \n  b: aqua;\
         \n}\n"
        );
    }
    #[test]
    fn fraction() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $hue: 0.5)}\n"),
            "a {\
         \n  b: #ff0200;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $hue: 359)}\n"),
            "a {\
         \n  b: #ff0004;\
         \n}\n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $hue: 123)}\n"),
            "a {\
         \n  b: #00ff0d;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: change-color(blue, $hue: 0)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $hue: -60)}\n"),
            "a {\
         \n  b: fuchsia;\
         \n}\n"
        );
    }
}
mod lightness {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn fraction() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $lightness: 0.5%)}\n"),
            "a {\
         \n  b: #030000;\
         \n}\n"
        );
    }
    #[test]
    fn high() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $lightness: 63%)}\n"),
            "a {\
         \n  b: #ff4242;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $lightness: 27%)}\n"),
            "a {\
         \n  b: #8a0000;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $lightness: 100%)}\n"),
            "a {\
         \n  b: white;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $lightness: 0%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "a {b: change-color($color: black, $hue: 12, $saturation: 24%, $lightness: 48%)}\n"
        ),
        "a {\
         \n  b: #98695d;\
         \n}\n"
    );
}
mod saturation {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("a {b: change-color(plum, $saturation: 76%)}\n"),
            "a {\
         \n  b: #f08df0;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("a {b: change-color(plum, $saturation: 14%)}\n"),
            "a {\
         \n  b: #c8b5c8;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: change-color(plum, $saturation: 100%)}\n"),
            "a {\
         \n  b: #ff7eff;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: change-color(plum, $saturation: 0%)}\n"),
            "a {\
         \n  b: #bfbfbf;\
         \n}\n"
        );
    }
}
