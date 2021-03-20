//! Tests auto-converted from "sass-spec/spec/core_functions/math/pow/arguments.hrx"

mod error {

    // Ignoring "base_has_units", error tests are not supported yet.

    // Ignoring "base_type", error tests are not supported yet.

    // Ignoring "exponent_has_units", error tests are not supported yet.

    // Ignoring "exponent_type", error tests are not supported yet.

    // Ignoring "one_arg", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "zero_args", error tests are not supported yet.
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
