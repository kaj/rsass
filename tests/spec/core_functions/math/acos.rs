//! Tests auto-converted from "sass-spec/spec/core_functions/math/acos.hrx"

#[test]
fn decimal() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.acos(0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 60deg;\
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
             \na {b: math.acos(0, 0)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.acos(0, 0)}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function acos($number) {\
         \n  |           ============= declaration\
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
             \na {b: math.acos(\"0\")}\
             \n"
            )
            .unwrap_err(),
            "Error: $number: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.acos(\"0\")}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn units() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.acos(1px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $number: Expected 1px to have no units.\
         \n  ,\
         \n2 | a {b: math.acos(1px)}\
         \n  |       ^^^^^^^^^^^^^^\
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
             \na {b: math.acos()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.acos()}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function acos($number) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
}
#[test]
fn greater_than_one() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.acos(2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: NaNdeg;\
        \n}\
        \n"
    );
}
#[test]
fn less_than_negative_one() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.acos(-2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: NaNdeg;\
        \n}\
        \n"
    );
}
#[test]
fn negative_decimal() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.acos(-0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 120deg;\
        \n}\
        \n"
    );
}
#[test]
fn one() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.acos(1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0deg;\
        \n}\
        \n"
    );
}
#[test]
fn one_fuzzy() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.acos(1.000000000001)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0deg;\
        \n}\
        \n"
    );
}
