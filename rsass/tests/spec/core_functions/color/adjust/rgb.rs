//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/rgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rgb")
}

#[test]
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(black, $red: 12, $green: 24, $blue: 48)}\n"
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
             \na {b: color.adjust(black, $red: 12, $green: 24, $blue: 48, $alpha: -0.3)}\n"
        ),
        "a {\
         \n  b: rgba(12, 24, 48, 0.7);\
         \n}\n"
    );
}
#[test]
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(rgba(black, 0.3), $red: 12, $green: 24, $blue: 48)}\n"
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
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#fedcba, $blue: 200)}\n"),
            "a {\
         \n  b: #fedcff;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#fedcba, $blue: 500)}\n"),
            "a {\
         \n  b: #fedcff;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#fedcba, $blue: -500)}\n"),
            "a {\
         \n  b: #fedc00;\
         \n}\n"
        );
    }
    #[test]
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#fedcba, $blue: -200)}\n"),
            "a {\
         \n  b: #fedc00;\
         \n}\n"
        );
    }
    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(black, $blue: 200)}\n"),
            "a {\
         \n  b: #0000c8;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(blue, $blue: -100)}\n"),
            "a {\
         \n  b: #00009b;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(black, $blue: 255)}\n"),
            "a {\
         \n  b: blue;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(blue, $blue: -255)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(black, $blue: 0)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
}
mod green {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#abcdef, $green: 200)}\n"),
            "a {\
         \n  b: #abffef;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#abcdef, $green: 500)}\n"),
            "a {\
         \n  b: #abffef;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#abcdef, $green: -500)}\n"),
            "a {\
         \n  b: #ab00ef;\
         \n}\n"
        );
    }
    #[test]
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#abcdef, $green: -210)}\n"),
            "a {\
         \n  b: #ab00ef;\
         \n}\n"
        );
    }
    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(black, $green: 200)}\n"),
            "a {\
         \n  b: #00c800;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lime, $green: -100)}\n"),
            "a {\
         \n  b: #009b00;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(black, $green: 255)}\n"),
            "a {\
         \n  b: lime;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lime, $green: -255)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(black, $green: 0)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust($color: black, $red: 12, $green: 24, $blue: 48)}\n"
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
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#abcdef, $red: 200)}\n"),
            "a {\
         \n  b: #ffcdef;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#abcdef, $red: 500)}\n"),
            "a {\
         \n  b: #ffcdef;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#abcdef, $red: -500)}\n"),
            "a {\
         \n  b: #00cdef;\
         \n}\n"
        );
    }
    #[test]
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(#abcdef, $red: -200)}\n"),
            "a {\
         \n  b: #00cdef;\
         \n}\n"
        );
    }
    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(black, $red: 200)}\n"),
            "a {\
         \n  b: #c80000;\
         \n}\n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $red: -100)}\n"),
            "a {\
         \n  b: #9b0000;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(black, $red: 255)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(red, $red: -255)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(black, $red: 0)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
}
