//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/four_args/clamped.hrx"

mod alpha {
    #[test]
    fn above() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(0, 0, 0, 1.1)}\
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
    fn below() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(0, 0, 0, -0.1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 0, 0, 0);\
        \n}\
        \n"
        );
    }
}
#[test]
fn blue() {
    assert_eq!(
        crate::rsass(
            "a {b: rgb(0, 0, 9999, 0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(0, 0, 255, 0.5);\
        \n}\
        \n"
    );
}
#[test]
fn green() {
    assert_eq!(
        crate::rsass(
            "a {b: rgb(0, -1, 0, 0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(0, 0, 0, 0.5);\
        \n}\
        \n"
    );
}
#[test]
fn red() {
    assert_eq!(
        crate::rsass(
            "a {b: rgb(256, 0, 0, 0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(255, 0, 0, 0.5);\
        \n}\
        \n"
    );
}
