//! Tests auto-converted from "sass-spec/spec/core_functions/math/atan2"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/core_functions/math/atan2/arguments.hrx"
mod arguments {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn compatible_units() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.atan2(1cm, -10mm)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 135deg;\
        \n}\
        \n"
        );
    }
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "incompatible_units", error tests are not supported yet.

        // Ignoring "one_arg", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "unitless_x", error tests are not supported yet.

        // Ignoring "unitless_y", error tests are not supported yet.

        // Ignoring "x_type", error tests are not supported yet.

        // Ignoring "y_type", error tests are not supported yet.

        // Ignoring "zero_args", error tests are not supported yet.
    }
    #[test]
    fn named_args() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.atan2($y: 1, $x: -1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 135deg;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/math/atan2/y_infinite.hrx"
mod y_infinite {
    #[allow(unused)]
    use super::rsass;
    mod negative {
        #[allow(unused)]
        use super::rsass;
        mod with_x {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn finite() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-1 / 0, 1)}\
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
            fn infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-1 / 0, 1 / 0)}\
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
            fn negative_finite() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-1 / 0, -1)}\
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
            fn negative_infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-1 / 0, -1 / 0)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: -135deg;\
        \n}\
        \n"
                );
            }
            #[test]
            fn negative_zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-1 / 0, -0.0)}\
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
            fn zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-1 / 0, 0)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: -90deg;\
        \n}\
        \n"
                );
            }
        }
    }
    mod positive {
        #[allow(unused)]
        use super::rsass;
        mod with_x {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn finite() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(1 / 0, 1)}\
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
            fn infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(1 / 0, 1 / 0)}\
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
            fn negative_finite() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(1 / 0, -1)}\
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
            fn negative_infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(1 / 0, -1 / 0)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: 135deg;\
        \n}\
        \n"
                );
            }
            #[test]
            fn negative_zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(1 / 0, -0.0)}\
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
            fn negative_zero_fuzzy() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(1 / 0, -0.000000000001)}\
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
            fn zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(1 / 0, 0)}\
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
            fn zero_fuzzy() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(1 / 0, 0.000000000001)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: 90deg;\
        \n}\
        \n"
                );
            }
        }
    }
}

// From "sass-spec/spec/core_functions/math/atan2/y_non_zero_finite.hrx"
mod y_non_zero_finite {
    #[allow(unused)]
    use super::rsass;
    mod negative {
        #[allow(unused)]
        use super::rsass;
        mod with_x {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn finite() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-1, 1)}\
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
            fn infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-1, 1 / 0)}\
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
            fn negative_finite() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-1, -1)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: -135deg;\
        \n}\
        \n"
                );
            }
            #[test]
            fn negative_infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-1, -1 / 0)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: -180deg;\
        \n}\
        \n"
                );
            }
            #[test]
            fn negative_zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-1, -0.0)}\
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
            fn negative_zero_fuzzy() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-1, -0.000000000001)}\
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
            fn zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-1, 0)}\
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
            fn zero_fuzzy() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-1, 0.000000000001)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: -90deg;\
        \n}\
        \n"
                );
            }
        }
    }
    mod positive {
        #[allow(unused)]
        use super::rsass;
        mod with_x {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn finite() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(1, 1)}\
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
            fn infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(1, 1 / 0)}\
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
            fn negative_finite() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(1, -1)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: 135deg;\
        \n}\
        \n"
                );
            }
            #[test]
            fn negative_infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(1, -1 / 0)}\
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
            fn negative_zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(1, -0.0)}\
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
            fn negative_zero_fuzzy() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(1, -0.000000000001)}\
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
            fn zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(1, 0)}\
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
            fn zero_fuzzy() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(1, 0.000000000001)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: 90deg;\
        \n}\
        \n"
                );
            }
        }
    }
}

// From "sass-spec/spec/core_functions/math/atan2/y_zero.hrx"
mod y_zero {
    #[allow(unused)]
    use super::rsass;
    mod negative {
        #[allow(unused)]
        use super::rsass;
        mod with_x {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn finite() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-0.0, 1)}\
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
            fn infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-0.0, 1 / 0)}\
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
            fn negative_finite() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-0.0, -1)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: -180deg;\
        \n}\
        \n"
                );
            }
            #[test]
            fn negative_infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-0.0, -1 / 0)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: -180deg;\
        \n}\
        \n"
                );
            }
            #[test]
            fn negative_zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-0.0, -0.0)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: -180deg;\
        \n}\
        \n"
                );
            }
            #[test]
            fn negative_zero_fuzzy() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-0.0, -0.000000000001)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: -180deg;\
        \n}\
        \n"
                );
            }
            #[test]
            fn zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-0.0, 0)}\
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
            fn zero_fuzzy() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-0.0, 0.000000000001)}\
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
    }
    mod negative_fuzzy {
        #[allow(unused)]
        use super::rsass;
        mod with_x {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn finite() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-0.000000000001, 1)}\
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
            fn infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-0.000000000001, 1 / 0)}\
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
            fn negative_finite() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-0.000000000001, -1)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: -180deg;\
        \n}\
        \n"
                );
            }
            #[test]
            fn negative_infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-0.000000000001, -1 / 0)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: -180deg;\
        \n}\
        \n"
                );
            }
            #[test]
            fn negative_zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-0.000000000001, -0.0)}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: -180deg;\
        \n}\
        \n"
                );
            }
            #[test]
            fn zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(-0.000000000001, 0)}\
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
    }
    mod positive {
        #[allow(unused)]
        use super::rsass;
        mod with_x {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn finite() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(0, 1)}\
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
            fn infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(0, 1 / 0)}\
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
            fn negative_finite() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(0, -1)}\
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
            fn negative_infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(0, -1 / 0)}\
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
            fn negative_zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(0, -0.0)}\
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
            fn negative_zero_fuzzy() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(0, -0.000000000001)}\
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
            fn zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(0, 0)}\
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
            fn zero_fuzzy() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(0, 0.000000000001)}\
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
    }
    mod positive_fuzzy {
        #[allow(unused)]
        use super::rsass;
        mod with_x {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn finite() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(0.000000000001, 1)}\
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
            fn infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(0.000000000001, 1 / 0)}\
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
            fn negative_finite() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(0.000000000001, -1)}\
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
            fn negative_infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(0.000000000001, -1 / 0)}\
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
            fn negative_zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(0.000000000001, -0.0)}\
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
            fn zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.atan2(0.000000000001, 0)}\
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
    }
}
