//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/lch.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lch")
}

mod foreign {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn chroma() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"chroma\", $space: lch)}\n"),
            "a {\
         \n  b: 24.7242595195;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"hue\", $space: lch)}\n"),
            "a {\
         \n  b: 8.7459803895deg;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"lightness\", $space: lch)}\n"),
            "a {\
         \n  b: 83.7872528656%;\
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
             \na {b: color.channel(lch(50 80% 0.5turn), \"chroma\")}\n"),
            "a {\
         \n  b: 120;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(lch(50 80% 0.5turn), \"hue\")}\n"),
            "a {\
         \n  b: 180deg;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(lch(50 80% 0.5turn), \"lightness\")}\n"),
            "a {\
         \n  b: 50%;\
         \n}\n"
        );
    }
}
