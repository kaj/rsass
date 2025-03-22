//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/oklab.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklab")
}

mod foreign {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"a\", $space: oklab)}\n"),
            "a {\
         \n  b: 0.072980372;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"b\", $space: oklab)}\n"),
            "a {\
         \n  b: 0.0090714168;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"lightness\", $space: oklab)}\n"),
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
             \na {b: color.channel(oklab(0.5 -1 50%), \"a\")}\n"),
            "a {\
         \n  b: -1;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(oklab(0.5 -1 50%), \"b\")}\n"),
            "a {\
         \n  b: 0.2;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(oklab(0.5 -1 50%), \"lightness\")}\n"),
            "a {\
         \n  b: 50%;\
         \n}\n"
        );
    }
}
