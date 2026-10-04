//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/hsl.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hsl")
}

#[test]
#[ignore] // wrong result
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(turquoise, $saturation: 24%, $lightness: -48%)}\n"
        ),
        "a {\
         \n  b: rgb(6.2327249603%, 52.4966868045%, 47.87029062%);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(turquoise, $saturation: 24%, $lightness: -48%, $alpha: -70%)}\n"
        ),
        "a {\
         \n  b: rgba(6.2327249603%, 52.4966868045%, 47.87029062%, 0.3);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(rgba(turquoise, 0.7), $saturation: 24%, $lightness: -48%)}\n"
        ),
        "a {\
         \n  b: rgba(6.2327249603%, 52.4966868045%, 47.87029062%, 0.7);\
         \n}\n"
    );
}
mod lightness {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(red, $lightness: 94%)}\n"),
            "a {\
         \n  b: rgb(100%, 94%, 94%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(red, $lightness: -14%)}\n"),
            "a {\
         \n  b: rgb(86%, 0%, 0%);\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(red, $lightness: 100%)}\n"),
            "a {\
         \n  b: white;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(red, $lightness: -100%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(red, $lightness: 0%)}\n"),
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
             \na {b: color.scale($color: turquoise, $saturation: 24%, $lightness: -48%)}\n"
        ),
        "a {\
         \n  b: rgb(6.2327249603%, 52.4966868045%, 47.87029062%);\
         \n}\n"
    );
}
mod saturation {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(plum, $saturation: 67%)}\n"),
            "a {\
         \n  b: rgb(95.6%, 53.8117647059%, 95.6%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(plum, $saturation: -43%)}\n"),
            "a {\
         \n  b: rgb(81.5235294118%, 67.8882352941%, 81.5235294118%);\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(plum, $saturation: 100%)}\n"),
            "a {\
         \n  b: #ff7eff;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(plum, $saturation: -100%)}\n"),
            "a {\
         \n  b: rgb(74.7058823529%, 74.7058823529%, 74.7058823529%);\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(plum, $saturation: 0%)}\n"),
            "a {\
         \n  b: plum;\
         \n}\n"
        );
    }
}
