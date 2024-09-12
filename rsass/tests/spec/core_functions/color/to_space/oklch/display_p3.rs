//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklch/display_p3.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("display_p3")
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
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.4), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.0863974377 -0.022155665 -0.0050290551 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.0), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.0863974377 -0.022155665 -0.0050290551 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(0% 0 0deg), display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10.123456789% 0.198765432 30.192837465deg), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.1548116084 -0.0741945938 -0.0069035722);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(50% 0 0deg), display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 0.3885728491 0.3885728621 0.3885729031);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg), display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 0.0863974377 -0.022155665 -0.0050290551);\
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
             \na {b: color.to-space(oklch(10% none 30deg), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 0.0129199993 0.0129200002 0.0129200031);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 none), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 0.0784514548 -0.0192081243 0.0093493065);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(none 0.1 30deg), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 0.0031329176 0.0002226378 -0.0075165654);\
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
             \na {b: color.to-space(oklch(10% 999999 0deg), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 16964742.301441267 -11051293.581589133 -2566311.5001529297);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklch(0% 1 0deg), $lightness: -10%), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.3493925785 -0.240514802 -0.0760588329);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(100% 0 0deg), display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 0.9999999764 1.0000000074 1.0000001047);\
         \n}\n"
    );
}
