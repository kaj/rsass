//! Tests auto-converted from "sass-spec/spec/core_functions/color"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

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
                rsass("a {b: alpha(red)}\n").unwrap(),
                "a {\n  b: 1;\n}\n"
            );
        }
        #[test]
        fn middle() {
            assert_eq!(
                rsass("a {b: alpha(rgba(red, 0.42))}\n").unwrap(),
                "a {\n  b: 0.42;\n}\n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                rsass("a {b: alpha(rgba(red, 0))}\n").unwrap(),
                "a {\n  b: 0;\n}\n"
            );
        }
        #[test]
        fn named() {
            assert_eq!(
                rsass("a {b: alpha($color: rgba(red, 0.73))}\n").unwrap(),
                "a {\n  b: 0.73;\n}\n"
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
        #[ignore] // failing
        fn multi_args() {
            assert_eq!(
                rsass("a {b: alpha(c=d, e=f, g=h)}\n").unwrap(),
                "a {\n  b: alpha(c=d, e=f, g=h);\n}\n"
            );
        }
        #[test]
        #[ignore] // failing
        fn one_arg() {
            assert_eq!(
                rsass("a {b: alpha(c=d)}\n").unwrap(),
                "a {\n  b: alpha(c=d);\n}\n"
            );
        }
        #[test]
        #[ignore] // failing
        fn space_before_equals() {
            assert_eq!(
                rsass("a {b: alpha(unquote(\"c = d\"))}\n").unwrap(),
                "a {\n  b: alpha(c = d);\n}\n"
            );
        }
    }
    mod opacity {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn filter() {
            assert_eq!(
                rsass("a {b: opacity(10%)}\n").unwrap(),
                "a {\n  b: opacity(10%);\n}\n"
            );
        }
        #[test]
        fn named() {
            assert_eq!(
                rsass("a {b: opacity($color: rgba(red, 0.2))}\n").unwrap(),
                "a {\n  b: 0.2;\n}\n"
            );
        }
        #[test]
        fn positional() {
            assert_eq!(
                rsass("a {b: opacity(rgba(red, 0.2))}\n").unwrap(),
                "a {\n  b: 0.2;\n}\n"
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
            rsass("a {b: blue(rgb(0, 0, 255))}\n").unwrap(),
            "a {\n  b: 255;\n}\n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            rsass("a {b: blue(rgb(0, 0, 123))}\n").unwrap(),
            "a {\n  b: 123;\n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            rsass("a {b: blue(rgb(0, 0, 0))}\n").unwrap(),
            "a {\n  b: 0;\n}\n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass("a {b: blue($color: rgb(0, 0, 234))}\n").unwrap(),
            "a {\n  b: 234;\n}\n"
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
            rsass("a {b: green(rgb(0, 255, 0))}\n").unwrap(),
            "a {\n  b: 255;\n}\n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            rsass("a {b: green(rgb(0, 123, 0))}\n").unwrap(),
            "a {\n  b: 123;\n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            rsass("a {b: green(rgb(0, 0, 0))}\n").unwrap(),
            "a {\n  b: 0;\n}\n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass("a {b: green($color: rgb(0, 234, 0))}\n").unwrap(),
            "a {\n  b: 234;\n}\n"
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
    #[ignore] // failing
    fn above_max() {
        assert_eq!(
            rsass("a {b: hue(hsl(540, 100%, 100%))}\n").unwrap(),
            "a {\n  b: 180deg;\n}\n"
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
    #[ignore] // failing
    fn fraction() {
        assert_eq!(
            rsass("a {b: hue(hsl(0.5, 100%, 100%))}\n").unwrap(),
            "a {\n  b: 0.5deg;\n}\n"
        );
    }
    #[test]
    #[ignore] // failing
    fn max() {
        assert_eq!(
            rsass("a {b: hue(hsl(359, 100%, 100%))}\n").unwrap(),
            "a {\n  b: 359deg;\n}\n"
        );
    }
    #[test]
    #[ignore] // failing
    fn middle() {
        assert_eq!(
            rsass("a {b: hue(hsl(123, 100%, 100%))}\n").unwrap(),
            "a {\n  b: 123deg;\n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            rsass("a {b: hue(hsl(0, 100%, 100%))}\n").unwrap(),
            "a {\n  b: 0deg;\n}\n"
        );
    }
    #[test]
    #[ignore] // failing
    fn named() {
        assert_eq!(
            rsass("a {b: hue($color: hsl(234, 100%, 100%))}\n").unwrap(),
            "a {\n  b: 234deg;\n}\n"
        );
    }
    #[test]
    #[ignore] // failing
    fn negative() {
        assert_eq!(
            rsass("a {b: hue(hsl(-180, 100%, 100%))}\n").unwrap(),
            "a {\n  b: 180deg;\n}\n"
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
            rsass("a {b: lightness(hsl(0, 100%, 0.5%))}\n").unwrap(),
            "a {\n  b: 0.5%;\n}\n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            rsass("a {b: lightness(hsl(0, 100%, 100%))}\n").unwrap(),
            "a {\n  b: 100%;\n}\n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            rsass("a {b: lightness(hsl(0, 100%, 50%))}\n").unwrap(),
            "a {\n  b: 50%;\n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            rsass("a {b: lightness(hsl(0, 100%, 0%))}\n").unwrap(),
            "a {\n  b: 0%;\n}\n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass("a {b: lightness($color: hsl(0, 100%, 42%))}\n").unwrap(),
            "a {\n  b: 42%;\n}\n"
        );
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
            rsass("a {b: red(rgb(255, 0, 0))}\n").unwrap(),
            "a {\n  b: 255;\n}\n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            rsass("a {b: red(rgb(123, 0, 0))}\n").unwrap(),
            "a {\n  b: 123;\n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            rsass("a {b: red(rgb(0, 0, 0))}\n").unwrap(),
            "a {\n  b: 0;\n}\n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass("a {b: red($color: rgb(234, 0, 0))}\n").unwrap(),
            "a {\n  b: 234;\n}\n"
        );
    }
}

mod rgb;

mod rgba;

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
    #[ignore] // failing
    fn fraction() {
        assert_eq!(
            rsass("a {b: saturation(hsl(0, 0.5%, 100%))}\n").unwrap(),
            "a {\n  b: 0.5%;\n}\n"
        );
    }
    #[test]
    #[ignore] // failing
    fn max() {
        assert_eq!(
            rsass("a {b: saturation(hsl(0, 100%, 100%))}\n").unwrap(),
            "a {\n  b: 100%;\n}\n"
        );
    }
    #[test]
    #[ignore] // failing
    fn middle() {
        assert_eq!(
            rsass("a {b: saturation(hsl(0, 50%, 100%))}\n").unwrap(),
            "a {\n  b: 50%;\n}\n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            rsass("a {b: saturation(hsl(0, 0%, 100%))}\n").unwrap(),
            "a {\n  b: 0%;\n}\n"
        );
    }
    #[test]
    #[ignore] // failing
    fn named() {
        assert_eq!(
            rsass("a {b: saturation($color: hsl(0, 42%, 100%))}\n").unwrap(),
            "a {\n  b: 42%;\n}\n"
        );
    }
}
