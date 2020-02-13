//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale_color"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

mod error;

// From "sass-spec/spec/core_functions/color/scale_color/hsl.hrx"
mod hsl {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn all() {
        assert_eq!(
        rsass(
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
        rsass(
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
        rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn high() {
            assert_eq!(
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
        rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn high() {
            assert_eq!(
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
}

// From "sass-spec/spec/core_functions/color/scale_color/no_rgb_hsl.hrx"
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
                    "a {b: scale-color(rgba(red, 0.5), $alpha: 14%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(255, 0, 0, 0.57);\
        \n}\
        \n"
            );
        }
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: scale-color(rgba(red, 0.3), $alpha: -36%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(255, 0, 0, 0.192);\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: scale-color(rgba(red, 0.5), $alpha: 100%)}\
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
                    "a {b: scale-color(rgba(red, 0.5), $alpha: -100%)}\
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
                    "a {b: scale-color(rgba(red, 0.5), $alpha: 0%)}\
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
                "a {b: scale-color($color: red)}\
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
                "a {b: scale-color(red)}\
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

// From "sass-spec/spec/core_functions/color/scale_color/rgb.hrx"
mod rgb {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn all() {
        assert_eq!(
        rsass(
            "a {b: scale-color(sienna, $red: 12%, $green: 24%, $blue: 48%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #ab7c92;\
        \n}\
        \n"
    );
    }
    #[test]
    fn alpha_arg() {
        assert_eq!(
        rsass(
            "a {b: scale-color(sienna, $red: 12%, $green: 24%, $blue: 48%, $alpha: -70%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(171, 124, 146, 0.3);\
        \n}\
        \n"
    );
    }
    #[test]
    fn alpha_input() {
        assert_eq!(
        rsass(
            "a {b: scale-color(rgba(sienna, 0.3), $red: 12%, $green: 24%, $blue: 48%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(171, 124, 146, 0.3);\
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
                    "a {b: scale-color(salmon, $blue: 42%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #fa80ad;\
        \n}\
        \n"
            );
        }
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: scale-color(slategray, $blue: -16%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #708079;\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: scale-color(black, $blue: 100%)}\
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
                    "a {b: scale-color(blue, $blue: -100%)}\
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
                    "a {b: scale-color(black, $blue: 0%)}\
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
                    "a {b: scale-color(cadetblue, $green: 12%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #5faaa0;\
        \n}\
        \n"
            );
        }
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: scale-color(seagreen, $green: -86%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #2e1357;\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: scale-color(black, $green: 100%)}\
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
                    "a {b: scale-color(lime, $green: -100%)}\
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
                    "a {b: scale-color(black, $green: 0%)}\
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
            "a {b: scale-color($color: sienna, $red: 12%, $green: 24%, $blue: 48%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #ab7c92;\
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
                    "a {b: scale-color(turquoise, $red: 86%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #e4e0d0;\
        \n}\
        \n"
            );
        }
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: scale-color(lightcoral, $red: -33%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #a18080;\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: scale-color(black, $red: 100%)}\
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
                    "a {b: scale-color(red, $red: -100%)}\
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
                    "a {b: scale-color(black, $red: 0%)}\
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
