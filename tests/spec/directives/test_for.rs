//! Tests auto-converted from "sass-spec/spec/directives/for.hrx"

#[test]
fn empty() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  @for $i from 1 to 1 {b: $i;}\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}
mod error {

    // Ignoring "from_float", error tests are not supported yet.

    // Ignoring "from_type", error tests are not supported yet.

    // Ignoring "incompatible_units", error tests are not supported yet.

    // Ignoring "to_float", error tests are not supported yet.

    // Ignoring "to_type", error tests are not supported yet.

    // Ignoring "unit_coersion_to_float", error tests are not supported yet.
}
#[test]
fn exclusive_backward() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  @for $i from 5 to 1 {b: $i;}\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 5;\
        \n  b: 4;\
        \n  b: 3;\
        \n  b: 2;\
        \n}\
        \n"
    );
}
#[test]
fn exclusive_forward() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  @for $i from 1 to 5 {b: $i;}\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n  b: 2;\
        \n  b: 3;\
        \n  b: 4;\
        \n}\
        \n"
    );
}
#[test]
fn inclusive_backward() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  @for $i from 5 through 1 {b: $i;}\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 5;\
        \n  b: 4;\
        \n  b: 3;\
        \n  b: 2;\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn inclusive_forward() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  @for $i from 1 through 5 {b: $i;}\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n  b: 2;\
        \n  b: 3;\
        \n  b: 4;\
        \n  b: 5;\
        \n}\
        \n"
    );
}
#[test]
fn negative_to_negative() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  @for $i from -5 through -1 {b: $i;}\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -5;\
        \n  b: -4;\
        \n  b: -3;\
        \n  b: -2;\
        \n  b: -1;\
        \n}\
        \n"
    );
}
#[test]
fn negative_to_positive() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  @for $i from -1 through 3 {b: $i;}\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -1;\
        \n  b: 0;\
        \n  b: 1;\
        \n  b: 2;\
        \n  b: 3;\
        \n}\
        \n"
    );
}
#[test]
fn positive_to_negative() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  @for $i from 2 through -1 {b: $i;}\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 2;\
        \n  b: 1;\
        \n  b: 0;\
        \n  b: -1;\
        \n}\
        \n"
    );
}
#[test]
fn to_scope() {
    assert_eq!(
        crate::rsass(
            "// Overriding the variable inside the loop will not impact the end of the loop.\
            \n$limit: 4;\
            \n\
            \n@for $i from 1 through $limit {\
            \n  $limit: 2;\
            \n  a {\
            \n    limit: $limit;\
            \n    i: $i;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  limit: 2;\
        \n  i: 1;\
        \n}\
        \na {\
        \n  limit: 2;\
        \n  i: 2;\
        \n}\
        \na {\
        \n  limit: 2;\
        \n  i: 3;\
        \n}\
        \na {\
        \n  limit: 2;\
        \n  i: 4;\
        \n}\
        \n"
    );
}
mod unit {
    #[test]
    fn compatible() {
        assert_eq!(
            crate::rsass(
                "a {\
            \n  @for $i from 5mm through 1cm {b: $i;}\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 5mm;\
        \n  b: 6mm;\
        \n  b: 7mm;\
        \n  b: 8mm;\
        \n  b: 9mm;\
        \n  b: 10mm;\
        \n}\
        \n"
        );
    }
    #[test]
    fn from_unitless() {
        assert_eq!(
            crate::rsass(
                "a {\
            \n  @for $i from 1 through 5px {b: $i;}\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n  b: 2;\
        \n  b: 3;\
        \n  b: 4;\
        \n  b: 5;\
        \n}\
        \n"
        );
    }
    #[test]
    fn same() {
        assert_eq!(
            crate::rsass(
                "a {\
            \n  @for $i from 1px through 5px {b: $i;}\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1px;\
        \n  b: 2px;\
        \n  b: 3px;\
        \n  b: 4px;\
        \n  b: 5px;\
        \n}\
        \n"
        );
    }
    #[test]
    fn to_unitless() {
        assert_eq!(
            crate::rsass(
                "a {\
            \n  @for $i from 1px through 5 {b: $i;}\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1px;\
        \n  b: 2px;\
        \n  b: 3px;\
        \n  b: 4px;\
        \n  b: 5px;\
        \n}\
        \n"
        );
    }
}
