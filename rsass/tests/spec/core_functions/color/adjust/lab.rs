//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/lab.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lab")
}

mod a {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $a: 70)}\n"),
            "a {\
         \n  b: lab(30% 130 -100);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $a: 130)}\n"),
            "a {\
         \n  b: lab(30% 190 -100);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $a: -500)}\n"),
            "a {\
         \n  b: lab(30% -440 -100);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $a: -200)}\n"),
            "a {\
         \n  b: lab(30% -140 -100);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $a: 40%)}\n"),
            "a {\
         \n  b: lab(30% 110 -100);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $a: -30)}\n"),
            "a {\
         \n  b: lab(30% 30 -100);\
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
             \na {b: color.adjust(lab(30% 60 -100), $lightness: 40%, $a: 50, $b: 60)}\n"
        ),
        "a {\
         \n  b: lab(70% 110 -40);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $lightness: 50%, $alpha: -0.1)}\n"
        ),
        "a {\
         \n  b: lab(80% 60 -100 / 0.9);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100 / 0.9), $lightness: 50%)}\n"
        ),
        "a {\
         \n  b: lab(80% 60 -100 / 0.9);\
         \n}\n"
    );
}
mod b {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $b: 70)}\n"),
            "a {\
         \n  b: lab(30% 60 -30);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $b: 500)}\n"),
            "a {\
         \n  b: lab(30% 60 400);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $b: -500)}\n"),
            "a {\
         \n  b: lab(30% 60 -600);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $b: -80)}\n"),
            "a {\
         \n  b: lab(30% 60 -180);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $b: 20%)}\n"),
            "a {\
         \n  b: lab(30% 60 -75);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $b: -30)}\n"),
            "a {\
         \n  b: lab(30% 60 -130);\
         \n}\n"
        );
    }
}
mod lightness {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $lightness: 90)}\n"),
            "a {\
         \n  b: lab(100% 60 -100);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $lightness: 120%)}\n"),
            "a {\
         \n  b: lab(100% 60 -100);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $lightness: -130%)}\n"),
            "a {\
         \n  b: lab(0% 60 -100);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $lightness: -40%)}\n"),
            "a {\
         \n  b: lab(0% 60 -100);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $lightness: -10%)}\n"),
            "a {\
         \n  b: lab(20% 60 -100);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lab(30% 60 -100), $lightness: 50)}\n"),
            "a {\
         \n  b: lab(80% 60 -100);\
         \n}\n"
        );
    }
}
