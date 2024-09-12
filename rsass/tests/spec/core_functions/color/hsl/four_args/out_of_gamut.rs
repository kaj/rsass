//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/four_args/out_of_gamut.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("out_of_gamut")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    mod degenerate {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn nan() {
            assert_eq!(
                runner().ok("a {b: hsl(0, 100%, 50%, calc(NaN))}\n"),
                "a {\
         \n  b: hsla(0, 100%, 50%, 0);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn negative_infinity() {
            assert_eq!(
                runner().ok("a {b: hsl(0, 100%, 50%, calc(-infinity))}\n"),
                "a {\
         \n  b: hsla(0, 100%, 50%, 0);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
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
        #[allow(unused)]
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
    #[allow(unused)]
    use super::runner;

    mod degenerate {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn nan() {
            assert_eq!(
                runner().ok("a {b: hsl(calc(NaN), 100%, 50%)}\n"),
                "a {\
         \n  b: hsl(calc(NaN), 100%, 50%);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn negative_infinity() {
            assert_eq!(
                runner().ok("a {b: hsl(calc(-infinity), 100%, 50%)}\n"),
                "a {\
         \n  b: hsl(calc(NaN), 100%, 50%);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
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
    #[allow(unused)]
    use super::runner;

    mod degenerate {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn nan() {
            assert_eq!(
                runner().ok("a {b: hsl(0, 100%, calc(NaN))}\n"),
                "a {\
         \n  b: hsl(0, 100%, calc(NaN * 1%));\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn negative_infinity() {
            assert_eq!(
                runner().ok("a {b: hsl(0, 100%, calc(-infinity))}\n"),
                "a {\
         \n  b: hsl(0, 100%, calc(-infinity * 1%));\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
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
    #[ignore] // wrong result
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
    #[allow(unused)]
    use super::runner;

    mod degenerate {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn nan() {
            assert_eq!(
                runner().ok("a {b: hsl(0, calc(NaN), 50%)}\n"),
                "a {\
         \n  b: hsl(0, 0%, 50%);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn negative_infinity() {
            assert_eq!(
                runner().ok("a {b: hsl(0, calc(-infinity), 50%)}\n"),
                "a {\
         \n  b: hsl(0, 0%, 50%);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
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
