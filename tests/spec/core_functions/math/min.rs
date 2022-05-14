//! Tests auto-converted from "sass-spec/spec/core_functions/math/min.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("min")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn incompatible_units() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.min(1px, 2s)}\n"
            ),
            "Error: 1px and 2s have incompatible units.\
         \n  ,\
         \n2 | a {b: math.min(1px, 2s)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.min()}\n"
            ),
            "Error: At least one argument must be passed.\
         \n  ,\
         \n2 | a {b: math.min()}\
         \n  |       ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\";\
             \na {b: math.min(c)}\n"
                ),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: math.min(c)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\";\
             \na {b: math.min(1, c)}\n"
                ),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: math.min(1, c)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\";\
             \na {b: math.min(1, 2, c)}\n"
                ),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: math.min(1, 2, c)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
mod global {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn modulo() {
        assert_eq!(
            runner().ok("a {b: min(1px, 7px % 4)}\n"),
            "a {\
         \n  b: 1px;\
         \n}\n"
        );
    }
    #[test]
    fn surrounding_whitespace() {
        assert_eq!(
        runner().ok(
            "// The extra whitespace doesn\'t cause this to be parsed as a Sass function, but\
             \n// we want to verify that it also doesn\'t interfere.\
             \nb {c: min( 1px, 2px, )}\n"
        ),
        "b {\
         \n  c: 1px;\
         \n}\n"
    );
    }
    #[test]
    fn trailing_comma() {
        assert_eq!(
            runner().ok("a {b: min(1px, 2px,)}\n"),
            "a {\
         \n  b: 1px;\
         \n}\n"
        );
    }
}
#[test]
fn one_arg() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.min(1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn three_args() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.min(3, 1, 2)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn two_args() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.min(1, 2)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
mod units {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn and_unitless() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.min(2px, 1)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
    #[test]
    fn compatible() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.min(1px, 1in, 1cm)}\n"),
            "a {\
         \n  b: 1px;\
         \n}\n"
        );
    }
    #[test]
    fn same() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.min(6px, 2px, 10px)}\n"),
            "a {\
         \n  b: 2px;\
         \n}\n"
        );
    }
}
