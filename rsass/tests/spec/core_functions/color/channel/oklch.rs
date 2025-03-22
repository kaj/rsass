//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/oklch.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklch")
}

mod foreign {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn chroma() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"chroma\", $space: oklch)}\n"),
            "a {\
         \n  b: 0.0735419968;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"hue\", $space: oklch)}\n"),
            "a {\
         \n  b: 7.0854893498deg;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"lightness\", $space: oklch)}\n"),
            "a {\
         \n  b: 86.7738450841%;\
         \n}\n"
        );
    }
}
mod local {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(oklch(0.3 80% 30), \"chroma\")}\n"),
            "a {\
         \n  b: 0.32;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(oklch(0.3 80% 30), \"hue\")}\n"),
            "a {\
         \n  b: 30deg;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(oklch(0.3 80% 30), \"lightness\")}\n"),
            "a {\
         \n  b: 30%;\
         \n}\n"
        );
    }
}
