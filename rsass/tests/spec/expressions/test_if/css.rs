//! Tests auto-converted from "sass-spec/spec/expressions/if/css.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css")
}

mod alone {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn argument() {
        assert_eq!(
            runner().ok(
                "a {b: if(css(!@#$%^&*(){}[]_-+=|:;\'\'\"\"<>,./?): c)}\n"
            ),
            "a {\
         \n  b: if(css(!@#$%^&*(){}[]_-+=|:;\'\'\"\"<>,./?): c);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn test_else() {
        assert_eq!(
            runner().ok("a {b: if(css(): c; else: d)}\n"),
            "a {\
         \n  b: if(css(): c; else: d);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn no_else() {
        assert_eq!(
            runner().ok("a {b: if(css(): c)}\n"),
            "a {\
         \n  b: if(css(): c);\
         \n}\n"
        );
    }
}
mod and {
    use super::runner;

    mod t2 {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn and_paren() {
            assert_eq!(
                runner().ok("a {b: if(css(1) and (css(2)): c)}\n"),
                "a {\
         \n  b: if(css(1) and (css(2)): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn css() {
            assert_eq!(
                runner().ok("a {b: if(css(1) and css(2): c)}\n"),
                "a {\
         \n  b: if(css(1) and css(2): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn paren_and() {
            assert_eq!(
                runner().ok("a {b: if((css(1)) and css(2): c)}\n"),
                "a {\
         \n  b: if((css(1)) and css(2): c);\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn t3() {
        assert_eq!(
            runner().ok("a {b: if(css(1) and css(2) and css(3): c)}\n"),
            "a {\
         \n  b: if(css(1) and css(2) and css(3): c);\
         \n}\n"
        );
    }
}
mod interpolation {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn argument() {
        assert_eq!(
            runner().ok("a {b: if(css(#{argument}): c)}\n"),
            "a {\
         \n  b: if(css(argument): c);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn name() {
        assert_eq!(
            runner().ok("a {b: if(#{css}(): c)}\n"),
            "a {\
         \n  b: if(css(): c);\
         \n}\n"
        );
    }
}
mod not {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn direct() {
        assert_eq!(
            runner().ok("a {b: if(not css(): c)}\n"),
            "a {\
         \n  b: if(not css(): c);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn paren() {
        assert_eq!(
            runner().ok("a {b: if(not (css()): c)}\n"),
            "a {\
         \n  b: if(not (css()): c);\
         \n}\n"
        );
    }
}
mod or {
    use super::runner;

    mod t2 {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn css() {
            assert_eq!(
                runner().ok("a {b: if(css(1) or css(2): c)}\n"),
                "a {\
         \n  b: if(css(1) or css(2): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn or_paren() {
            assert_eq!(
                runner().ok("a {b: if(css(1) or (css(2)): c)}\n"),
                "a {\
         \n  b: if(css(1) or (css(2)): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn paren_or() {
            assert_eq!(
                runner().ok("a {b: if((css(1)) or css(2): c)}\n"),
                "a {\
         \n  b: if((css(1)) or css(2): c);\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn t3() {
        assert_eq!(
            runner().ok("a {b: if(css(1) or css(2) or css(3): c)}\n"),
            "a {\
         \n  b: if(css(1) or css(2) or css(3): c);\
         \n}\n"
        );
    }
}
mod paren {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn and() {
        assert_eq!(
            runner().ok("a {b: if((css(1) and css(2)): c)}\n"),
            "a {\
         \n  b: if((css(1) and css(2)): c);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn clause() {
        assert_eq!(
            runner().ok("a {b: if((css()): c)}\n"),
            "a {\
         \n  b: if((css()): c);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn not() {
        assert_eq!(
            runner().ok("a {b: if((not css()): c)}\n"),
            "a {\
         \n  b: if((not css()): c);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn or() {
        assert_eq!(
            runner().ok("a {b: if((css(1) or css(2)): c)}\n"),
            "a {\
         \n  b: if((css(1) or css(2)): c);\
         \n}\n"
        );
    }
}
