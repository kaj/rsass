//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz/rgb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rgb")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.4), rgb)}\n"),
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
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.0), rgb)}\n"),
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
             \na {b: color.to-space(color(xyz 0 0 0), rgb)}\n"),
        "a {\
         \n  b: black;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.5 0.5 0.5), rgb)}\n"),
        "a {\
         \n  b: rgb(79.9209297495%, 71.8060236826%, 70.4422580488%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.2 0.4 0.8), rgb)}\n"),
        "a {\
         \n  b: hsl(183.9973689591, 600.9357681928%, 12.7508937669%);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn x() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz none 0.2 0.3), rgb)}\n"),
            "a {\
         \n  b: rgb(0%, 65.5843037912%, 56.2293801732%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn y() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 none 0.3), rgb)}\n"),
            "a {\
         \n  b: rgb(45.4739353957%, 0%, 60.3506133579%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn z() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 none), rgb)}\n"),
            "a {\
         \n  b: rgb(13.6360643575%, 56.4126737659%, 0%);\
         \n}\n"
        );
    }
}
mod out_of_range {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn far() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz -999999 0 0), rgb)}\n"),
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
             \na {b: color.to-space(color(xyz -1 0.4 2), rgb)}\n"),
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
             \na {b: color.to-space(color(xyz 1 1 1), rgb)}\n"),
        "a {\
         \n  b: hsl(188.6326376323, 287.948753728%, 102.1970070346%);\
         \n}\n"
    );
}
