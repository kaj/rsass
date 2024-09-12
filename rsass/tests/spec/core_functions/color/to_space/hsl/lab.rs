//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/hsl/lab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lab")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg 20% 30% / 0.4), lab)}\n"),
            "a {\
         \n  b: lab(30.8688486066% 10.7899742234 7.6662278505 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg 20% 30% / 0.0), lab)}\n"),
            "a {\
         \n  b: lab(30.8688486066% 10.7899742234 7.6662278505 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(0deg 0% 0%), lab)}\n"),
        "a {\
         \n  b: lab(0% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hsl(20.123456789deg 30.987654321% 60.192837465%), lab)}\n"
        ),
        "a {\
         \n  b: lab(63.0942649983% 14.1332827511 17.6588648805);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(0deg 0% 50%), lab)}\n"),
        "a {\
         \n  b: lab(53.3889647411% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(80deg 30% 60%), lab)}\n"),
        "a {\
         \n  b: lab(71.9297767616% -15.056348508 28.7197360328);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(none 20% 30%), lab)}\n"),
            "a {\
         \n  b: lab(29.467709003% 13.8451863493 5.7231513228);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg 20% none), lab)}\n"),
            "a {\
         \n  b: lab(none 0 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn saturation() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg none 30%), lab)}\n"),
            "a {\
         \n  b: lab(32.5331750598% 0 0);\
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
             \na {b: color.to-space(hsl(20deg 999999% 50%), lab)}\n"
        ),
        "a {\
         \n  b: color-mix(in lab, color(xyz 136956388.39988723 59264689.52803929 -623200798.6169883) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hsl(20deg 200% -50%), lab)}\n"
        ),
        "a {\
         \n  b: color-mix(in lab, color(xyz -1.0161268876 -0.540961491 0.1515884565) 100%, black);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(0deg 0% 100%), lab)}\n"),
        "a {\
         \n  b: lab(100% 0 0);\
         \n}\n"
    );
}
