//! Tests auto-converted from "sass-spec/spec/values/numbers/modulo/zeros.hrx"

#[test]
fn negative_negative() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: -0 % -1;\
            \n}\
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
fn negative_positive() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: -0 % 1;\
            \n}\
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
fn positive_negative() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: +0 % -1;\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  b: 0;\
        \n}\
        \n"
    );
}
#[test]
fn positive_positive() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: +0 % +1;\
            \n}\
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
fn zero_divider() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: inspect(1 % 0);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: NaN;\
        \n}\
        \n"
    );
}
