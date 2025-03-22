//! Tests auto-converted from "sass-spec/spec/values/numbers/degenerate.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("degenerate")
}

mod error {
    use super::runner;

    mod infinity {
        use super::runner;

        #[test]
        fn denominator_unit() {
            assert_eq!(
                runner().err(
                    "@use \'sass:math\';\
             \na {b: math.div(1, 0px)}\n"
                ),
                "Error: calc(infinity / 1px) isn\'t a valid CSS value.\
         \n  ,\
         \n2 | a {b: math.div(1, 0px)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn multiple_numerator_units() {
            assert_eq!(
                runner().err(
                    "@use \'sass:math\';\
             \na {b: math.div(1px * 1em, 0)}\n"
                ),
                "Error: calc(infinity * 1px * 1em) isn\'t a valid CSS value.\
         \n  ,\
         \n2 | a {b: math.div(1px * 1em, 0)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn numerator_and_denominator_unit() {
            assert_eq!(
                runner().err(
                    "@use \'sass:math\';\
             \na {b: math.div(1px, 0em)}\n"
                ),
                "Error: calc(infinity * 1px / 1em) isn\'t a valid CSS value.\
         \n  ,\
         \n2 | a {b: math.div(1px, 0em)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    mod minus_infinity {
        use super::runner;

        #[test]
        fn denominator_unit() {
            assert_eq!(
                runner().err(
                    "@use \'sass:math\';\
             \na {b: math.div(-1, 0px)}\n"
                ),
                "Error: calc(-infinity / 1px) isn\'t a valid CSS value.\
         \n  ,\
         \n2 | a {b: math.div(-1, 0px)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn multiple_numerator_units() {
            assert_eq!(
        runner().err(
            "@use \'sass:math\';\
             \na {b: math.div(-1px * 1em, 0)}\n"
        ),
        "Error: calc(-infinity * 1px * 1em) isn\'t a valid CSS value.\
         \n  ,\
         \n2 | a {b: math.div(-1px * 1em, 0)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        fn numerator_and_denominator_unit() {
            assert_eq!(
        runner().err(
            "@use \'sass:math\';\
             \na {b: math.div(-1px, 0em)}\n"
        ),
        "Error: calc(-infinity * 1px / 1em) isn\'t a valid CSS value.\
         \n  ,\
         \n2 | a {b: math.div(-1px, 0em)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
    }
    mod nan {
        use super::runner;

        #[test]
        fn denominator_unit() {
            assert_eq!(
                runner().err(
                    "@use \'sass:math\';\
             \na {b: math.div(0, 0px)}\n"
                ),
                "Error: calc(NaN / 1px) isn\'t a valid CSS value.\
         \n  ,\
         \n2 | a {b: math.div(0, 0px)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn multiple_numerator_units() {
            assert_eq!(
                runner().err(
                    "@use \'sass:math\';\
             \na {b: math.div(0px * 0em, 0)}\n"
                ),
                "Error: calc(NaN * 1px * 1em) isn\'t a valid CSS value.\
         \n  ,\
         \n2 | a {b: math.div(0px * 0em, 0)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn numerator_and_denominator_unit() {
            assert_eq!(
                runner().err(
                    "@use \'sass:math\';\
             \na {b: math.div(0px, 0em)}\n"
                ),
                "Error: calc(NaN * 1px / 1em) isn\'t a valid CSS value.\
         \n  ,\
         \n2 | a {b: math.div(0px, 0em)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
mod infinity {
    use super::runner;

    #[test]
    fn single_unit() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(1px, 0)}\n"),
            "a {\
         \n  b: calc(infinity * 1px);\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(1, 0)}\n"),
            "a {\
         \n  b: calc(infinity);\
         \n}\n"
        );
    }
}
mod minus_infinity {
    use super::runner;

    #[test]
    fn single_unit() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(-1px, 0)}\n"),
            "a {\
         \n  b: calc(-infinity * 1px);\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(-1, 0)}\n"),
            "a {\
         \n  b: calc(-infinity);\
         \n}\n"
        );
    }
}
mod nan {
    use super::runner;

    #[test]
    fn single_unit() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(0px, 0)}\n"),
            "a {\
         \n  b: calc(NaN * 1px);\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(0, 0)}\n"),
            "a {\
         \n  b: calc(NaN);\
         \n}\n"
        );
    }
}
