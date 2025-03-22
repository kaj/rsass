//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/rgb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rgb")
}

#[test]
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(sienna, $red: 12%, $green: 24%, $blue: 48%)}\n"
        ),
        "a {\
         \n  b: rgb(171.4, 123.52, 145.8);\
         \n}\n"
    );
}
#[test]
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(sienna, $red: 12%, $green: 24%, $blue: 48%, $alpha: -70%)}\n"
        ),
        "a {\
         \n  b: rgba(171.4, 123.52, 145.8, 0.3);\
         \n}\n"
    );
}
#[test]
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(rgba(sienna, 0.3), $red: 12%, $green: 24%, $blue: 48%)}\n"
        ),
        "a {\
         \n  b: rgba(171.4, 123.52, 145.8, 0.3);\
         \n}\n"
    );
}
mod blue {
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(salmon, $blue: 42%)}\n"),
            "a {\
         \n  b: rgb(250, 128, 173.22);\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(slategray, $blue: -16%)}\n"),
            "a {\
         \n  b: rgb(112, 128, 120.96);\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(black, $blue: 100%)}\n"),
            "a {\
         \n  b: blue;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(blue, $blue: -100%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(black, $blue: 0%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
}
mod green {
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(cadetblue, $green: 12%)}\n"),
            "a {\
         \n  b: rgb(95, 169.64, 160);\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(seagreen, $green: -86%)}\n"),
            "a {\
         \n  b: rgb(46, 19.46, 87);\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(black, $green: 100%)}\n"),
            "a {\
         \n  b: lime;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lime, $green: -100%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(black, $green: 0%)}\n"),
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
            "@use \"sass:color\";\
             \na {b: color.scale($color: sienna, $red: 12%, $green: 24%, $blue: 48%)}\n"
        ),
        "a {\
         \n  b: rgb(171.4, 123.52, 145.8);\
         \n}\n"
    );
}
mod red {
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(turquoise, $red: 86%)}\n"),
            "a {\
         \n  b: rgb(228.26, 224, 208);\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lightcoral, $red: -33%)}\n"),
            "a {\
         \n  b: rgb(160.8, 128, 128);\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(black, $red: 100%)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(red, $red: -100%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(black, $red: 0%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
}
