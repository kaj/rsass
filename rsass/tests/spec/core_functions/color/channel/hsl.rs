//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/hsl.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hsl")
}

mod foreign {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"hue\", $space: hsl)}\n"),
            "a {\
         \n  b: 349.5238095238deg;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"lightness\", $space: hsl)}\n"),
            "a {\
         \n  b: 87.6470588235%;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn saturation() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"saturation\", $space: hsl)}\n"),
            "a {\
         \n  b: 100%;\
         \n}\n"
        );
    }
}
mod local {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(hsl(10 40% 60%), \"hue\")}\n"),
            "a {\
         \n  b: 10deg;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(hsl(10deg 40% 60%), \"lightness\")}\n"),
            "a {\
         \n  b: 60%;\
         \n}\n"
        );
    }
    mod normalized {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn hue() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.channel(hsl(540 40% 60%), \"hue\")}\n"),
                "a {\
         \n  b: 180deg;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn saturation() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(hsl(10deg 40% 60%), \"saturation\")}\n"),
            "a {\
         \n  b: 40%;\
         \n}\n"
        );
    }
}
