//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/three_args/clamped.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("clamped")
}

mod lightness {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn above() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 100%, 500%)}\n"),
            "a {\
         \n  b: hsl(0, 100%, 100%);\
         \n}\n"
        );
    }
    #[test]
    fn below() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 100%, -100%)}\n"),
            "a {\
         \n  b: hsl(0, 100%, 0%);\
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
            runner().ok("a {b: hsl(0, 500%, 50%)}\n"),
            "a {\
         \n  b: hsl(0, 100%, 50%);\
         \n}\n"
        );
    }
    #[test]
    fn below() {
        assert_eq!(
            runner().ok("a {b: hsl(0, -100%, 50%)}\n"),
            "a {\
         \n  b: hsl(0, 0%, 50%);\
         \n}\n"
        );
    }
}
