//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/four_args/clamped.hrx"

mod alpha {
    #[test]
    fn above() {
        assert_eq!(
            crate::rsass(
                "a {b: hsl(0, 100%, 50%, 1.1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: red;\
        \n}\
        \n"
        );
    }
    #[test]
    fn below() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(0, 100%, 50%, -0.1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 255, 128, 0);\
        \n}\
        \n"
        );
    }
}
#[test]
fn blue() {
    assert_eq!(
        crate::rsass(
            "a {b: hsl(0, 100%, 9999%, 0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(255, 255, 255, 0.5);\
        \n}\
        \n"
    );
}
#[test]
fn saturation() {
    assert_eq!(
        crate::rsass(
            "a {b: hsl(0, -0.1%, 50%, 0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(128, 128, 128, 0.5);\
        \n}\
        \n"
    );
}
