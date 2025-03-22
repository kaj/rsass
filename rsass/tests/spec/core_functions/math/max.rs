//! Tests auto-converted from "sass-spec/spec/core_functions/math/max.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("max")
}

mod error {
    use super::runner;

    #[test]
    fn incompatible_units() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.max(1px, 2s)}\n"
            ),
            "Error: 1px and 2s have incompatible units.\
         \n  ,\
         \n2 | a {b: math.max(1px, 2s)}\
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
             \na {b: math.max()}\n"
            ),
            "Error: At least one argument must be passed.\
         \n  ,\
         \n2 | a {b: math.max()}\
         \n  |       ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod test_type {
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\";\
             \na {b: math.max(c)}\n"
                ),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: math.max(c)}\
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
             \na {b: math.max(1, c)}\n"
                ),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: math.max(1, c)}\
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
             \na {b: math.max(1, 2, c)}\n"
                ),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: math.max(1, 2, c)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
mod global {
    use super::runner;

    #[test]
    fn modulo() {
        assert_eq!(
            runner().ok("a {b: max(1px, 7px % 4)}\n"),
            "a {\
         \n  b: 3px;\
         \n}\n"
        );
    }
    #[test]
    fn surrounding_whitespace() {
        assert_eq!(
        runner().ok(
            "// The extra whitespace doesn\'t cause this to be parsed as a Sass function, but\
             \n// we want to verify that it also doesn\'t interfere.\
             \nb {c: max( 1px, 2px, )}\n"
        ),
        "b {\
         \n  c: 2px;\
         \n}\n"
    );
    }
    #[test]
    fn trailing_comma() {
        assert_eq!(
            runner().ok("a {b: max(1px, 2px,)}\n"),
            "a {\
         \n  b: 2px;\
         \n}\n"
        );
    }
}
#[test]
fn one_arg() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.max(1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn three_args() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.max(3, 1, 2)}\n"),
        "a {\
         \n  b: 3;\
         \n}\n"
    );
}
#[test]
fn two_args() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.max(1, 2)}\n"),
        "a {\
         \n  b: 2;\
         \n}\n"
    );
}
mod units {
    use super::runner;

    #[test]
    fn and_unitless() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.max(2px, 1)}\n"),
            "a {\
         \n  b: 2px;\
         \n}\n"
        );
    }
    #[test]
    fn compatible() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.max(1px, 1in, 1cm)}\n"),
            "a {\
         \n  b: 1in;\
         \n}\n"
        );
    }
    #[test]
    fn same() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.max(6px, 2px, 10px)}\n"),
            "a {\
         \n  b: 10px;\
         \n}\n"
        );
    }
}
