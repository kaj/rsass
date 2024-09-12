//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklch/srgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("srgb")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.4), srgb)}\n"),
            "a {\
         \n  b: color(srgb 0.1013537513 -0.027456184 -0.0058209203 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.0), srgb)}\n"),
            "a {\
         \n  b: color(srgb 0.1013537513 -0.027456184 -0.0058209203 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(0% 0 0deg), srgb)}\n"),
        "a {\
         \n  b: color(srgb 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10.123456789% 0.198765432 30.192837465deg), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb 0.1786875221 -0.0832582912 -0.0062638365);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(50% 0 0deg), srgb)}\n"),
        "a {\
         \n  b: color(srgb 0.3885728462 0.3885728627 0.3885729073);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg), srgb)}\n"),
        "a {\
         \n  b: color(srgb 0.1013537513 -0.027456184 -0.0058209203);\
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
             \na {b: color.to-space(oklch(10% none 30deg), srgb)}\n"),
            "a {\
         \n  b: color(srgb 0.0129199991 0.0129200003 0.0129200034);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 none), srgb)}\n"),
            "a {\
         \n  b: color(srgb 0.0925581424 -0.0238184697 0.0100030504);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(none 0.1 30deg), srgb)}\n"),
            "a {\
         \n  b: color(srgb 0.0037875565 0.0001002403 -0.0083342755);\
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
             \na {b: color.to-space(oklch(10% 999999 0deg), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb 18956919.35191059 -11755024.352390574 -1575227.1966261775);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklch(0% 1 0deg), $lightness: -10%), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb 0.4005437063 -0.2561070992 -0.066768286);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(100% 0 0deg), srgb)}\n"),
        "a {\
         \n  b: color(srgb 0.9999999694 1.0000000087 1.0000001149);\
         \n}\n"
    );
}
