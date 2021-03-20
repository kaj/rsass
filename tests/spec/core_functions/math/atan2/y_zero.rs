//! Tests auto-converted from "sass-spec/spec/core_functions/math/atan2/y_zero.hrx"

mod negative {
    mod with_x {
        #[test]
        fn finite() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
    mod with_x {
        #[test]
        fn finite() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
    mod with_x {
        #[test]
        fn finite() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
    mod with_x {
        #[test]
        fn finite() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
