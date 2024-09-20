//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/four_args/clamped.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("clamped")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn above() {
        assert_eq!(
            runner().ok("a {b: rgb(0, 0, 0, 1.1)}\n"),
            "a {\
         \n  b: rgb(0, 0, 0);\
         \n}\n"
        );
    }
    #[test]
    fn below() {
        assert_eq!(
            runner().ok("a {b: rgb(0, 0, 0, -0.1)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0);\
         \n}\n"
        );
    }
    mod degenerate {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn nan() {
            assert_eq!(
                runner().ok("a {b: rgb(0, 0, 0, calc(NaN))}\n"),
                "a {\
         \n  b: rgba(0, 0, 0, 0);\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("a {b: rgb(0, 0, 0, calc(-infinity))}\n"),
                "a {\
         \n  b: rgba(0, 0, 0, 0);\
         \n}\n"
            );
        }
        #[test]
        fn positive_infinity() {
            assert_eq!(
                runner().ok("a {b: rgb(0, 0, 0, calc(infinity))}\n"),
                "a {\
         \n  b: rgb(0, 0, 0);\
         \n}\n"
            );
        }
    }
}
mod blue {
    #[allow(unused)]
    use super::runner;

    mod degenerate {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn nan() {
            assert_eq!(
                runner().ok("a {b: rgb(0, 0, calc(NaN), 0.5)}\n"),
                "a {\
         \n  b: rgba(0, 0, 0, 0.5);\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("a {b: rgb(0, 0, calc(-infinity), 0.5)}\n"),
                "a {\
         \n  b: rgba(0, 0, 0, 0.5);\
         \n}\n"
            );
        }
        #[test]
        fn positive_infinity() {
            assert_eq!(
                runner().ok("a {b: rgb(0, 0, calc(infinity), 0.5)}\n"),
                "a {\
         \n  b: rgba(0, 0, 255, 0.5);\
         \n}\n"
            );
        }
    }
    #[test]
    fn finite() {
        assert_eq!(
            runner().ok("a {b: rgb(0, 0, 9999, 0.5)}\n"),
            "a {\
         \n  b: rgba(0, 0, 255, 0.5);\
         \n}\n"
        );
    }
}
mod green {
    #[allow(unused)]
    use super::runner;

    mod degenerate {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn nan() {
            assert_eq!(
                runner().ok("a {b: rgb(0, calc(NaN), 0, 0.5)}\n"),
                "a {\
         \n  b: rgba(0, 0, 0, 0.5);\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("a {b: rgb(0, calc(-infinity), 0, 0.5)}\n"),
                "a {\
         \n  b: rgba(0, 0, 0, 0.5);\
         \n}\n"
            );
        }
        #[test]
        fn positive_infinity() {
            assert_eq!(
                runner().ok("a {b: rgb(0, calc(infinity), 0, 0.5)}\n"),
                "a {\
         \n  b: rgba(0, 255, 0, 0.5);\
         \n}\n"
            );
        }
    }
    #[test]
    fn finite() {
        assert_eq!(
            runner().ok("a {b: rgb(0, -1, 0, 0.5)}\n"),
            "a {\
         \n  b: rgba(0, 0, 0, 0.5);\
         \n}\n"
        );
    }
}
mod red {
    #[allow(unused)]
    use super::runner;

    mod degenerate {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn nan() {
            assert_eq!(
                runner().ok("a {b: rgb(calc(NaN), 0, 0, 0.5)}\n"),
                "a {\
         \n  b: rgba(0, 0, 0, 0.5);\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("a {b: rgb(calc(-infinity), 0, 0, 0.5)}\n"),
                "a {\
         \n  b: rgba(0, 0, 0, 0.5);\
         \n}\n"
            );
        }
        #[test]
        fn positive_infinity() {
            assert_eq!(
                runner().ok("a {b: rgb(calc(infinity), 0, 0, 0.5)}\n"),
                "a {\
         \n  b: rgba(255, 0, 0, 0.5);\
         \n}\n"
            );
        }
    }
    #[test]
    fn finite() {
        assert_eq!(
            runner().ok("a {b: rgb(256, 0, 0, 0.5)}\n"),
            "a {\
         \n  b: rgba(255, 0, 0, 0.5);\
         \n}\n"
        );
    }
}
