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
         \n  b: color(srgb-linear 0.0102342031 -0.0021250916 -0.0004505356 / 0.4);\
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
         \n  b: color(srgb-linear 0.0102342031 -0.0021250916 -0.0004505356 / 0);\
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
         \n  b: color(srgb-linear 0.0268484583 -0.00761821 -0.0004848171);\
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
         \n  b: color(srgb-linear 0.1249999913 0.1250000025 0.1250000327);\
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
         \n  b: color(srgb-linear 0.0102342031 -0.0021250916 -0.0004505356);\
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
         \n  b: color(srgb-linear 0.0009999999 0.001 0.0010000003);\
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
         \n  b: color(srgb-linear 0.0089064724 -0.0018435348 0.0007742299);\
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
         \n  b: color(srgb-linear 257535278804612064 -81795720332859872 -657382012471774.5);\
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
         \n  b: color(srgb-linear 0.1332496936 -0.0533553421 -0.0056166345);\
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
         \n  b: color(srgb-linear 0.9999999305 1.0000000197 1.0000002613);\
         \n}\n"
    );
}
