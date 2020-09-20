//! Tests auto-converted from "sass-spec/spec/core_functions/math"
#[allow(unused)]
use super::rsass;

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
    #[ignore] // unexepected error
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

// From "sass-spec/spec/core_functions/math/acos.hrx"
mod acos {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn decimal() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.acos(0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 60deg;\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.

        // Ignoring "units", error tests are not supported yet.

        // Ignoring "zero_args", error tests are not supported yet.
    }
    #[test]
    #[ignore] // wrong result
    fn greater_than_one() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.acos(2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: NaNdeg;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn less_than_negative_one() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.acos(-2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: NaNdeg;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn negative_decimal() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.acos(-0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 120deg;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn one() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.acos(1)}\
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
    fn one_fuzzy() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.acos(1.000000000001)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0deg;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/math/asin.hrx"
mod asin {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn decimal() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.asin(0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 30deg;\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.

        // Ignoring "units", error tests are not supported yet.

        // Ignoring "zero_args", error tests are not supported yet.
    }
    #[test]
    #[ignore] // wrong result
    fn greater_than_one() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.asin(2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: NaNdeg;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn less_than_negative_one() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.asin(-2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: NaNdeg;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn negative_decimal() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.asin(-0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: -30deg;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn negative_zero() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.asin(-0.0)}\
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
    fn negative_zero_fuzzy() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.asin(-0.000000000001)}\
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
    fn one() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.asin(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 90deg;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn one_fuzzy() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.asin(1.000000000001)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 90deg;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn zero() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.asin(0)}\
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
    fn zero_fuzzy() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.asin(0.000000000001)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0deg;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/math/atan.hrx"
mod atan {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.

        // Ignoring "units", error tests are not supported yet.

        // Ignoring "zero_args", error tests are not supported yet.
    }
    #[test]
    #[ignore] // wrong result
    fn infinity() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.atan(1 / 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 90deg;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn negative() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.atan(-1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: -45deg;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn negative_infinity() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.atan(-1 / 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: -90deg;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn negative_zero() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.atan(-0.0)}\
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
    fn negative_zero_fuzzy() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.atan(-0.000000000001)}\
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
    fn positive() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.atan(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 45deg;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn zero() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.atan(0)}\
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
    fn zero_fuzzy() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.atan(0.000000000001)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0deg;\
        \n}\
        \n"
        );
    }
}

mod atan2;

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
    #[ignore] // unexepected error
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

// From "sass-spec/spec/core_functions/math/clamp.hrx"
mod clamp {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn chooses_max() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.clamp(0, 2, 1)}\
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
    #[ignore] // wrong result
    fn chooses_min() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.clamp(1, 0, 2)}\
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
    #[ignore] // wrong result
    fn chooses_number() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.clamp(0, 1, 2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod incompatible_units {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "all", error tests are not supported yet.

            // Ignoring "min_and_max", error tests are not supported yet.

            // Ignoring "min_and_number", error tests are not supported yet.

            // Ignoring "number_and_max", error tests are not supported yet.
        }

        // Ignoring "one_arg", error tests are not supported yet.
        mod some_unitless {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "max", error tests are not supported yet.

            // Ignoring "min", error tests are not supported yet.

            // Ignoring "min_and_max", error tests are not supported yet.

            // Ignoring "min_and_number", error tests are not supported yet.

            // Ignoring "number", error tests are not supported yet.

            // Ignoring "number_and_max", error tests are not supported yet.
        }

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "two_args", error tests are not supported yet.
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "max", error tests are not supported yet.

            // Ignoring "min", error tests are not supported yet.

            // Ignoring "number", error tests are not supported yet.
        }

        // Ignoring "zero_args", error tests are not supported yet.
    }
    #[test]
    #[ignore] // wrong result
    fn min_equals_max() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {\
            \n  b: math.clamp(1, 2, 1);\
            \n}\
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
    #[ignore] // wrong result
    fn min_greater_than_max() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {\
            \n  b: math.clamp(1, 2, 0);\
            \n}\
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
    #[ignore] // wrong result
    fn named_args() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.clamp($min: 0, $number: 1, $max: 2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
    mod preserves_units {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn max() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.clamp(180deg, 1turn, 360deg)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 360deg;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn min() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.clamp(180deg, 0.5turn, 360deg)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 180deg;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn number() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.clamp(180deg, 0.75turn, 360deg)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 0.75turn;\
        \n}\
        \n"
            );
        }
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
        #[ignore] // wrong result
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
        #[ignore] // unexepected error
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
        #[ignore] // wrong result
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

