//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/three_args/units.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("units")
}

mod hue {
    use super::runner;

    #[test]
    fn deg() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0deg, 30%, 40%)}\n"),
            "a {\
         \n  b: hsl(0, 33.3333333333%, 45%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_angle() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(1in, 30%, 40%)}\n"),
            "a {\
         \n  b: hsl(1, 33.3333333333%, 45%);\
         \n}\n"
        );
    }
    #[test]
    fn rad() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(1rad, 30%, 40%)}\n"),
            "a {\
         \n  b: hsl(57.2957795131, 33.3333333333%, 45%);\
         \n}\n"
        );
    }
}
