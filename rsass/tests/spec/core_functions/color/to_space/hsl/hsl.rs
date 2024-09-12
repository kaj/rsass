//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/hsl/hsl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hsl")
}

#[test]
#[ignore] // unexepected error
fn in_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(0deg 20% 40%), hsl)}\n"),
        "a {\
         \n  b: hsl(0, 20%, 40%);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(none 20% 30%), hsl)}\n"),
            "a {\
         \n  b: hsl(none 20% 30%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg 20% none), hsl)}\n"),
            "a {\
         \n  b: hsl(10deg 20% none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn saturation() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg none 30%), hsl)}\n"),
            "a {\
         \n  b: hsl(10deg none 30%);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn out_of_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(20deg 200% -50%), hsl)}\n"),
        "a {\
         \n  b: hsl(20, 200%, -50%);\
         \n}\n"
    );
}
