//! Tests auto-converted from "sass-spec/spec/core_functions/color/complement.hrx"

#[test]
fn alpha() {
    assert_eq!(
        crate::rsass(
            "a {b: complement(rgba(turquoise, 0.7))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(224, 64, 80, 0.7);\
        \n}\
        \n"
    );
}
mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
mod grayscale {
    #[test]
    fn black() {
        assert_eq!(
            crate::rsass(
                "a {b: complement(black)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: black;\
        \n}\
        \n"
        );
    }
    #[test]
    fn gray() {
        assert_eq!(
            crate::rsass(
                "a {b: complement(gray)}\
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
    fn white() {
        assert_eq!(
            crate::rsass(
                "a {b: complement(white)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: white;\
        \n}\
        \n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: complement($color: red)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: aqua;\
        \n}\
        \n"
    );
}
#[test]
fn red() {
    assert_eq!(
        crate::rsass(
            "a {b: complement(red)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: aqua;\
        \n}\
        \n"
    );
}
#[test]
fn turquoise() {
    assert_eq!(
        crate::rsass(
            "a {b: complement(turquoise)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #e04050;\
        \n}\
        \n"
    );
}
