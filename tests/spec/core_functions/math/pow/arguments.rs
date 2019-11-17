//! Tests auto-converted from "sass-spec/spec/core_functions/math/pow/arguments.hrx"

mod error {
    #[test]
    fn base_has_units() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.pow(1px, 0)}\
             \n"
            )
            .unwrap_err(),
            "Error: $base: Expected 1px to have no units.\
         \n  ,\
         \n2 | a {b: math.pow(1px, 0)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn base_type() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.pow(\"0\", 0)}\
             \n"
            )
            .unwrap_err(),
            "Error: $base: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.pow(\"0\", 0)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn exponent_has_units() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.pow(0, 1px)}\
             \n"
            )
            .unwrap_err(),
            "Error: $exponent: Expected 1px to have no units.\
         \n  ,\
         \n2 | a {b: math.pow(0, 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn exponent_type() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.pow(0, \"0\")}\
             \n"
            )
            .unwrap_err(),
            "Error: $exponent: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.pow(0, \"0\")}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn one_arg() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.pow(0)}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $exponent.\
         \n  ,--> input.scss\
         \n2 | a {b: math.pow(0)}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function pow($base, $exponent) {\
         \n  |           ===================== declaration\
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
             \na {b: math.pow(0, 0, 0)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.pow(0, 0, 0)}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function pow($base, $exponent) {\
         \n  |           ===================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn zero_args() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.pow()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $base.\
         \n  ,--> input.scss\
         \n2 | a {b: math.pow()}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function pow($base, $exponent) {\
         \n  |           ===================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n\
         \n",
        );
    }
}
#[test]
fn named_args() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.pow($base: 2, $exponent: 3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 8;\
        \n}\
        \n"
    );
}
