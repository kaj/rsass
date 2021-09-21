//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/no_operator.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod calculation {
    #[allow(unused)]
    use super::runner;

    mod calc {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn preserved() {
            assert_eq!(
                runner().ok("a {b: calc(calc(1px + 1%))}\n"),
                "a {\
         \n  b: calc(1px + 1%);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn simplified() {
            assert_eq!(
                runner().ok("a {b: calc(calc(1px))}\n"),
                "a {\
         \n  b: 1px;\
         \n}\n"
            );
        }
    }
    mod clamp {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn preserved() {
            assert_eq!(
                runner().ok("a {b: calc(clamp(1%, 2px, 3%))}\n"),
                "a {\
         \n  b: clamp(1%, 2px, 3%);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn simplified() {
            assert_eq!(
                runner().ok("a {b: calc(clamp(1px, 2px, 3px))}\n"),
                "a {\
         \n  b: 2px;\
         \n}\n"
            );
        }
    }
    mod max {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn preserved() {
            assert_eq!(
                runner().ok("a {b: calc(max(1%, 2px))}\n"),
                "a {\
         \n  b: max(1%, 2px);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn simplified() {
            assert_eq!(
                runner().ok("a {b: calc(max(1px, 2px))}\n"),
                "a {\
         \n  b: 2px;\
         \n}\n"
            );
        }
    }
    mod min {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn preserved() {
            assert_eq!(
                runner().ok("a {b: calc(min(1%, 2px))}\n"),
                "a {\
         \n  b: min(1%, 2px);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn simplified() {
            assert_eq!(
                runner().ok("a {b: calc(min(1px, 2px))}\n"),
                "a {\
         \n  b: 1px;\
         \n}\n"
            );
        }
    }
}
#[test]
#[ignore] // wrong result
fn case_insensitive() {
    assert_eq!(
        runner().ok("a {b: CaLc(1px)}\n"),
        "a {\
         \n  b: 1px;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn extra_whitespace() {
    assert_eq!(
        runner().ok("a {b: calc( 1px )}\n"),
        "a {\
         \n  b: 1px;\
         \n}\n"
    );
}
mod function {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn css() {
        assert_eq!(
            runner().ok("a {b: calc(c())}\n"),
            "a {\
         \n  b: calc(c());\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn test_if() {
        assert_eq!(
            runner().ok("b {c: calc(if(true, 1, $undefined))}\n"),
            "b {\
         \n  c: 1;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn max() {
        assert_eq!(
            runner().ok("b {c: calc(max(1 2 3...))}\n"),
            "b {\
         \n  c: 3;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn min() {
        assert_eq!(
            runner().ok("b {c: calc(min(1 2 3...))}\n"),
            "b {\
         \n  c: 1;\
         \n}\n"
        );
    }
    mod sass {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn global() {
            assert_eq!(
                runner().ok("@function a() {@return 1px}\n\
             \nb {c: calc(a())}\n"),
                "b {\
         \n  c: 1px;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn namespace() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \nb {c: calc(math.round(2.3))}\n"),
                "b {\
         \n  c: 2;\
         \n}\n"
            );
        }
    }
}
mod interpolation {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn line_noise() {
        assert_eq!(
        runner().ok(
            "// Interpolation shifts the parser into a special mode where it allows any\
             \n// interpolated declaration value.\
             \na {b: calc(!{@}#$%^&*#{c}_-[+]=)}\n"
        ),
        "a {\
         \n  b: calc(!{@}#$%^&*c_-[+]=);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn nested() {
        assert_eq!(
            runner().ok("a {b: calc(calc(#{c}))}\n"),
            "a {\
         \n  b: calc(c);\
         \n}\n"
        );
    }
    #[test]
    fn number() {
        assert_eq!(
        runner().ok(
            "// Although interpolation may return a value that looks like a valid `calc()`,\
             \n// it\'s always treated as an unquoted string.\
             \na {b: calc(#{1px + 2px})}\n"
        ),
        "a {\
         \n  b: calc(3px);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn parens() {
        assert_eq!(
        runner().ok(
            "// Interpolation is isolated to a single parenthesized context, so the\
             \n// parentheses themselves are stripped off.\
             \na {b: calc((#{1px + 2px}))}\n"
        ),
        "a {\
         \n  b: calc(3px);\
         \n}\n"
    );
    }
}
mod number {
    #[allow(unused)]
    use super::runner;

    mod decimal {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn leading_digit() {
            assert_eq!(
                runner().ok("a {b: calc(1.2px)}\n"),
                "a {\
         \n  b: 1.2px;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn leading_dot() {
            assert_eq!(
                runner().ok("a {b: calc(.2px)}\n"),
                "a {\
         \n  b: 0.2px;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn exponent() {
        assert_eq!(
            runner().ok("a {b: calc(1e2px)}\n"),
            "a {\
         \n  b: 100px;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn integer() {
        assert_eq!(
            runner().ok("a {b: calc(1px)}\n"),
            "a {\
         \n  b: 1px;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn leading_minus() {
        assert_eq!(
            runner().ok("a {b: calc(-1px)}\n"),
            "a {\
         \n  b: -1px;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn leading_plus() {
        assert_eq!(
            runner().ok("a {b: calc(+1px)}\n"),
            "a {\
         \n  b: 1px;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: calc(1)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn parentheses() {
    assert_eq!(
        runner().ok("a {b: calc((1px))}\n"),
        "a {\
         \n  b: 1px;\
         \n}\n"
    );
}
mod syntax {
    #[allow(unused)]
    use super::runner;

    mod extra_whitespace {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn number() {
            assert_eq!(
                runner().ok("a {b: calc( 1 )}\n"),
                "a {\
         \n  b: 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn parens() {
            assert_eq!(
                runner().ok("a {b: calc( ( 1 ) )}\n"),
                "a {\
         \n  b: 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn parenthesized_var() {
            assert_eq!(
                runner().ok("a {b: calc( ( var(--c) ) )}\n"),
                "a {\
         \n  b: calc((var(--c)));\
         \n}\n"
            );
        }
    }
}
mod var {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn bare() {
        assert_eq!(
            runner().ok("a {b: calc(var(--c))}\n"),
            "a {\
         \n  b: calc(var(--c));\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn double_parenthesized() {
        assert_eq!(
            runner().ok("a {b: calc(((var(--c))))}\n"),
            "a {\
         \n  b: calc((var(--c)));\
         \n}\n"
        );
    }
    #[test]
    fn parenthesized() {
        assert_eq!(
            runner().ok("a {b: calc((var(--c)))}\n"),
            "a {\
         \n  b: calc((var(--c)));\
         \n}\n"
        );
    }
}
mod variable {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn calculation() {
        assert_eq!(
            runner().ok("$a: calc(1px + 1%);\
             \nb {c: calc($a)}\n"),
            "b {\
         \n  c: calc(1px + 1%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn namespace() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: calc(math.$pi)}\n"),
            "a {\
         \n  b: 3.1415926536;\
         \n}\n"
        );
    }
    mod number {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn complex_unit() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \n$a: math.div(1px*1s, 1rad);\
             \nb {c: math.div(calc($a) * 2rad, 2s)}\n"),
                "b {\
         \n  c: 1px;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn simple_unit() {
            assert_eq!(
                runner().ok("$a: 1px;\
             \nb {c: calc($a)}\n"),
                "b {\
         \n  c: 1px;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn unquoted_string() {
        assert_eq!(
            runner().ok("$a: foobar;\
             \nb {c: calc($a)}\n"),
            "b {\
         \n  c: calc(foobar);\
         \n}\n"
        );
    }
}
