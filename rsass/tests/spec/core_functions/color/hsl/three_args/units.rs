//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/three_args/units.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("units")
}

mod hue {
    use super::runner;

    #[test]
    fn angle() {
        assert_eq!(
            runner().ok("a {b: hsl(60rad, 100%, 50%)}\n"),
            "a {\
         \n  b: hsl(197.7467707849, 100%, 50%);\
         \n}\n"
        );
    }
    #[test]
    fn deg() {
        assert_eq!(
            runner().ok("a {b: hsl(0deg, 100%, 50%)}\n"),
            "a {\
         \n  b: hsl(0, 100%, 50%);\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: hsl(60, 100%, 50%)}\n"),
            "a {\
         \n  b: hsl(60, 100%, 50%);\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {b: hsl(60in, 100%, 50%)}\n"),
            "a {\
         \n  b: hsl(60, 100%, 50%);\
         \n}\n"
        );
    }
}
mod lightness {
    use super::runner;

    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 100%, 50)}\n"),
            "a {\
         \n  b: hsl(0, 100%, 50%);\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 100%, 50in)}\n"),
            "a {\
         \n  b: hsl(0, 100%, 50%);\
         \n}\n"
        );
    }
}
mod saturation {
    use super::runner;

    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 50, 50%)}\n"),
            "a {\
         \n  b: hsl(0, 50%, 50%);\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 50in, 50%)}\n"),
            "a {\
         \n  b: hsl(0, 50%, 50%);\
         \n}\n"
        );
    }
}
