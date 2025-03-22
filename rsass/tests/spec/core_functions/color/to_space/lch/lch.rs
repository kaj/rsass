//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/lch/lch.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lch")
}

#[test]
#[ignore] // unexepected error
fn in_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 30deg), lch)}\n"),
        "a {\
         \n  b: lch(10% 20 30deg);\
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
             \na {b: color.to-space(lch(10% none 30deg), lch)}\n"),
            "a {\
         \n  b: lch(10% none 30deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 none), lch)}\n"),
            "a {\
         \n  b: lch(10% 20 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(none 20 30deg), lch)}\n"),
            "a {\
         \n  b: lch(none 20 30deg);\
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
             \na {b: color.to-space(color.change(lch(0% 200 0deg), $lightness: -10%), lch)}\n"
        ),
        "a {\
         \n  b: color-mix(in lch, color(xyz 0.0846054544 -0.0138950708 -0.0108304931) 100%, black);\
         \n}\n"
    );
}
