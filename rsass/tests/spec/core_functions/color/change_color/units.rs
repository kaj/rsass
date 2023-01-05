//! Tests auto-converted from "sass-spec/spec/core_functions/color/change_color/units.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("units")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn percent() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $alpha: 0.5%)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $alpha: 0.5)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.5);\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {b: adjust-color(red, $alpha: 0.5px)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
}
mod hue {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn angle() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $hue: 60rad)}\n"),
            "a {\
         \n  b: #00b4ff;\
         \n}\n"
        );
    }
    #[test]
    fn deg() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $hue: 60deg)}\n"),
            "a {\
         \n  b: yellow;\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $hue: 60)}\n"),
            "a {\
         \n  b: yellow;\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $hue: 60in)}\n"),
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
            runner().ok("a {b: change-color(red, $lightness: 30%)}\n"),
            "a {\
         \n  b: #990000;\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $lightness: 30)}\n"),
            "a {\
         \n  b: #990000;\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $lightness: 30in)}\n"),
            "a {\
         \n  b: #990000;\
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
            runner().ok("a {b: change-color(red, $saturation: 50%)}\n"),
            "a {\
         \n  b: #bf4040;\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $saturation: 50)}\n"),
            "a {\
         \n  b: #bf4040;\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {b: change-color(red, $saturation: 50in)}\n"),
            "a {\
         \n  b: #bf4040;\
         \n}\n"
        );
    }
}
