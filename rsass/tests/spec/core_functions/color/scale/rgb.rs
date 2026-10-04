//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/rgb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rgb")
}

#[test]
#[ignore] // wrong result
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(sienna, $red: 12%, $green: 24%, $blue: 48%)}\n"
        ),
        "a {\
         \n  b: rgb(67.2156862745%, 48.4392156863%, 57.1764705882%);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(sienna, $red: 12%, $green: 24%, $blue: 48%, $alpha: -70%)}\n"
        ),
        "a {\
         \n  b: rgba(67.2156862745%, 48.4392156863%, 57.1764705882%, 0.3);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(rgba(sienna, 0.3), $red: 12%, $green: 24%, $blue: 48%)}\n"
        ),
        "a {\
         \n  b: rgba(67.2156862745%, 48.4392156863%, 57.1764705882%, 0.3);\
         \n}\n"
    );
}
mod blue {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(salmon, $blue: 42%)}\n"),
            "a {\
         \n  b: rgb(98.0392156863%, 50.1960784314%, 67.9294117647%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(slategray, $blue: -16%)}\n"),
            "a {\
         \n  b: rgb(43.9215686275%, 50.1960784314%, 47.4352941176%);\
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
    #[ignore] // wrong result
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(cadetblue, $green: 12%)}\n"),
            "a {\
         \n  b: rgb(37.2549019608%, 66.5254901961%, 62.7450980392%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(seagreen, $green: -86%)}\n"),
            "a {\
         \n  b: rgb(18.0392156863%, 7.631372549%, 34.1176470588%);\
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
#[ignore] // wrong result
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale($color: sienna, $red: 12%, $green: 24%, $blue: 48%)}\n"
        ),
        "a {\
         \n  b: rgb(67.2156862745%, 48.4392156863%, 57.1764705882%);\
         \n}\n"
    );
}
mod red {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(turquoise, $red: 86%)}\n"),
            "a {\
         \n  b: rgb(89.5137254902%, 87.8431372549%, 81.568627451%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lightcoral, $red: -33%)}\n"),
            "a {\
         \n  b: rgb(63.0588235294%, 50.1960784314%, 50.1960784314%);\
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
