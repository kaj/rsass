//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale_color/hsl.hrx"

#[test]
fn all() {
    assert_eq!(
        crate::rsass(
            "a {b: scale-color(turquoise, $saturation: 24%, $lightness: -48%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #10867a;\
        \n}\
        \n"
    );
}
#[test]
fn alpha_arg() {
    assert_eq!(
        crate::rsass(
            "a {b: scale-color(turquoise, $saturation: 24%, $lightness: -48%, $alpha: -70%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(16, 134, 122, 0.3);\
        \n}\
        \n"
    );
}
#[test]
fn alpha_input() {
    assert_eq!(
        crate::rsass(
            "a {b: scale-color(rgba(turquoise, 0.7), $saturation: 24%, $lightness: -48%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(16, 134, 122, 0.7);\
        \n}\
        \n"
    );
}
mod lightness {
    #[test]
    fn high() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(red, $lightness: 94%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #fff0f0;\
        \n}\
        \n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(red, $lightness: -14%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #db0000;\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(red, $lightness: 100%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: white;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(red, $lightness: -100%)}\
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
    fn zero() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(red, $lightness: 0%)}\
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
            "a {b: scale-color($color: turquoise, $saturation: 24%, $lightness: -48%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #10867a;\
        \n}\
        \n"
    );
}
mod saturation {
    #[test]
    fn high() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(plum, $saturation: 67%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #f489f4;\
        \n}\
        \n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(plum, $saturation: -43%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #d0add0;\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(plum, $saturation: 100%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #ff7eff;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(plum, $saturation: -100%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #bfbfbf;\
        \n}\
        \n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            crate::rsass(
                "a {b: scale-color(plum, $saturation: 0%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: plum;\
        \n}\
        \n"
        );
    }
}
