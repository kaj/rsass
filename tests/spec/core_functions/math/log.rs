//! Tests auto-converted from "sass-spec/spec/core_functions/math/log.hrx"

mod base {
    #[test]
    fn between_zero_and_one() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(2, 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: -1;\
        \n}\
        \n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(2, -1)}\
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
    #[ignore] // wrong result
    fn null() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(2, null)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.6931471806;\
        \n}\
        \n"
        );
    }
    #[test]
    fn one() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(2, 1)}\
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
    fn one_fuzzy() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(2, 1.000000000001)}\
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
    #[ignore] // wrong result
    fn positive() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(2, 10)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.3010299957;\
        \n}\
        \n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(2, 0)}\
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
            \na {b: math.log(2, 0.000000000001)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0;\
        \n}\
        \n"
        );
    }
}
mod error {
    #[test]
    fn base_has_units() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.log(1, 1px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $base: Expected 1px to have no units.\
         \n  ,\
         \n2 | a {b: math.log(1, 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn number_has_units() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.log(1px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $number: Expected 1px to have no units.\
         \n  ,\
         \n2 | a {b: math.log(1px)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.log(0, 0, 0)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.log(0, 0, 0)}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function log($number, $base: null) {\
         \n  |           ========================= declaration\
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
             \na {b: math.log(\"0\")}\
             \n"
            )
            .unwrap_err(),
            "Error: $number: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.log(\"0\")}\
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
             \na {b: math.log()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.log()}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function log($number, $base: null) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
}
#[test]
fn infinity() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.log(1 / 0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: Infinity;\
        \n}\
        \n"
    );
}
mod named_arg {
    #[test]
    #[ignore] // wrong result
    fn number() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log($number: 2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.6931471806;\
        \n}\
        \n"
        );
    }
}
mod named_args {
    #[test]
    #[ignore] // wrong result
    fn number_with_base() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log($number: 2, $base: 10)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.3010299957;\
        \n}\
        \n"
        );
    }
}
#[test]
fn negative() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.log(-1)}\
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
#[ignore] // wrong result
fn positive() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.log(2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.6931471806;\
        \n}\
        \n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.log(0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -Infinity;\
        \n}\
        \n"
    );
}
#[test]
fn zero_fuzzy() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.log(0.000000000001)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -Infinity;\
        \n}\
        \n"
    );
}
