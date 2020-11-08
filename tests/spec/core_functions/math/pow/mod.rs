//! Tests auto-converted from "sass-spec/spec/core_functions/math/pow"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/core_functions/math/pow/arguments.hrx"
mod arguments {
    #[allow(unused)]
    use super::rsass;
    mod error {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "base_has_units", error tests are not supported yet.

        // Ignoring "base_type", error tests are not supported yet.

        // Ignoring "exponent_has_units", error tests are not supported yet.

        // Ignoring "exponent_type", error tests are not supported yet.

        // Ignoring "one_arg", error tests are not supported yet.

        // Ignoring "too_many_args", error tests are not supported yet.

        // Ignoring "zero_args", error tests are not supported yet.
    }
    #[test]
    fn named_args() {
        assert_eq!(
            rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.pow($base: 2, $exponent: 3)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 8;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/math/pow/base_greater_than_zero.hrx"
mod base_greater_than_zero {
    #[allow(unused)]
    use super::rsass;
    mod base {
        #[allow(unused)]
        use super::rsass;
        mod greater_than_one {
            #[allow(unused)]
            use super::rsass;
            mod with_exponent {
                #[allow(unused)]
                use super::rsass;
                #[test]
                fn decimal() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(2, 0.5)}\
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
                fn infinity() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(2, 1 / 0)}\
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
                fn integer() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(2, 3)}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  b: 8;\
        \n}\
        \n"
                    );
                }
                #[test]
                fn integer_fuzzy() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(2, 3.000000000001)}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  b: 8;\
        \n}\
        \n"
                    );
                }
                #[test]
                fn negative_infinity() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(2, -1 / 0)}\
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
                fn zero() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(2, 0)}\
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
                fn zero_fuzzy() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(2, 0.000000000001)}\
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
        mod less_than_one {
            #[allow(unused)]
            use super::rsass;
            mod with_exponent {
                #[allow(unused)]
                use super::rsass;
                #[test]
                fn infinity() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.5, 1 / 0)}\
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
                fn negative_infinity() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(0.5, -1 / 0)}\
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
        }
        mod one {
            #[allow(unused)]
            use super::rsass;
            mod with_exponent {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn infinity() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(1, 1 / 0)}\
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
                fn negative_infinity() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(1, -1 / 0)}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  b: NaN;\
        \n}\
        \n"
                    );
                }
            }
        }
        mod one_fuzzy {
            #[allow(unused)]
            use super::rsass;
            mod with_exponent {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn infinity() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(1.000000000001, 1 / 0)}\
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
                fn negative_infinity() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(1.000000000001, -1 / 0)}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  b: NaN;\
        \n}\
        \n"
                    );
                }
            }
        }
    }
}

// From "sass-spec/spec/core_functions/math/pow/base_less_than_zero.hrx"
mod base_less_than_zero {
    #[allow(unused)]
    use super::rsass;
    mod base {
        #[allow(unused)]
        use super::rsass;
        mod greater_than_negative_one {
            #[allow(unused)]
            use super::rsass;
            mod with_exponent {
                #[allow(unused)]
                use super::rsass;
                #[test]
                fn infinity() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.5, 1 / 0)}\
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
                fn negative_infinity() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.5, -1 / 0)}\
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
        }
        mod less_than_negative_one {
            #[allow(unused)]
            use super::rsass;
            mod with_exponent {
                #[allow(unused)]
                use super::rsass;
                #[test]
                fn decimal() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(-2, 0.5)}\
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
                fn infinity() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(-2, 1 / 0)}\
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
                fn integer() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(-2, 2)}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  b: 4;\
        \n}\
        \n"
                    );
                }
                #[test]
                fn integer_fuzzy() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(-2, 2.000000000001)}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  b: 4;\
        \n}\
        \n"
                    );
                }
                #[test]
                fn negative_infinity() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(-2, -1 / 0)}\
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
                fn zero() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(-2, 0)}\
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
                fn zero_fuzzy() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(-2, 0.000000000001)}\
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
        mod negative_one {
            #[allow(unused)]
            use super::rsass;
            mod with_exponent {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn infinity() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(-1, 1 / 0)}\
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
                fn negative_infinity() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(-1, -1 / 0)}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  b: NaN;\
        \n}\
        \n"
                    );
                }
            }
        }
        mod negative_one_fuzzy {
            #[allow(unused)]
            use super::rsass;
            mod with_exponent {
                #[allow(unused)]
                use super::rsass;
                #[test]
                #[ignore] // wrong result
                fn infinity() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(-1.000000000001, 1 / 0)}\
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
                fn negative_infinity() {
                    assert_eq!(
                        rsass(
                            "@use \"sass:math\" as math;\
            \na {b: math.pow(-1.000000000001, -1 / 0)}\
            \n"
                        )
                        .unwrap(),
                        "a {\
        \n  b: NaN;\
        \n}\
        \n"
                    );
                }
            }
        }
    }
}

