//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/hwb/srgb_linear.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("srgb_linear")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg 20% 30% / 0.4), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.4479884124 0.0652561709 0.0331047666 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg 20% 30% / 0.0), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.4479884124 0.0652561709 0.0331047666 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 0% 100%), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hwb(20.123456789deg 30.987654321% 40.192837465%), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.316300627 0.1374941598 0.0782247825);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 50% 50%), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 0.2140411405 0.2140411405 0.2140411405);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(80deg 20% 40%), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 0.1844749945 0.3185467781 0.0331047666);\
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
             \na {b: color.to-space(hwb(10deg 20% none), srgb-linear)}\n"),
            "a {\
         \n  b: color(srgb-linear 1 0.0908417112 0.0331047666);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(none 20% 30%), srgb-linear)}\n"),
            "a {\
         \n  b: color(srgb-linear 0.4479884124 0.0331047666 0.0331047666);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn whiteness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg none 30%), srgb-linear)}\n"),
            "a {\
         \n  b: color(srgb-linear 0.4479884124 0.0128067943 0);\
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
             \na {b: color.to-space(hwb(20deg 999999% -999950%), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 3501482364.2479005 3501196665.942984 3501053821.890021);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(20deg 200% -125%), srgb-linear)}\n"),
            "a {\
         \n  b: color(srgb-linear 6.5253577732 5.4497310669 4.9538457516);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 100% 0%), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 1 1 1);\
         \n}\n"
    );
}
