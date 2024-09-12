//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/lab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lab")
}

mod a {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lab(70% 100 -60), $a: 12%)}\n"),
            "a {\
         \n  b: lab(70% 103 -60);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lab(70% 100 -60), $a: -86%)}\n"),
            "a {\
         \n  b: lab(70% -93.5 -60);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lab(70% 100 -60), $a: 100%)}\n"),
            "a {\
         \n  b: lab(70% 125 -60);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lab(70% 100 -60), $a: -100%)}\n"),
            "a {\
         \n  b: lab(70% -125 -60);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lab(70% 100 -60), $a: 0%)}\n"),
            "a {\
         \n  b: lab(70% 100 -60);\
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
             \na {\
             \n  b: color.scale(lab(70% 100 -60), $lightness: 12%, $a: 24%, $b: 48%);\
             \n}\n"
        ),
        "a {\
         \n  b: lab(73.6% 106 28.8);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_arg() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.scale(\
             \n    lab(70% 100 -60),\
             \n    $lightness: 12%, $a: 24%, $b: 48%, $alpha: -70%\
             \n  );\
             \n}\n"),
        "a {\
         \n  b: lab(73.6% 106 28.8 / 0.3);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {\
             \n  b: color.scale(lab(70% 100 -60 / 0.3), $lightness: 12%, $a: 24%, $b: 48%);\
             \n}\n"
        ),
        "a {\
         \n  b: lab(73.6% 106 28.8 / 0.3);\
         \n}\n"
    );
}
mod b {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lab(70% 100 -60), $b: 42%)}\n"),
            "a {\
         \n  b: lab(70% 100 17.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lab(70% 100 -60), $b: -16%)}\n"),
            "a {\
         \n  b: lab(70% 100 -70.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lab(70% 100 -60), $b: 100%)}\n"),
            "a {\
         \n  b: lab(70% 100 125);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lab(70% 100 -60), $b: -100%)}\n"),
            "a {\
         \n  b: lab(70% 100 -125);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lab(70% 100 -60), $b: 0%)}\n"),
            "a {\
         \n  b: lab(70% 100 -60);\
         \n}\n"
        );
    }
}
mod lightness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lab(70% 100 -60), $lightness: 86%)}\n"),
            "a {\
         \n  b: lab(95.8% 100 -60);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lab(70% 100 -60), $lightness: -33%)}\n"),
            "a {\
         \n  b: lab(46.9% 100 -60);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lab(70% 100 -60), $lightness: 100%)}\n"),
            "a {\
         \n  b: lab(100% 100 -60);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lab(70% 100 -60), $lightness: -100%)}\n"),
            "a {\
         \n  b: lab(0% 100 -60);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lab(70% 100 -60), $lightness: 0%)}\n"),
            "a {\
         \n  b: lab(70% 100 -60);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {\
             \n  b: color.scale($color: lab(70% 100 -60), $lightness: 12%, $a: 24%, $b: 48%);\
             \n}\n"
        ),
        "a {\
         \n  b: lab(73.6% 106 28.8);\
         \n}\n"
    );
}
