//! Tests auto-converted from "sass-spec/spec/core_functions/color/change_color/no_rgb_hsl.hrx"

mod alpha {
    #[test]
    fn high() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(rgba(red, 0.5), $alpha: 0.72)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 0, 0, 0.72);\
        \n}\
        \n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(rgba(red, 0.5), $alpha: 0.36)}\
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
                "a {b: change-color(rgba(red, 0.5), $alpha: 1)}\
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
                "a {b: change-color(rgba(red, 0.5), $alpha: 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 0, 0, 0);\
        \n}\
        \n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color($color: red)}\
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
            "a {b: change-color(red)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: red;\
        \n}\
        \n"
    );
}
