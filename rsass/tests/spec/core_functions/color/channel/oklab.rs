//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/oklab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklab")
}

mod foreign {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"a\", $space: oklab)}\n"),
            "a {\
         \n  b: 0.0729803698;\
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
         \n  b: 0.0090714488;\
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
         \n  b: 86.7738445294%;\
         \n}\n"
        );
    }
}
mod local {
    #[allow(unused)]
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
