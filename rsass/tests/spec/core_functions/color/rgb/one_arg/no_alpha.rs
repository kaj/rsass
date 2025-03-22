//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/one_arg/no_alpha.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_alpha")
}

mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("a {b: rgb(18 52 none)}\n"),
            "a {\
         \n  b: rgb(18 52 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("a {b: rgb(18 none 66)}\n"),
            "a {\
         \n  b: rgb(18 none 66);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("a {b: rgb(none 52 66)}\n"),
            "a {\
         \n  b: rgb(none 52 66);\
         \n}\n"
        );
    }
}
mod percents {
    use super::runner;

    mod all {
        use super::runner;

        #[test]
        fn percent() {
            assert_eq!(
                runner().ok("a {b: rgb(7.1% 20.4% 33.9%)}\n"),
                "a {\
         \n  b: rgb(18.105, 52.02, 86.445);\
         \n}\n"
            );
        }
    }
    #[test]
    fn boundaries() {
        assert_eq!(
            runner().ok("a {b: rgb(0% 100% 50%)}\n"),
            "a {\
         \n  b: rgb(0, 255, 127.5);\
         \n}\n"
        );
    }
    mod clamped {
        use super::runner;

        #[test]
        fn blue() {
            assert_eq!(
                runner().ok("a {b: rgb(0 0 200%)}\n"),
                "a {\
         \n  b: rgb(0, 0, 255);\
         \n}\n"
            );
        }
        #[test]
        fn green() {
            assert_eq!(
                runner().ok("a {b: rgb(0 -0.1% 0)}\n"),
                "a {\
         \n  b: rgb(0, 0, 0);\
         \n}\n"
            );
        }
        #[test]
        fn red() {
            assert_eq!(
                runner().ok("a {b: rgb(100.1% 0 0)}\n"),
                "a {\
         \n  b: rgb(255, 0, 0);\
         \n}\n"
            );
        }
    }
    mod percent {
        use super::runner;

        #[test]
        fn green() {
            assert_eq!(
                runner().ok("a {b: rgb(190 68% 237)}\n"),
                "a {\
         \n  b: rgb(190, 173.4, 237);\
         \n}\n"
            );
        }
    }
    mod unitless {
        use super::runner;

        #[test]
        fn green() {
            assert_eq!(
                runner().ok("a {b: rgb(74.7% 173 93%)}\n"),
                "a {\
         \n  b: rgb(190.485, 173, 237.15);\
         \n}\n"
            );
        }
    }
}
mod unitless {
    use super::runner;

    #[test]
    fn beaded() {
        assert_eq!(
            runner().ok("a {b: rgb(190 173 237)}\n"),
            "a {\
         \n  b: rgb(190, 173, 237);\
         \n}\n"
        );
    }
    mod clamped {
        use super::runner;

        #[test]
        fn blue() {
            assert_eq!(
                runner().ok("a {b: rgb(0 0 9999)}\n"),
                "a {\
         \n  b: rgb(0, 0, 255);\
         \n}\n"
            );
        }
        #[test]
        fn green() {
            assert_eq!(
                runner().ok("a {b: rgb(0 -1 0)}\n"),
                "a {\
         \n  b: rgb(0, 0, 0);\
         \n}\n"
            );
        }
        #[test]
        fn red() {
            assert_eq!(
                runner().ok("a {b: rgb(256 0 0)}\n"),
                "a {\
         \n  b: rgb(255, 0, 0);\
         \n}\n"
            );
        }
    }
    #[test]
    fn named() {
        assert_eq!(
            runner().ok("a {b: rgb($channels: 0 255 127)}\n"),
            "a {\
         \n  b: rgb(0, 255, 127);\
         \n}\n"
        );
    }
    #[test]
    fn numbers() {
        assert_eq!(
            runner().ok("a {b: rgb(18 52 86)}\n"),
            "a {\
         \n  b: rgb(18, 52, 86);\
         \n}\n"
        );
    }
    #[test]
    fn springgreen() {
        assert_eq!(
            runner().ok("a {b: rgb(0 255 127)}\n"),
            "a {\
         \n  b: rgb(0, 255, 127);\
         \n}\n"
        );
    }
}
