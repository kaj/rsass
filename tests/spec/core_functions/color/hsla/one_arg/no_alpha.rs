//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/one_arg/no_alpha.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
                runner().ok("a {b: hsla(0 100% 500%)}\n"),
                "a {\
         \n  b: white;\
         \n}\n"
            );
        }
        #[test]
        fn below() {
            assert_eq!(
                runner().ok("a {b: hsla(0 100% -100%)}\n"),
                "a {\
         \n  b: black;\
         \n}\n"
            );
        }
    }
    mod saturation {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn above() {
            assert_eq!(
                runner().ok("a {b: hsla(0 500% 50%)}\n"),
                "a {\
         \n  b: red;\
         \n}\n"
            );
        }
        #[test]
        fn below() {
            assert_eq!(
                runner().ok("a {b: hsla(0 -100% 50%)}\n"),
                "a {\
         \n  b: gray;\
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
            runner().ok("a {b: hsla(240 100% 50%)}\n"),
            "a {\
         \n  b: blue;\
         \n}\n"
        );
    }
    mod grayish {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn yellow() {
            assert_eq!(
                runner().ok("a {b: hsla(60 60% 50%)}\n"),
                "a {\
         \n  b: #cccc33;\
         \n}\n"
            );
        }
    }
    #[test]
    fn red() {
        assert_eq!(
            runner().ok("a {b: hsla(0 100% 50%)}\n"),
            "a {\
         \n  b: red;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: hsla($channels: 0 100% 50%)}\n"),
        "a {\
         \n  b: red;\
         \n}\n"
    );
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
                runner().ok("a {b: hsla(0deg 100% 50%)}\n"),
                "a {\
         \n  b: red;\
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
                runner().ok("a {b: hsla(0 100% 50)}\n"),
                "a {\
         \n  b: red;\
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
                runner().ok("a {b: hsla(0 50 50%)}\n"),
                "a {\
         \n  b: #bf4040;\
         \n}\n"
            );
        }
    }
}
