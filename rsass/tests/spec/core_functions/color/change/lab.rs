//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/lab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lab")
}

mod a {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 20 -30), $a: none)}\n"),
            "a {\
         \n  b: lab(50% none -30);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 20 -30), $a: 200)}\n"),
            "a {\
         \n  b: lab(50% 200 -30);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 20 -30), $a: -40%)}\n"),
            "a {\
         \n  b: lab(50% -50 -30);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 20 -30), $a: 50)}\n"),
            "a {\
         \n  b: lab(50% 50 -30);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(lab(50% 20 -30), $lightness: 20%, $a: -30, $b: 40)}\n"
        ),
        "a {\
         \n  b: lab(20% -30 40);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(lab(50% 20 -30), $lightness: 30%, $alpha: 0.9)}\n"
        ),
        "a {\
         \n  b: lab(30% 20 -30 / 0.9);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(lab(50% 20 -30 / 0.9), $lightness: 30%)}\n"
        ),
        "a {\
         \n  b: lab(30% 20 -30 / 0.9);\
         \n}\n"
    );
}
mod b {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 20 -30), $b: none)}\n"),
            "a {\
         \n  b: lab(50% 20 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 20 -30), $b: -200)}\n"),
            "a {\
         \n  b: lab(50% 20 -200);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 20 -30), $b: -40%)}\n"),
            "a {\
         \n  b: lab(50% 20 -50);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 20 -30), $b: 50)}\n"),
            "a {\
         \n  b: lab(50% 20 50);\
         \n}\n"
        );
    }
}
mod lightness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 20 -30), $lightness: none)}\n"),
            "a {\
         \n  b: lab(none 20 -30);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(lab(50% 20 -30), $lightness: 120%)}\n"
        ),
        "a {\
         \n  b: color-mix(in lab, color(xyz 1.7255148283 1.6190494947 2.5266428703) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 20 -30), $lightness: 30%)}\n"),
            "a {\
         \n  b: lab(30% 20 -30);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lab(50% 20 -30), $lightness: 30)}\n"),
            "a {\
         \n  b: lab(30% 20 -30);\
         \n}\n"
        );
    }
}
