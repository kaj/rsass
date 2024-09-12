//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/srgb/oklab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklab")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.1 0.2 0.3 / 0.4), oklab)}\n"
            ),
            "a {\
         \n  b: oklab(31.3834098842% -0.0190902788 -0.0525285242 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.1 0.2 0.3 / 0.0), oklab)}\n"
            ),
            "a {\
         \n  b: oklab(31.3834098842% -0.0190902788 -0.0525285242 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0 0 0), oklab)}\n"),
        "a {\
         \n  b: oklab(0% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.5 0.5 0.5), oklab)}\n"),
        "a {\
         \n  b: oklab(59.8180726623% 0 0.0000000223);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.2 0.4 0.8), oklab)}\n"),
        "a {\
         \n  b: oklab(53.248255955% -0.0225118457 -0.1663491641);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.1 0.2 none), oklab)}\n"),
            "a {\
         \n  b: oklab(28.8978411941% -0.0558272225 0.0596397714);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.1 none 0.3), oklab)}\n"),
            "a {\
         \n  b: oklab(21.1364004026% 0.0335334923 -0.1181436252);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb none 0.2 0.3), oklab)}\n"),
            "a {\
         \n  b: oklab(30.4674630654% -0.0359109243 -0.0568929135);\
         \n}\n"
        );
    }
}
mod out_of_range {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn far() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb -999999 0 0), oklab)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz -91096581353071.4 -46971674760177.49 -4270152250925.199) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb -1 0.4 2), oklab)}\n"),
            "a {\
         \n  b: oklab(49.0997083563% -2.0235857478 -0.6049594977);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 1 1 1), oklab)}\n"),
        "a {\
         \n  b: oklab(99.9999993474% 0.0000000001 0.0000000373);\
         \n}\n"
    );
}
