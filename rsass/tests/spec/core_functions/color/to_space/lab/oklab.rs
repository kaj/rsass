//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/lab/oklab.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklab")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 30 / 0.4), oklab)}\n"),
            "a {\
         \n  b: oklab(22.8064235633% -0.061853133 0.2798052255 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 30 / 0.0), oklab)}\n"),
            "a {\
         \n  b: oklab(22.8064235633% -0.061853133 0.2798052255 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(0% 0 0), oklab)}\n"),
        "a {\
         \n  b: oklab(0% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(50% 0 0), oklab)}\n"),
        "a {\
         \n  b: oklab(56.8965517241% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(50% 50 -75), oklab)}\n"),
        "a {\
         \n  b: oklab(60.2352364768% 0.0926046221 -0.211043843);\
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
             \na {b: color.to-space(lab(10% none 30), oklab)}\n"),
            "a {\
         \n  b: oklab(22.1884319999% none 0.2838925169);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 none), oklab)}\n"),
            "a {\
         \n  b: oklab(23.0834634055% 0.0614345616 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(none 20 30), oklab)}\n"),
            "a {\
         \n  b: oklab(none 0.4936529834 0.1158908054);\
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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lab(50% -999999 0), oklab)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz -236.5825482412 7.2139420412 -2.8507109462) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(lab(0% -150 150), $lightness: -50%), oklab)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz -0.0931334424 -0.0559710307 -0.1664628061) 100%, black);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(100% 0 0), oklab)}\n"),
        "a {\
         \n  b: oklab(100% 0 0);\
         \n}\n"
    );
}
