//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/xyz_d50.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz_d50")
}

#[test]
#[ignore] // unexepected error
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {\
             \n  b: color.scale(color(xyz-d50 0.2 0.5 0.7), $x: 12%, $y: 24%, $z: 48%);\
             \n}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 0.296 0.62 0.844);\
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
             \n    color(xyz-d50 0.2 0.5 0.7),\
             \n    $x: 12%, $y: 24%, $z: 48%, $alpha: -70%\
             \n  );\
             \n}\n"),
        "a {\
         \n  b: color(xyz-d50 0.296 0.62 0.844 / 0.3);\
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
             \n  b: color.scale(color(xyz-d50 0.2 0.5 0.7 / 0.3), $x: 12%, $y: 24%, $z: 48%);\
             \n}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 0.296 0.62 0.844 / 0.3);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {\
             \n  b: color.scale($color: color(xyz-d50 0.2 0.5 0.7), $x: 12%, $y: 24%, $z: 48%);\
             \n}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 0.296 0.62 0.844);\
         \n}\n"
    );
}
mod x {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $x: 86%)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.888 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $x: -33%)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.134 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $x: 100%)}\n"),
            "a {\
         \n  b: color(xyz-d50 1 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $x: -100%)}\n"),
            "a {\
         \n  b: color(xyz-d50 0 0.5 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $x: 0%)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.2 0.5 0.7);\
         \n}\n"
        );
    }
}
mod y {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $y: 12%)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.2 0.56 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $y: -86%)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.2 0.07 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $y: 100%)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.2 1 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $y: -100%)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.2 0 0.7);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $y: 0%)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.2 0.5 0.7);\
         \n}\n"
        );
    }
}
mod z {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $z: 42%)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.2 0.5 0.826);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $z: -16%)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.2 0.5 0.588);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $z: 100%)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.2 0.5 1);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $z: -100%)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.2 0.5 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(color(xyz-d50 0.2 0.5 0.7), $z: 0%)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.2 0.5 0.7);\
         \n}\n"
        );
    }
}
