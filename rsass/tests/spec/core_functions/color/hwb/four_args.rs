//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/four_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("four_args")
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
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 30%, 40%, calc(NaN))}\n"),
                "a {\
         \n  b: hsla(0, 33.3333333333%, 45%, 0);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn negative_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 30%, 40%, calc(-infinity))}\n"),
                "a {\
         \n  b: hsla(0, 33.3333333333%, 45%, 0);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn positive_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 30%, 40%, calc(infinity))}\n"),
                "a {\
         \n  b: hsl(0, 33.3333333333%, 45%);\
         \n}\n"
            );
        }
    }
    mod percent {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn above_max() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 0%, 0%, 250%)}\n"),
                "a {\
         \n  b: red;\
         \n}\n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 0%, 0%, 100%)}\n"),
                "a {\
         \n  b: red;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn min() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 0%, 0%, 0%)}\n"),
                "a {\
         \n  b: hsla(0, 100%, 50%, 0);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn negative() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 0%, 0%, -10%)}\n"),
                "a {\
         \n  b: hsla(0, 100%, 50%, 0);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn positive() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 0%, 0%, 45.6%)}\n"),
                "a {\
         \n  b: hsla(0, 100%, 50%, 0.456);\
         \n}\n"
            );
        }
    }
    mod unitless {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn above_max() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 0%, 0%, 250)}\n"),
                "a {\
         \n  b: red;\
         \n}\n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 0%, 0%, 1)}\n"),
                "a {\
         \n  b: red;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn min() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 0%, 0%, 0)}\n"),
                "a {\
         \n  b: hsla(0, 100%, 50%, 0);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn negative() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 0%, 0%, -10)}\n"),
                "a {\
         \n  b: hsla(0, 100%, 50%, 0);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn positive() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 0%, 0%, 0.456)}\n"),
                "a {\
         \n  b: hsla(0, 100%, 50%, 0.456);\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn var() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 0%, 0%, var(--c))}\n"),
            "a {\
         \n  b: hwb(0 0% 0% / var(--c));\
         \n}\n"
        );
    }
}
mod blackness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 30%, 101%, 0.5)}\n"),
            "a {\
         \n  b: hsla(0, 0%, 22.9007633588%, 0.5);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 30%, -1%, 0.5)}\n"),
            "a {\
         \n  b: hsla(0, 102.8985507246%, 65.5%, 0.5);\
         \n}\n"
        );
    }
    mod degenerate {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn nan() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 30%, calc(NaN * 1%), 0.5)}\n"),
                "a {\
         \n  b: hsla(calc(NaN), calc(NaN * 1%), calc(NaN * 1%), 0.5);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn negative_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 30%, calc(-infinity * 1%), 0.5)}\n"),
                "a {\
         \n  b: hsla(calc(NaN), calc(NaN * 1%), calc(NaN * 1%), 0.5);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn positive_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 30%, calc(infinity * 1%), 0.5)}\n"),
                "a {\
         \n  b: hsla(calc(NaN), calc(NaN * 1%), calc(NaN * 1%), 0.5);\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn var() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 30%, var(--c), 0.5)}\n"),
            "a {\
         \n  b: hwb(0 30% var(--c) / 0.5);\
         \n}\n"
        );
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
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(calc(NaN), 30%, 40%, 0.5)}\n"),
                "a {\
         \n  b: hsla(0, 0%, 30%, 0.5);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn negative_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(calc(-infinity), 30%, 40%, 0.5)}\n"),
                "a {\
         \n  b: hsla(0, 0%, 30%, 0.5);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn positive_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(calc(infinity), 30%, 40%, 0.5)}\n"),
                "a {\
         \n  b: hsla(0, 0%, 30%, 0.5);\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn var() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(var(--c), 30%, 40%, 0.5)}\n"),
            "a {\
         \n  b: hwb(var(--c) 30% 40% / 0.5);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.hwb($hue: 180, $whiteness: 30%, $blackness: 40%, $alpha: 0.4)}\n"
        ),
        "a {\
         \n  b: hsla(180, 33.3333333333%, 45%, 0.4);\
         \n}\n"
    );
}
mod whiteness {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above_max() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, 101%, 40%, 0.5)}\n"),
            "a {\
         \n  b: hsla(0, 0%, 71.6312056738%, 0.5);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below_min() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, -1%, 40%, 0.5)}\n"),
            "a {\
         \n  b: hsla(0, 103.3898305085%, 29.5%, 0.5);\
         \n}\n"
        );
    }
    mod degenerate {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn nan() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, calc(NaN * 1%), 40%, 0.5)}\n"),
                "a {\
         \n  b: hsla(calc(NaN), calc(NaN * 1%), calc(NaN * 1%), 0.5);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn negative_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, calc(-infinity * 1%), 40%, 0.5)}\n"),
                "a {\
         \n  b: hsla(calc(NaN), calc(NaN * 1%), calc(NaN * 1%), 0.5);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn positive_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, calc(infinity * 1%), 40%, 0.5)}\n"),
                "a {\
         \n  b: hsla(calc(NaN), calc(NaN * 1%), calc(NaN * 1%), 0.5);\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn var() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.hwb(0, var(--c), 40%, 0.5)}\n"),
            "a {\
         \n  b: hwb(0 var(--c) 40% / 0.5);\
         \n}\n"
        );
    }
}
