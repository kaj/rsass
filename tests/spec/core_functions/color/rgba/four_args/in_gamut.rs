//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/four_args/in_gamut.hrx"

#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: rgba($red: 0, $green: 255, $blue: 127, $alpha: 0.3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(0, 255, 127, 0.3);\
        \n}\
        \n"
    );
}
#[test]
fn opaque() {
    assert_eq!(
        crate::rsass(
            "a {b: rgba(190, 173, 237, 1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #beaded;\
        \n}\
        \n"
    );
}
#[test]
fn partial() {
    assert_eq!(
        crate::rsass(
            "a {b: rgba(18, 52, 86, 0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(18, 52, 86, 0.5);\
        \n}\
        \n"
    );
}
#[test]
fn transparent() {
    assert_eq!(
        crate::rsass(
            "a {b: rgba(0, 255, 127, 0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(0, 255, 127, 0);\
        \n}\
        \n"
    );
}
