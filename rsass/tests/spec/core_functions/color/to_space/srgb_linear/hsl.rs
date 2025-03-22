//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/srgb_linear/hsl.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hsl")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.4), hsl)}\n"
        ),
        "a {\
         \n  b: hsla(205.3925362149, 25.148533711%, 46.6510851344%, 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.0), hsl)}\n"
        ),
        "a {\
         \n  b: hsla(205.3925362149, 25.148533711%, 46.6510851344%, 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0 0 0), hsl)}\n"),
        "a {\
         \n  b: hsl(0, 0%, 0%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.5 0.5 0.5), hsl)}\n"),
        "a {\
         \n  b: hsl(0, 0%, 73.5356983052%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.2 0.4 0.8), hsl)}\n"),
        "a {\
         \n  b: hsl(214.3023060477, 69.2456926348%, 69.5430478913%);\
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
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 none), hsl)}\n"
            ),
            "a {\
         \n  b: hsl(76.7592364631, 100%, 24.2264602241%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 none 0.3), hsl)}\n"
            ),
            "a {\
         \n  b: hsl(275.8860614996, 100%, 29.191574503%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear none 0.2 0.3), hsl)}\n"
            ),
            "a {\
         \n  b: hsl(190.2052342776, 100%, 29.191574503%);\
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
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear -999999 0 0), hsl)}\n"),
            "a {\
         \n  b: hsl(0, 100%, -16678.2577069634%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear -1 0.4 2), hsl)}\n"),
            "a {\
         \n  b: hsl(197.5434618594, 666.1615765111%, 17.6628023075%);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 1 1 1), hsl)}\n"),
        "a {\
         \n  b: hsl(0, 0%, 100%);\
         \n}\n"
    );
}
