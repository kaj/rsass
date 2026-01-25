//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/lab/display_p3_linear.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("display_p3_linear")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 30 / 0.4), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0342590994 0.0051805075 -0.0088494437 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 30 / 0.0), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0342590994 0.0051805075 -0.0088494437 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(0% 0 0), display-p3-linear)}\n"),
        "a {\
         \n  b: color(display-p3-linear 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lab(50% 0 0), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.1841865185 0.1841865185 0.1841865185);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(50% 50 -75), display-p3-linear)}\n"),
        "a {\
         \n  b: color(display-p3-linear 0.23566973 0.099330787 0.879940124);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lab(10% none 30), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0176420089 0.0110026625 -0.0091825814);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 none), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0278772897 0.0054380443 0.0115933369);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lab(none 20 30), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0182270941 -0.0044270743 -0.020004771);\
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
             \na {b: color.to-space(lab(50% -999999 0), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear -595.488788855 208.8910233756 -11.7578022511);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(lab(0% -150 150), $lightness: -50%), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear -0.1130610827 -0.0253375925 -0.1583606814);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(100% 0 0), display-p3-linear)}\n"),
        "a {\
         \n  b: color(display-p3-linear 1 1 1);\
         \n}\n"
    );
}
