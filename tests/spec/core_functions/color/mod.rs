//! Tests auto-converted from "sass-spec/spec/core_functions/color"
#[allow(unused)]
use super::rsass;

mod adjust_color;

// From "sass-spec/spec/core_functions/color/adjust_hue.hrx"
mod adjust_hue {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn above_max() {
        assert_eq!(
            rsass(
                "a {b: adjust-hue(red, 540)}\
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
    fn alpha() {
        assert_eq!(
            rsass(
                "a {b: adjust-hue(rgba(red, 0.1), 359)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 0, 4, 0.1);\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "color", error tests are not supported yet.

            // Ignoring "hue", error tests are not supported yet.
        }
    }
    #[test]
    fn fraction() {
        assert_eq!(
            rsass(
                "a {b: adjust-hue(red, 0.5)}\
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
                "a {b: adjust-hue(red, 359)}\
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
                "a {b: adjust-hue(red, 123)}\
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
                "a {b: adjust-hue(blue, 0)}\
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
    fn named() {
        assert_eq!(
            rsass(
                "a {b: adjust-hue($color: red, $degrees: 123)}\
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
    fn negative() {
        assert_eq!(
            rsass(
                "a {b: adjust-hue(red, -180)}\
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

// From "sass-spec/spec/core_functions/color/alpha.hrx"
mod alpha {
    #[allow(unused)]
    use super::rsass;
    mod color {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: alpha(red)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 1;\
        \n}\
        \n"
            );
        }
        #[test]
        fn middle() {
            assert_eq!(
                rsass(
                    "a {b: alpha(rgba(red, 0.42))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 0.42;\
        \n}\
        \n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                rsass(
                    "a {b: alpha(rgba(red, 0))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 0;\
        \n}\
        \n"
            );
        }
        #[test]
        fn named() {
            assert_eq!(
                rsass(
                    "a {b: alpha($color: rgba(red, 0.73))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 0.73;\
        \n}\
        \n"
            );
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "quoted_string", error tests are not supported yet.

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
        mod unquoted_string {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "no_equals", error tests are not supported yet.

            // Ignoring "non_identifier_before_equals", error tests are not supported yet.
        }
    }
    mod filter {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn multi_args() {
            assert_eq!(
                rsass(
                    "a {b: alpha(c=d, e=f, g=h)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: alpha(c=d, e=f, g=h);\
        \n}\
        \n"
            );
        }
        #[test]
        fn one_arg() {
            assert_eq!(
                rsass(
                    "a {b: alpha(c=d)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: alpha(c=d);\
        \n}\
        \n"
            );
        }
        #[test]
        fn space_before_equals() {
            assert_eq!(
                rsass(
                    "a {b: alpha(unquote(\"c = d\"))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: alpha(c = d);\
        \n}\
        \n"
            );
        }
    }
    mod opacity {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn filter() {
            assert_eq!(
                rsass(
                    "a {b: opacity(10%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: opacity(10%);\
        \n}\
        \n"
            );
        }
        #[test]
        fn named() {
            assert_eq!(
                rsass(
                    "a {b: opacity($color: rgba(red, 0.2))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 0.2;\
        \n}\
        \n"
            );
        }
        #[test]
        fn positional() {
            assert_eq!(
                rsass(
                    "a {b: opacity(rgba(red, 0.2))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 0.2;\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/color/blue.hrx"
mod blue {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn max() {
        assert_eq!(
            rsass(
                "a {b: blue(rgb(0, 0, 255))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 255;\
        \n}\
        \n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            rsass(
                "a {b: blue(rgb(0, 0, 123))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 123;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            rsass(
                "a {b: blue(rgb(0, 0, 0))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0;\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: blue($color: rgb(0, 0, 234))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 234;\
        \n}\
        \n"
        );
    }
}

mod change_color;

// From "sass-spec/spec/core_functions/color/complement.hrx"
mod complement {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn alpha() {
        assert_eq!(
            rsass(
                "a {b: complement(rgba(turquoise, 0.7))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(224, 64, 80, 0.7);\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    mod grayscale {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn black() {
            assert_eq!(
                rsass(
                    "a {b: complement(black)}\
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
        fn gray() {
            assert_eq!(
                rsass(
                    "a {b: complement(gray)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: gray;\
        \n}\
        \n"
            );
        }
        #[test]
        fn white() {
            assert_eq!(
                rsass(
                    "a {b: complement(white)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: white;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: complement($color: red)}\
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
    fn red() {
        assert_eq!(
            rsass(
                "a {b: complement(red)}\
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
    fn turquoise() {
        assert_eq!(
            rsass(
                "a {b: complement(turquoise)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #e04050;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/color/darken.hrx"
mod darken {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn alpha() {
        assert_eq!(
            rsass(
                "a {b: darken(rgba(red, 0.2), 100%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 0, 0, 0.2);\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod bounds {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "too_high", error tests are not supported yet.

            // Ignoring "too_low", error tests are not supported yet.
        }

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "color", error tests are not supported yet.

            // Ignoring "lightness", error tests are not supported yet.
        }
    }
    #[test]
    fn fraction() {
        assert_eq!(
            rsass(
                "a {b: darken(red, 0.5%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #fc0000;\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            rsass(
                "a {b: darken(red, 100%)}\
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
    fn max_remaining() {
        assert_eq!(
            rsass(
                "a {b: darken(red, 50%)}\
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
    fn middle() {
        assert_eq!(
            rsass(
                "a {b: darken(red, 14%)}\
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
    fn min() {
        assert_eq!(
            rsass(
                "a {b: darken(red, 0%)}\
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
    fn named() {
        assert_eq!(
            rsass(
                "a {b: darken($color: red, $amount: 14%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #b80000;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/color/desaturate.hrx"
mod desaturate {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn alpha() {
        assert_eq!(
            rsass(
                "a {b: desaturate(rgba(plum, 0.3), 100%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(191, 191, 191, 0.3);\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod bounds {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "too_high", error tests are not supported yet.

            // Ignoring "too_low", error tests are not supported yet.
        }
        mod one_arg {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "test_type", error tests are not supported yet.
        }

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "color", error tests are not supported yet.

            // Ignoring "lightness", error tests are not supported yet.
        }
    }
    #[test]
    fn max() {
        assert_eq!(
            rsass(
                "a {b: desaturate(plum, 100%)}\
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
    fn max_remaining() {
        assert_eq!(
            rsass(
                "a {b: desaturate(plum, 48%)}\
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
    fn middle() {
        assert_eq!(
            rsass(
                "a {b: desaturate(plum, 14%)}\
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
    fn min() {
        assert_eq!(
            rsass(
                "a {b: desaturate(plum, 0%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: plum;\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: desaturate($color: plum, $amount: 14%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #d4a9d4;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/color/fade_in.hrx"
mod fade_in {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod bounds {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "too_high", error tests are not supported yet.

            // Ignoring "too_low", error tests are not supported yet.
        }

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "alpha", error tests are not supported yet.

            // Ignoring "color", error tests are not supported yet.
        }
    }
    #[test]
    fn max() {
        assert_eq!(
            rsass(
                "a {b: fade-in(rgba(red, 0.5), 1)}\
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
                "a {b: fade-in(rgba(red, 0.5), 0.5)}\
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
    fn middle() {
        assert_eq!(
            rsass(
                "a {b: fade-in(rgba(red, 0.5), 0.14)}\
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
    fn min() {
        assert_eq!(
            rsass(
                "a {b: fade-in(rgba(red, 0.5), 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 0, 0, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: fade-in($color: rgba(red, 0.5), $amount: 0.14)}\
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
    fn opacify() {
        assert_eq!(
            rsass(
                "a {b: opacify($color: rgba(red, 0.5), $amount: 0.14)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 0, 0, 0.64);\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/color/fade_out.hrx"
mod fade_out {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod bounds {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "too_high", error tests are not supported yet.

            // Ignoring "too_low", error tests are not supported yet.
        }

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "alpha", error tests are not supported yet.

            // Ignoring "color", error tests are not supported yet.
        }
    }
    #[test]
    fn max() {
        assert_eq!(
            rsass(
                "a {b: fade-out(rgba(red, 0.5), 1)}\
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
    fn max_remaining() {
        assert_eq!(
            rsass(
                "a {b: fade-out(rgba(red, 0.5), 0.5)}\
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
    fn middle() {
        assert_eq!(
            rsass(
                "a {b: fade-out(rgba(red, 0.5), 0.14)}\
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
    fn min() {
        assert_eq!(
            rsass(
                "a {b: fade-out(rgba(red, 0.5), 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 0, 0, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: fade-out($color: rgba(red, 0.5), $amount: 0.14)}\
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
    fn transparentize() {
        assert_eq!(
        rsass(
            "a {b: transparentize($color: rgba(red, 0.5), $amount: 0.14)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(255, 0, 0, 0.36);\
        \n}\
        \n"
    );
    }
}

// From "sass-spec/spec/core_functions/color/grayscale.hrx"
mod grayscale {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn alpha() {
        assert_eq!(
            rsass(
                "a {b: grayscale(rgba(#633736, 0.3))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(77, 77, 77, 0.3);\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn max_saturation() {
        assert_eq!(
            rsass(
                "a {b: grayscale(red)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: gray;\
        \n}\
        \n"
        );
    }
    #[test]
    fn mid_saturation() {
        assert_eq!(
            rsass(
                "a {b: grayscale(#633736)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #4d4d4d;\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: grayscale($color: white)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: white;\
        \n}\
        \n"
        );
    }
    mod no_saturation {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn black() {
            assert_eq!(
                rsass(
                    "a {b: grayscale(black)}\
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
        fn gray() {
            assert_eq!(
                rsass(
                    "a {b: grayscale(#494949)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #494949;\
        \n}\
        \n"
            );
        }
        #[test]
        fn white() {
            assert_eq!(
                rsass(
                    "a {b: grayscale(white)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: white;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn number() {
        assert_eq!(
        rsass(
            "// A number should produce a plain function string, for CSS filter functions.\
            \na {b: grayscale(15%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: grayscale(15%);\
        \n}\
        \n"
    );
    }
}

// From "sass-spec/spec/core_functions/color/green.hrx"
mod green {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn max() {
        assert_eq!(
            rsass(
                "a {b: green(rgb(0, 255, 0))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 255;\
        \n}\
        \n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            rsass(
                "a {b: green(rgb(0, 123, 0))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 123;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            rsass(
                "a {b: green(rgb(0, 0, 0))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0;\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: green($color: rgb(0, 234, 0))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 234;\
        \n}\
        \n"
        );
    }
}

mod hsl;

mod hsla;

// From "sass-spec/spec/core_functions/color/hue.hrx"
mod hue {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn above_max() {
        assert_eq!(
            rsass(
                "a {b: hue(hsl(540, 100%, 100%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 180deg;\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    #[ignore] // wrong result
    fn fraction() {
        assert_eq!(
            rsass(
                "a {b: hue(hsl(0.5, 100%, 100%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.5deg;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn max() {
        assert_eq!(
            rsass(
                "a {b: hue(hsl(359, 100%, 100%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 359deg;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn middle() {
        assert_eq!(
            rsass(
                "a {b: hue(hsl(123, 100%, 100%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 123deg;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            rsass(
                "a {b: hue(hsl(0, 100%, 100%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0deg;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn named() {
        assert_eq!(
            rsass(
                "a {b: hue($color: hsl(234, 100%, 100%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 234deg;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn negative() {
        assert_eq!(
            rsass(
                "a {b: hue(hsl(-180, 100%, 100%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 180deg;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/color/ie_hex_str.hrx"
mod ie_hex_str {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn leading_zero() {
        assert_eq!(
            rsass(
                "a {b: ie-hex-str(rgba(#020304, 0.003))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #01020304;\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: ie-hex-str($color: #daddee)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #FFDADDEE;\
        \n}\
        \n"
        );
    }
    #[test]
    fn opaque() {
        assert_eq!(
            rsass(
                "a {b: ie-hex-str(#daddee)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #FFDADDEE;\
        \n}\
        \n"
        );
    }
    #[test]
    fn translucent() {
        assert_eq!(
            rsass(
                "a {b: ie-hex-str(rgba(#daddee, 0.3))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #4DDADDEE;\
        \n}\
        \n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            rsass(
                "a {b: ie-hex-str(rgba(turquoise, 0))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #0040E0D0;\
        \n}\
        \n"
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            rsass(
                "a {b: type-of(ie-hex-str(#daddee))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: string;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/color/invert.hrx"
mod invert {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn alpha() {
        assert_eq!(
            rsass(
                "a {b: invert(rgba(turquoise, 0.4))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(191, 31, 47, 0.4);\
        \n}\
        \n"
        );
    }
    #[test]
    fn black() {
        assert_eq!(
            rsass(
                "a {b: invert(black)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: white;\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod bounds {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "too_high", error tests are not supported yet.

            // Ignoring "too_low", error tests are not supported yet.
        }

        // Ignoring "number_with_weight", error tests are not supported yet.

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "color", error tests are not supported yet.

            // Ignoring "weight", error tests are not supported yet.
        }
    }
    #[test]
    fn gray() {
        assert_eq!(
            rsass(
                "a {b: invert(gray)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #7f7f7f;\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: invert($color: turquoise, $weight: 0%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: turquoise;\
        \n}\
        \n"
        );
    }
    #[test]
    fn number() {
        assert_eq!(
            rsass(
                "a {b: invert(10%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: invert(10%);\
        \n}\
        \n"
        );
    }
    #[test]
    fn turquoise() {
        assert_eq!(
            rsass(
                "a {b: invert(turquoise)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #bf1f2f;\
        \n}\
        \n"
        );
    }
    mod weighted {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn high() {
            assert_eq!(
                rsass(
                    "a {b: invert(turquoise, 92%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #b52e3c;\
        \n}\
        \n"
            );
        }
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: invert(turquoise, 23%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #5db4ab;\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: invert(turquoise, 100%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #bf1f2f;\
        \n}\
        \n"
            );
        }
        #[test]
        fn middle() {
            assert_eq!(
                rsass(
                    "a {b: invert(turquoise, 50%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: gray;\
        \n}\
        \n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                rsass(
                    "a {b: invert(turquoise, 0%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: turquoise;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn white() {
        assert_eq!(
            rsass(
                "a {b: invert(white)}\
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

// From "sass-spec/spec/core_functions/color/lighten.hrx"
mod lighten {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn alpha() {
        assert_eq!(
            rsass(
                "a {b: lighten(rgba(red, 0.4), 100%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 255, 255, 0.4);\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod bounds {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "too_high", error tests are not supported yet.

            // Ignoring "too_low", error tests are not supported yet.
        }

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "color", error tests are not supported yet.

            // Ignoring "lightness", error tests are not supported yet.
        }
    }
    #[test]
    fn fraction() {
        assert_eq!(
            rsass(
                "a {b: lighten(red, 0.5%)}\
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
    fn max() {
        assert_eq!(
            rsass(
                "a {b: lighten(red, 100%)}\
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
                "a {b: lighten(red, 50%)}\
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
    fn middle() {
        assert_eq!(
            rsass(
                "a {b: lighten(red, 14%)}\
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
    fn min() {
        assert_eq!(
            rsass(
                "a {b: lighten(red, 0%)}\
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
    fn named() {
        assert_eq!(
            rsass(
                "a {b: lighten($color: red, $amount: 14%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #ff4747;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/color/lightness.hrx"
mod lightness {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn fraction() {
        assert_eq!(
            rsass(
                "a {b: lightness(hsl(0, 100%, 0.5%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.5%;\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            rsass(
                "a {b: lightness(hsl(0, 100%, 100%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 100%;\
        \n}\
        \n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            rsass(
                "a {b: lightness(hsl(0, 100%, 50%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 50%;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            rsass(
                "a {b: lightness(hsl(0, 100%, 0%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0%;\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: lightness($color: hsl(0, 100%, 42%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 42%;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/color/mix.hrx"
mod mix {
    #[allow(unused)]
    use super::rsass;
    mod alpha {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn even() {
            assert_eq!(
                rsass(
                    "a {b: mix(rgba(#91e16f, 0.3), rgba(#0144bf, 0.3))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(73, 147, 151, 0.3);\
        \n}\
        \n"
            );
        }
        #[test]
        fn first() {
            assert_eq!(
                rsass(
                    "a {b: mix(#91e16f, transparent)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(145, 225, 111, 0.5);\
        \n}\
        \n"
            );
        }
        #[test]
        fn firstwards() {
            assert_eq!(
                rsass(
                    "a {b: mix(rgba(#91e16f, 0.8), rgba(#0144bf, 0.3))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(109, 186, 131, 0.55);\
        \n}\
        \n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                rsass(
                    "a {b: mix(transparent, #0144bf)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(1, 68, 191, 0.5);\
        \n}\
        \n"
            );
        }
        #[test]
        fn lastwards() {
            assert_eq!(
                rsass(
                    "a {b: mix(rgba(#91e16f, 0.4), rgba(#0144bf, 0.9))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(37, 107, 171, 0.65);\
        \n}\
        \n"
            );
        }
    }
    mod both_weights {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn contradiction() {
            assert_eq!(
        rsass(
            "// When we weight entirely towards a transparent color, the formula for\
            \n// computing the combined alpha would divide by zero, so we just return\
            \n// transparent as a special case.\
            \na {b: mix(transparent, #0144bf, 100%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(0, 0, 0, 0);\
        \n}\
        \n"
    );
        }
        mod mixed {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn firstwards() {
                assert_eq!(
        rsass(
            "a {b: mix(rgba(#91e16f, 0.8), rgba(#0144bf, 0.3), 63%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(121, 199, 124, 0.615);\
        \n}\
        \n"
    );
            }
            #[test]
            fn lastwards() {
                assert_eq!(
        rsass(
            "a {b: mix(rgba(#91e16f, 0.2), rgba(#0144bf, 0.7), 42%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(29, 99, 175, 0.49);\
        \n}\
        \n"
    );
            }
        }
        mod transparent {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn first() {
                assert_eq!(
                    rsass(
                        "a {b: mix(transparent, #0144bf, 70%)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: rgba(1, 68, 191, 0.3);\
        \n}\
        \n"
                );
            }
            #[test]
            fn last() {
                assert_eq!(
                    rsass(
                        "a {b: mix(#91e16f, transparent, 70%)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: rgba(145, 225, 111, 0.7);\
        \n}\
        \n"
                );
            }
        }
        mod weighted {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn first() {
                assert_eq!(
        rsass(
            "a {b: mix(rgba(#91e16f, 0.2), rgba(#0144bf, 0.7), 100%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(145, 225, 111, 0.2);\
        \n}\
        \n"
    );
            }
            #[test]
            fn last() {
                assert_eq!(
        rsass(
            "a {b: mix(rgba(#91e16f, 0.2), rgba(#0144bf, 0.7), 0%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(1, 68, 191, 0.7);\
        \n}\
        \n"
    );
            }
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod bounds {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "too_high", error tests are not supported yet.

            // Ignoring "too_low", error tests are not supported yet.
        }

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "color1", error tests are not supported yet.

            // Ignoring "color2", error tests are not supported yet.

            // Ignoring "weight", error tests are not supported yet.
        }
    }
    mod explicit_weight {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn even() {
            assert_eq!(
                rsass(
                    "a {b: mix(#91e16f, #0144bf, 50%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #499397;\
        \n}\
        \n"
            );
        }
        #[test]
        fn first() {
            assert_eq!(
                rsass(
                    "a {b: mix(#91e16f, #0144bf, 100%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #91e16f;\
        \n}\
        \n"
            );
        }
        #[test]
        fn firstwards() {
            assert_eq!(
                rsass(
                    "a {b: mix(#91e16f, #0144bf, 92%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #85d475;\
        \n}\
        \n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                rsass(
                    "a {b: mix(#91e16f, #0144bf, 0%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #0144bf;\
        \n}\
        \n"
            );
        }
        #[test]
        fn lastwards() {
            assert_eq!(
                rsass(
                    "a {b: mix(#91e16f, #0144bf, 43%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #3f889d;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn named() {
        assert_eq!(
        rsass(
            "a {b: mix($color1: #91e16f, $color2: #0144bf, $weight: 92%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #85d475;\
        \n}\
        \n"
    );
    }
    mod unweighted {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn average() {
            assert_eq!(
        rsass(
            "// All channels should be averaged across the two colors.\
            \na {b: mix(#91e16f, #0144bf)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #499397;\
        \n}\
        \n"
    );
        }
        #[test]
        fn identical() {
            assert_eq!(
        rsass(
            "// If two channels have the same values, they should be the same in the output.\
            \na {b: mix(#123456, #123456)}\
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
        fn min_and_max() {
            assert_eq!(
        rsass(
            "// Each channel becomes the average of 255 and 0, which is 128 = 0xAA.\
            \na {b: mix(#ff00ff, #00ff00)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: gray;\
        \n}\
        \n"
    );
        }
    }
}

// From "sass-spec/spec/core_functions/color/red.hrx"
mod red {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn max() {
        assert_eq!(
            rsass(
                "a {b: red(rgb(255, 0, 0))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 255;\
        \n}\
        \n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            rsass(
                "a {b: red(rgb(123, 0, 0))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 123;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            rsass(
                "a {b: red(rgb(0, 0, 0))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0;\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: red($color: rgb(234, 0, 0))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 234;\
        \n}\
        \n"
        );
    }
}

mod rgb;

mod rgba;

// From "sass-spec/spec/core_functions/color/saturate.hrx"
mod saturate {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod one_arg {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "test_type", error tests are not supported yet.
        }

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod two_args {
            #[allow(unused)]
            use super::rsass;
            mod bounds {
                #[allow(unused)]
                use super::rsass;

                // Ignoring "too_high", error tests are not supported yet.

                // Ignoring "too_low", error tests are not supported yet.
            }
            mod test_type {
                #[allow(unused)]
                use super::rsass;

                // Ignoring "color", error tests are not supported yet.

                // Ignoring "lightness", error tests are not supported yet.
            }
        }
    }
    mod one_arg {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn named() {
            assert_eq!(
                rsass(
                    "a {b: saturate($amount: 50%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: saturate(50%);\
        \n}\
        \n"
            );
        }
        #[test]
        fn unit() {
            assert_eq!(
                rsass(
                    "a {b: saturate(50%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: saturate(50%);\
        \n}\
        \n"
            );
        }
        #[test]
        fn unitless() {
            assert_eq!(
                rsass(
                    "a {b: saturate(1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: saturate(1);\
        \n}\
        \n"
            );
        }
    }
    mod two_args {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn alpha() {
            assert_eq!(
                rsass(
                    "a {b: saturate(rgba(plum, 0.5), 100%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(255, 126, 255, 0.5);\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: saturate(plum, 100%)}\
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
                    "a {b: saturate(plum, 53%)}\
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
        fn middle() {
            assert_eq!(
                rsass(
                    "a {b: saturate(plum, 14%)}\
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
        fn min() {
            assert_eq!(
                rsass(
                    "a {b: saturate(plum, 0%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: plum;\
        \n}\
        \n"
            );
        }
        #[test]
        fn named() {
            assert_eq!(
                rsass(
                    "a {b: saturate($color: plum, $amount: 14%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #e697e6;\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/color/saturation.hrx"
mod saturation {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    #[ignore] // wrong result
    fn fraction() {
        assert_eq!(
            rsass(
                "a {b: saturation(hsl(0, 0.5%, 100%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.5%;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn max() {
        assert_eq!(
            rsass(
                "a {b: saturation(hsl(0, 100%, 100%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 100%;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn middle() {
        assert_eq!(
            rsass(
                "a {b: saturation(hsl(0, 50%, 100%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 50%;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            rsass(
                "a {b: saturation(hsl(0, 0%, 100%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0%;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn named() {
        assert_eq!(
            rsass(
                "a {b: saturation($color: hsl(0, 42%, 100%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 42%;\
        \n}\
        \n"
        );
    }
}

mod scale_color;
