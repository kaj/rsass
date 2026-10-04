//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rec2020/lch.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lch")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.4), lch)}\n"
        ),
        "a {\
         \n  b: lch(14.3089075958% 26.1183647557 244.4054932929deg / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.0), lch)}\n"
        ),
        "a {\
         \n  b: lch(14.3089075958% 26.1183647557 244.4054932929deg / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0 0 0), lch)}\n"),
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
             \na {b: color.to-space(color(rec2020 0.5 0.5 0.5), lch)}\n"),
        "a {\
         \n  b: lch(50.6245045898% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.2 0.4 0.8), lch)}\n"),
        "a {\
         \n  b: lch(39.1500175267% 70.7041037559 271.3041040241deg);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn all() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 none none none), lch)}\n"),
            "a {\
         \n  b: lch(none none none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 none), lch)}\n"),
            "a {\
         \n  b: lch(12.7980788363% 36.7875010155 145.3968458287deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 none 0.3), lch)}\n"),
            "a {\
         \n  b: lch(3.2949420149% 48.8014697745 300.1735777513deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 none 0.2 0.3), lch)}\n"),
            "a {\
         \n  b: lch(13.6661446251% 30.1137108719 235.0010040919deg);\
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
             \na {b: color.to-space(color(rec2020 -999999 0 0), lch)}\n"),
            "a {\
         \n  b: color-mix(in lch, color(xyz -159996240949325.34 -65987150457941.19 62164121.716430664) 100%, black);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 -1 0.4 2), lch)}\n"),
            "a {\
         \n  b: lch(22.5264188072% 288.3351975871 254.5739360813deg);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 1 1 1), lch)}\n"),
        "a {\
         \n  b: lch(100% 0 none);\
         \n}\n"
    );
}
