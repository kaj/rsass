//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/prophoto_rgb/rec2020.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rec2020")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 0.3 / 0.4), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 -0.0432446288 0.3038001186 0.4081087118 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 0.3 / 0.0), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 -0.0432446288 0.3038001186 0.4081087118 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0 0 0), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.5 0.5 0.5), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.5946035575 0.5946035575 0.5946035575);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.2 0.4 0.8), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 -0.2630461834 0.5078350487 0.8542926787);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 none), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.1778424423 0.3064837711 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.1 none 0.3), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.0843320335 none 0.4113790691);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb none 0.2 0.3), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 none 0.3062349485 0.4079803537);\
         \n}\n"
    );
    }
}
mod out_of_range {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn far() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb -999999 0 0), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 -34126.4613425073 10438.5304334963 -3629.4726472762);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb -1 0.4 2), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 -1.2504929227 0.5521844512 1.7035970907);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 1 1 1), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 1 1 1);\
         \n}\n"
    );
}
