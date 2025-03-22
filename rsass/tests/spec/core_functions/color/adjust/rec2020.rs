//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/rec2020.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rec2020")
}

#[test]
#[ignore] // unexepected error
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $red: 0.3, $green: 0.2, $blue: 0.1)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.5 0.7 0.8);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $red: 0.5, $alpha: -0.1)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.7 0.5 0.7 / 0.9);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7 / 0.9), $red: 0.5)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.7 0.5 0.7 / 0.9);\
         \n}\n"
    );
}
mod blue {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $blue: 0.7)}\n"
            ),
            "a {\
         \n  b: color(rec2020 0.2 0.5 1.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $blue: 1.7)}\n"
            ),
            "a {\
         \n  b: color(rec2020 0.2 0.5 2.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $blue: -1.8)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.2 0.5 -1.1);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $blue: -0.8)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.2 0.5 -0.1);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $blue: 20%)}\n"
            ),
            "a {\
         \n  b: color(rec2020 0.2 0.5 0.9);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $blue: -0.3)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.2 0.5 0.4);\
         \n}\n"
    );
    }
}
mod green {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $green: 0.7)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.2 1.2 0.7);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $green: 1.7)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.2 2.2 0.7);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $green: -1.8)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.2 -1.3 0.7);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $green: -0.8)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.2 -0.3 0.7);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $green: 40%)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.2 0.9 0.7);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $green: -0.3)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.2 0.2 0.7);\
         \n}\n"
    );
    }
}
mod red {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $red: 0.9)}\n"),
            "a {\
         \n  b: color(rec2020 1.1 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $red: 1.9)}\n"),
            "a {\
         \n  b: color(rec2020 2.1 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $red: -1.3)}\n"
            ),
            "a {\
         \n  b: color(rec2020 -1.1 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $red: -0.3)}\n"
            ),
            "a {\
         \n  b: color(rec2020 -0.1 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $red: -10%)}\n"
            ),
            "a {\
         \n  b: color(rec2020 0.1 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(rec2020 0.2 0.5 0.7), $red: 0.5)}\n"),
            "a {\
         \n  b: color(rec2020 0.7 0.5 0.7);\
         \n}\n"
        );
    }
}
