//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz/a98_rgb.hrx"

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
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.4), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb -0.1351623531 0.5702285503 0.5625764827 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.0), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb -0.1351623531 0.5702285503 0.5625764827 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0 0 0), a98-rgb)}\n"),
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
             \na {b: color.to-space(color(xyz 0.5 0.5 0.5), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 0.7719295678 0.7122496746 0.6991190732);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.2 0.4 0.8), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb -0.3403753886 0.7865601683 0.8866254663);\
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
             \na {b: color.to-space(color(xyz 0.1 0.2 none), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.3365194536 0.5589776013 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 none 0.3), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.3521663165 none 0.5835625865);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz none 0.2 0.3), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb none 0.6499288625 0.5613563103);\
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
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz -999999 0 0), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb -739.9221025629 527.3176861205 -75.3831765931);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz -1 0.4 2), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb -1.6372044815 1.3072981109 1.360980168);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 1 1 1), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 1.0579328451 0.9761412908 0.9581457442);\
         \n}\n"
    );
}
