//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/oklab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklab")
}

mod a {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(oklab(70% 0.3 -0.25), $a: 12%)}\n"),
            "a {\
         \n  b: oklab(70% 0.312 -0.25);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(oklab(70% 0.3 -0.25), $a: -86%)}\n"),
            "a {\
         \n  b: oklab(70% -0.302 -0.25);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(oklab(70% 0.3 -0.25), $a: 100%)}\n"),
            "a {\
         \n  b: oklab(70% 0.4 -0.25);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(oklab(70% 0.3 -0.25), $a: -100%)}\n"),
            "a {\
         \n  b: oklab(70% -0.4 -0.25);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(oklab(70% 0.3 -0.25), $a: 0%)}\n"),
            "a {\
         \n  b: oklab(70% 0.3 -0.25);\
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
             \n  b: color.scale(oklab(70% 0.3 -0.25), $lightness: 12%, $a: 24%, $b: 48%);\
             \n}\n"
        ),
        "a {\
         \n  b: oklab(73.6% 0.324 0.062);\
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
             \n    oklab(70% 0.3 -0.25),\
             \n    $lightness: 12%, $a: 24%, $b: 48%, $alpha: -70%\
             \n  );\
             \n}\n"),
        "a {\
         \n  b: oklab(73.6% 0.324 0.062 / 0.3);\
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
             \n  b: color.scale(oklab(70% 0.3 -0.25 / 0.3), $lightness: 12%, $a: 24%, $b: 48%);\
             \n}\n"
        ),
        "a {\
         \n  b: oklab(73.6% 0.324 0.062 / 0.3);\
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
             \na {b: color.scale(oklab(70% 0.3 -0.25), $b: 42%)}\n"),
            "a {\
         \n  b: oklab(70% 0.3 0.023);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(oklab(70% 0.3 -0.25), $b: -16%)}\n"),
            "a {\
         \n  b: oklab(70% 0.3 -0.274);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(oklab(70% 0.3 -0.25), $b: 100%)}\n"),
            "a {\
         \n  b: oklab(70% 0.3 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(oklab(70% 0.3 -0.25), $b: -100%)}\n"),
            "a {\
         \n  b: oklab(70% 0.3 -0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(oklab(70% 0.3 -0.25), $b: 0%)}\n"),
            "a {\
         \n  b: oklab(70% 0.3 -0.25);\
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
             \na {b: color.scale(oklab(70% 0.3 -0.25), $lightness: 86%)}\n"),
            "a {\
         \n  b: oklab(95.8% 0.3 -0.25);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(oklab(70% 0.3 -0.25), $lightness: -33%)}\n"),
            "a {\
         \n  b: oklab(46.9% 0.3 -0.25);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(oklab(70% 0.3 -0.25), $lightness: 100%)}\n"),
            "a {\
         \n  b: oklab(100% 0.3 -0.25);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn min() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.scale(oklab(70% 0.3 -0.25), $lightness: -100%)}\n"
            ),
            "a {\
         \n  b: oklab(0% 0.3 -0.25);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(oklab(70% 0.3 -0.25), $lightness: 0%)}\n"),
            "a {\
         \n  b: oklab(70% 0.3 -0.25);\
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
             \n  b: color.scale($color: oklab(70% 0.3 -0.25), $lightness: 12%, $a: 24%, $b: 48%);\
             \n}\n"
        ),
        "a {\
         \n  b: oklab(73.6% 0.324 0.062);\
         \n}\n"
    );
}
