//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/hsl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hsl")
}

#[test]
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(turquoise, $saturation: 24%, $lightness: -48%)}\n"
        ),
        "a {\
         \n  b: rgb(15.8934486486, 133.8665513514, 122.0692410811);\
         \n}\n"
    );
}
#[test]
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(turquoise, $saturation: 24%, $lightness: -48%, $alpha: -70%)}\n"
        ),
        "a {\
         \n  b: rgba(15.8934486486, 133.8665513514, 122.0692410811, 0.3);\
         \n}\n"
    );
}
#[test]
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(rgba(turquoise, 0.7), $saturation: 24%, $lightness: -48%)}\n"
        ),
        "a {\
         \n  b: rgba(15.8934486486, 133.8665513514, 122.0692410811, 0.7);\
         \n}\n"
    );
}
mod lightness {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(red, $lightness: 94%)}\n"),
            "a {\
         \n  b: rgb(255, 239.7, 239.7);\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(red, $lightness: -14%)}\n"),
            "a {\
         \n  b: rgb(219.3, 0, 0);\
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
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale($color: turquoise, $saturation: 24%, $lightness: -48%)}\n"
        ),
        "a {\
         \n  b: rgb(15.8934486486, 133.8665513514, 122.0692410811);\
         \n}\n"
    );
}
mod saturation {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(plum, $saturation: 67%)}\n"),
            "a {\
         \n  b: rgb(243.78, 137.22, 243.78);\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(plum, $saturation: -43%)}\n"),
            "a {\
         \n  b: rgb(207.885, 173.115, 207.885);\
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
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(plum, $saturation: -100%)}\n"),
            "a {\
         \n  b: rgb(190.5, 190.5, 190.5);\
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
