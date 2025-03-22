//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/lch.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lch")
}

#[test]
#[ignore] // unexepected error
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(lch(50% 20 30deg), $lightness: 20%, $chroma: 30, $hue: 40deg)}\n"
        ),
        "a {\
         \n  b: lch(20% 30 40deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(lch(50% 20 30deg), $lightness: 30%, $alpha: 0.9)}\n"
        ),
        "a {\
         \n  b: lch(30% 20 30deg / 0.9);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(lch(50% 20 30deg / 0.9), $lightness: 30%)}\n"
        ),
        "a {\
         \n  b: lch(30% 20 30deg / 0.9);\
         \n}\n"
    );
}
mod chroma {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn negative() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lch(50% 20 30deg), $chroma: -10)}\n"),
            "a {\
         \n  b: lch(50% 10 210deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lch(50% 20 30deg), $chroma: none)}\n"),
            "a {\
         \n  b: lch(50% none 30deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lch(50% 20 30deg), $chroma: 200)}\n"),
            "a {\
         \n  b: lch(50% 200 30deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lch(50% 20 30deg), $chroma: 40%)}\n"),
            "a {\
         \n  b: lch(50% 60 30deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lch(50% 20 30deg), $chroma: 50)}\n"),
            "a {\
         \n  b: lch(50% 50 30deg);\
         \n}\n"
        );
    }
}
mod hue {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lch(50% 20 30deg), $hue: 400deg)}\n"),
            "a {\
         \n  b: lch(50% 20 40deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn negative() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lch(50% 20 30deg), $hue: -20deg)}\n"),
            "a {\
         \n  b: lch(50% 20 340deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lch(50% 20 30deg), $hue: none)}\n"),
            "a {\
         \n  b: lch(50% 20 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lch(50% 20 30deg), $hue: 50deg)}\n"),
            "a {\
         \n  b: lch(50% 20 50deg);\
         \n}\n"
        );
    }
}
mod lightness {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn none() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lch(50% 20 30deg), $lightness: none)}\n"),
            "a {\
         \n  b: lch(none 20 30deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_range() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.change(lch(50% 20 30deg), $lightness: 120%)}\n"
        ),
        "a {\
         \n  b: color-mix(in lch, color(xyz 1.6569354424 1.6040925936 1.5400032443) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lch(50% 20 30deg), $lightness: 30%)}\n"),
            "a {\
         \n  b: lch(30% 20 30deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.change(lch(50% 20 30deg), $lightness: 30)}\n"),
            "a {\
         \n  b: lch(30% 20 30deg);\
         \n}\n"
        );
    }
}
