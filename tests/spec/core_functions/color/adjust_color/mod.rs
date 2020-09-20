//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_color"
#[allow(unused)]
use super::rsass;

mod error;

// From "sass-spec/spec/core_functions/color/adjust_color/hsl.hrx"
mod hsl {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn all() {
        assert_eq!(
        rsass(
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
        rsass(
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
        rsass(
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
        rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn above_max() {
            assert_eq!(
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn fraction() {
            assert_eq!(
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
        rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn high() {
            assert_eq!(
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
}

// From "sass-spec/spec/core_functions/color/adjust_color/no_rgb_hsl.hrx"
mod no_rgb_hsl {
    #[allow(unused)]
    use super::rsass;
    mod alpha {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn high() {
            assert_eq!(
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
            rsass(
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
            rsass(
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
}

// From "sass-spec/spec/core_functions/color/adjust_color/rgb.hrx"
mod rgb {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn all() {
        assert_eq!(
            rsass(
                "a {b: adjust-color(black, $red: 12, $green: 24, $blue: 48)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #0c1830;\
        \n}\
        \n"
        );
    }
    #[test]
    fn alpha_arg() {
        assert_eq!(
        rsass(
            "a {b: adjust-color(black, $red: 12, $green: 24, $blue: 48, $alpha: -0.3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(12, 24, 48, 0.7);\
        \n}\
        \n"
    );
    }
    #[test]
    fn alpha_input() {
        assert_eq!(
        rsass(
            "a {b: adjust-color(rgba(black, 0.3), $red: 12, $green: 24, $blue: 48)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(12, 24, 48, 0.3);\
        \n}\
        \n"
    );
    }
    mod blue {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn high() {
            assert_eq!(
                rsass(
                    "a {b: adjust-color(black, $blue: 200)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #0000c8;\
        \n}\
        \n"
            );
        }
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: adjust-color(blue, $blue: -100)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #00009b;\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: adjust-color(black, $blue: 255)}\
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
        fn min() {
            assert_eq!(
                rsass(
                    "a {b: adjust-color(blue, $blue: -255)}\
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
                rsass(
                    "a {b: adjust-color(black, $blue: 0)}\
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
    mod green {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn high() {
            assert_eq!(
                rsass(
                    "a {b: adjust-color(black, $green: 200)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #00c800;\
        \n}\
        \n"
            );
        }
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: adjust-color(lime, $green: -100)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #009b00;\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: adjust-color(black, $green: 255)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: lime;\
        \n}\
        \n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                rsass(
                    "a {b: adjust-color(lime, $green: -255)}\
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
                rsass(
                    "a {b: adjust-color(black, $green: 0)}\
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
        rsass(
            "a {b: adjust-color($color: black, $red: 12, $green: 24, $blue: 48)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #0c1830;\
        \n}\
        \n"
    );
    }
    mod red {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn high() {
            assert_eq!(
                rsass(
                    "a {b: adjust-color(black, $red: 200)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #c80000;\
        \n}\
        \n"
            );
        }
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: adjust-color(red, $red: -100)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #9b0000;\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: adjust-color(black, $red: 255)}\
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
                rsass(
                    "a {b: adjust-color(red, $red: -255)}\
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
                rsass(
                    "a {b: adjust-color(black, $red: 0)}\
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
}
