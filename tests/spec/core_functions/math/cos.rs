//! Tests auto-converted from "sass-spec/spec/core_functions/math/cos.hrx"

#[test]
fn deg() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.cos(1deg)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.9998476952;\
        \n}\
        \n"
    );
}
mod error {
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.cos(0, 0)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.cos(0, 0)}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function cos($number) {\
         \n  |           ============ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.cos(\"0\")}\
             \n"
            )
            .unwrap_err(),
            "Error: $number: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.cos(\"0\")}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn unit() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.cos(1px)}\
             \n"
        ).unwrap_err(),
        "Error: $number: Expected 1px to have an angle unit (deg, grad, rad, turn).\
         \n  ,\
         \n2 | a {b: math.cos(1px)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
    }
    #[test]
    fn zero_args() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.cos()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.cos()}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function cos($number) {\
         \n  |           ============ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
}
#[test]
fn grad() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.cos(1grad)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.9998766325;\
        \n}\
        \n"
    );
}
#[test]
fn infinity() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.cos(1 / 0)}\
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
fn named_arg() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.cos($number: 1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.5403023059;\
        \n}\
        \n"
    );
}
#[test]
fn negative_infinity() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.cos(-1 / 0)}\
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
fn rad() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.cos(1rad)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.5403023059;\
        \n}\
        \n"
    );
}
#[test]
fn turn() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.cos(1turn)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn unitless() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.cos(1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.5403023059;\
        \n}\
        \n"
    );
}
