//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklab/a98_rgb.hrx"

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
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.4), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.2172671878 -0.1122840377 -0.249357286 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.0), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.2172671878 -0.1122840377 -0.249357286 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(0% 0 0), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0 0), a98-rgb)}\n"),
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
             \na {b: color.to-space(oklab(50% 0.2 -0.3), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 0.4723956036 -0.3075476912 0.9952387017);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% none 0.3), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.1171245441 0.0794773291 -0.2290203788);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 none), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.1433510521 -0.0992363373 0.0248601563);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(none 0.2 0.3), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.0734724326 0.1640446312 -0.3638546852);\
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
             \na {b: color.to-space(oklab(50% -999999 0), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb -66665443.535555646 49015673.69381805 12422983.980182407);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklab(0% -2 2), $lightness: -50%), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb -1.6915316605 2.7130843323 -5.3317336714);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(100% 0 0), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 1 1 1);\
         \n}\n"
    );
}
