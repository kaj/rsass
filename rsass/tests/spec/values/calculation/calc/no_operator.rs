//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/no_operator.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_operator")
}

mod calculation {
    #[allow(unused)]
    use super::runner;

    mod calc {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn preserved() {
            assert_eq!(
                runner().ok("a {b: calc(calc(1px + 1%))}\n"),
                "a {\
         \n  b: calc(1px + 1%);\
         \n}\n"
            );
        }
        #[test]
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
        fn preserved() {
            assert_eq!(
                runner().ok("a {b: calc(clamp(1%, 2px, 3%))}\n"),
                "a {\
         \n  b: clamp(1%, 2px, 3%);\
         \n}\n"
            );
        }
        #[test]
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
        fn preserved() {
            assert_eq!(
                runner().ok("a {b: calc(max(1%, 2px))}\n"),
                "a {\
         \n  b: max(1%, 2px);\
         \n}\n"
            );
        }
        #[test]
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
        fn preserved() {
            assert_eq!(
                runner().ok("a {b: calc(min(1%, 2px))}\n"),
                "a {\
         \n  b: min(1%, 2px);\
         \n}\n"
            );
        }
        #[test]
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
fn case_insensitive() {
    assert_eq!(
        runner().ok("a {b: CaLc(1px)}\n"),
        "a {\
         \n  b: 1px;\
         \n}\n"
    );
}
#[test]
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
    fn test_if() {
        assert_eq!(
            runner().ok("b {c: calc(if(true, 1, $undefined))}\n"),
            "b {\
         \n  c: 1;\
         \n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            runner().ok("b {c: calc(max(1 2 3...))}\n"),
            "b {\
         \n  c: 3;\
         \n}\n"
        );
    }
    #[test]
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
    fn parens() {
        assert_eq!(
        runner().ok(
            "// Interpolation is isolated to a single parenthesized context, so the\
             \n// parentheses themselves are stripped off.\
             \na {b: calc((#{1px + 2px}))}\n"
        ),
        "a {\
         \n  b: calc((3px));\
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
        fn leading_digit() {
            assert_eq!(
                runner().ok("a {b: calc(1.2px)}\n"),
                "a {\
         \n  b: 1.2px;\
         \n}\n"
            );
        }
        #[test]
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
    fn exponent() {
        assert_eq!(
            runner().ok("a {b: calc(1e2px)}\n"),
            "a {\
         \n  b: 100px;\
         \n}\n"
        );
    }
    #[test]
    fn integer() {
        assert_eq!(
            runner().ok("a {b: calc(1px)}\n"),
            "a {\
         \n  b: 1px;\
         \n}\n"
        );
    }
    #[test]
    fn leading_minus() {
        assert_eq!(
            runner().ok("a {b: calc(-1px)}\n"),
            "a {\
         \n  b: -1px;\
         \n}\n"
        );
    }
    #[test]
    fn leading_plus() {
        assert_eq!(
            runner().ok("a {b: calc(+1px)}\n"),
            "a {\
         \n  b: 1px;\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: calc(1)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
}
mod syntax {
    #[allow(unused)]
    use super::runner;

    mod extra_whitespace {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn number() {
            assert_eq!(
                runner().ok("a {b: calc( 1 )}\n"),
                "a {\
         \n  b: 1;\
         \n}\n"
            );
        }
        #[test]
        fn parens() {
            assert_eq!(
                runner().ok("a {b: calc( ( 1 ) )}\n"),
                "a {\
         \n  b: 1;\
         \n}\n"
            );
        }
        #[test]
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
}
mod variable {
    #[allow(unused)]
    use super::runner;

    #[test]
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
    fn namespace() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: calc(math.$pi)}\n"),
            "a {\
         \n  b: 3.1415926536;\
         \n}\n"
        );
    }
    mod not_parsed_as_interpolation {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn followed_by_parenthesized_interp() {
            assert_eq!(
                runner().ok("$a: 1;\
             \nb {\
             \n  c: calc($a);\
             \n  $_: (\"#{\'\'}\");\
             \n}\n"),
                "b {\
         \n  c: 1;\
         \n}\n"
            );
        }
        #[test]
        fn in_comment() {
            assert_eq!(
        runner().ok(
            "$a: 1;\
             \nb {\
             \n  // A naive parser might check for interpolation in a comment.\
             \n  c: calc($a /* #{\'\'} */);\
             \n}\n"
        ),
        "b {\
         \n  c: 1;\
         \n}\n"
    );
        }
        #[test]
        fn parentheses_in_string() {
            assert_eq!(
        runner().ok(
            "@function a($arg) {@return 1}\n\
             \n$b: 2;\
             \nc {\
             \n  // A naive parser might check for closing parentheses regardless of string\
             \n  // context when looking for interpolation in a calc.\
             \n  d: calc($b + a(\")#{\'\'}\"));\
             \n}\n"
        ),
        "c {\
         \n  d: 3;\
         \n}\n"
    );
        }
    }
    mod number {
        #[allow(unused)]
        use super::runner;

        #[test]
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
