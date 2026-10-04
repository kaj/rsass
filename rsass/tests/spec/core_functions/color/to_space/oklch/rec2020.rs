//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklch/rec2020.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rec2020")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.4), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.1161454157 -0.0617559389 -0.0392838997 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.0), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.1161454157 -0.0617559389 -0.0392838997 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(0% 0 0deg), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10.123456789% 0.198765432 30.192837465deg), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.1704446117 -0.1113738318 -0.0474320537);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(50% 0 0deg), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.4204482076 0.4204482076 0.4204482076);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.1161454157 -0.0617559389 -0.0392838997);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn all() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(none none none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 none none none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn chroma() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% none 30deg), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.0562341325 0.0562341325 0.0562341325);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.1100929542 -0.0578643858 0.0478020915);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(none 0.1 30deg), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.02610513 0.0110318166 -0.044564419);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(none none 10deg), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0 0 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(50% none none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.4204482076 0.4204482076 0.4204482076);\
         \n}\n"
        );
    }
}
mod out_of_range {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn far() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 999999 0deg), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 13712729.015620274 -9615344.930416511 -3020648.353808308);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklch(0% 1 0deg), $lightness: -10%), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.3217837476 -0.2613113378 -0.1305030615);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(100% 0 0deg), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 1 1 1);\
         \n}\n"
    );
}
