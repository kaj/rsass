//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/oklch.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklch")
}

#[test]
#[ignore] // unexepected error
fn all() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(oklch(30% 0.15 180deg), $lightness: 40%, $chroma: 0.2, $hue: 60)}\n"
        ),
        "a {\
         \n  b: oklch(70% 0.35 240deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_arg() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(oklch(30% 0.15 180deg), $lightness: 50%, $alpha: -0.1)}\n"
        ),
        "a {\
         \n  b: oklch(80% 0.15 180deg / 0.9);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn alpha_input() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(oklch(30% 0.15 180deg / 0.9), $lightness: 50%)}\n"
        ),
        "a {\
         \n  b: oklch(80% 0.15 180deg / 0.9);\
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
             \na {b: color.adjust(oklch(30% 0.15 180deg), $chroma: 0.3)}\n"),
            "a {\
         \n  b: oklch(30% 0.45 180deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklch(30% 0.15 180deg), $chroma: 1)}\n"),
            "a {\
         \n  b: oklch(30% 1.15 180deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklch(30% 0.15 180deg), $chroma: -1)}\n"),
            "a {\
         \n  b: oklch(30% 0 180deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklch(30% 0.15 180deg), $chroma: -0.2)}\n"),
            "a {\
         \n  b: oklch(30% 0 180deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklch(30% 0.15 180deg), $chroma: 40%)}\n"),
            "a {\
         \n  b: oklch(30% 0.31 180deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklch(30% 0.15 180deg), $chroma: -0.1)}\n"),
            "a {\
         \n  b: oklch(30% 0.05 180deg);\
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
             \na {b: color.adjust(oklch(30% 0.15 180deg), $hue: 0.5turn)}\n"),
            "a {\
         \n  b: oklch(30% 0.15 0deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.adjust(oklch(30% 0.15 180deg), $hue: -30)}\n"),
            "a {\
         \n  b: oklch(30% 0.15 150deg);\
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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(oklch(30% 0.15 180deg), $lightness: 0.9)}\n"
        ),
        "a {\
         \n  b: oklch(100% 0.15 180deg);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_above_max() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(oklch(30% 0.15 180deg), $lightness: 120%)}\n"
        ),
        "a {\
         \n  b: oklch(100% 0.15 180deg);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_below_min() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(oklch(30% 0.15 180deg), $lightness: -130%)}\n"
        ),
        "a {\
         \n  b: oklch(0% 0.15 180deg);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(oklch(30% 0.15 180deg), $lightness: -40%)}\n"
        ),
        "a {\
         \n  b: oklch(0% 0.15 180deg);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn percent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(oklch(30% 0.15 180deg), $lightness: -10%)}\n"
        ),
        "a {\
         \n  b: oklch(20% 0.15 180deg);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.adjust(oklch(30% 0.15 180deg), $lightness: 0.5)}\n"
        ),
        "a {\
         \n  b: oklch(80% 0.15 180deg);\
         \n}\n"
    );
    }
}
