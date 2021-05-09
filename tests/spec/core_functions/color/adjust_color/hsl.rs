//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_color/hsl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn all() {
    assert_eq!(
        runner().ok(
            "a {b: adjust-color(black, $hue: 12, $saturation: 24%, $lightness: 48%)}\n"
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
            "a {b: adjust-color(black, $hue: 12, $saturation: 24%, $lightness: 48%, $alpha: -0.7)}\n"
        ),
        "a {\
         \n  b: rgba(152, 105, 93, 0.3);\
         \n}\n"
    );
}
#[test]
fn alpha_arg_above_max() {
    assert_eq!(
        runner().ok(
            "// Regression test for sass/dart-sass#708.\
             \na {b: adjust-color(black, $hue: 12, $saturation: 24%, $lightness: 48%, $alpha: 0.7)}\n"
        ),
        "a {\
         \n  b: #98695d;\
         \n}\n"
    );
}
#[test]
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "a {b: adjust-color(rgba(black, 0.7), $hue: 12, $saturation: 24%, $lightness: 48%)}\n"
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
            runner().ok("a {b: adjust-color(red, $hue: 540)}\n"),
            "a {\
         \n  b: aqua;\
         \n}\n"
        );
    }
    #[test]
    fn fraction() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $hue: 0.5)}\n"),
            "a {\
         \n  b: #ff0200;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $hue: 359)}\n"),
            "a {\
         \n  b: #ff0004;\
         \n}\n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $hue: 123)}\n"),
            "a {\
         \n  b: #00ff0d;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: adjust-color(blue, $hue: 0)}\n"),
            "a {\
         \n  b: blue;\
         \n}\n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $hue: -180)}\n"),
            "a {\
         \n  b: aqua;\
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
            runner().ok("a {b: adjust-color(red, $lightness: 0.5%)}\n"),
            "a {\
         \n  b: #ff0303;\
         \n}\n"
        );
    }
    #[test]
    fn high() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $lightness: 14%)}\n"),
            "a {\
         \n  b: #ff4747;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $lightness: -14%)}\n"),
            "a {\
         \n  b: #b80000;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $lightness: 100%)}\n"),
            "a {\
         \n  b: white;\
         \n}\n"
        );
    }
    #[test]
    fn max_remaining() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $lightness: 50%)}\n"),
            "a {\
         \n  b: white;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $lightness: -100%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn min_remaining() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $lightness: -50%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $lightness: 0%)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "a {b: adjust-color($color: black, $hue: 12, $saturation: 24%, $lightness: 48%)}\n"
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
            runner().ok("a {b: adjust-color(plum, $saturation: 14%)}\n"),
            "a {\
         \n  b: #e697e6;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("a {b: adjust-color(plum, $saturation: -14%)}\n"),
            "a {\
         \n  b: #d4a9d4;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("a {b: adjust-color(plum, $saturation: 100%)}\n"),
            "a {\
         \n  b: #ff7eff;\
         \n}\n"
        );
    }
    #[test]
    fn max_remaining() {
        assert_eq!(
            runner().ok("a {b: adjust-color(plum, $saturation: 53%)}\n"),
            "a {\
         \n  b: #ff7eff;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("a {b: adjust-color(plum, $saturation: -100%)}\n"),
            "a {\
         \n  b: #bfbfbf;\
         \n}\n"
        );
    }
    #[test]
    fn min_remaining() {
        assert_eq!(
            runner().ok("a {b: adjust-color(plum, $saturation: -48%)}\n"),
            "a {\
         \n  b: #bfbfbf;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("a {b: adjust-color(plum, $saturation: 0%)}\n"),
            "a {\
         \n  b: plum;\
         \n}\n"
        );
    }
}
