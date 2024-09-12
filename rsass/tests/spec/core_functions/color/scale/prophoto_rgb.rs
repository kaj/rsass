//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/prophoto_rgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("prophoto_rgb")
}

#[test]
#[ignore] // unexepected error
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {\
             \n  b: color.scale(\
             \n    color(prophoto-rgb 0.2 0.5 0.7), $red: 12%, $green: 24%, $blue: 48%\
             \n  );\
             \n}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.296 0.62 0.844);\
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
             \n    color(prophoto-rgb 0.2 0.5 0.7),\
             \n    $red: 12%, $green: 24%, $blue: 48%, $alpha: -70%\
             \n  );\
             \n}\n"),
        "a {\
         \n  b: color(prophoto-rgb 0.296 0.62 0.844 / 0.3);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_input() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.scale(\
             \n    color(prophoto-rgb 0.2 0.5 0.7 / 0.3),\
             \n    $red: 12%, $green: 24%, $blue: 48%\
             \n  );\
             \n}\n"),
        "a {\
         \n  b: color(prophoto-rgb 0.296 0.62 0.844 / 0.3);\
         \n}\n"
    );
}
mod blue {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn high() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(color(prophoto-rgb 0.2 0.5 0.7), $blue: 42%)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2 0.5 0.826);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn low() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(color(prophoto-rgb 0.2 0.5 0.7), $blue: -16%)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2 0.5 0.588);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn max() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(color(prophoto-rgb 0.2 0.5 0.7), $blue: 100%)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2 0.5 1);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn min() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(color(prophoto-rgb 0.2 0.5 0.7), $blue: -100%)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2 0.5 0);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(color(prophoto-rgb 0.2 0.5 0.7), $blue: 0%)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2 0.5 0.7);\
         \n}\n"
    );
    }
}
mod green {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn high() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(color(prophoto-rgb 0.2 0.5 0.7), $green: 12%)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2 0.56 0.7);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn low() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(color(prophoto-rgb 0.2 0.5 0.7), $green: -86%)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2 0.07 0.7);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn max() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(color(prophoto-rgb 0.2 0.5 0.7), $green: 100%)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2 1 0.7);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn min() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(color(prophoto-rgb 0.2 0.5 0.7), $green: -100%)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2 0 0.7);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(color(prophoto-rgb 0.2 0.5 0.7), $green: 0%)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2 0.5 0.7);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {\
             \n  b: color.scale(\
             \n    $color: color(prophoto-rgb 0.2 0.5 0.7),\
             \n    $red: 12%, $green: 24%, $blue: 48%\
             \n  );\
             \n}\n"),
        "a {\
         \n  b: color(prophoto-rgb 0.296 0.62 0.844);\
         \n}\n"
    );
}
mod red {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn high() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(color(prophoto-rgb 0.2 0.5 0.7), $red: 86%)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.888 0.5 0.7);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn low() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(color(prophoto-rgb 0.2 0.5 0.7), $red: -33%)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.134 0.5 0.7);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn max() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(color(prophoto-rgb 0.2 0.5 0.7), $red: 100%)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 1 0.5 0.7);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn min() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(color(prophoto-rgb 0.2 0.5 0.7), $red: -100%)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0 0.5 0.7);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(color(prophoto-rgb 0.2 0.5 0.7), $red: 0%)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2 0.5 0.7);\
         \n}\n"
    );
    }
}
