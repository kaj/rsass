//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/lch.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lch")
}

#[test]
#[ignore] // unexepected error
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale(lch(70% 20% 80deg), $lightness: 12%, $chroma: 24%)}\n"
        ),
        "a {\
         \n  b: lch(73.6% 58.8 80deg);\
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
             \n    lch(70% 20% 80deg),\
             \n    $lightness: 12%, $chroma: 24%, $alpha: -70%\
             \n  );\
             \n}\n"),
        "a {\
         \n  b: lch(73.6% 58.8 80deg / 0.3);\
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
             \n  b: color.scale(lch(70% 20% 80deg / 0.3), $lightness: 12%, $chroma: 24%);\
             \n}\n"
        ),
        "a {\
         \n  b: lch(73.6% 58.8 80deg / 0.3);\
         \n}\n"
    );
}
mod chroma {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lch(70% 20% 80deg), $chroma: 12%)}\n"),
            "a {\
         \n  b: lch(70% 44.4 80deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lch(70% 20% 80deg), $chroma: -86%)}\n"),
            "a {\
         \n  b: lch(70% 4.2 80deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lch(70% 20% 80deg), $chroma: 100%)}\n"),
            "a {\
         \n  b: lch(70% 150 80deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lch(70% 20% 80deg), $chroma: -100%)}\n"),
            "a {\
         \n  b: lch(70% 0 80deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lch(70% 20% 80deg), $chroma: 0%)}\n"),
            "a {\
         \n  b: lch(70% 30 80deg);\
         \n}\n"
        );
    }
}
mod lightness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lch(70% 20% 80deg), $lightness: 86%)}\n"),
            "a {\
         \n  b: lch(95.8% 30 80deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lch(70% 20% 80deg), $lightness: -33%)}\n"),
            "a {\
         \n  b: lch(46.9% 30 80deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lch(70% 20% 80deg), $lightness: 100%)}\n"),
            "a {\
         \n  b: lch(100% 30 80deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lch(70% 20% 80deg), $lightness: -100%)}\n"),
            "a {\
         \n  b: lch(0% 30 80deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.scale(lch(70% 20% 80deg), $lightness: 0%)}\n"),
            "a {\
         \n  b: lch(70% 30 80deg);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.scale($color: lch(70% 20% 80deg), $lightness: 12%, $chroma: 24%)}\n"
        ),
        "a {\
         \n  b: lch(73.6% 58.8 80deg);\
         \n}\n"
    );
}
