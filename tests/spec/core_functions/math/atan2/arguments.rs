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

    // Ignoring "incompatible_units", error tests are not supported yet.

    // Ignoring "one_arg", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "unitless_x", error tests are not supported yet.

    // Ignoring "unitless_y", error tests are not supported yet.

    // Ignoring "x_type", error tests are not supported yet.

    // Ignoring "y_type", error tests are not supported yet.

    // Ignoring "zero_args", error tests are not supported yet.
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
