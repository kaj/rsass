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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.4), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.2172671914 -0.1122840454 -0.2493572785 / 0.4);\
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
         \n  b: color(a98-rgb 0.2172671914 -0.1122840454 -0.2493572785 / 0);\
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
         \n  b: color(a98-rgb 0.3884711027 0.388471114 0.3884711549);\
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
         \n  b: color(a98-rgb 0.4723956119 -0.3075477289 0.9952387413);\
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
         \n  b: color(a98-rgb 0.1171245434 0.0794773258 -0.2290203701);\
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
         \n  b: color(a98-rgb 0.1433510554 -0.0992363406 0.0248601585);\
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
         \n  b: color(a98-rgb 0.0734724303 0.1640446338 -0.3638546832);\
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
         \n  b: color(a98-rgb -66665446.96678426 49015676.54589439 12422985.883539313);\
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
         \n  b: color(a98-rgb -1.6915317691 2.7130844081 -5.3317336738);\
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
         \n  b: color(a98-rgb 0.9999999799 1.000000009 1.0000001143);\
         \n}\n"
    );
}
