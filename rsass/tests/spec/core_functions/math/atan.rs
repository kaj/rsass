//! Tests auto-converted from "sass-spec/spec/core_functions/math/atan.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("atan")
}

mod error {
    use super::runner;

    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.atan(0, 0)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.atan(0, 0)}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function atan($number) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.atan(\"0\")}\n"
            ),
            "Error: $number: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.atan(\"0\")}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn units() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.atan(1px)}\n"
            ),
            "Error: $number: Expected 1px to have no units.\
         \n  ,\
         \n2 | a {b: math.atan(1px)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn zero_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.atan()}\n"
            ),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.atan()}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function atan($number) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn infinity() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.atan(math.div(1, 0))}\n"),
        "a {\
         \n  b: 90deg;\
         \n}\n"
    );
}
#[test]
fn negative() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.atan(-1)}\n"),
        "a {\
         \n  b: -45deg;\
         \n}\n"
    );
}
#[test]
fn negative_infinity() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.atan(math.div(-1, 0))}\n"),
        "a {\
         \n  b: -90deg;\
         \n}\n"
    );
}
#[test]
fn negative_zero() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.atan(-0.0)}\n"),
        "a {\
         \n  b: 0deg;\
         \n}\n"
    );
}
#[test]
fn negative_zero_fuzzy() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.atan(-0.000000000001)}\n"),
        "a {\
         \n  b: -0.0000000001deg;\
         \n}\n"
    );
}
#[test]
fn positive() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.atan(1)}\n"),
        "a {\
         \n  b: 45deg;\
         \n}\n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.atan(0)}\n"),
        "a {\
         \n  b: 0deg;\
         \n}\n"
    );
}
#[test]
fn zero_fuzzy() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.atan(0.000000000001)}\n"),
        "a {\
         \n  b: 0.0000000001deg;\
         \n}\n"
    );
}
