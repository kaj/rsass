//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklch/a98_rgb.hrx"

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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.4), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.1027766429 -0.0609172236 -0.0321025963 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.0), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.1027766429 -0.0609172236 -0.0321025963 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(0% 0 0deg), a98-rgb)}\n"),
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
             \na {b: color.to-space(oklch(10.123456789% 0.198765432 30.192837465deg), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.1569345516 -0.1088588763 -0.0385850338);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(50% 0 0deg), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 0.3884711105 0.3884711105 0.3884711105);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 0.1027766429 -0.0609172236 -0.0321025963);\
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
             \na {b: color.to-space(oklch(10% none 30deg), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.0432393561 0.0432393561 0.0432393561);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 none), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.0964961623 -0.0571048125 0.0359544994);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(none 0.1 30deg), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.0213511871 0.0047462183 -0.0347458469);\
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
             \na {b: color.to-space(oklch(10% 999999 0deg), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 66665573.66889219 -49015758.49665135 -12422989.459212372);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklch(0% 1 0deg), $lightness: -10%), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.3172861757 -0.2637782362 -0.1086210221);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(100% 0 0deg), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 1 1 1);\
         \n}\n"
    );
}
