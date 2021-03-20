//! Tests auto-converted from "sass-spec/spec/core_functions/color/ie_hex_str.hrx"

mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn leading_zero() {
    assert_eq!(
        crate::rsass(
            "a {b: ie-hex-str(rgba(#020304, 0.003))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #01020304;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: ie-hex-str($color: #daddee)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #FFDADDEE;\
        \n}\
        \n"
    );
}
#[test]
fn opaque() {
    assert_eq!(
        crate::rsass(
            "a {b: ie-hex-str(#daddee)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #FFDADDEE;\
        \n}\
        \n"
    );
}
#[test]
fn translucent() {
    assert_eq!(
        crate::rsass(
            "a {b: ie-hex-str(rgba(#daddee, 0.3))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #4DDADDEE;\
        \n}\
        \n"
    );
}
#[test]
fn transparent() {
    assert_eq!(
        crate::rsass(
            "a {b: ie-hex-str(rgba(turquoise, 0))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #0040E0D0;\
        \n}\
        \n"
    );
}
#[test]
fn test_type() {
    assert_eq!(
        crate::rsass(
            "a {b: type-of(ie-hex-str(#daddee))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: string;\
        \n}\
        \n"
    );
}
