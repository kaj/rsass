//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/a98_rgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("a98_rgb")
}

#[test]
#[ignore] // unexepected error
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $red: 0.7, $green: 0.4, $blue: 0.2)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.7 0.4 0.2);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $red: 0.5, $alpha: 0.9)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.5 0.5 0.7 / 0.9);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7 / 0.9), $red: 0.5)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.5 0.5 0.7 / 0.9);\
         \n}\n"
    );
}
mod blue {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $blue: none)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.2 0.5 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $blue: 100)}\n"
            ),
            "a {\
         \n  b: color(a98-rgb 0.2 0.5 100);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $blue: 50%)}\n"
            ),
            "a {\
         \n  b: color(a98-rgb 0.2 0.5 0.5);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $blue: 0.5)}\n"
            ),
            "a {\
         \n  b: color(a98-rgb 0.2 0.5 0.5);\
         \n}\n"
        );
    }
}
mod green {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $green: none)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.2 none 0.7);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $green: -0.2)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.2 -0.2 0.7);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $green: 40%)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.2 0.4 0.7);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $green: 0.4)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.2 0.4 0.7);\
         \n}\n"
    );
    }
}
mod red {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $red: none)}\n"
            ),
            "a {\
         \n  b: color(a98-rgb none 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $red: 1.2)}\n"),
            "a {\
         \n  b: color(a98-rgb 1.2 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $red: 50%)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.5 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(color(a98-rgb 0.2 0.5 0.7), $red: 0.5)}\n"),
            "a {\
         \n  b: color(a98-rgb 0.5 0.5 0.7);\
         \n}\n"
        );
    }
}
