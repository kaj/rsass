//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/hsl/oklch.hrx"

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
             \na {b: color.to-space(hsl(10deg 20% 30% / 0.4), oklch)}\n"),
            "a {\
         \n  b: oklch(40.6685296273% 0.0367364512 31.4972209471deg / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg 20% 30% / 0.0), oklch)}\n"),
            "a {\
         \n  b: oklch(40.6685296273% 0.0367364512 31.4972209471deg / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(0deg 0% 0%), oklch)}\n"),
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
             \na {b: color.to-space(hsl(20.123456789deg 30.987654321% 60.192837465%), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(68.4523446414% 0.0592534093 48.0572806164deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(0deg 0% 50%), oklch)}\n"),
        "a {\
         \n  b: oklch(59.8180726623% 0.0000000223 89.8755628286deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(80deg 30% 60%), oklch)}\n"),
        "a {\
         \n  b: oklch(75.0578295808% 0.0860164419 123.4432897448deg);\
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
             \na {b: color.to-space(hsl(none 20% 30%), oklch)}\n"),
            "a {\
         \n  b: oklch(39.5815064245% 0.0433028033 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg 20% none), oklch)}\n"),
            "a {\
         \n  b: oklch(none 0 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn saturation() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg none 30%), oklch)}\n"),
            "a {\
         \n  b: oklch(41.838943744% none 89.8755626743deg);\
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
             \na {b: color.to-space(hsl(20deg 999999% 50%), oklch)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz 136956388.39988703 59264689.52803931 -623200798.6169883) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hsl(20deg 200% -50%), oklch)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz -1.0161268876 -0.540961491 0.1515884565) 100%, black);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(0deg 0% 100%), oklch)}\n"),
        "a {\
         \n  b: oklch(99.9999993474% 0.0000000373 89.8755630959deg);\
         \n}\n"
    );
}
