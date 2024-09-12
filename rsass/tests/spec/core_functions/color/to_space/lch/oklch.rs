//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/lch/oklch.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklch")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 30deg / 0.4), oklch)}\n"),
            "a {\
         \n  b: oklch(22.8705495852% 0.05599277 28.0673245733deg / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 30deg / 0.0), oklch)}\n"),
            "a {\
         \n  b: oklch(22.8705495852% 0.05599277 28.0673245733deg / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(0% 0 0deg), oklch)}\n"),
        "a {\
         \n  b: oklch(0% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lch(10.123456789% 20.987654321 30.192837465deg), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(22.9987502615% 0.0586453899 28.320677744deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(50% 0 0deg), oklch)}\n"),
        "a {\
         \n  b: oklch(56.8965513528% 0.0000000212 89.8755629589deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 30deg), oklch)}\n"),
        "a {\
         \n  b: oklch(22.8705495852% 0.05599277 28.0673245733deg);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn chroma() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% none 30deg), oklch)}\n"),
            "a {\
         \n  b: oklch(22.4137929572% none 89.8755627743deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 none), oklch)}\n"),
            "a {\
         \n  b: oklch(23.0834632697% 0.0614385657 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(none 20 30deg), oklch)}\n"),
            "a {\
         \n  b: oklch(none 0.4163050536 11.1889160032deg);\
         \n}\n"
        );
    }
}
mod out_of_range {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn far() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 999999 0deg), oklch)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz 7373327412.161987 -218927236.2695358 95026466.80033481) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(lch(0% 200 0deg), $lightness: -10%), oklch)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz 0.0846054544 -0.0138950708 -0.0108304931) 100%, black);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(100% 0 0deg), oklch)}\n"),
        "a {\
         \n  b: oklch(99.9999993474% 0.0000000373 89.8755630959deg);\
         \n}\n"
    );
}
