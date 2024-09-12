//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/rgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rgb")
}

#[test]
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(black, $red: 12, $green: 24, $blue: 48)}\n"
        ),
        "a {\
         \n  b: #0c1830;\
         \n}\n"
    );
}
#[test]
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(black, $red: 12, $green: 24, $blue: 48, $alpha: 0.3)}\n"
        ),
        "a {\
         \n  b: rgba(12, 24, 48, 0.3);\
         \n}\n"
    );
}
#[test]
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(rgba(black, 0.3), $red: 12, $green: 24, $blue: 48)}\n"
        ),
        "a {\
         \n  b: rgba(12, 24, 48, 0.3);\
         \n}\n"
    );
}
mod blue {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(black, $blue: 200)}\n"),
            "a {\
         \n  b: #0000c8;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(blue, $blue: 100)}\n"),
            "a {\
         \n  b: #000064;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(black, $blue: 255)}\n"),
            "a {\
         \n  b: blue;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(blue, $blue: 0)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(black, $blue: none)}\n"),
            "a {\
         \n  b: rgb(0 0 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(black, $blue: 256)}\n"),
            "a {\
         \n  b: hsl(240, 100.7874015748%, 50.1960784314%);\
         \n}\n"
        );
    }
}
mod green {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(black, $green: 200)}\n"),
            "a {\
         \n  b: #00c800;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lime, $green: 100)}\n"),
            "a {\
         \n  b: darkgreen;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(black, $green: 255)}\n"),
            "a {\
         \n  b: lime;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lime, $green: 0)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(black, $green: none)}\n"),
            "a {\
         \n  b: rgb(0 none 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(black, $green: -50)}\n"),
            "a {\
         \n  b: hsl(120, 100%, -9.8039215686%);\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change($color: black, $red: 12, $green: 24, $blue: 48)}\n"
        ),
        "a {\
         \n  b: #0c1830;\
         \n}\n"
    );
}
mod red {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(black, $red: 200)}\n"),
            "a {\
         \n  b: #c80000;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $red: 100)}\n"),
            "a {\
         \n  b: #640000;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(black, $red: 255)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(red, $red: 0)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(black, $red: none)}\n"),
            "a {\
         \n  b: rgb(none 0 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(black, $red: 500)}\n"),
            "a {\
         \n  b: hsl(0, 5000%, 98.0392156863%);\
         \n}\n"
        );
    }
}
