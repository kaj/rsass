//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz_d50/rgb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rgb")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 0.3 / 0.4), rgb)}\n"
        ),
        "a {\
         \n  b: hsla(184.0103843189, 495.2078632431%, 10.9589006248%, 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 0.3 / 0.0), rgb)}\n"
        ),
        "a {\
         \n  b: hsla(184.0103843189, 495.2078632431%, 10.9589006248%, 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0 0 0), rgb)}\n"),
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
             \na {b: color.to-space(color(xyz-d50 0.5 0.5 0.5), rgb)}\n"),
        "a {\
         \n  b: rgb(74.3883560573%, 72.569188952%, 81.1893154045%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.2 0.4 0.8), rgb)}\n"),
        "a {\
         \n  b: hsl(187.9353554297, 490.1229331153%, 17.2918334784%);\
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
             \na {b: color.to-space(color(xyz-d50 none 0.2 0.3), rgb)}\n"),
            "a {\
         \n  b: rgb(0%, 66.0121696941%, 64.6715334456%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn y() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 none 0.3), rgb)}\n"),
            "a {\
         \n  b: rgb(44.4495555519%, 0%, 68.636157535%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn z() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 none), rgb)}\n"),
            "a {\
         \n  b: hsl(128.9663541465, 142.6286256266%, 23.5199973212%);\
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
             \na {b: color.to-space(color(xyz-d50 -999999 0 0), rgb)}\n"),
            "a {\
         \n  b: hsl(329.431996419, 420.4439814741%, -10316.9080915762%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 -1 0.4 2), rgb)}\n"),
            "a {\
         \n  b: hsl(3.9698519642, 796.3834139233%, -21.9385057601%);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 1 1 1), rgb)}\n"),
        "a {\
         \n  b: hsl(72.6622302958, 128.9066481357%, 104.4631089642%);\
         \n}\n"
    );
}