// From "sass-spec/spec/core_functions/math/cos.hrx"
mod cos {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn deg() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.cos(1deg)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.9998476952;\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.

        // Ignoring "unit", error tests are not supported yet.

        // Ignoring "zero_args", error tests are not supported yet.
    }
    #[test]
    #[ignore] // wrong result
    fn grad() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.cos(1grad)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.9998766325;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn infinity() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.cos(1 / 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: NaN;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn named_arg() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.cos($number: 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.5403023059;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn negative_infinity() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.cos(-1 / 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: NaN;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn rad() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.cos(1rad)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.5403023059;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn turn() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.cos(1turn)}\
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
    #[ignore] // wrong result
    fn unitless() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.cos(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.5403023059;\
        \n}\
        \n"
        );
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
    #[ignore] // unexepected error
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

// From "sass-spec/spec/core_functions/math/hypot.hrx"
mod hypot {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn compatible_units() {
        assert_eq!(
        rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.hypot(3cm, 4mm * 10, 5q * 40, 6in / 2.54, 7px * 96 / 2.54)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 11.6189500386cm;\
        \n}\
        \n"
    );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod incompatible_units {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "all", error tests are not supported yet.

            // Ignoring "first_and_second", error tests are not supported yet.

            // Ignoring "first_and_third", error tests are not supported yet.

            // Ignoring "second_and_third", error tests are not supported yet.
        }
        mod some_unitless {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "first", error tests are not supported yet.

            // Ignoring "first_and_second", error tests are not supported yet.

            // Ignoring "first_and_third", error tests are not supported yet.

            // Ignoring "second", error tests are not supported yet.

            // Ignoring "second_and_third", error tests are not supported yet.

            // Ignoring "third", error tests are not supported yet.
        }
        mod test_type {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "first", error tests are not supported yet.

            // Ignoring "second", error tests are not supported yet.

            // Ignoring "third", error tests are not supported yet.
        }

        // Ignoring "zero_args", error tests are not supported yet.
    }
    mod infinity {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn first() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.hypot(1/0, 1, 1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: Infinity;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn second() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.hypot(1, 1/0, 1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: Infinity;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn third() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.hypot(1, 1, 1/0)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: Infinity;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn unitless() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.hypot(3, 4, 5, 6, 7)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 11.6189500386;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/math/log.hrx"
mod log {
    #[allow(unused)]
    use super::rsass;
    mod base {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn between_zero_and_one() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.log(2, 0.5)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: -1;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn negative() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.log(2, -1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: NaN;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn null() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.log(2, null)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 0.6931471806;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn one() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.log(2, 1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: Infinity;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn one_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.log(2, 1.000000000001)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: Infinity;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn positive() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.log(2, 10)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 0.3010299957;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn zero() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.log(2, 0)}\
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
        #[ignore] // wrong result
        fn zero_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.log(2, 0.000000000001)}\
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
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "base_has_units", error tests are not supported yet.

        // Ignoring "number_has_units", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.

        // Ignoring "zero_args", error tests are not supported yet.
    }
    #[test]
    #[ignore] // wrong result
    fn infinity() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(1 / 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: Infinity;\
        \n}\
        \n"
        );
    }
    mod named_arg {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn number() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.log($number: 2)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 0.6931471806;\
        \n}\
        \n"
            );
        }
    }
    mod named_args {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn number_with_base() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.log($number: 2, $base: 10)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: 0.3010299957;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn negative() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(-1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: NaN;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn positive() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.6931471806;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn zero() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: -Infinity;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn zero_fuzzy() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(0.000000000001)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: -Infinity;\
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

mod pow;

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
    #[ignore] // unexepected error
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
    #[ignore] // unexepected error
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
    #[ignore] // unexepected error
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
    #[ignore] // unexepected error
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
    #[ignore] // unexepected error
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
        #[ignore] // wrong result
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

// From "sass-spec/spec/core_functions/math/sin.hrx"
mod sin {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn deg() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sin(1deg)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.0174524064;\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.

        // Ignoring "unit", error tests are not supported yet.

        // Ignoring "zero_args", error tests are not supported yet.
    }
    #[test]
    #[ignore] // wrong result
    fn grad() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sin(1grad)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.0157073173;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn infinity() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sin(1 / 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: NaN;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn named_arg() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sin($number: 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.8414709848;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn negative_infinity() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sin(-1 / 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: NaN;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn negative_zero() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sin(-0.0)}\
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
    #[ignore] // wrong result
    fn negative_zero_fuzzy() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sin(-0.000000000001)}\
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
    #[ignore] // wrong result
    fn rad() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sin(1rad)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.8414709848;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn turn() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sin(1turn)}\
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
    #[ignore] // wrong result
    fn unitless() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sin(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.8414709848;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn zero() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sin(0)}\
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
    #[ignore] // wrong result
    fn zero_fuzzy() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sin(0.000000000001)}\
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

// From "sass-spec/spec/core_functions/math/sqrt.hrx"
mod sqrt {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.

        // Ignoring "units", error tests are not supported yet.

        // Ignoring "zero_args", error tests are not supported yet.
    }
    #[test]
    #[ignore] // wrong result
    fn infinity() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sqrt(1 / 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: Infinity;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn named_arg() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sqrt($number: 2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1.4142135624;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn negative() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sqrt(-1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: NaN;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn negative_zero() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sqrt(-0.0)}\
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
    #[ignore] // wrong result
    fn negative_zero_fuzzy() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sqrt(-0.000000000001)}\
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
    #[ignore] // wrong result
    fn positive() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sqrt(2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1.4142135624;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn zero() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sqrt(0)}\
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
    #[ignore] // wrong result
    fn zero_fuzzy() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.sqrt(0.000000000001)}\
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

// From "sass-spec/spec/core_functions/math/tan.hrx"
mod tan {
    #[allow(unused)]
    use super::rsass;
    mod asymptote {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn radian() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.tan(0.5rad * math.$pi)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: Infinity;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn deg() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.tan(1deg)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.0174550649;\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.

        // Ignoring "unit", error tests are not supported yet.

        // Ignoring "zero_args", error tests are not supported yet.
    }
    #[test]
    #[ignore] // wrong result
    fn grad() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.tan(1grad)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.0157092553;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn infinity() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.tan(1 / 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: NaN;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn named_arg() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.tan($number: 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1.5574077247;\
        \n}\
        \n"
        );
    }
    mod negative_asymptote {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn radian() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.tan(-0.5rad * math.$pi)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: -Infinity;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn negative_infinity() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.tan(-1 / 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: NaN;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn negative_zero() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.tan(-0.0)}\
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
    #[ignore] // wrong result
    fn negative_zero_fuzzy() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.tan(-0.000000000001)}\
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
    #[ignore] // wrong result
    fn rad() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.tan(1rad)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1.5574077247;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn turn() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.tan(1turn)}\
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
    #[ignore] // wrong result
    fn unitless() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.tan(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1.5574077247;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn zero() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.tan(0)}\
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
    #[ignore] // wrong result
    fn zero_fuzzy() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.tan(0.000000000001)}\
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
    #[ignore] // wrong result
    fn multiple_denominators() {
        assert_eq!(
            rsass(
                "a {b: unit(1 / 1px / 3em / 4rad)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"(px*em*rad)^-1\";\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn multiple_numerators() {
        assert_eq!(
            rsass(
                "a {b: unit(1px * 1em * 1rad)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"px*em*rad\";\
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
        #[ignore] // wrong result
        fn multiple() {
            assert_eq!(
                rsass(
                    "a {b: unit(1px * 1em / 1rad / 1s)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: \"px*em/rad*s\";\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
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
    #[ignore] // wrong result
    fn one_denominator() {
        assert_eq!(
            rsass(
                "a {b: unit(1/1px)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"px^-1\";\
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
    #[ignore] // unexepected error
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
    #[ignore] // unexepected error
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

// From "sass-spec/spec/core_functions/math/variables.hrx"
mod variables {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // unexepected error
    fn e() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.$e}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 2.7182818285;\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;
        mod assignment {
            #[allow(unused)]
            use super::rsass;

            // Ignoring "e", error tests are not supported yet.

            // Ignoring "pi", error tests are not supported yet.
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn pi() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.$pi}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 3.1415926536;\
        \n}\
        \n"
        );
    }
}
