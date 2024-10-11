//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/oklch.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklch")
}

#[test]
#[ignore] // unexepected error
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 30deg), $lightness: 20%, $chroma: 0.1, $hue: 40deg)}\n"
        ),
        "a {\
         \n  b: oklch(20% 0.1 40deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 30deg), $lightness: 30%, $alpha: 0.9)}\n"
        ),
        "a {\
         \n  b: oklch(30% 0.2 30deg / 0.9);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 30deg / 0.9), $lightness: 30%)}\n"
        ),
        "a {\
         \n  b: oklch(30% 0.2 30deg / 0.9);\
         \n}\n"
    );
}
mod chroma {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn negative() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 30deg), $chroma: -0.1)}\n"),
            "a {\
         \n  b: oklch(50% 0.1 210deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 30deg), $chroma: none)}\n"),
            "a {\
         \n  b: oklch(50% none 30deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 30deg), $chroma: 1)}\n"),
            "a {\
         \n  b: oklch(50% 1 30deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 30deg), $chroma: 40%)}\n"),
            "a {\
         \n  b: oklch(50% 0.16 30deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 30deg), $chroma: 0.1)}\n"),
            "a {\
         \n  b: oklch(50% 0.1 30deg);\
         \n}\n"
        );
    }
}
mod hue {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 30deg), $hue: 400deg)}\n"),
            "a {\
         \n  b: oklch(50% 0.2 40deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn negative() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 30deg), $hue: -20deg)}\n"),
            "a {\
         \n  b: oklch(50% 0.2 340deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 30deg), $hue: none)}\n"),
            "a {\
         \n  b: oklch(50% 0.2 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 30deg), $hue: 50deg)}\n"),
            "a {\
         \n  b: oklch(50% 0.2 50deg);\
         \n}\n"
        );
    }
}
mod lightness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 30deg), $lightness: none)}\n"
            ),
            "a {\
         \n  b: oklch(none 0.2 30deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 30deg), $lightness: 120%)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz 2.0602077969 1.6344741917 1.0169248199) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 30deg), $lightness: 30%)}\n"),
            "a {\
         \n  b: oklch(30% 0.2 30deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(oklch(50% 0.2 30deg), $lightness: 0.3)}\n"),
            "a {\
         \n  b: oklch(30% 0.2 30deg);\
         \n}\n"
        );
    }
}
