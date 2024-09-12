//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/hwb/srgb.hrx"

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
             \na {b: color.to-space(hwb(10deg 20% 30% / 0.4), srgb)}\n"),
            "a {\
         \n  b: color(srgb 0.7 0.2833333333 0.2 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg 20% 30% / 0.0), srgb)}\n"),
            "a {\
         \n  b: color(srgb 0.7 0.2833333333 0.2 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 0% 100%), srgb)}\n"),
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
             \na {b: color.to-space(hwb(20.123456789deg 30.987654321% 40.192837465%), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb 0.5980716254 0.4065345646 0.3098765432);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 50% 50%), srgb)}\n"),
        "a {\
         \n  b: color(srgb 0.5 0.5 0.5);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(80deg 20% 40%), srgb)}\n"),
        "a {\
         \n  b: color(srgb 0.4666666667 0.6 0.2);\
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
             \na {b: color.to-space(hwb(10deg 20% none), srgb)}\n"),
            "a {\
         \n  b: color(srgb 1 0.3333333333 0.2);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(none 20% 30%), srgb)}\n"),
            "a {\
         \n  b: color(srgb 0.7 0.2 0.2);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn whiteness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg none 30%), srgb)}\n"),
            "a {\
         \n  b: color(srgb 0.7 0.1166666667 0);\
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
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(20deg 999999% -999950%), srgb)}\n"),
            "a {\
         \n  b: color(srgb 10000.5 10000.16 9999.99);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(20deg 200% -125%), srgb)}\n"),
            "a {\
         \n  b: color(srgb 2.25 2.0833333333 2);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 100% 0%), srgb)}\n"),
        "a {\
         \n  b: color(srgb 1 1 1);\
         \n}\n"
    );
}
