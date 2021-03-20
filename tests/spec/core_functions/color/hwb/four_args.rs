//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/four_args.hrx"

mod alpha {
    mod percent {
        #[test]
        fn above_max() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb(0, 0%, 0%, 250%)}\
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
        fn max() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb(0, 0%, 0%, 100%)}\
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
                crate::rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb(0, 0%, 0%, 0%)}\
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
        fn negative() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb(0, 0%, 0%, -10%)}\
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
        fn positive() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb(0, 0%, 0%, 45.6%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(255, 0, 0, 0.456);\
        \n}\
        \n"
            );
        }
    }
    mod unitless {
        #[test]
        fn above_max() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb(0, 0%, 0%, 250)}\
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
        fn max() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb(0, 0%, 0%, 1)}\
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
                crate::rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb(0, 0%, 0%, 0)}\
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
        fn negative() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb(0, 0%, 0%, -10)}\
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
        fn positive() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb(0, 0%, 0%, 0.456)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(255, 0, 0, 0.456);\
        \n}\
        \n"
            );
        }
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
            \na {b: color.hwb($hue: 180, $whiteness: 30%, $blackness: 40%, $alpha: 0.4)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(77, 153, 153, 0.4);\
        \n}\
        \n"
    );
}
