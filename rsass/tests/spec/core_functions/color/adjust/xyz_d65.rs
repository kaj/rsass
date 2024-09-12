//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/xyz_d65.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz_d65")
}

#[test]
#[ignore] // unexepected error
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $x: 0.3, $y: 0.2, $z: 0.1)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.5 0.7 0.8);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $x: 0.5, $alpha: -0.1)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.7 0.5 0.7 / 0.9);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7 / 0.9), $x: 0.5)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.7 0.5 0.7 / 0.9);\
         \n}\n"
    );
}
mod x {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $x: 0.9)}\n"),
            "a {\
         \n  b: color(xyz 1.1 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $x: 1.9)}\n"),
            "a {\
         \n  b: color(xyz 2.1 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $x: -1.3)}\n"),
            "a {\
         \n  b: color(xyz -1.1 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $x: -0.3)}\n"),
            "a {\
         \n  b: color(xyz -0.1 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $x: -10%)}\n"),
            "a {\
         \n  b: color(xyz 0.1 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $x: 0.5)}\n"),
            "a {\
         \n  b: color(xyz 0.7 0.5 0.7);\
         \n}\n"
        );
    }
}
mod y {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $y: 0.7)}\n"),
            "a {\
         \n  b: color(xyz 0.2 1.2 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $y: 1.7)}\n"),
            "a {\
         \n  b: color(xyz 0.2 2.2 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $y: -1.8)}\n"),
            "a {\
         \n  b: color(xyz 0.2 -1.3 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $y: -0.8)}\n"),
            "a {\
         \n  b: color(xyz 0.2 -0.3 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $y: 40%)}\n"),
            "a {\
         \n  b: color(xyz 0.2 0.9 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $y: -0.3)}\n"),
            "a {\
         \n  b: color(xyz 0.2 0.2 0.7);\
         \n}\n"
        );
    }
}
mod z {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $z: 0.7)}\n"),
            "a {\
         \n  b: color(xyz 0.2 0.5 1.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $z: 1.7)}\n"),
            "a {\
         \n  b: color(xyz 0.2 0.5 2.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $z: -1.8)}\n"),
            "a {\
         \n  b: color(xyz 0.2 0.5 -1.1);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $z: -0.8)}\n"),
            "a {\
         \n  b: color(xyz 0.2 0.5 -0.1);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $z: 20%)}\n"),
            "a {\
         \n  b: color(xyz 0.2 0.5 0.9);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(color(xyz-d65 0.2 0.5 0.7), $z: -0.3)}\n"),
            "a {\
         \n  b: color(xyz 0.2 0.5 0.4);\
         \n}\n"
        );
    }
}
