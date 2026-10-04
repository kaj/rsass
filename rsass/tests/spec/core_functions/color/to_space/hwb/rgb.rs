//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/hwb/rgb.hrx"

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
             \na {b: color.to-space(hwb(10deg 20% 30% / 0.4), rgb)}\n"),
            "a {\
         \n  b: rgba(70%, 28.3333333333%, 20%, 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg 20% 30% / 0.0), rgb)}\n"),
            "a {\
         \n  b: rgba(70%, 28.3333333333%, 20%, 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 0% 100%), rgb)}\n"),
        "a {\
         \n  b: black;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hwb(20.123456789deg 30.987654321% 40.192837465%), rgb)}\n"
        ),
        "a {\
         \n  b: rgb(59.807162535%, 40.6534564581%, 30.987654321%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 50% 50%), rgb)}\n"),
        "a {\
         \n  b: rgb(50%, 50%, 50%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(80deg 20% 40%), rgb)}\n"),
        "a {\
         \n  b: #779933;\
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
             \na {b: color.to-space(hwb(none none none), rgb)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn blackness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg 20% none), rgb)}\n"),
            "a {\
         \n  b: #ff5533;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(none 20% 30%), rgb)}\n"),
            "a {\
         \n  b: rgb(70%, 20%, 20%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg none none), rgb)}\n"),
            "a {\
         \n  b: rgb(100%, 16.6666666667%, 0%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn whiteness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg none 30%), rgb)}\n"),
            "a {\
         \n  b: rgb(70%, 11.6666666667%, 0%);\
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
             \na {b: color.to-space(hwb(20deg 999999% -999950%), rgb)}\n"),
            "a {\
         \n  b: hsl(200, 0.0025501925%, 1000024.4999999999%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(20deg 200% -125%), rgb)}\n"),
            "a {\
         \n  b: hsl(200, 11.1111111111%, 212.5%);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 100% 0%), rgb)}\n"),
        "a {\
         \n  b: white;\
         \n}\n"
    );
}
