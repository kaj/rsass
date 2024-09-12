//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/oklab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklab")
}

mod a {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklab(50% 0.2 -0.3), $a: none)}\n"),
            "a {\
         \n  b: oklab(50% none -0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklab(50% 0.2 -0.3), $a: 1)}\n"),
            "a {\
         \n  b: oklab(50% 1 -0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklab(50% 0.2 -0.3), $a: -40%)}\n"),
            "a {\
         \n  b: oklab(50% -0.16 -0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklab(50% 0.2 -0.3), $a: 0.1)}\n"),
            "a {\
         \n  b: oklab(50% 0.1 -0.3);\
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
             \na {b: color.change(oklab(50% 0.2 -0.3), $lightness: 20%, $a: -0.3, $b: 0.4)}\n"
        ),
        "a {\
         \n  b: oklab(20% -0.3 0.4);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(oklab(50% 0.2 -0.3), $lightness: 30%, $alpha: 0.9)}\n"
        ),
        "a {\
         \n  b: oklab(30% 0.2 -0.3 / 0.9);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(oklab(50% 0.2 -0.3 / 0.9), $lightness: 30%)}\n"
        ),
        "a {\
         \n  b: oklab(30% 0.2 -0.3 / 0.9);\
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
             \na {b: color.change(oklab(50% 0.2 -0.3), $b: none)}\n"),
            "a {\
         \n  b: oklab(50% 0.2 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklab(50% 0.2 -0.3), $b: -1)}\n"),
            "a {\
         \n  b: oklab(50% 0.2 -1);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklab(50% 0.2 -0.3), $b: -40%)}\n"),
            "a {\
         \n  b: oklab(50% 0.2 -0.16);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklab(50% 0.2 -0.3), $b: 0.4)}\n"),
            "a {\
         \n  b: oklab(50% 0.2 0.4);\
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
             \na {b: color.change(oklab(50% 0.2 -0.3), $lightness: none)}\n"),
            "a {\
         \n  b: oklab(none 0.2 -0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(oklab(50% 0.2 -0.3), $lightness: 1.2)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz 2.3267923962 1.5626810194 5.2743259015) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklab(50% 0.2 -0.3), $lightness: 30%)}\n"),
            "a {\
         \n  b: oklab(30% 0.2 -0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklab(50% 0.2 -0.3), $lightness: 0.3)}\n"),
            "a {\
         \n  b: oklab(30% 0.2 -0.3);\
         \n}\n"
        );
    }
}
