//! Tests auto-converted from "sass-spec/spec/core_functions/math/asin.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("asin")
}

#[test]
fn decimal() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.asin(0.5)}\n"),
        "a {\
         \n  b: 30deg;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\" as math;\
             \na {b: math.asin(0, 0)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.asin(0, 0)}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function asin($number) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\" as math;\
             \na {b: math.asin(\"0\")}\n"
            ),
            "Error: $number: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.asin(\"0\")}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn units() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\" as math;\
             \na {b: math.asin(1px)}\n"
            ),
            "Error: $number: Expected 1px to have no units.\
         \n  ,\
         \n2 | a {b: math.asin(1px)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn zero_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\" as math;\
             \na {b: math.asin()}\n"
            ),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.asin()}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function asin($number) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn greater_than_one() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.asin(2)}\n"),
        "a {\
         \n  b: NaNdeg;\
         \n}\n"
    );
}
#[test]
fn less_than_negative_one() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.asin(-2)}\n"),
        "a {\
         \n  b: NaNdeg;\
         \n}\n"
    );
}
#[test]
fn negative_decimal() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.asin(-0.5)}\n"),
        "a {\
         \n  b: -30deg;\
         \n}\n"
    );
}
#[test]
fn negative_zero() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.asin(-0.0)}\n"),
        "a {\
         \n  b: 0deg;\
         \n}\n"
    );
}
#[test]
fn negative_zero_fuzzy() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.asin(-0.000000000001)}\n"),
        "a {\
         \n  b: 0deg;\
         \n}\n"
    );
}
#[test]
fn one() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.asin(1)}\n"),
        "a {\
         \n  b: 90deg;\
         \n}\n"
    );
}
#[test]
fn one_fuzzy() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.asin(1.000000000001)}\n"),
        "a {\
         \n  b: 90deg;\
         \n}\n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.asin(0)}\n"),
        "a {\
         \n  b: 0deg;\
         \n}\n"
    );
}
#[test]
fn zero_fuzzy() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.asin(0.000000000001)}\n"),
        "a {\
         \n  b: 0deg;\
         \n}\n"
    );
}
