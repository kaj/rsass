//! Tests auto-converted from "sass-spec/spec/core_functions/math/pow/base_greater_than_zero.hrx"

mod base {
    mod greater_than_one {
        mod with_exponent {
            #[test]
            fn decimal() {
                assert_eq!(
                    crate::rsass(
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
                    crate::rsass(
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
                    crate::rsass(
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
                    crate::rsass(
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
                    crate::rsass(
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
                    crate::rsass(
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
                    crate::rsass(
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
        mod with_exponent {
            #[test]
            fn infinity() {
                assert_eq!(
                    crate::rsass(
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
                    crate::rsass(
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
        mod with_exponent {
            #[test]
            fn infinity() {
                assert_eq!(
                    crate::rsass(
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
            fn negative_infinity() {
                assert_eq!(
                    crate::rsass(
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
        mod with_exponent {
            #[test]
            fn infinity() {
                assert_eq!(
                    crate::rsass(
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
            fn negative_infinity() {
                assert_eq!(
                    crate::rsass(
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
