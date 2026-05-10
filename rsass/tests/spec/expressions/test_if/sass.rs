//! Tests auto-converted from "sass-spec/spec/expressions/if/sass.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("sass")
}

mod alone {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn expression() {
        assert_eq!(
            runner().ok("$a: true;\
             \nb {c: if(sass($a): d; else: e)}\n"),
            "b {\
         \n  c: d;\
         \n}\n"
        );
    }
    mod test_false {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn test_else() {
            assert_eq!(
                runner().ok("a {b: if(sass(false): c; else: d)}\n"),
                "a {\
         \n  b: d;\
         \n}\n"
            );
        }
        #[test]
        fn no_else() {
            assert_eq!(
                runner().ok("a {b: if(sass(false): c) == null}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn test_true() {
        assert_eq!(
            runner().ok("a {b: if(sass(true): c; else: d)}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
}
mod and {
    use super::runner;

    mod t2 {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn and_paren() {
            assert_eq!(
                runner().ok(
                    "a {b: if(sass(true) and (sass(true)): c; else: d)}\n"
                ),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn css_and_false() {
            assert_eq!(
                runner().ok("a {b: if(css() and sass(false): c; else: d)}\n"),
                "a {\
         \n  b: d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn css_and_true() {
            assert_eq!(
                runner().ok("a {b: if(css() and sass(true): c; else: d)}\n"),
                "a {\
         \n  b: if(css(): c; else: d);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn false_and_css() {
            assert_eq!(
                runner().ok("a {b: if(sass(false) and css(): c; else: d)}\n"),
                "a {\
         \n  b: d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn false_and_false() {
            assert_eq!(
                runner().ok(
                    "a {b: if(sass(false) and sass(false): c; else: d)}\n"
                ),
                "a {\
         \n  b: d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn false_and_interp() {
            assert_eq!(
                runner()
                    .ok("a {b: if(sass(true) and #{css()}: c; else: d)}\n"),
                "a {\
         \n  b: if(css(): c; else: d);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn false_and_true() {
            assert_eq!(
                runner().ok(
                    "a {b: if(sass(false) and sass(true): c; else: d)}\n"
                ),
                "a {\
         \n  b: d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn interp_and_false() {
            assert_eq!(
                runner()
                    .ok("a {b: if(#{css()} and sass(false): c; else: d)}\n"),
                "a {\
         \n  b: d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn interp_and_true() {
            assert_eq!(
                runner()
                    .ok("a {b: if(#{css()} and sass(true): c; else: d)}\n"),
                "a {\
         \n  b: if(css(): c; else: d);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn paren_and() {
            assert_eq!(
                runner().ok(
                    "a {b: if((sass(true)) and sass(true): c; else: d)}\n"
                ),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn true_and_css() {
            assert_eq!(
                runner().ok("a {b: if(sass(true) and css(): c; else: d)}\n"),
                "a {\
         \n  b: if(css(): c; else: d);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn true_and_false() {
            assert_eq!(
                runner().ok(
                    "a {b: if(sass(true) and sass(false): c; else: d)}\n"
                ),
                "a {\
         \n  b: d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn true_and_interp() {
            assert_eq!(
                runner()
                    .ok("a {b: if(sass(true) and #{css()}: c; else: d)}\n"),
                "a {\
         \n  b: if(css(): c; else: d);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn true_and_true() {
            assert_eq!(
                runner()
                    .ok("a {b: if(sass(true) and sass(true): c; else: d)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
    }
    mod t3 {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn css_and_false_and_css() {
            assert_eq!(
        runner().ok(
            "a {b: if(css(1) and sass(false) and css(2): c; else: d)}\n"
        ),
        "a {\
         \n  b: d;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn css_and_true_and_css() {
            assert_eq!(
        runner().ok(
            "a {b: if(css(1) and sass(true) and css(2): c; else: d)}\n"
        ),
        "a {\
         \n  b: if(css(1) and css(2): c; else: d);\
         \n}\n"
    );
        }
    }
    mod t4 {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn test_false() {
            assert_eq!(
        runner().ok(
            "a {b: if(sass(true) and sass(true) and sass(false) and sass(true): c; else: d)}\n"
        ),
        "a {\
         \n  b: d;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn test_true() {
            assert_eq!(
        runner().ok(
            "a {b: if(sass(true) and sass(true) and sass(true) and sass(true): c; else: d)}\n"
        ),
        "a {\
         \n  b: c;\
         \n}\n"
    );
        }
    }
}
mod not {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn test_false() {
        assert_eq!(
            runner().ok("a {b: if(not sass(false): c; else: d)}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn paren() {
        assert_eq!(
            runner().ok("a {b: if(not (sass(true)): c; else: d)}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn test_true() {
        assert_eq!(
            runner().ok("a {b: if(not sass(true): c; else: d)}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
}
mod or {
    use super::runner;

    mod t2 {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn css_or_false() {
            assert_eq!(
                runner().ok("a {b: if(css() or sass(false): c; else: d)}\n"),
                "a {\
         \n  b: if(css(): c; else: d);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn css_or_true() {
            assert_eq!(
                runner().ok("a {b: if(css() or sass(true): c; else: d)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn false_or_css() {
            assert_eq!(
                runner().ok("a {b: if(sass(false) or css(): c; else: d)}\n"),
                "a {\
         \n  b: if(css(): c; else: d);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn false_or_false() {
            assert_eq!(
                runner().ok(
                    "a {b: if(sass(false) or sass(false): c; else: d)}\n"
                ),
                "a {\
         \n  b: d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn false_or_interp() {
            assert_eq!(
                runner()
                    .ok("a {b: if(sass(true) or #{css()}: c; else: d)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn false_or_true() {
            assert_eq!(
                runner()
                    .ok("a {b: if(sass(false) or sass(true): c; else: d)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn interp_or_false() {
            assert_eq!(
                runner()
                    .ok("a {b: if(#{css()} or sass(false): c; else: d)}\n"),
                "a {\
         \n  b: if(css(): c; else: d);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn interp_or_true() {
            assert_eq!(
                runner()
                    .ok("a {b: if(#{css()} or sass(true): c; else: d)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn or_paren() {
            assert_eq!(
                runner().ok(
                    "a {b: if(sass(true) or (sass(true)): c; else: d)}\n"
                ),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn paren_or() {
            assert_eq!(
                runner().ok(
                    "a {b: if((sass(true)) or sass(true): c; else: d)}\n"
                ),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn true_or_css() {
            assert_eq!(
                runner().ok("a {b: if(sass(true) or css(): c; else: d)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn true_or_false() {
            assert_eq!(
                runner()
                    .ok("a {b: if(sass(true) or sass(false): c; else: d)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn true_or_interp() {
            assert_eq!(
                runner()
                    .ok("a {b: if(sass(true) or #{css()}: c; else: d)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn true_or_true() {
            assert_eq!(
                runner()
                    .ok("a {b: if(sass(true) or sass(true): c; else: d)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
    }
    mod t3 {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn css_or_false_or_css() {
            assert_eq!(
                runner().ok(
                    "a {b: if(css(1) or sass(false) or css(2): c; else: d)}\n"
                ),
                "a {\
         \n  b: if(css(1) or css(2): c; else: d);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn css_or_true_or_css() {
            assert_eq!(
                runner().ok(
                    "a {b: if(css(1) or sass(true) or css(2): c; else: d)}\n"
                ),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
    }
    mod t4 {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn test_false() {
            assert_eq!(
        runner().ok(
            "a {b: if(sass(false) or sass(false) or sass(false) or sass(false): c; else: d)}\n"
        ),
        "a {\
         \n  b: d;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn test_true() {
            assert_eq!(
        runner().ok(
            "a {b: if(sass(false) or sass(false) or sass(true) or sass(false): c; else: d)}\n"
        ),
        "a {\
         \n  b: c;\
         \n}\n"
    );
        }
    }
}
mod paren {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn and() {
        assert_eq!(
            runner()
                .ok("a {b: if((sass(true) and sass(true)): c; else: d)}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn test_false() {
        assert_eq!(
            runner().ok("a {b: if((sass(false)): c; else: d)}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn not() {
        assert_eq!(
            runner().ok("a {b: if((not sass(true)): c; else: d)}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn or() {
        assert_eq!(
            runner()
                .ok("a {b: if((sass(true) or sass(true)): c; else: d)}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn test_true() {
        assert_eq!(
            runner().ok("a {b: if((sass(true)): c; else: d)}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
}
