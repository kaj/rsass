//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_color/hsl.hrx"

#[test]
fn all() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-color(black, $hue: 12, $saturation: 24%, $lightness: 48%)}\
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
            "a {b: adjust-color(black, $hue: 12, $saturation: 24%, $lightness: 48%, $alpha: -0.7)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(152, 105, 93, 0.3);\
        \n}\
        \n"
    );
}
#[test]
fn alpha_arg_above_max() {
    assert_eq!(
        crate::rsass(
            "// Regression test for sass/dart-sass#708.\
            \na {b: adjust-color(black, $hue: 12, $saturation: 24%, $lightness: 48%, $alpha: 0.7)}\
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
fn alpha_input() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-color(rgba(black, 0.7), $hue: 12, $saturation: 24%, $lightness: 48%)}\
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
                "a {b: adjust-color(red, $hue: 540)}\
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
                "a {b: adjust-color(red, $hue: 0.5)}\
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
                "a {b: adjust-color(red, $hue: 359)}\
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
                "a {b: adjust-color(red, $hue: 123)}\
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
                "a {b: adjust-color(blue, $hue: 0)}\
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
    fn negative() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(red, $hue: -180)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: aqua;\
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
                "a {b: adjust-color(red, $lightness: 0.5%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #ff0303;\
        \n}\
        \n"
        );
    }
    #[test]
    fn high() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(red, $lightness: 14%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #ff4747;\
        \n}\
        \n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(red, $lightness: -14%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #b80000;\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(red, $lightness: 100%)}\
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
    fn max_remaining() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(red, $lightness: 50%)}\
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
                "a {b: adjust-color(red, $lightness: -100%)}\
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
    fn min_remaining() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(red, $lightness: -50%)}\
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
                "a {b: adjust-color(red, $lightness: 0%)}\
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
            "a {b: adjust-color($color: black, $hue: 12, $saturation: 24%, $lightness: 48%)}\
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
                "a {b: adjust-color(plum, $saturation: 14%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #e697e6;\
        \n}\
        \n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(plum, $saturation: -14%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #d4a9d4;\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(plum, $saturation: 100%)}\
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
    fn max_remaining() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(plum, $saturation: 53%)}\
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
                "a {b: adjust-color(plum, $saturation: -100%)}\
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
    fn min_remaining() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(plum, $saturation: -48%)}\
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
                "a {b: adjust-color(plum, $saturation: 0%)}\
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
