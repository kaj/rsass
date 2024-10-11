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
         \n  b: color(display-p3 0.0863974362 -0.022155664 -0.0050290548 / 0.4);\
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
         \n  b: color(display-p3 0.0863974362 -0.022155664 -0.0050290548 / 0);\
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
         \n  b: color(display-p3 0.1548116046 -0.0741945902 -0.0069035716);\
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
         \n  b: color(display-p3 0.388572859 0.388572859 0.388572859);\
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
         \n  b: color(display-p3 0.0863974362 -0.022155664 -0.0050290548);\
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
         \n  b: color(display-p3 0.01292 0.01292 0.01292);\
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
         \n  b: color(display-p3 0.078451453 -0.0192081229 0.0093493046);\
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
         \n  b: color(display-p3 0.0031329174 0.0002226379 -0.0075165654);\
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
         \n  b: color(display-p3 16964741.49482702 -11051292.985141436 -2566310.9347816953);\
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
         \n  b: color(display-p3 0.3493925583 -0.240514791 -0.076058814);\
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
         \n  b: color(display-p3 1 1 1);\
         \n}\n"
    );
}
