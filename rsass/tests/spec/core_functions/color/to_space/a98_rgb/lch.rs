//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/a98_rgb/lch.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lch")
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
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 0.3 / 0.4), lch)}\n"
        ),
        "a {\
         \n  b: lch(17.7387491045% 23.8457560364 252.4602846081deg / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 0.3 / 0.0), lch)}\n"
        ),
        "a {\
         \n  b: lch(17.7387491045% 23.8457560364 252.4602846081deg / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0 0 0), lch)}\n"),
        "a {\
         \n  b: lch(0% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.5 0.5 0.5), lch)}\n"),
        "a {\
         \n  b: lch(53.7880481926% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.2 0.4 0.8), lch)}\n"),
        "a {\
         \n  b: lch(42.9339299516% 63.8746494696 275.8433881087deg);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 none), lch)}\n"),
            "a {\
         \n  b: lch(15.5541953165% 36.1300043959 139.9466505913deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 none 0.3), lch)}\n"),
            "a {\
         \n  b: lch(5.8200823344% 51.4475613847 304.4897928318deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb none 0.2 0.3), lch)}\n"),
            "a {\
         \n  b: lch(16.8147782656% 27.3150184488 242.6852480988deg);\
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
             \na {b: color.to-space(color(a98-rgb -999999 0 0), lch)}\n"
        ),
        "a {\
         \n  b: color-mix(in lch, color(xyz -9041452038524.758 -4661998707364.329 -423818064305.86096) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb -1 0.4 2), lch)}\n"),
            "a {\
         \n  b: lch(30.0623308533% 244.5624171683 279.0769417748deg);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 1 1 1), lch)}\n"),
        "a {\
         \n  b: lch(100% 0 none);\
         \n}\n"
    );
}
