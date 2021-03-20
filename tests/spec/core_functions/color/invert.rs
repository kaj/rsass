//! Tests auto-converted from "sass-spec/spec/core_functions/color/invert.hrx"

#[test]
fn alpha() {
    assert_eq!(
        crate::rsass(
            "a {b: invert(rgba(turquoise, 0.4))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(191, 31, 47, 0.4);\
        \n}\
        \n"
    );
}
#[test]
fn black() {
    assert_eq!(
        crate::rsass(
            "a {b: invert(black)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: white;\
        \n}\
        \n"
    );
}
mod error {
    mod bounds {

        // Ignoring "too_high", error tests are not supported yet.

        // Ignoring "too_low", error tests are not supported yet.
    }

    // Ignoring "number_with_weight", error tests are not supported yet.

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
    mod test_type {

        // Ignoring "color", error tests are not supported yet.

        // Ignoring "weight", error tests are not supported yet.
    }
}
#[test]
fn gray() {
    assert_eq!(
        crate::rsass(
            "a {b: invert(gray)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #7f7f7f;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: invert($color: turquoise, $weight: 0%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: turquoise;\
        \n}\
        \n"
    );
}
#[test]
fn number() {
    assert_eq!(
        crate::rsass(
            "a {b: invert(10%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: invert(10%);\
        \n}\
        \n"
    );
}
#[test]
fn turquoise() {
    assert_eq!(
        crate::rsass(
            "a {b: invert(turquoise)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #bf1f2f;\
        \n}\
        \n"
    );
}
mod weighted {
    #[test]
    fn high() {
        assert_eq!(
            crate::rsass(
                "a {b: invert(turquoise, 92%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #b52e3c;\
        \n}\
        \n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            crate::rsass(
                "a {b: invert(turquoise, 23%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #5db4ab;\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: invert(turquoise, 100%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #bf1f2f;\
        \n}\
        \n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            crate::rsass(
                "a {b: invert(turquoise, 50%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: gray;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            crate::rsass(
                "a {b: invert(turquoise, 0%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: turquoise;\
        \n}\
        \n"
        );
    }
}
#[test]
fn white() {
    assert_eq!(
        crate::rsass(
            "a {b: invert(white)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: black;\
        \n}\
        \n"
    );
}
