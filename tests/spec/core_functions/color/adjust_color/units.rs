//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_color/units.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("units")
}

mod hue {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn angle() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $hue: 60rad)}\n"),
            "a {\
         \n  b: yellow;\
         \n}\n"
        );
    }
    #[test]
    fn deg() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $hue: 60deg)}\n"),
            "a {\
         \n  b: yellow;\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $hue: 60)}\n"),
            "a {\
         \n  b: yellow;\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $hue: 60in)}\n"),
            "a {\
         \n  b: yellow;\
         \n}\n"
        );
    }
}
mod lightness {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn percent() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $lightness: 10%)}\n"),
            "a {\
         \n  b: #ff3333;\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $lightness: 10)}\n"),
            "a {\
         \n  b: #ff3333;\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $lightness: 10in)}\n"),
            "a {\
         \n  b: #ff3333;\
         \n}\n"
        );
    }
}
mod saturation {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn percent() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $saturation: -10%)}\n"),
            "a {\
         \n  b: #f20d0d;\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $saturation: -10)}\n"),
            "a {\
         \n  b: #f20d0d;\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $saturation: -10in)}\n"),
            "a {\
         \n  b: #f20d0d;\
         \n}\n"
        );
    }
}
