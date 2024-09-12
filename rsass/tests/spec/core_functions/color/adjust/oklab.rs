//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/oklab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklab")
}

mod a {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $a: 0.7)}\n"),
            "a {\
         \n  b: oklab(30% 0.8 -0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $a: 1)}\n"),
            "a {\
         \n  b: oklab(30% 1.1 -0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $a: -1)}\n"),
            "a {\
         \n  b: oklab(30% -0.9 -0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $a: -0.6)}\n"),
            "a {\
         \n  b: oklab(30% -0.5 -0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $a: 40%)}\n"),
            "a {\
         \n  b: oklab(30% 0.26 -0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $a: -0.2)}\n"),
            "a {\
         \n  b: oklab(30% -0.1 -0.3);\
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
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $lightness: 40%, $a: 0.2, $b: 0.3)}\n"
        ),
        "a {\
         \n  b: oklab(70% 0.3 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $lightness: 50%, $alpha: -0.1)}\n"
        ),
        "a {\
         \n  b: oklab(80% 0.1 -0.3 / 0.9);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3 / 0.9), $lightness: 50%)}\n"
        ),
        "a {\
         \n  b: oklab(80% 0.1 -0.3 / 0.9);\
         \n}\n"
    );
}
mod b {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $b: 0.8)}\n"),
            "a {\
         \n  b: oklab(30% 0.1 0.5);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $b: 1)}\n"),
            "a {\
         \n  b: oklab(30% 0.1 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $b: -1)}\n"),
            "a {\
         \n  b: oklab(30% 0.1 -1.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $b: -0.2)}\n"),
            "a {\
         \n  b: oklab(30% 0.1 -0.5);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $b: 20%)}\n"),
            "a {\
         \n  b: oklab(30% 0.1 -0.22);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $b: -0.05)}\n"),
            "a {\
         \n  b: oklab(30% 0.1 -0.35);\
         \n}\n"
        );
    }
}
mod lightness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $lightness: 0.9)}\n"),
            "a {\
         \n  b: oklab(100% 0.1 -0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $lightness: 120%)}\n"),
            "a {\
         \n  b: oklab(100% 0.1 -0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $lightness: -130%)}\n"
            ),
            "a {\
         \n  b: oklab(0% 0.1 -0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $lightness: -40%)}\n"),
            "a {\
         \n  b: oklab(0% 0.1 -0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $lightness: -10%)}\n"),
            "a {\
         \n  b: oklab(20% 0.1 -0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklab(30% 0.1 -0.3), $lightness: 0.5)}\n"),
            "a {\
         \n  b: oklab(80% 0.1 -0.3);\
         \n}\n"
        );
    }
}
