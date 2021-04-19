//! Tests auto-converted from "sass-spec/spec/core_functions/math/sin.hrx"

#[test]
fn deg() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.sin(1deg)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.0174524064;\
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
             \na {b: math.sin(0, 0)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.sin(0, 0)}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function sin($number) {\
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
             \na {b: math.sin(\"0\")}\
             \n"
            )
            .unwrap_err(),
            "Error: $number: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.sin(\"0\")}\
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
             \na {b: math.sin(1px)}\
             \n"
        ).unwrap_err(),
        "Error: $number: Expected 1px to have an angle unit (deg, grad, rad, turn).\
         \n  ,\
         \n2 | a {b: math.sin(1px)}\
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
             \na {b: math.sin()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.sin()}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function sin($number) {\
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
            \na {b: math.sin(1grad)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.0157073173;\
        \n}\
        \n"
    );
}
#[test]
fn infinity() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.sin(1 / 0)}\
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
            \na {b: math.sin($number: 1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.8414709848;\
        \n}\
        \n"
    );
}
#[test]
fn negative_infinity() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.sin(-1 / 0)}\
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
            \na {b: math.sin(-0.0)}\
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
            \na {b: math.sin(-0.000000000001)}\
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
fn rad() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.sin(1rad)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.8414709848;\
        \n}\
        \n"
    );
}
#[test]
fn turn() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.sin(1turn)}\
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
fn unitless() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.sin(1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.8414709848;\
        \n}\
        \n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.sin(0)}\
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
            \na {b: math.sin(0.000000000001)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0;\
        \n}\
        \n"
    );
}
