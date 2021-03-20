//! Tests auto-converted from "sass-spec/spec/core_functions/math/atan2/y_infinite.hrx"

mod negative {
    mod with_x {
        #[test]
        fn finite() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
    mod with_x {
        #[test]
        fn finite() {
            assert_eq!(
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
                crate::rsass(
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
