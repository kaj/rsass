//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/lch.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lch")
}

#[test]
#[ignore] // unexepected error
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg), $lightness: 40%, $chroma: 50, $hue: 60)}\n"
        ),
        "a {\
         \n  b: lch(70% 110 240deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg), $lightness: 50%, $alpha: -0.1)}\n"
        ),
        "a {\
         \n  b: lch(80% 60 180deg / 0.9);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg / 0.9), $lightness: 50%)}\n"
        ),
        "a {\
         \n  b: lch(80% 60 180deg / 0.9);\
         \n}\n"
    );
}
mod chroma {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg), $chroma: 100)}\n"),
            "a {\
         \n  b: lch(30% 160 180deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg), $chroma: 200)}\n"),
            "a {\
         \n  b: lch(30% 260 180deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg), $chroma: -200)}\n"),
            "a {\
         \n  b: lch(30% 0 180deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg), $chroma: -100)}\n"),
            "a {\
         \n  b: lch(30% 0 180deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg), $chroma: 40%)}\n"),
            "a {\
         \n  b: lch(30% 120 180deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg), $chroma: -30)}\n"),
            "a {\
         \n  b: lch(30% 30 180deg);\
         \n}\n"
        );
    }
}
mod hue {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn turn() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg), $hue: 0.5turn)}\n"),
            "a {\
         \n  b: lch(30% 60 0deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg), $hue: -30)}\n"),
            "a {\
         \n  b: lch(30% 60 150deg);\
         \n}\n"
        );
    }
}
mod lightness {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg), $lightness: 90)}\n"),
            "a {\
         \n  b: lch(100% 60 180deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg), $lightness: 120%)}\n"),
            "a {\
         \n  b: lch(100% 60 180deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg), $lightness: -130%)}\n"),
            "a {\
         \n  b: lch(0% 60 180deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg), $lightness: -40%)}\n"),
            "a {\
         \n  b: lch(0% 60 180deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg), $lightness: -10%)}\n"),
            "a {\
         \n  b: lch(20% 60 180deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(lch(30% 60 180deg), $lightness: 50)}\n"),
            "a {\
         \n  b: lch(80% 60 180deg);\
         \n}\n"
        );
    }
}
