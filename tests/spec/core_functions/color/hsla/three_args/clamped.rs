//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/three_args/clamped.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod lightness {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn above() {
        assert_eq!(
            runner().ok("a {b: hsla(0, 100%, 500%)}\n"),
            "a {\
         \n  b: white;\
         \n}\n"
        );
    }
    #[test]
    fn below() {
        assert_eq!(
            runner().ok("a {b: hsla(0, 100%, -100%)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
}
mod saturation {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn above() {
        assert_eq!(
            runner().ok("a {b: hsla(0, 500%, 50%)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    fn below() {
        assert_eq!(
            runner().ok("a {b: hsla(0, -100%, 50%)}\n"),
            "a {\
         \n  b: gray;\
         \n}\n"
        );
    }
}
