//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklch/prophoto_rgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("prophoto_rgb")
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
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.4), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.050609646 -0.0137951632 -0.0074760684 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.0), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.050609646 -0.0137951632 -0.0074760684 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(0% 0 0deg), prophoto-rgb)}\n"),
        "a {\
         \n  b: color(prophoto-rgb 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10.123456789% 0.198765432 30.192837465deg), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.0841896008 -0.0467123987 -0.0138059119);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(50% 0 0deg), prophoto-rgb)}\n"),
        "a {\
         \n  b: color(prophoto-rgb 0.3149802625 0.3149802625 0.3149802625);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg), prophoto-rgb)}\n"),
        "a {\
         \n  b: color(prophoto-rgb 0.050609646 -0.0137951632 -0.0074760684);\
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
             \na {b: color.to-space(oklch(10% none 30deg), prophoto-rgb)}\n"),
            "a {\
         \n  b: color(prophoto-rgb 0.016 0.016 0.016);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 none), prophoto-rgb)}\n"),
            "a {\
         \n  b: color(prophoto-rgb 0.0479066366 -0.0113980128 0.0096553515);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(none 0.1 30deg), prophoto-rgb)}\n"),
            "a {\
         \n  b: color(prophoto-rgb 0.0010727236 0.000279135 -0.0088387805);\
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
             \na {b: color.to-space(oklch(10% 999999 0deg), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 2922139901.256741 -1810418671.4016082 -574654054.8066751);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklch(0% 1 0deg), $lightness: -10%), prophoto-rgb)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.1937443812 -0.1519484557 -0.0725277722);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(100% 0 0deg), prophoto-rgb)}\n"),
        "a {\
         \n  b: color(prophoto-rgb 1 1 1);\
         \n}\n"
    );
}
