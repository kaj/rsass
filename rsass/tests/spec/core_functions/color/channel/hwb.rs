//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/hwb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hwb")
}

mod foreign {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blackness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"blackness\", $space: hwb)}\n"),
            "a {\
         \n  b: 0%;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"hue\", $space: hwb)}\n"),
            "a {\
         \n  b: 349.5238095238deg;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn whiteness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(pink, \"whiteness\", $space: hwb)}\n"),
            "a {\
         \n  b: 75.2941176471%;\
         \n}\n"
        );
    }
}
mod local {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blackness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(hwb(10deg 30% 40%), \"blackness\")}\n"),
            "a {\
         \n  b: 40%;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(hwb(10 30% 40%), \"hue\")}\n"),
            "a {\
         \n  b: 10deg;\
         \n}\n"
        );
    }
    mod normalized {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn blackness() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.channel(hwb(10deg 70% 50%), \"blackness\")}\n"),
                "a {\
         \n  b: 41.6666666667%;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn whiteness() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.channel(hwb(10deg 70% 50%), \"whiteness\")}\n"),
                "a {\
         \n  b: 58.3333333333%;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn whiteness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.channel(hwb(10deg 30% 40%), \"whiteness\")}\n"),
            "a {\
         \n  b: 30%;\
         \n}\n"
        );
    }
}
