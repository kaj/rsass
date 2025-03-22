//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/four_args/out_of_gamut.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("out_of_gamut")
}

mod alpha {
    use super::runner;

    mod degenerate {
        use super::runner;

        #[test]
        fn nan() {
            assert_eq!(
                runner().ok("a {b: hsl(0, 100%, 50%, calc(NaN))}\n"),
                "a {\
         \n  b: hsla(0, 100%, 50%, 0);\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("a {b: hsl(0, 100%, 50%, calc(-infinity))}\n"),
                "a {\
         \n  b: hsla(0, 100%, 50%, 0);\
         \n}\n"
            );
        }
        #[test]
        fn positive_infinity() {
            assert_eq!(
                runner().ok("a {b: hsl(0, 100%, 50%, calc(infinity))}\n"),
                "a {\
         \n  b: hsl(0, 100%, 50%);\
         \n}\n"
            );
        }
    }
    mod finite {
        use super::runner;

        #[test]
        fn above() {
            assert_eq!(
                runner().ok("a {b: hsl(0, 100%, 50%, 1.1)}\n"),
                "a {\
         \n  b: hsl(0, 100%, 50%);\
         \n}\n"
            );
        }
        #[test]
        fn below() {
            assert_eq!(
                runner().ok("a {b: hsla(0, 100%, 50%, -0.1)}\n"),
                "a {\
         \n  b: hsla(0, 100%, 50%, 0);\
         \n}\n"
            );
        }
    }
}
mod hue {
    use super::runner;

    mod degenerate {
        use super::runner;

        #[test]
        fn nan() {
            assert_eq!(
                runner().ok("a {b: hsl(calc(NaN), 100%, 50%)}\n"),
                "a {\
         \n  b: hsl(calc(NaN), 100%, 50%);\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("a {b: hsl(calc(-infinity), 100%, 50%)}\n"),
                "a {\
         \n  b: hsl(calc(NaN), 100%, 50%);\
         \n}\n"
            );
        }
        #[test]
        fn positive_infinity() {
            assert_eq!(
                runner().ok("a {b: hsl(calc(infinity), 100%, 50%)}\n"),
                "a {\
         \n  b: hsl(calc(NaN), 100%, 50%);\
         \n}\n"
            );
        }
    }
}
mod lightness {
    use super::runner;

    mod degenerate {
        use super::runner;

        #[test]
        fn nan() {
            assert_eq!(
                runner().ok("a {b: hsl(0, 100%, calc(NaN))}\n"),
                "a {\
         \n  b: hsl(0, 100%, calc(NaN * 1%));\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("a {b: hsl(0, 100%, calc(-infinity))}\n"),
                "a {\
         \n  b: hsl(0, 100%, calc(-infinity * 1%));\
         \n}\n"
            );
        }
        #[test]
        fn positive_infinity() {
            assert_eq!(
                runner().ok("a {b: hsl(0, 100%, calc(infinity))}\n"),
                "a {\
         \n  b: hsl(0, 100%, calc(infinity * 1%));\
         \n}\n"
            );
        }
    }
    #[test]
    fn finite() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 100%, 9999%, 0.5)}\n"),
            "a {\
         \n  b: hsla(0, 100%, 9999%, 0.5);\
         \n}\n"
        );
    }
}
mod saturation {
    use super::runner;

    mod degenerate {
        use super::runner;

        #[test]
        fn nan() {
            assert_eq!(
                runner().ok("a {b: hsl(0, calc(NaN), 50%)}\n"),
                "a {\
         \n  b: hsl(0, 0%, 50%);\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("a {b: hsl(0, calc(-infinity), 50%)}\n"),
                "a {\
         \n  b: hsl(0, 0%, 50%);\
         \n}\n"
            );
        }
        #[test]
        fn positive_infinity() {
            assert_eq!(
                runner().ok("a {b: hsl(0, calc(infinity), 50%)}\n"),
                "a {\
         \n  b: hsl(0, calc(infinity * 1%), 50%);\
         \n}\n"
            );
        }
    }
    #[test]
    fn finite() {
        assert_eq!(
            runner().ok("a {b: hsl(0, -0.1%, 50%, 0.5)}\n"),
            "a {\
         \n  b: hsla(0, 0%, 50%, 0.5);\
         \n}\n"
        );
    }
}
