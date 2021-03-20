//! Tests auto-converted from "sass-spec/spec/core_functions/math/clamp.hrx"

#[test]
fn chooses_max() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.clamp(0, 2, 1)}\
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
fn chooses_min() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.clamp(1, 0, 2)}\
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
fn chooses_number() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.clamp(0, 1, 2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
mod error {
    mod incompatible_units {

        // Ignoring "all", error tests are not supported yet.

        // Ignoring "min_and_max", error tests are not supported yet.

        // Ignoring "min_and_number", error tests are not supported yet.

        // Ignoring "number_and_max", error tests are not supported yet.
    }

    // Ignoring "one_arg", error tests are not supported yet.
    mod some_unitless {

        // Ignoring "max", error tests are not supported yet.

        // Ignoring "min", error tests are not supported yet.

        // Ignoring "min_and_max", error tests are not supported yet.

        // Ignoring "min_and_number", error tests are not supported yet.

        // Ignoring "number", error tests are not supported yet.

        // Ignoring "number_and_max", error tests are not supported yet.
    }

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "two_args", error tests are not supported yet.
    mod test_type {

        // Ignoring "max", error tests are not supported yet.

        // Ignoring "min", error tests are not supported yet.

        // Ignoring "number", error tests are not supported yet.
    }

    // Ignoring "zero_args", error tests are not supported yet.
}
#[test]
fn min_equals_max() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {\
            \n  b: math.clamp(1, 2, 1);\
            \n}\
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
fn min_greater_than_max() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {\
            \n  b: math.clamp(1, 2, 0);\
            \n}\
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
fn named_args() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.clamp($min: 0, $number: 1, $max: 2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
mod preserves_units {
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.clamp(180deg, 1turn, 360deg)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 360deg;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.clamp(180deg, 0.5turn, 360deg)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 180deg;\
        \n}\
        \n"
        );
    }
    #[test]
    fn number() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.clamp(180deg, 0.75turn, 360deg)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.75turn;\
        \n}\
        \n"
        );
    }
}
