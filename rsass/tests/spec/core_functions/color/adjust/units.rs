//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/units.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("units")
}

mod alpha {
    use super::runner;

    #[test]
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $alpha: -0.3%)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.7);\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $alpha: -0.3)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.7);\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $alpha: -0.3px)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.7);\
         \n}\n"
        );
    }
}
mod hue {
    use super::runner;

    #[test]
    fn angle() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $hue: 60rad)}\n"),
            "a {\
         \n  b: rgb(0, 179.576224164, 255);\
         \n}\n"
        );
    }
    #[test]
    fn deg() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $hue: 60deg)}\n"),
            "a {\
         \n  b: yellow;\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $hue: 60)}\n"),
            "a {\
         \n  b: yellow;\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $hue: 60in)}\n"),
            "a {\
         \n  b: yellow;\
         \n}\n"
        );
    }
}
mod lightness {
    use super::runner;

    #[test]
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $lightness: 10%)}\n"),
            "a {\
         \n  b: #ff3333;\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $lightness: 10)}\n"),
            "a {\
         \n  b: #ff3333;\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $lightness: 10in)}\n"),
            "a {\
         \n  b: #ff3333;\
         \n}\n"
        );
    }
}
mod saturation {
    use super::runner;

    #[test]
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $saturation: -10%)}\n"),
            "a {\
         \n  b: rgb(242.25, 12.75, 12.75);\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $saturation: -10)}\n"),
            "a {\
         \n  b: rgb(242.25, 12.75, 12.75);\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $saturation: -10in)}\n"),
            "a {\
         \n  b: rgb(242.25, 12.75, 12.75);\
         \n}\n"
        );
    }
}
