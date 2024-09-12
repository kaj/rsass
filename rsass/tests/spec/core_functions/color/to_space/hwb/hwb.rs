//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/hwb/hwb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hwb")
}

#[test]
#[ignore] // unexepected error
fn in_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 20% 30%), hwb)}\n"),
        "a {\
         \n  b: hsl(0, 55.5555555556%, 45%);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blackness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg 20% none), hsl)}\n"),
            "a {\
         \n  b: hsl(10, 100%, 60%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(none 20% 30%), hsl)}\n"),
            "a {\
         \n  b: hsl(0, 55.5555555556%, 45%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn whiteness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg none 30%), hsl)}\n"),
            "a {\
         \n  b: hsl(10, 100%, 35%);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn out_of_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(20deg 200% -125%), hwb)}\n"),
        "a {\
         \n  b: hsl(200, 11.1111111111%, 212.5%);\
         \n}\n"
    );
}
