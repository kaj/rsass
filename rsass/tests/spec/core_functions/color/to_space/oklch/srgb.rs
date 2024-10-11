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
         \n  b: color(srgb 0.1013537496 -0.0274561828 -0.00582092 / 0.4);\
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
         \n  b: color(srgb 0.1013537496 -0.0274561828 -0.00582092 / 0);\
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
         \n  b: color(srgb 0.1786875178 -0.0832582876 -0.0062638361);\
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
         \n  b: color(srgb 0.388572859 0.388572859 0.388572859);\
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
         \n  b: color(srgb 0.1013537496 -0.0274561828 -0.00582092);\
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
         \n  b: color(srgb 0.01292 0.01292 0.01292);\
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
         \n  b: color(srgb 0.0925581404 -0.0238184681 0.0100030482);\
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
         \n  b: color(srgb 0.0037875562 0.0001002404 -0.0083342755);\
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
         \n  b: color(srgb 18956918.44307268 -11755023.725622604 -1575226.242810937);\
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
         \n  b: color(srgb 0.400543684 -0.2561070874 -0.0667682639);\
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
         \n  b: color(srgb 1 1 1);\
         \n}\n"
    );
}
