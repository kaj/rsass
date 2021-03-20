//! Tests auto-converted from "sass-spec/spec/core_functions/color/change_color/hsl.hrx"

#[test]
fn all() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color(black, $hue: 12, $saturation: 24%, $lightness: 48%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #98695d;\
        \n}\
        \n"
    );
}
#[test]
fn alpha_arg() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color(black, $hue: 12, $saturation: 24%, $lightness: 48%, $alpha: 0.7)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(152, 105, 93, 0.7);\
        \n}\
        \n"
    );
}
#[test]
fn alpha_input() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color(rgba(black, 0.7), $hue: 12, $saturation: 24%, $lightness: 48%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(152, 105, 93, 0.7);\
        \n}\
        \n"
    );
}
mod hue {
    #[test]
    fn above_max() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $hue: 540)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: aqua;\
        \n}\
        \n"
        );
    }
    #[test]
    fn fraction() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $hue: 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #ff0200;\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $hue: 359)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #ff0004;\
        \n}\
        \n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $hue: 123)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #00ff0d;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(blue, $hue: 0)}\
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
    fn negative() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $hue: -60)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: fuchsia;\
        \n}\
        \n"
        );
    }
}
mod lightness {
    #[test]
    fn fraction() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $lightness: 0.5%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #030000;\
        \n}\
        \n"
        );
    }
    #[test]
    fn high() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $lightness: 63%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #ff4242;\
        \n}\
        \n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $lightness: 27%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #8a0000;\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $lightness: 100%)}\
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
                "a {b: change-color(red, $lightness: 0%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: black;\
        \n}\
        \n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color($color: black, $hue: 12, $saturation: 24%, $lightness: 48%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #98695d;\
        \n}\
        \n"
    );
}
mod saturation {
    #[test]
    fn high() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(plum, $saturation: 76%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #f08df0;\
        \n}\
        \n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(plum, $saturation: 14%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #c8b5c8;\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(plum, $saturation: 100%)}\
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
                "a {b: change-color(plum, $saturation: 0%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #bfbfbf;\
        \n}\
        \n"
        );
    }
}
