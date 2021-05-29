//! Tests auto-converted from "sass-spec/spec/core_functions/math/sin.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn deg() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.sin(1deg)}\n"),
        "a {\
         \n  b: 0.0174524064;\
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
             \na {b: math.sin(0, 0)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.sin(0, 0)}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function sin($number) {\
         \n  |           ============ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\" as math;\
             \na {b: math.sin(\"0\")}\n"
            ),
            "Error: $number: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.sin(\"0\")}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn unit() {
        assert_eq!(
        runner().err(
            "@use \"sass:math\" as math;\
             \na {b: math.sin(1px)}\n"
        ),
        "Error: $number: Expected 1px to have an angle unit (deg, grad, rad, turn).\
         \n  ,\
         \n2 | a {b: math.sin(1px)}\
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
             \na {b: math.sin()}\n"
            ),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.sin()}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function sin($number) {\
         \n  |           ============ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn grad() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.sin(1grad)}\n"),
        "a {\
         \n  b: 0.0157073173;\
         \n}\n"
    );
}
#[test]
fn infinity() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.sin(math.div(1, 0))}\n"),
        "a {\
         \n  b: NaN;\
         \n}\n"
    );
}
#[test]
fn named_arg() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.sin($number: 1)}\n"),
        "a {\
         \n  b: 0.8414709848;\
         \n}\n"
    );
}
#[test]
fn negative_infinity() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.sin(math.div(-1, 0))}\n"),
        "a {\
         \n  b: NaN;\
         \n}\n"
    );
}
#[test]
fn negative_zero() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.sin(-0.0)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn negative_zero_fuzzy() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.sin(-0.000000000001)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn rad() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.sin(1rad)}\n"),
        "a {\
         \n  b: 0.8414709848;\
         \n}\n"
    );
}
#[test]
fn turn() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.sin(1turn)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn unitless() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.sin(1)}\n"),
        "a {\
         \n  b: 0.8414709848;\
         \n}\n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.sin(0)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn zero_fuzzy() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.sin(0.000000000001)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
