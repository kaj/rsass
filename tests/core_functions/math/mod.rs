//! Tests auto-converted from "sass-spec/spec/core_functions/math"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/core_functions/math/abs.hrx"
mod abs {
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
    fn named() {
        assert_eq!(
            rsass(
                "a {b: abs($number: -3)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 3;\
             \n}\
             \n"
        );
    }
    mod negative {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn decimal() {
            assert_eq!(
                rsass(
                    "a {b: abs(-123.456)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 123.456;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn integer() {
            assert_eq!(
                rsass(
                    "a {b: abs(-17)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 17;\
                 \n}\
                 \n"
            );
        }
    }
    mod positive {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn decimal() {
            assert_eq!(
                rsass(
                    "a {b: abs(5.6)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 5.6;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn integer() {
            assert_eq!(
                rsass(
                    "a {b: abs(1)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 1;\
                 \n}\
                 \n"
            );
        }
    }
    #[test]
    #[ignore] // failing
    fn preserves_units() {
        assert_eq!(
            rsass(
                "a {b: abs(-7px / 4em) * 1em}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 1.75px;\
             \n}\
             \n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            rsass(
                "a {b: abs(0)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 0;\
             \n}\
             \n"
        );
    }
}

// From "sass-spec/spec/core_functions/math/ceil.hrx"
mod ceil {
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
    fn high() {
        assert_eq!(
            rsass(
                "a {b: ceil(2.9)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 3;\
             \n}\
             \n"
        );
    }
    #[test]
    fn integer() {
        assert_eq!(
            rsass(
                "a {b: ceil(1)}\
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
    fn low() {
        assert_eq!(
            rsass(
                "a {b: ceil(6.000000000000001)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 7;\
             \n}\
             \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: ceil($number: 1.6)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 2;\
             \n}\
             \n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            rsass(
                "a {b: ceil(-7.6)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: -7;\
             \n}\
             \n"
        );
    }
    #[test]
    #[ignore] // failing
    fn preserves_units() {
        assert_eq!(
            rsass(
                "a {b: ceil(7px / 4em) * 1em}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 2px;\
             \n}\
             \n"
        );
    }
}

// From "sass-spec/spec/core_functions/math/comparable.hrx"
mod comparable {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "arg_1", error tests are not supported yet.

            // Ignoring "arg_2", error tests are not supported yet.
        }
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: comparable($number1: 1, $number2: 2)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
    mod unit {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // failing
        fn to_compatible() {
            assert_eq!(
                rsass(
                    "a {b: comparable(1px, 2in)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn to_different() {
            assert_eq!(
                rsass(
                    "a {b: comparable(1px, 2em)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: false;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // failing
        fn to_inverse() {
            assert_eq!(
                rsass(
                    "a {b: comparable(1px, 1/1px)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: false;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn to_same() {
            assert_eq!(
                rsass(
                    "a {b: comparable(1px, 2px)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
    }
    mod unitless {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // failing
        fn to_unit() {
            assert_eq!(
                rsass(
                    "a {b: comparable(1, 2px)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn to_unitless() {
            assert_eq!(
                rsass(
                    "a {b: comparable(1, 2)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/math/floor.hrx"
mod floor {
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
    fn high() {
        assert_eq!(
            rsass(
                "a {b: floor(2.999999999999999)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 2;\
             \n}\
             \n"
        );
    }
    #[test]
    fn integer() {
        assert_eq!(
            rsass(
                "a {b: floor(1)}\
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
    fn low() {
        assert_eq!(
            rsass(
                "a {b: floor(6.1)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 6;\
             \n}\
             \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: floor($number: 1.6)}\
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
    fn negative() {
        assert_eq!(
            rsass(
                "a {b: floor(-7.2)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: -8;\
             \n}\
             \n"
        );
    }
    #[test]
    #[ignore] // failing
    fn preserves_units() {
        assert_eq!(
            rsass(
                "a {b: floor(7px / 4em) * 1em}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 1px;\
             \n}\
             \n"
        );
    }
}

// From "sass-spec/spec/core_functions/math/max.hrx"
mod max {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "incompatible_units", error tests are not supported yet.

        // Ignoring "too_few_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "arg_1", error tests are not supported yet.

            // Ignoring "arg_2", error tests are not supported yet.

            // Ignoring "arg_3", error tests are not supported yet.
        }
    }
    #[test]
    fn one_arg() {
        assert_eq!(
            rsass(
                "$arg: 1;\
                 \na {b: max($arg)}\
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
    fn three_args() {
        assert_eq!(
            rsass(
                "$arg: 1;\
                 \na {b: max(3, $arg, 2)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 3;\
             \n}\
             \n"
        );
    }
    #[test]
    fn two_args() {
        assert_eq!(
            rsass(
                "$arg: 1;\
                 \na {b: max($arg, 2)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 2;\
             \n}\
             \n"
        );
    }
    mod units {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn and_unitless() {
            assert_eq!(
                rsass(
                    "$arg: 2px;\
                     \na {b: max($arg, 1)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 2px;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn compatible() {
            assert_eq!(
                rsass(
                    "$arg: 1px;\
                     \na {b: max($arg, 1in, 1cm)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 1in;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                rsass(
                    "$arg: 6px;\
                     \na {b: max($arg, 2px, 10px)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 10px;\
                 \n}\
                 \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/math/min.hrx"
mod min {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "incompatible_units", error tests are not supported yet.

        // Ignoring "too_few_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "arg_1", error tests are not supported yet.

            // Ignoring "arg_2", error tests are not supported yet.

            // Ignoring "arg_3", error tests are not supported yet.
        }
    }
    #[test]
    fn one_arg() {
        assert_eq!(
            rsass(
                "$arg: 1;\
                 \na {b: min($arg)}\
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
    fn three_args() {
        assert_eq!(
            rsass(
                "$arg: 1;\
                 \na {b: min(3, $arg, 2)}\
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
    fn two_args() {
        assert_eq!(
            rsass(
                "$arg: 1;\
                 \na {b: min($arg, 2)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 1;\
             \n}\
             \n"
        );
    }
    mod units {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn and_unitless() {
            assert_eq!(
                rsass(
                    "$arg: 2px;\
                     \na {b: min($arg, 1)}\
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
        fn compatible() {
            assert_eq!(
                rsass(
                    "$arg: 1px;\
                     \na {b: min($arg, 1in, 1cm)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 1px;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                rsass(
                    "$arg: 6px;\
                     \na {b: min($arg, 2px, 10px)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 2px;\
                 \n}\
                 \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/math/percentage.hrx"
mod percentage {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.

        // Ignoring "unit", error tests are not supported yet.
    }
    #[test]
    fn integer() {
        assert_eq!(
            rsass(
                "a {b: percentage(42)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 4200%;\
             \n}\
             \n"
        );
    }
    #[test]
    fn large() {
        assert_eq!(
            rsass(
                "a {b: percentage(123.456)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 12345.6%;\
             \n}\
             \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: percentage($number: 1)}\
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
    fn negative() {
        assert_eq!(
            rsass(
                "a {b: percentage(-0.4)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: -40%;\
             \n}\
             \n"
        );
    }
    #[test]
    fn small() {
        assert_eq!(
            rsass(
                "a {b: percentage(0.246)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 24.6%;\
             \n}\
             \n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            rsass(
                "a {b: percentage(0)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 0%;\
             \n}\
             \n"
        );
    }
}

// From "sass-spec/spec/core_functions/math/random.hrx"
mod random {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "decimal", error tests are not supported yet.

        // Ignoring "negative", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.

        // Ignoring "zero", error tests are not supported yet.
    }
    #[test]
    fn ignores_units() {
        assert_eq!(
            rsass(
                "a {b: random(1px)}\
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
    fn named() {
        assert_eq!(
            rsass(
                "$value: random($limit: 10);\
                 \na {b: $value > 0 and $value <= 10}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
    #[test]
    fn no_arg() {
        assert_eq!(
            rsass(
                "$value: random();\
                 \na {b: $value >= 0 and $value < 1}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
    #[test]
    #[ignore] // failing
    fn null() {
        assert_eq!(
        rsass(
            "@import \"../util\";\
            \n@function check($value) {@return $value >= 0 and $value < 1}\
            \n@include check-values(null, get-function(check));\
            \n"
        )
        .unwrap(),
        ""
    );
    }
    #[test]
    #[ignore] // failing
    fn one() {
        assert_eq!(
            rsass(
                "@import \"../util\";\
                 \n@function check($value) {@return $value == 1}\
                 \n@include check-values(1, get-function(check));\
                 \n"
            )
            .unwrap(),
            ""
        );
    }
    #[test]
    #[ignore] // failing
    fn one_hundred() {
        assert_eq!(
        rsass(
            "@import \"../util\";\
            \n@function check($value) {@return $value == round($value) and $value > 0 and $value <= 100}\
            \n@include check-values(100, get-function(check));\
            \n"
        )
        .unwrap(),
        ""
    );
    }
    #[test]
    #[ignore] // failing
    fn two() {
        assert_eq!(
        rsass(
            "@import \"../util\";\
            \n@function check($value) {@return $value == 1 or $value == 2}\
            \n@include check-values(2, get-function(check));\
            \n"
        )
        .unwrap(),
        ""
    );
    }
    #[test]
    fn within_precision() {
        assert_eq!(
        rsass(
            "// This is within the precision limit to be considered identical to 1.\
            \na {b: random(1.0000000000001)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
    }
}

// From "sass-spec/spec/core_functions/math/round.hrx"
mod round {
    #[allow(unused)]
    use super::rsass;
    mod down {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: round(2.2)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 2;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn negative() {
            assert_eq!(
                rsass(
                    "a {b: round(-5.6)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: -6;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn to_zero() {
            assert_eq!(
                rsass(
                    "a {b: round(0.2)}\
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
        fn within_precision() {
            assert_eq!(
        rsass(
            "// This is the largest number that\'s representable as a float and outside the\
            \n// precision range to be considered equal to 5.\
            \na {b: round(1.49999999999)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
        }
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_few_args", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
    #[test]
    fn integer() {
        assert_eq!(
            rsass(
                "a {b: round(1)}\
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
    fn named() {
        assert_eq!(
            rsass(
                "a {b: round($number: 1.6)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 2;\
             \n}\
             \n"
        );
    }
    #[test]
    #[ignore] // failing
    fn preserves_units() {
        assert_eq!(
            rsass(
                "a {b: round(7px / 4em) * 1em}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: 2px;\
             \n}\
             \n"
        );
    }
    mod up {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn high() {
            assert_eq!(
                rsass(
                    "a {b: round(2.9)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 3;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn negative() {
            assert_eq!(
                rsass(
                    "a {b: round(-5.4)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: -5;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn point_five() {
            assert_eq!(
                rsass(
                    "a {b: round(16.5)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: 17;\
                 \n}\
                 \n"
            );
        }
        #[test]
        fn to_zero() {
            assert_eq!(
                rsass(
                    "a {b: round(-0.2)}\
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
        #[ignore] // failing
        fn within_precision() {
            assert_eq!(
        rsass(
            "// This is the smallest number that\'s representable as a float and in the\
            \n// precision range to be considered equal to 5.\
            \na {b: round(0.4999999999900001)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
        }
    }
}

// From "sass-spec/spec/core_functions/math/unit.hrx"
mod unit {
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
    fn multiple_denominators() {
        assert_eq!(
            rsass(
                "a {b: unit(1 / 1px / 3em / 4rad)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: \"/em*px*rad\";\
             \n}\
             \n"
        );
    }
    #[test]
    #[ignore] // failing
    fn multiple_numerators() {
        assert_eq!(
            rsass(
                "a {b: unit(1px * 1em * 1rad)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: \"em*px*rad\";\
             \n}\
             \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: unit($number: 1)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: \"\";\
             \n}\
             \n"
        );
    }
    #[test]
    fn none() {
        assert_eq!(
            rsass(
                "a {b: unit(1)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: \"\";\
             \n}\
             \n"
        );
    }
    mod numerator_and_denominator {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // failing
        fn multiple() {
            assert_eq!(
                rsass(
                    "a {b: unit(1px * 1em / 1rad / 1s)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: \"em*px/rad*s\";\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // failing
        fn single() {
            assert_eq!(
                rsass(
                    "a {b: unit(1px / 1em)}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: \"px/em\";\
                 \n}\
                 \n"
            );
        }
    }
    #[test]
    #[ignore] // failing
    fn one_denominator() {
        assert_eq!(
            rsass(
                "a {b: unit(1/1px)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: \"/px\";\
             \n}\
             \n"
        );
    }
    #[test]
    fn one_numerator() {
        assert_eq!(
            rsass(
                "a {b: unit(1px)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: \"px\";\
             \n}\
             \n"
        );
    }
}

// From "sass-spec/spec/core_functions/math/unitless.hrx"
mod unitless {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // failing
    fn denominator() {
        assert_eq!(
            rsass(
                "a {b: unitless(1/1px)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: false;\
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
    fn named() {
        assert_eq!(
            rsass(
                "a {b: unitless($number: 100)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
    #[test]
    fn numerator() {
        assert_eq!(
            rsass(
                "a {b: unitless(1px)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: false;\
             \n}\
             \n"
        );
    }
    #[test]
    #[ignore] // failing
    fn numerator_and_denominator() {
        assert_eq!(
            rsass(
                "a {b: unitless(1px/1em)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: false;\
             \n}\
             \n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            rsass(
                "a {b: unitless(1)}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
}
