//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/xyz.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz")
}

#[test]
#[ignore] // unexepected error
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(color(xyz 0.2 0.5 0.7), $x: 0.7, $y: 0.4, $z: 0.2)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.7 0.4 0.2);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(color(xyz 0.2 0.5 0.7), $x: 0.5, $alpha: 0.9)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.5 0.5 0.7 / 0.9);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_input() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(xyz 0.2 0.5 0.7 / 0.9), $x: 0.5)}\n"),
        "a {\
         \n  b: color(xyz 0.5 0.5 0.7 / 0.9);\
         \n}\n"
    );
}
mod x {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(xyz 0.2 0.5 0.7), $x: none)}\n"),
            "a {\
         \n  b: color(xyz none 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(xyz 0.2 0.5 0.7), $x: 1.2)}\n"),
            "a {\
         \n  b: color(xyz 1.2 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(xyz 0.2 0.5 0.7), $x: 50%)}\n"),
            "a {\
         \n  b: color(xyz 0.5 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(xyz 0.2 0.5 0.7), $x: 0.5)}\n"),
            "a {\
         \n  b: color(xyz 0.5 0.5 0.7);\
         \n}\n"
        );
    }
}
mod y {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(xyz 0.2 0.5 0.7), $y: none)}\n"),
            "a {\
         \n  b: color(xyz 0.2 none 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(xyz 0.2 0.5 0.7), $y: -0.2)}\n"),
            "a {\
         \n  b: color(xyz 0.2 -0.2 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(xyz 0.2 0.5 0.7), $y: 40%)}\n"),
            "a {\
         \n  b: color(xyz 0.2 0.4 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(xyz 0.2 0.5 0.7), $y: 0.4)}\n"),
            "a {\
         \n  b: color(xyz 0.2 0.4 0.7);\
         \n}\n"
        );
    }
}
mod z {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(xyz 0.2 0.5 0.7), $z: none)}\n"),
            "a {\
         \n  b: color(xyz 0.2 0.5 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(xyz 0.2 0.5 0.7), $z: 100)}\n"),
            "a {\
         \n  b: color(xyz 0.2 0.5 100);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(xyz 0.2 0.5 0.7), $z: 50%)}\n"),
            "a {\
         \n  b: color(xyz 0.2 0.5 0.5);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(xyz 0.2 0.5 0.7), $z: 0.5)}\n"),
            "a {\
         \n  b: color(xyz 0.2 0.5 0.5);\
         \n}\n"
        );
    }
}
