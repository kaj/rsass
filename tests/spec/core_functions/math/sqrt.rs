//! Tests auto-converted from "sass-spec/spec/core_functions/math/sqrt.hrx"

mod error {
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.sqrt(0, 0)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.sqrt(0, 0)}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function sqrt($number) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.sqrt(\"0\")}\
             \n"
            )
            .unwrap_err(),
            "Error: $number: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.sqrt(\"0\")}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn units() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.sqrt(1px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $number: Expected 1px to have no units.\
         \n  ,\
         \n2 | a {b: math.sqrt(1px)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn zero_args() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.sqrt()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.sqrt()}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function sqrt($number) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn infinity() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.sqrt(1 / 0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: Infinity;\
        \n}\
        \n"
    );
}
#[test]
fn named_arg() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.sqrt($number: 2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1.4142135624;\
        \n}\
        \n"
    );
}
#[test]
fn negative() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.sqrt(-1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: NaN;\
        \n}\
        \n"
    );
}
#[test]
fn negative_zero() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.sqrt(-0.0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0;\
        \n}\
        \n"
    );
}
#[test]
fn negative_zero_fuzzy() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.sqrt(-0.000000000001)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0;\
        \n}\
        \n"
    );
}
#[test]
fn positive() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.sqrt(2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1.4142135624;\
        \n}\
        \n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.sqrt(0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0;\
        \n}\
        \n"
    );
}
#[test]
fn zero_fuzzy() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.sqrt(0.000000000001)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0;\
        \n}\
        \n"
    );
}
