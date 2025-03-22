//! Tests auto-converted from "sass-spec/spec/core_functions/color/is_powerless/hsl.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hsl")
}

mod hue {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn full_lightness() {
        assert_eq!(
        runner().ok(
            "// These used to be considered powerless by the CSS spec, but are no longer, so\
             \n// Sass shouldn\'t consdier it powerless either.\
             \n@use \"sass:color\";\
             \na {b: color.is-powerless(hsl(0deg 50% 100%), \"hue\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near_zero_saturation() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.is-powerless(hsl(0deg 0.000000000001% 50%), \"hue\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn positive_saturation() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(hsl(0deg 50% 50%), \"hue\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero_lightness() {
        assert_eq!(
        runner().ok(
            "// These used to be considered powerless by the CSS spec, but are no longer, so\
             \n// Sass shouldn\'t consdier it powerless either.\
             \n@use \"sass:color\";\
             \na {b: color.is-powerless(hsl(0deg 50% 0%), \"hue\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero_saturation() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(hsl(0deg 0% 50%), \"hue\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
}
mod saturation {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn full_lightness() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.is-powerless(hsl(0deg 0% 100%), \"saturation\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero_lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(hsl(0deg 0% 0%), \"saturation\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