// From "sass-spec/spec/core_functions/math/pow/base_negative_infinity.hrx"
mod base_negative_infinity {
    #[allow(unused)]
    use super::rsass;
    mod with_exponent {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn decimal() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-1 / 0, 2.5)}\
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
        fn even_integer() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-1 / 0, 2)}\
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
        fn even_integer_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-1 / 0, 2.000000000001)}\
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
        fn infinity() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-1 / 0, 1 / 0)}\
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
        fn negative_decimal() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-1 / 0, -2.5)}\
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
        fn negative_even_integer() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-1 / 0, -2)}\
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
        fn negative_even_integer_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-1 / 0, -2.000000000001)}\
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
        fn negative_infinity() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-1 / 0, -1 / 0)}\
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
        fn negative_odd_integer() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-1 / 0, -3)}\
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
        fn negative_odd_integer_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-1 / 0, -3.000000000001)}\
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
        fn odd_integer() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-1 / 0, 3)}\
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
        fn odd_integer_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-1 / 0, 3.000000000001)}\
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
        fn zero() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-1 / 0, 0)}\
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
        fn zero_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-1 / 0, 0.000000000001)}\
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

// From "sass-spec/spec/core_functions/math/pow/base_negative_zero.hrx"
mod base_negative_zero {
    #[allow(unused)]
    use super::rsass;
    mod fuzzy {
        #[allow(unused)]
        use super::rsass;
        mod with_exponent {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn decimal() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.000000000001, 0.5)}\
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
            fn even_integer() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.000000000001, 2)}\
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
            fn infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.000000000001, 1 / 0)}\
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
            fn negative_decimal() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.000000000001, -0.5)}\
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
            fn negative_even_integer() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.000000000001, -2)}\
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
            fn negative_infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.000000000001, -1 / 0)}\
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
            fn negative_odd_integer() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.000000000001, -3)}\
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
            fn odd_integer() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.000000000001, 3)}\
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
            fn zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.000000000001, 0)}\
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
    mod with_exponent {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn decimal() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.0, 0.5)}\
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
        fn even_integer() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.0, 2)}\
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
        fn even_integer_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.0, 2.000000000001)}\
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
        fn infinity() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.0, 1 / 0)}\
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
        fn negative_decimal() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.0, -0.5)}\
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
        fn negative_even_integer() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.0, -2)}\
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
        fn negative_even_integer_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.0, -2.000000000001)}\
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
        fn negative_infinity() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.0, -1 / 0)}\
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
        fn negative_odd_integer() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.0, -3)}\
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
        fn negative_odd_integer_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.0, -3.000000000001)}\
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
        fn odd_integer() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.0, 3)}\
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
        fn odd_integer_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.0, 3.000000000001)}\
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
        fn zero() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.0, 0)}\
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
        fn zero_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(-0.0, 0.000000000001)}\
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

// From "sass-spec/spec/core_functions/math/pow/base_positive_infinity.hrx"
mod base_positive_infinity {
    #[allow(unused)]
    use super::rsass;
    mod with_exponent {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn decimal() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(1 / 0, 0.5)}\
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
        fn even_integer() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(1 / 0, 2)}\
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
        fn even_integer_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(1 / 0, 2.000000000001)}\
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
        fn infinity() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(1 / 0, 1 / 0)}\
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
        fn negative_decimal() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(1 / 0, -0.5)}\
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
        fn negative_even_integer() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(1 / 0, -2)}\
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
        fn negative_even_integer_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(1 / 0, -2.000000000001)}\
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
        fn negative_infinity() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(1 / 0, -1 / 0)}\
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
        fn negative_odd_integer() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(1 / 0, -3)}\
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
        fn negative_odd_integer_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(1 / 0, -3.000000000001)}\
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
        fn odd_integer() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(1 / 0, 3)}\
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
        fn odd_integer_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(1 / 0, 3.000000000001)}\
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
        fn zero() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(1 / 0, 0)}\
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
        fn zero_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(1 / 0, 0.000000000001)}\
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

// From "sass-spec/spec/core_functions/math/pow/base_positive_zero.hrx"
mod base_positive_zero {
    #[allow(unused)]
    use super::rsass;
    mod fuzzy {
        #[allow(unused)]
        use super::rsass;
        mod with_exponent {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn decimal() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(0.000000000001, 0.5)}\
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
            fn even_integer() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(0.000000000001, 2)}\
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
            fn infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(0.000000000001, 1 / 0)}\
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
            fn negative_decimal() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(0.000000000001, -0.5)}\
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
            fn negative_even_integer() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(0.000000000001, -2)}\
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
            fn negative_infinity() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(0.000000000001, -1 / 0)}\
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
            fn negative_odd_integer() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(0.000000000001, -3)}\
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
            fn odd_integer() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(0.000000000001, 3)}\
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
            fn zero() {
                assert_eq!(
                    rsass(
                        "@use \"sass:math\" as math;\
            \na {b: math.pow(0.000000000001, 0)}\
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
    mod with_exponent {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn decimal() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(0, 0.5)}\
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
        fn even_integer() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(0, 2)}\
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
        fn even_integer_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(0, 2.000000000001)}\
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
        fn infinity() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(0, 1 / 0)}\
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
        fn negative_decimal() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(0, -0.5)}\
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
        fn negative_even_integer() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(0, -2)}\
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
        fn negative_even_integer_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(0, -2.000000000001)}\
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
        fn negative_infinity() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(0, -1 / 0)}\
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
        fn negative_odd_integer() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(0, -3)}\
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
        fn negative_odd_integer_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(0, -3.000000000001)}\
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
        fn odd_integer() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(0, 3)}\
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
        fn odd_integer_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(0, 3.000000000001)}\
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
        fn zero() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(0, 0)}\
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
        fn zero_fuzzy() {
            assert_eq!(
                rsass(
                    "@use \"sass:math\" as math;\
            \na {b: math.pow(0, 0.000000000001)}\
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
