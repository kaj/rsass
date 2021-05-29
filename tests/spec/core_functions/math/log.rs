//! Tests auto-converted from "sass-spec/spec/core_functions/math/log.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod base {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn between_zero_and_one() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.log(2, 0.5)}\n"),
            "a {\
         \n  b: -1;\
         \n}\n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.log(2, -1)}\n"),
            "a {\
         \n  b: NaN;\
         \n}\n"
        );
    }
    #[test]
    fn null() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.log(2, null)}\n"),
            "a {\
         \n  b: 0.6931471806;\
         \n}\n"
        );
    }
    #[test]
    fn one() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.log(2, 1)}\n"),
            "a {\
         \n  b: Infinity;\
         \n}\n"
        );
    }
    #[test]
    fn one_fuzzy() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.log(2, 1.000000000001)}\n"),
            "a {\
         \n  b: Infinity;\
         \n}\n"
        );
    }
    #[test]
    fn positive() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.log(2, 10)}\n"),
            "a {\
         \n  b: 0.3010299957;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.log(2, 0)}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
    #[test]
    fn zero_fuzzy() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.log(2, 0.000000000001)}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn base_has_units() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\" as math;\
             \na {b: math.log(1, 1px)}\n"
            ),
            "Error: $base: Expected 1px to have no units.\
         \n  ,\
         \n2 | a {b: math.log(1, 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn number_has_units() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\" as math;\
             \na {b: math.log(1px)}\n"
            ),
            "Error: $number: Expected 1px to have no units.\
         \n  ,\
         \n2 | a {b: math.log(1px)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\" as math;\
             \na {b: math.log(0, 0, 0)}\n"
            ),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.log(0, 0, 0)}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function log($number, $base: null) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\" as math;\
             \na {b: math.log(\"0\")}\n"
            ),
            "Error: $number: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.log(\"0\")}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn zero_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\" as math;\
             \na {b: math.log()}\n"
            ),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.log()}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function log($number, $base: null) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn infinity() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.log(math.div(1, 0))}\n"),
        "a {\
         \n  b: Infinity;\
         \n}\n"
    );
}
mod named_arg {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn number() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.log($number: 2)}\n"),
            "a {\
         \n  b: 0.6931471806;\
         \n}\n"
        );
    }
}
mod named_args {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn number_with_base() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.log($number: 2, $base: 10)}\n"),
            "a {\
         \n  b: 0.3010299957;\
         \n}\n"
        );
    }
}
#[test]
fn negative() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.log(-1)}\n"),
        "a {\
         \n  b: NaN;\
         \n}\n"
    );
}
#[test]
fn positive() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.log(2)}\n"),
        "a {\
         \n  b: 0.6931471806;\
         \n}\n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.log(0)}\n"),
        "a {\
         \n  b: -Infinity;\
         \n}\n"
    );
}
#[test]
fn zero_fuzzy() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.log(0.000000000001)}\n"),
        "a {\
         \n  b: -Infinity;\
         \n}\n"
    );
}
