//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/one_arg/no_alpha.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_alpha")
}

mod clamped {
    #[allow(unused)]
    use super::runner;

    mod lightness {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn above() {
            assert_eq!(
                runner().ok("a {b: hsl(0 100% 500%)}\n"),
                "a {\
         \n  b: hsl(0, 100%, 500%);\
         \n}\n"
            );
        }
        #[test]
        fn below() {
            assert_eq!(
                runner().ok("a {b: hsl(0 100% -100%)}\n"),
                "a {\
         \n  b: hsl(0, 100%, -100%);\
         \n}\n"
            );
        }
    }
    mod saturation {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn below() {
            assert_eq!(
                runner().ok("a {b: hsl(0 -100% 50%)}\n"),
                "a {\
         \n  b: hsl(0, 0%, 50%);\
         \n}\n"
            );
        }
    }
}
mod in_gamut {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn blue() {
        assert_eq!(
            runner().ok("a {b: hsl(240 100% 50%)}\n"),
            "a {\
         \n  b: hsl(240, 100%, 50%);\
         \n}\n"
        );
    }
    mod grayish {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn yellow() {
            assert_eq!(
                runner().ok("a {b: hsl(60 60% 50%)}\n"),
                "a {\
         \n  b: hsl(60, 60%, 50%);\
         \n}\n"
            );
        }
    }
    #[test]
    fn red() {
        assert_eq!(
            runner().ok("a {b: hsl(0 100% 50%)}\n"),
            "a {\
         \n  b: hsl(0, 100%, 50%);\
         \n}\n"
        );
    }
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("a {b: hsl(none 100% 50%)}\n"),
            "a {\
         \n  b: hsl(none 100% 50%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("a {b: hsl(0 100% none)}\n"),
            "a {\
         \n  b: hsl(0deg 100% none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn saturation() {
        assert_eq!(
            runner().ok("a {b: hsl(0 none 50%)}\n"),
            "a {\
         \n  b: hsl(0deg none 50%);\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: hsl($channels: 0 100% 50%)}\n"),
        "a {\
         \n  b: hsl(0, 100%, 50%);\
         \n}\n"
    );
}
mod out_of_gamut {
    #[allow(unused)]
    use super::runner;

    mod saturation {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn above() {
            assert_eq!(
                runner().ok("a {b: hsl(0 500% 50%)}\n"),
                "a {\
         \n  b: hsl(0, 500%, 50%);\
         \n}\n"
            );
        }
    }
}
mod units {
    #[allow(unused)]
    use super::runner;

    mod hue {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn deg() {
            assert_eq!(
                runner().ok("a {b: hsl(0deg 100% 50%)}\n"),
                "a {\
         \n  b: hsl(0, 100%, 50%);\
         \n}\n"
            );
        }
    }
    mod lightness {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn unitless() {
            assert_eq!(
                runner().ok("a {b: hsl(0 100% 50)}\n"),
                "a {\
         \n  b: hsl(0, 100%, 50%);\
         \n}\n"
            );
        }
    }
    mod saturation {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn unitless() {
            assert_eq!(
                runner().ok("a {b: hsl(0 50 50%)}\n"),
                "a {\
         \n  b: hsl(0, 50%, 50%);\
         \n}\n"
            );
        }
    }
}
