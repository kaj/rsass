//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_color/no_rgb_hsl.hrx"

mod alpha {
    #[test]
    fn high() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(rgba(red, 0.5), $alpha: 0.14)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 0, 0, 0.64);\
        \n}\
        \n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(rgba(red, 0.5), $alpha: -0.14)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 0, 0, 0.36);\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(rgba(red, 0.5), $alpha: 1)}\
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
    fn max_remaining() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(rgba(red, 0.5), $alpha: 0.5)}\
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
    fn min() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(rgba(red, 0.5), $alpha: -1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 0, 0, 0);\
        \n}\
        \n"
        );
    }
    #[test]
    fn min_remaining() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(rgba(red, 0.5), $alpha: -0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 0, 0, 0);\
        \n}\
        \n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(rgba(red, 0.5), $alpha: 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 0, 0, 0.5);\
        \n}\
        \n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-color($color: red)}\
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
fn positional() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-color(red)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: red;\
        \n}\
        \n"
    );
}
