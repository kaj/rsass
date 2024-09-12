//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz/hsl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hsl")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.4), hsl)}\n"),
            "a {\
         \n  b: hsla(179.5022543706, 556.250481638%, 8.7700702541%, 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.0), hsl)}\n"),
            "a {\
         \n  b: hsla(179.5022543706, 556.250481638%, 8.7700702541%, 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0 0 0), hsl)}\n"),
        "a {\
         \n  b: hsl(0, 0%, 0%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.5 0.5 0.5), hsl)}\n"),
        "a {\
         \n  b: hsl(8.6326376323, 19.0960524665%, 75.1815938992%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.2 0.4 0.8), hsl)}\n"),
        "a {\
         \n  b: hsl(183.9973689591, 600.9357681928%, 12.7508937669%);\
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
             \na {b: color.to-space(color(xyz 0.1 0.2 none), hsl)}\n"),
            "a {\
         \n  b: hsl(93.2964667331, 215.664278299%, 17.8710983929%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 none 0.3), hsl)}\n"),
            "a {\
         \n  b: hsl(290.3526254976, 328.3439800543%, 14.0892871543%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz none 0.2 0.3), hsl)}\n"),
            "a {\
         \n  b: hsl(355.8794204538, 2697.9214173204%, -2.5244914397%);\
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
             \na {b: color.to-space(color(xyz -999999 0 0), hsl)}\n"),
            "a {\
         \n  b: hsl(330.5196564153, 405.9398117154%, -10761.9459979264%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz -1 0.4 2), hsl)}\n"),
            "a {\
         \n  b: hsl(0.951270101, 523.3395920082%, -31.8043324514%);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 1 1 1), hsl)}\n"),
        "a {\
         \n  b: hsl(188.6326376323, 287.948753728%, 102.1970070346%);\
         \n}\n"
    );
}
