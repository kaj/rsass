//! Tests auto-converted from "sass-spec/spec/core_functions/math/hypot.hrx"

#[test]
fn compatible_units() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.hypot(3cm, 4mm * 10, 5q * 40, 6in / 2.54, 7px * 96 / 2.54)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 11.6189500386cm;\
        \n}\
        \n"
    );
}
mod error {
    mod incompatible_units {

        // Ignoring "all", error tests are not supported yet.

        // Ignoring "first_and_second", error tests are not supported yet.

        // Ignoring "first_and_third", error tests are not supported yet.

        // Ignoring "second_and_third", error tests are not supported yet.
    }
    mod some_unitless {

        // Ignoring "first", error tests are not supported yet.

        // Ignoring "first_and_second", error tests are not supported yet.

        // Ignoring "first_and_third", error tests are not supported yet.

        // Ignoring "second", error tests are not supported yet.

        // Ignoring "second_and_third", error tests are not supported yet.

        // Ignoring "third", error tests are not supported yet.
    }
    mod test_type {

        // Ignoring "first", error tests are not supported yet.

        // Ignoring "second", error tests are not supported yet.

        // Ignoring "third", error tests are not supported yet.
    }

    // Ignoring "zero_args", error tests are not supported yet.
}
mod infinity {
    #[test]
    fn first() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.hypot(1/0, 1, 1)}\
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
    fn second() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.hypot(1, 1/0, 1)}\
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
    fn third() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.hypot(1, 1, 1/0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: Infinity;\
        \n}\
        \n"
        );
    }
}
#[test]
fn unitless() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.hypot(3, 4, 5, 6, 7)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 11.6189500386;\
        \n}\
        \n"
    );
}
