//! Tests auto-converted from "sass-spec/spec/core_functions/math/comparable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("comparable")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.compatible(1)}\n"
            ),
            "Error: Missing argument $number2.\
         \n  ,--> input.scss\
         \n2 | a {b: math.compatible(1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function compatible($number1, $number2) {\
         \n  |           ============================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.compatible(1, 2, 3)}\n"
            ),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.compatible(1, 2, 3)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function compatible($number1, $number2) {\
         \n  |           ============================== declaration\
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
             \na {b: math.compatible(c, 1)}\n"
                ),
                "Error: $number1: c is not a number.\
         \n  ,\
         \n2 | a {b: math.compatible(c, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().err(
                    "@use \"sass:math\";\
             \na {b: math.compatible(1, c)}\n"
                ),
                "Error: $number2: c is not a number.\
         \n  ,\
         \n2 | a {b: math.compatible(1, c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
    #[test]
    fn wrong_name() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.comparable(1px, 1in)}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: math.comparable(1px, 1in)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.compatible($number1: 1, $number2: 2)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod unit {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn to_compatible() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.compatible(1px, 2in)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn to_different() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.compatible(1px, 2em)}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn to_inverse() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.compatible(1px, 1/1px)}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn to_same() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.compatible(1px, 2px)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
}
mod unitless {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn to_unit() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.compatible(1, 2px)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn to_unitless() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.compatible(1, 2)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
}
