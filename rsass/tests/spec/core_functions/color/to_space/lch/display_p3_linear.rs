//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/lch/display_p3_linear.hrx"

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
             \na {b: color.to-space(lch(10% 20 30deg / 0.4), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0274982844 0.0062431844 0.0045951401 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 30deg / 0.0), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0274982844 0.0062431844 0.0045951401 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(0% 0 0deg), display-p3-linear)}\n"),
        "a {\
         \n  b: color(display-p3-linear 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lch(10.123456789% 20.987654321 30.192837465deg), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0286833121 0.0060843483 0.0043741236);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(50% 0 0deg), display-p3-linear)}\n"),
        "a {\
         \n  b: color(display-p3-linear 0.1841865185 0.1841865185 0.1841865185);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 30deg), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0274982844 0.0062431844 0.0045951401);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn chroma() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lch(10% none 30deg), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0112601993 0.0112601993 0.0112601993);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 none), display-p3-linear)}\n"),
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
             \na {b: color.to-space(lch(none 20 30deg), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.012418728 -0.0036981084 -0.0065410926);\
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
             \na {b: color.to-space(lch(10% 999999 0deg), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 18551006191.348015 -6499743958.1266775 371908608.86307263);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(lch(0% 200 0deg), $lightness: -10%), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.2282666369 -0.0949275001 -0.0062723577);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(100% 0 0deg), display-p3-linear)}\n"),
        "a {\
         \n  b: color(display-p3-linear 1 1 1);\
         \n}\n"
    );
}
