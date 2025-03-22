//! Tests auto-converted from "sass-spec/spec/core_functions/color/is_powerless/lch.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lch")
}

mod chroma {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn full_lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(lch(100% 0% 0deg), \"chroma\")}\n"),
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
             \na {b: color.is-powerless(lch(0% 0% 0deg), \"chroma\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
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
             \na {b: color.is-powerless(lch(100% 50% 0deg), \"hue\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near_zero_chroma() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.is-powerless(lch(50% 0.000000000001% 0deg), \"hue\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn positive_chroma() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(lch(50% 50% 0deg), \"hue\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero_chroma() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(lch(50% 0% 0deg), \"hue\")}\n"),
            "a {\
         \n  b: true;\
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
             \na {b: color.is-powerless(lch(0% 50% 0deg), \"hue\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
}
