//! Tests auto-converted from "sass-spec/spec/core_functions/math/hypot.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hypot")
}

#[test]
fn compatible_units() {
    assert_eq!(
        runner().ok(
            "@use \"sass:math\";\
             \na {b: math.hypot(3cm, 4mm * 10, 5q * 40, math.div(6in, 2.54), 7px * math.div(96, 2.54))}\n"
        ),
        "a {\
         \n  b: 11.6189500386cm;\
         \n}\n"
    );
}
mod error {
    use super::runner;

    mod incompatible_units {
        use super::runner;

        #[test]
        fn all() {
            assert_eq!(
        runner().err(
            "@use \"sass:math\";\
             \na {b: math.hypot(1turn, 1px, 1s)}\n"
        ),
        "Error: $numbers[2]: 1px and $numbers[1]: 1turn have incompatible units.\
         \n  ,\
         \n2 | a {b: math.hypot(1turn, 1px, 1s)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        fn first_and_second() {
            assert_eq!(
        runner().err(
            "@use \"sass:math\";\
             \na {b: math.hypot(1deg, 1px, 1turn)}\n"
        ),
        "Error: $numbers[2]: 1px and $numbers[1]: 1deg have incompatible units.\
         \n  ,\
         \n2 | a {b: math.hypot(1deg, 1px, 1turn)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        fn first_and_third() {
            assert_eq!(
        runner().err(
            "@use \"sass:math\";\
             \na {b: math.hypot(1deg, 1turn, 1px)}\n"
        ),
        "Error: $numbers[3]: 1px and $numbers[1]: 1deg have incompatible units.\
         \n  ,\
         \n2 | a {b: math.hypot(1deg, 1turn, 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        fn second_and_third() {
            assert_eq!(
        runner().err(
            "@use \"sass:math\";\
             \na {b: math.hypot(1turn, 1deg, 1px)}\n"
        ),
        "Error: $numbers[3]: 1px and $numbers[1]: 1turn have incompatible units.\
         \n  ,\
         \n2 | a {b: math.hypot(1turn, 1deg, 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
    }
    mod some_unitless {
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
        runner().err(
            "@use \"sass:math\";\
             \na {b: math.hypot(0, 1px, 2px)}\n"
        ),
        "Error: $numbers[2]: 1px and $numbers[1]: 0 have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.hypot(0, 1px, 2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        fn first_and_second() {
            assert_eq!(
        runner().err(
            "@use \"sass:math\";\
             \na {b: math.hypot(0, 1, 2px)}\n"
        ),
        "Error: $numbers[3]: 2px and $numbers[1]: 0 have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.hypot(0, 1, 2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        fn first_and_third() {
            assert_eq!(
        runner().err(
            "@use \"sass:math\";\
             \na {b: math.hypot(0, 1px, 2)}\n"
        ),
        "Error: $numbers[2]: 1px and $numbers[1]: 0 have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.hypot(0, 1px, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        fn second() {
            assert_eq!(
        runner().err(
            "@use \"sass:math\";\
             \na {b: math.hypot(0px, 1, 2px)}\n"
        ),
        "Error: $numbers[2]: 1 and $numbers[1]: 0px have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.hypot(0px, 1, 2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        fn second_and_third() {
            assert_eq!(
        runner().err(
            "@use \"sass:math\";\
             \na {b: math.hypot(0px, 1, 2)}\n"
        ),
        "Error: $numbers[2]: 1 and $numbers[1]: 0px have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.hypot(0px, 1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        fn third() {
            assert_eq!(
        runner().err(
            "@use \"sass:math\";\
             \na {b: math.hypot(0px, 1px, 2)}\n"
        ),
        "Error: $numbers[3]: 2 and $numbers[1]: 0px have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.hypot(0px, 1px, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
    }
    mod test_type {
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\";\
             \na {b: math.hypot(\"0\", 1px, 1px)}\n"
                ),
                "Error: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.hypot(\"0\", 1px, 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn second() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\";\
             \na {b: math.hypot(1px, \"0\", 1px)}\n"
                ),
                "Error: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.hypot(1px, \"0\", 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn third() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\";\
             \na {b: math.hypot(1px, 1px, \"0\")}\n"
                ),
                "Error: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.hypot(1px, 1px, \"0\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    #[test]
    fn zero_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.hypot()}\n"
            ),
            "Error: At least one argument must be passed.\
         \n  ,\
         \n2 | a {b: math.hypot()}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
mod infinity {
    use super::runner;

    #[test]
    fn first() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.hypot(math.div(1, 0), 1, 1)}\n"),
            "a {\
         \n  b: calc(infinity);\
         \n}\n"
        );
    }
    #[test]
    fn second() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.hypot(1, math.div(1, 0), 1)}\n"),
            "a {\
         \n  b: calc(infinity);\
         \n}\n"
        );
    }
    #[test]
    fn third() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.hypot(1, 1, math.div(1, 0))}\n"),
            "a {\
         \n  b: calc(infinity);\
         \n}\n"
        );
    }
}
#[test]
fn unitless() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.hypot(3, 4, 5, 6, 7)}\n"),
        "a {\
         \n  b: 11.6189500386;\
         \n}\n"
    );
}
