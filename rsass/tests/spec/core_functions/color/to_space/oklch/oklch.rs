//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklch/oklch.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklch")
}

#[test]
#[ignore] // unexepected error
fn in_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg), oklch)}\n"),
        "a {\
         \n  b: oklch(10% 0.1 30deg);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn chroma() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% none 30deg), oklch)}\n"),
            "a {\
         \n  b: oklch(10% none 30deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 none), oklch)}\n"),
            "a {\
         \n  b: oklch(10% 0.1 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(none 0.1 30deg), oklch)}\n"),
            "a {\
         \n  b: oklch(none 0.1 30deg);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn out_of_range() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklch(0% 1 0deg), $lightness: -10%), oklch)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz 0.034858214 -0.0102294646 -0.0091226416) 100%, black);\
         \n}\n"
    );
}
