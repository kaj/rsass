//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/lch/a98_rgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("a98_rgb")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 30deg / 0.4), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.1855277377 0.0926913904 0.0818650595 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 30deg / 0.0), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.1855277377 0.0926913904 0.0818650595 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(0% 0 0deg), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lch(10.123456789% 20.987654321 30.192837465deg), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.1889263665 0.0909755641 0.0795209428);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(50% 0 0deg), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 0.4633483808 0.4633483808 0.4633483808);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 30deg), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 0.1855277377 0.0926913904 0.0818650595);\
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
             \na {b: color.to-space(lch(10% none 30deg), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.1300242938 0.1300242938 0.1300242938);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 none), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.1862731835 0.0856347366 0.1310611506);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(none 20 30deg), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.1244563961 -0.084601587 -0.1049078937);\
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
             \na {b: color.to-space(lch(10% 999999 0deg), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 42562.6792376747 -31021.1046455384 6233.1625417899);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(lch(0% 200 0deg), $lightness: -10%), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.4634940567 -0.3642836921 -0.1126427123);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(100% 0 0deg), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 1 1 1);\
         \n}\n"
    );
}
