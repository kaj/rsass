//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/srgb/oklch.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklch")
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
             \na {b: color.to-space(color(srgb 0.1 0.2 0.3 / 0.4), oklch)}\n"
            ),
            "a {\
         \n  b: oklch(31.3834098842% 0.0558899329 250.0274938863deg / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.1 0.2 0.3 / 0.0), oklch)}\n"
            ),
            "a {\
         \n  b: oklch(31.3834098842% 0.0558899329 250.0274938863deg / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0 0 0), oklch)}\n"),
        "a {\
         \n  b: oklch(0% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.5 0.5 0.5), oklch)}\n"),
        "a {\
         \n  b: oklch(59.8180726623% 0.0000000223 89.8755628286deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.2 0.4 0.8), oklch)}\n"),
        "a {\
         \n  b: oklch(53.248255955% 0.1678655044 262.2930469968deg);\
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
             \na {b: color.to-space(color(srgb 0.1 0.2 none), oklch)}\n"),
            "a {\
         \n  b: oklch(28.8978411941% 0.0816919892 133.1088653771deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.1 none 0.3), oklch)}\n"),
            "a {\
         \n  b: oklch(21.1364004026% 0.1228104689 285.8458860201deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb none 0.2 0.3), oklch)}\n"),
            "a {\
         \n  b: oklch(30.4674630654% 0.0672785114 237.7397947754deg);\
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
             \na {b: color.to-space(color(srgb -999999 0 0), oklch)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz -91096581353071.39 -46971674760177.49 -4270152250925.1875) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb -1 0.4 2), oklch)}\n"),
            "a {\
         \n  b: oklch(49.0997083563% 2.1120784248 196.6442570354deg);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 1 1 1), oklch)}\n"),
        "a {\
         \n  b: oklch(99.9999993474% 0.0000000373 89.8755630959deg);\
         \n}\n"
    );
}
