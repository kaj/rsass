//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/lab/xyz_d50.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz_d50")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 30 / 0.4), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.0177706181 0.0112601993 -0.0067594366 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 30 / 0.0), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.0177706181 0.0112601993 -0.0067594366 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(0% 0 0), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(50% 0 0), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0.1776102635 0.1841865185 0.1519731441);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(50% 50 -75), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0.2886831368 0.1841865185 0.6940286511);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% none 30), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.0108581615 0.0112601993 -0.0067594366);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 none), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.0177706181 0.0112601993 0.0092908422);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(none 20 30), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.0049533381 0 -0.0158938104);\
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
             \na {b: color.to-space(lab(50% -999999 0), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 -247.6132796606 0.1841865185 0.1519731441);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(lab(0% -150 150), $lightness: -50%), xyz-d50)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 -0.0905265235 -0.055352823 -0.1251409211);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(100% 0 0), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0.9642956764 1 0.8251046025);\
         \n}\n"
    );
}
