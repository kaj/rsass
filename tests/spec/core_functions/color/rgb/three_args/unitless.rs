//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/three_args/unitless.hrx"

#[test]
fn beaded() {
    assert_eq!(
        crate::rsass(
            "a {b: rgb(190, 173, 237)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #beaded;\
        \n}\
        \n"
    );
}
mod clamped {
    #[test]
    fn blue() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(0, 0, 9999)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: blue;\
        \n}\
        \n"
        );
    }
    #[test]
    fn green() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(0, -1, 0)}\
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
    fn red() {
        assert_eq!(
            crate::rsass(
                "a {b: rgb(256, 0, 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: red;\
        \n}\
        \n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: rgb($red: 0, $green: 255, $blue: 127)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: springgreen;\
        \n}\
        \n"
    );
}
#[test]
fn numbers() {
    assert_eq!(
        crate::rsass(
            "a {b: rgb(18, 52, 86)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #123456;\
        \n}\
        \n"
    );
}
#[test]
fn springgreen() {
    assert_eq!(
        crate::rsass(
            "a {b: rgb(0, 255, 127)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: springgreen;\
        \n}\
        \n"
    );
}
