//! Tests auto-converted from "sass-spec/spec/core_functions/math/tan.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod asymptote {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn radian() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.tan(0.5rad * math.$pi)}\n"),
            "a {\
         \n  b: Infinity;\
         \n}\n"
        );
    }
}
#[test]
fn deg() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.tan(1deg)}\n"),
        "a {\
         \n  b: 0.0174550649;\
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
             \na {b: math.tan(0, 0)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.tan(0, 0)}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function tan($number) {\
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
             \na {b: math.tan(\"0\")}\n"
            ),
            "Error: $number: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.tan(\"0\")}\
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
             \na {b: math.tan(1px)}\n"
        ),
        "Error: $number: Expected 1px to have an angle unit (deg, grad, rad, turn).\
         \n  ,\
         \n2 | a {b: math.tan(1px)}\
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
             \na {b: math.tan()}\n"
            ),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.tan()}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function tan($number) {\
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
             \na {b: math.tan(1grad)}\n"),
        "a {\
         \n  b: 0.0157092553;\
         \n}\n"
    );
}
#[test]
fn infinity() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.tan(1 / 0)}\n"),
        "a {\
         \n  b: NaN;\
         \n}\n"
    );
}
#[test]
fn named_arg() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.tan($number: 1)}\n"),
        "a {\
         \n  b: 1.5574077247;\
         \n}\n"
    );
}
mod negative_asymptote {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn radian() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.tan(-0.5rad * math.$pi)}\n"),
            "a {\
         \n  b: -Infinity;\
         \n}\n"
        );
    }
}
#[test]
fn negative_infinity() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.tan(-1 / 0)}\n"),
        "a {\
         \n  b: NaN;\
         \n}\n"
    );
}
#[test]
fn negative_zero() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.tan(-0.0)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn negative_zero_fuzzy() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.tan(-0.000000000001)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn rad() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.tan(1rad)}\n"),
        "a {\
         \n  b: 1.5574077247;\
         \n}\n"
    );
}
#[test]
fn turn() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.tan(1turn)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn unitless() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.tan(1)}\n"),
        "a {\
         \n  b: 1.5574077247;\
         \n}\n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.tan(0)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn zero_fuzzy() {
    assert_eq!(
        runner().ok("@use \"sass:math\" as math;\
             \na {b: math.tan(0.000000000001)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
