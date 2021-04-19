//! Tests auto-converted from "sass-spec/spec/core_functions/math/atan2/arguments.hrx"

#[test]
fn compatible_units() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.atan2(1cm, -10mm)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 135deg;\
        \n}\
        \n"
    );
}
mod error {
    #[test]
    fn incompatible_units() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.atan2(1px, 1deg)}\
             \n"
            )
            .unwrap_err(),
            "Error: $x: 1deg and $y: 1px have incompatible units.\
         \n  ,\
         \n2 | a {b: math.atan2(1px, 1deg)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn one_arg() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.atan2(0)}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $x.\
         \n  ,--> input.scss\
         \n2 | a {b: math.atan2(0)}\
         \n  |       ^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function atan2($y, $x) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.atan2(0, 0, 0)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.atan2(0, 0, 0)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function atan2($y, $x) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn unitless_x() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.atan2(1px, 1)}\
             \n"
        ).unwrap_err(),
        "Error: $x: 1 and $y: 1px have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.atan2(1px, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    fn unitless_y() {
        assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.atan2(1, 1px)}\
             \n"
        ).unwrap_err(),
        "Error: $x: 1px and $y: 1 have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.atan2(1, 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    fn x_type() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.atan2(0, \"0\")}\
             \n"
            )
            .unwrap_err(),
            "Error: $x: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.atan2(0, \"0\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn y_type() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.atan2(\"0\", 0)}\
             \n"
            )
            .unwrap_err(),
            "Error: $y: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.atan2(\"0\", 0)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn zero_args() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.atan2()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $y.\
         \n  ,--> input.scss\
         \n2 | a {b: math.atan2()}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function atan2($y, $x) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn named_args() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.atan2($y: 1, $x: -1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 135deg;\
        \n}\
        \n"
    );
}
