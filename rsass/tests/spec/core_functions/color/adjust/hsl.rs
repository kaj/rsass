//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/hsl.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hsl")
}

#[test]
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(black, $hue: 12, $saturation: 24%, $lightness: 48%)}\n"
        ),
        "a {\
         \n  b: rgb(151.776, 104.7744, 93.024);\
         \n}\n"
    );
}
#[test]
fn alpha_arg() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.adjust(\
             \n    black,\
             \n    $hue: 12,\
             \n    $saturation: 24%,\
             \n    $lightness: 48%,\
             \n    $alpha: -0.7\
             \n  );\
             \n}\n"),
        "a {\
         \n  b: rgba(151.776, 104.7744, 93.024, 0.3);\
         \n}\n"
    );
}
#[test]
fn alpha_arg_above_max() {
    assert_eq!(
        runner().ok("// Regression test for sass/dart-sass#708.\
             \n@use \"sass:color\";\
             \na {\
             \n  b: color.adjust(\
             \n    black,\
             \n    $hue: 12,\
             \n    $saturation: 24%,\
             \n    $lightness: 48%,\
             \n    $alpha: 0.7\
             \n  );\
             \n}\n"),
        "a {\
         \n  b: rgb(151.776, 104.7744, 93.024);\
         \n}\n"
    );
}
#[test]
fn alpha_input() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.adjust(\
             \n    rgba(black, 0.7),\
             \n    $hue: 12,\
             \n    $saturation: 24%,\
             \n    $lightness: 48%\
             \n  );\
             \n}\n"),
        "a {\
         \n  b: rgba(151.776, 104.7744, 93.024, 0.7);\
         \n}\n"
    );
}
mod hue {
    use super::runner;

    #[test]
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $hue: 540)}\n"),
            "a {\
         \n  b: aqua;\
         \n}\n"
        );
    }
    #[test]
    fn fraction() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $hue: 0.5)}\n"),
            "a {\
         \n  b: rgb(255, 2.125, 0);\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $hue: 359)}\n"),
            "a {\
         \n  b: rgb(255, 0, 4.25);\
         \n}\n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $hue: 123)}\n"),
            "a {\
         \n  b: rgb(0, 255, 12.75);\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(blue, $hue: 0)}\n"),
            "a {\
         \n  b: blue;\
         \n}\n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $hue: -180)}\n"),
            "a {\
         \n  b: aqua;\
         \n}\n"
        );
    }
}
mod lightness {
    use super::runner;

    #[test]
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $lightness: 100%)}\n"),
            "a {\
         \n  b: hsl(0, 100%, 150%);\
         \n}\n"
        );
    }
    #[test]
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $lightness: 200%)}\n"),
            "a {\
         \n  b: hsl(0, 100%, 250%);\
         \n}\n"
        );
    }
    #[test]
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $lightness: -200%)}\n"),
            "a {\
         \n  b: hsl(300, 47.2868217054%, -125.2941176471%);\
         \n}\n"
        );
    }
    #[test]
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $lightness: -100%)}\n"),
            "a {\
         \n  b: hsl(300, 47.2868217054%, -25.2941176471%);\
         \n}\n"
        );
    }
    #[test]
    fn fraction() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $lightness: 0.5%)}\n"),
            "a {\
         \n  b: rgb(255, 2.55, 2.55);\
         \n}\n"
        );
    }
    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $lightness: 14%)}\n"),
            "a {\
         \n  b: rgb(255, 71.4, 71.4);\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $lightness: -14%)}\n"),
            "a {\
         \n  b: rgb(183.6, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn max_remaining() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $lightness: 50%)}\n"),
            "a {\
         \n  b: white;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $lightness: -100%)}\n"),
            "a {\
         \n  b: hsl(0, 100%, -50%);\
         \n}\n"
        );
    }
    #[test]
    fn min_remaining() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $lightness: -50%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $lightness: 0%)}\n"),
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
            "@use \"sass:color\";\
             \na {b: color.adjust($color: black, $hue: 12, $saturation: 24%, $lightness: 48%)}\n"
        ),
        "a {\
         \n  b: rgb(151.776, 104.7744, 93.024);\
         \n}\n"
    );
}
mod saturation {
    use super::runner;

    #[test]
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $saturation: 100%)}\n"),
            "a {\
         \n  b: hsl(300, 147.2868217054%, 74.7058823529%);\
         \n}\n"
        );
    }
    #[test]
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $saturation: 200%)}\n"),
            "a {\
         \n  b: hsl(300, 247.2868217054%, 74.7058823529%);\
         \n}\n"
        );
    }
    #[test]
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $saturation: -200%)}\n"),
            "a {\
         \n  b: rgb(190.5, 190.5, 190.5);\
         \n}\n"
        );
    }
    #[test]
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $saturation: -100%)}\n"),
            "a {\
         \n  b: rgb(190.5, 190.5, 190.5);\
         \n}\n"
        );
    }
    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $saturation: 14%)}\n"),
            "a {\
         \n  b: rgb(230.03, 150.97, 230.03);\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $saturation: -14%)}\n"),
            "a {\
         \n  b: rgb(211.97, 169.03, 211.97);\
         \n}\n"
        );
    }
    #[test]
    fn max_remaining() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $saturation: 53%)}\n"),
            "a {\
         \n  b: hsl(300, 100.2868217054%, 74.7058823529%);\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $saturation: -100%)}\n"),
            "a {\
         \n  b: rgb(190.5, 190.5, 190.5);\
         \n}\n"
        );
    }
    #[test]
    fn min_remaining() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $saturation: -48%)}\n"),
            "a {\
         \n  b: rgb(190.5, 190.5, 190.5);\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $saturation: 0%)}\n"),
            "a {\
         \n  b: plum;\
         \n}\n"
        );
    }
}
