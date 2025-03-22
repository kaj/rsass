//! Tests auto-converted from "sass-spec/spec/core_functions/color/is_powerless/hwb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hwb")
}

mod hue {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn full_blackness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(hwb(0deg 0% 100%), \"hue\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn full_whiteness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(hwb(0deg 100% 0%), \"hue\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn grey() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(hwb(0deg 40% 60%), \"hue\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn normalized_grey() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(hwb(0deg 60% 80%), \"hue\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn not_greyscale() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(hwb(0deg 30% 40%), \"hue\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    mod out_of_gamut {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn colorful() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(hwb(0deg -30% 50%), \"hue\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn grey() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(hwb(0deg -30% 130%), \"hue\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
    }
}
