//! Tests auto-converted from "sass-spec/spec/core_functions/math/clamp.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn chooses_max() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.clamp(0, 2, 1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn chooses_min() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.clamp(1, 0, 2)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn chooses_number() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.clamp(0, 1, 2)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    mod incompatible_units {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn all() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\" as math;\
             \na {b: math.clamp(1deg, 1px, 1s)}\n"
                ),
                "Error: $number: 1px and $min: 1deg have incompatible units.\
         \n  ,\
         \n2 | a {b: math.clamp(1deg, 1px, 1s)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn min_and_max() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\" as math;\
             \na {b: math.clamp(1deg, 1turn, 1px)}\n"
                ),
                "Error: $max: 1px and $min: 1deg have incompatible units.\
         \n  ,\
         \n2 | a {b: math.clamp(1deg, 1turn, 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn min_and_number() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\" as math;\
             \na {b: math.clamp(1deg, 1px, 1turn)}\n"
                ),
                "Error: $number: 1px and $min: 1deg have incompatible units.\
         \n  ,\
         \n2 | a {b: math.clamp(1deg, 1px, 1turn)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn number_and_max() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\" as math;\
             \na {b: math.clamp(1turn, 1deg, 1px)}\n"
                ),
                "Error: $max: 1px and $min: 1turn have incompatible units.\
         \n  ,\
         \n2 | a {b: math.clamp(1turn, 1deg, 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    #[test]
    fn one_arg() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\" as math;\
             \na {b: math.clamp(0)}\n"
            ),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.clamp(0)}\
         \n  |       ^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function clamp($min, $number, $max) {\
         \n  |           ========================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod some_unitless {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn max() {
            assert_eq!(
        runner().err(
            "@use \"sass:math\" as math;\
             \na {b: math.clamp(0px, 1px, 2)}\n"
        ),
        "Error: $max: 2 and $min: 0px have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.clamp(0px, 1px, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        fn min() {
            assert_eq!(
        runner().err(
            "@use \"sass:math\" as math;\
             \na {b: math.clamp(0, 1px, 2px)}\n"
        ),
        "Error: $number: 1px and $min: 0 have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.clamp(0, 1px, 2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        fn min_and_max() {
            assert_eq!(
        runner().err(
            "@use \"sass:math\" as math;\
             \na {b: math.clamp(0, 1px, 2)}\n"
        ),
        "Error: $number: 1px and $min: 0 have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.clamp(0, 1px, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        fn min_and_number() {
            assert_eq!(
        runner().err(
            "@use \"sass:math\" as math;\
             \na {b: math.clamp(0, 1, 2px)}\n"
        ),
        "Error: $max: 2px and $min: 0 have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.clamp(0, 1, 2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        fn number() {
            assert_eq!(
        runner().err(
            "@use \"sass:math\" as math;\
             \na {b: math.clamp(0px, 1, 2px)}\n"
        ),
        "Error: $number: 1 and $min: 0px have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.clamp(0px, 1, 2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
        #[test]
        fn number_and_max() {
            assert_eq!(
        runner().err(
            "@use \"sass:math\" as math;\
             \na {b: math.clamp(0px, 1, 2)}\n"
        ),
        "Error: $number: 1 and $min: 0px have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.clamp(0px, 1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
        }
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\" as math;\
             \na {b: math.clamp(0, 0, 0, 0)}\n"
            ),
            "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.clamp(0, 0, 0, 0)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function clamp($min, $number, $max) {\
         \n  |           ========================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn two_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\" as math;\
             \na {b: math.clamp(0, 0)}\n"
            ),
            "Error: Missing argument $max.\
         \n  ,--> input.scss\
         \n2 | a {b: math.clamp(0, 0)}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function clamp($min, $number, $max) {\
         \n  |           ========================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn max() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\" as math;\
             \na {b: math.clamp(1, 2, \"0\")}\n"
                ),
                "Error: $max: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.clamp(1, 2, \"0\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\" as math;\
             \na {b: math.clamp(\"0\", 1, 2)}\n"
                ),
                "Error: $min: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.clamp(\"0\", 1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn number() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\" as math;\
             \na {b: math.clamp(1, \"0\", 2)}\n"
                ),
                "Error: $number: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.clamp(1, \"0\", 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    #[test]
    fn zero_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\" as math;\
             \na {b: math.clamp()}\n"
            ),
            "Error: Missing argument $min.\
         \n  ,--> input.scss\
         \n2 | a {b: math.clamp()}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function clamp($min, $number, $max) {\
         \n  |           ========================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn min_equals_max() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {\
             \n  b: math.clamp(1, 2, 1);\
             \n}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn min_greater_than_max() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {\
             \n  b: math.clamp(1, 2, 0);\
             \n}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn named_args() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.clamp($min: 0, $number: 1, $max: 2)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
mod preserves_units {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn max() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.clamp(180deg, 1turn, 360deg)}\n"),
            "a {\
         \n  b: 360deg;\
         \n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.clamp(180deg, 0.5turn, 360deg)}\n"),
            "a {\
         \n  b: 180deg;\
         \n}\n"
        );
    }
    #[test]
    fn number() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.clamp(180deg, 0.75turn, 360deg)}\n"),
            "a {\
         \n  b: 0.75turn;\
         \n}\n"
        );
    }
}
