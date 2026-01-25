//! Tests auto-converted from "sass-spec/spec/expressions/if/syntax.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("syntax")
}

mod case {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn and() {
        assert_eq!(
            runner().ok("a {b: if(sass(true) AND sass(true): c)}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn not() {
        assert_eq!(
            runner().ok("a {b: if(NOT sass(false): c)}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn or() {
        assert_eq!(
            runner().ok("a {b: if(sass(true) OR sass(false): c)}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn trailing_semi() {
    assert_eq!(
        runner().ok("a {b: if(else: c;)}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
mod whitespace {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn after_open_paren() {
        assert_eq!(
            runner().ok("a {b: if( else: c)}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn after_trailing_semi() {
        assert_eq!(
            runner().ok("a {b: if(css(): c; else: d; )}\n"),
            "a {\
         \n  b: if(css(): c; else: d);\
         \n}\n"
        );
    }
    mod and {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn none_before() {
            assert_eq!(
                runner().ok("a {b: if(css(1)and css(2): c)}\n"),
                "a {\
         \n  b: if(css(1) and css(2): c);\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn before_close_paren() {
        assert_eq!(
            runner().ok("a {b: if(else: c )}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn before_colon() {
        assert_eq!(
            runner().ok("a {b: if(else : c)}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn before_semi() {
        assert_eq!(
            runner().ok("a {b: if(css(): c ; else: d)}\n"),
            "a {\
         \n  b: if(css(): c; else: d);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn before_trailing_semi() {
        assert_eq!(
            runner().ok("a {b: if(css(): c; else: d ;)}\n"),
            "a {\
         \n  b: if(css(): c; else: d);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn none_after_colon() {
        assert_eq!(
            runner().ok("a {b: if(else:c)}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn none_after_semi() {
        assert_eq!(
            runner().ok("a {b: if(css(): c;else: d)}\n"),
            "a {\
         \n  b: if(css(): c; else: d);\
         \n}\n"
        );
    }
    mod or {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn none_before() {
            assert_eq!(
                runner().ok("a {b: if(css(1)or css(2): c)}\n"),
                "a {\
         \n  b: if(css(1) or css(2): c);\
         \n}\n"
            );
        }
    }
    mod parens {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn after_open() {
            assert_eq!(
                runner().ok("a {b: if(( css()): c)}\n"),
                "a {\
         \n  b: if((css()): c);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn before_close() {
            assert_eq!(
                runner().ok("a {b: if((css() ): c)}\n"),
                "a {\
         \n  b: if((css()): c);\
         \n}\n"
            );
        }
    }
}
