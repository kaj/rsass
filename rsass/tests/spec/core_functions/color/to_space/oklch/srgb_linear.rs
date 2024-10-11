//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklch/srgb_linear.hrx"

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
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.4), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.0102342028 -0.0021250915 -0.0004505356 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.0), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.0102342028 -0.0021250915 -0.0004505356 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(0% 0 0deg), srgb-linear)}\n"),
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
             \na {b: color.to-space(oklch(10.123456789% 0.198765432 30.192837465deg), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.0268484572 -0.0076182095 -0.000484817);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(50% 0 0deg), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 0.125 0.125 0.125);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 0.0102342028 -0.0021250915 -0.0004505356);\
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
             \na {b: color.to-space(oklch(10% none 30deg), srgb-linear)}\n"),
            "a {\
         \n  b: color(srgb-linear 0.001 0.001 0.001);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 none), srgb-linear)}\n"),
            "a {\
         \n  b: color(srgb-linear 0.0089064721 -0.0018435347 0.0007742297);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(none 0.1 30deg), srgb-linear)}\n"),
            "a {\
         \n  b: color(srgb-linear 0.0002931545 0.0000077585 -0.0006450678);\
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
             \na {b: color.to-space(oklch(10% 999999 0deg), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 257535249172225600 -81795709865791376 -657381057149475.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklch(0% 1 0deg), $lightness: -10%), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.1332496779 -0.0533553372 -0.005616632);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(100% 0 0deg), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 1 1 1);\
         \n}\n"
    );
}
