//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/three_args/units.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod hue {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn angle() {
        assert_eq!(
            runner().ok("a {b: hsla(60rad, 100%, 50%)}\n"),
            "a {\
         \n  b: yellow;\
         \n}\n"
        );
    }
    #[test]
    fn deg() {
        assert_eq!(
            runner().ok("a {b: hsla(0deg, 100%, 50%)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: hsla(60, 100%, 50%)}\n"),
            "a {\
         \n  b: yellow;\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {b: hsla(60in, 100%, 50%)}\n"),
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
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: hsla(0, 100%, 50)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {b: hsla(0, 100%, 50in)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
}
mod saturation {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: hsla(0, 50, 50%)}\n"),
            "a {\
         \n  b: #bf4040;\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {b: hsla(0, 50in, 50%)}\n"),
            "a {\
         \n  b: #bf4040;\
         \n}\n"
        );
    }
}
