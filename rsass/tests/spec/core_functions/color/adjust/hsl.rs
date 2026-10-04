//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/hsl.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hsl")
}

#[test]
#[ignore] // wrong result
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(black, $hue: 12, $saturation: 24%, $lightness: 48%)}\n"
        ),
        "a {\
         \n  b: rgb(59.52%, 41.088%, 36.48%);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
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
         \n  b: rgba(59.52%, 41.088%, 36.48%, 0.3);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
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
         \n  b: rgb(59.52%, 41.088%, 36.48%);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
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
         \n  b: rgba(59.52%, 41.088%, 36.48%, 0.7);\
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
    #[ignore] // wrong result
    fn fraction() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $hue: 0.5)}\n"),
            "a {\
         \n  b: rgb(100%, 0.8333333333%, 0%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $hue: 359)}\n"),
            "a {\
         \n  b: rgb(100%, 0%, 1.6666666667%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn middle() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $hue: 123)}\n"),
            "a {\
         \n  b: rgb(0%, 100%, 5%);\
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
    #[ignore] // wrong result
    fn fraction() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $lightness: 0.5%)}\n"),
            "a {\
         \n  b: rgb(100%, 1%, 1%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $lightness: 14%)}\n"),
            "a {\
         \n  b: rgb(100%, 28%, 28%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $lightness: -14%)}\n"),
            "a {\
         \n  b: rgb(72%, 0%, 0%);\
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
#[ignore] // wrong result
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust($color: black, $hue: 12, $saturation: 24%, $lightness: 48%)}\n"
        ),
        "a {\
         \n  b: rgb(59.52%, 41.088%, 36.48%);\
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
    #[ignore] // wrong result
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $saturation: -200%)}\n"),
            "a {\
         \n  b: rgb(74.7058823529%, 74.7058823529%, 74.7058823529%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $saturation: -100%)}\n"),
            "a {\
         \n  b: rgb(74.7058823529%, 74.7058823529%, 74.7058823529%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $saturation: 14%)}\n"),
            "a {\
         \n  b: rgb(90.2078431373%, 59.2039215686%, 90.2078431373%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $saturation: -14%)}\n"),
            "a {\
         \n  b: rgb(83.1254901961%, 66.2862745098%, 83.1254901961%);\
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
    #[ignore] // wrong result
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $saturation: -100%)}\n"),
            "a {\
         \n  b: rgb(74.7058823529%, 74.7058823529%, 74.7058823529%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn min_remaining() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(plum, $saturation: -48%)}\n"),
            "a {\
         \n  b: rgb(74.7058823529%, 74.7058823529%, 74.7058823529%);\
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
