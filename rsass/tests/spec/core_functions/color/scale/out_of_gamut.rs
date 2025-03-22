//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/out_of_gamut.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("out_of_gamut")
}

mod above_gamut {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn down() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(srgb 1.2 0.5 0.7), $red: -10%)}\n"),
            "a {\
         \n  b: color(srgb 1.08 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn up() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(srgb 1.2 0.5 0.7), $red: 10%)}\n"),
            "a {\
         \n  b: color(srgb 1.2 0.5 0.7);\
         \n}\n"
        );
    }
}
mod below_gamut {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn down() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(srgb -0.5 0.5 0.7), $red: -10%)}\n"),
            "a {\
         \n  b: color(srgb -0.5 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn up() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(srgb -0.5 0.5 0.7), $red: 10%)}\n"),
            "a {\
         \n  b: color(srgb -0.35 0.5 0.7);\
         \n}\n"
        );
    }
}
