//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/lab/oklch.hrx"

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
             \na {b: color.to-space(lab(10% 20 30 / 0.4), oklch)}\n"),
            "a {\
         \n  b: oklch(22.8064235633% 0.2865602455 102.4652162351deg / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 30 / 0.0), oklch)}\n"),
            "a {\
         \n  b: oklch(22.8064235633% 0.2865602455 102.4652162351deg / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(0% 0 0), oklch)}\n"),
        "a {\
         \n  b: oklch(0% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(50% 0 0), oklch)}\n"),
        "a {\
         \n  b: oklch(56.8965517241% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(50% 50 -75), oklch)}\n"),
        "a {\
         \n  b: oklch(60.2352364768% 0.2304671771 293.6915436279deg);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% none 30), oklch)}\n"),
            "a {\
         \n  b: oklch(22.1884319999% 0.3083157421 112.9588823005deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 none), oklch)}\n"),
            "a {\
         \n  b: oklch(23.0834634055% 0.0614385679 359.3456789541deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(none 20 30), oklch)}\n"),
            "a {\
         \n  b: oklch(none 0.5070739066 13.2116105787deg);\
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
             \na {b: color.to-space(lab(50% -999999 0), oklch)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz -236.5825482412 7.2139420412 -2.8507109462) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(lab(0% -150 150), $lightness: -50%), oklch)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz -0.0931334424 -0.0559710307 -0.1664628061) 100%, black);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(100% 0 0), oklch)}\n"),
        "a {\
         \n  b: oklch(100% 0 none);\
         \n}\n"
    );
}
