//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/one_arg.hrx"

mod alpha {
    mod clamped {
        #[test]
        fn above() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb(0 30% 40% / 1.1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #994d4d;\
        \n}\
        \n"
            );
        }
        #[test]
        fn below() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb(0 30% 40% / -0.1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(153, 77, 77, 0);\
        \n}\
        \n"
            );
        }
    }
    mod in_gamut {
        #[test]
        fn named() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb($channels: 180 30% 40% / 0.4)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(77, 153, 153, 0.4);\
        \n}\
        \n"
            );
        }
        #[test]
        fn opaque() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb(180 30% 40% / 1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #4d9999;\
        \n}\
        \n"
            );
        }
        #[test]
        fn parenthesized() {
            assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
            \n\
            \n// Extra parens shouldn\'t cause the slash to be forced into division.\
            \na {b: (color.hwb(180 30% 40% / 0.4))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(77, 153, 153, 0.4);\
        \n}\
        \n"
    );
        }
        #[test]
        fn partial() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb(180 30% 40% / 0.5)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(77, 153, 153, 0.5);\
        \n}\
        \n"
            );
        }
        #[test]
        fn transparent() {
            assert_eq!(
                crate::rsass(
                    "@use \'sass:color\';\
            \na {b: color.hwb(180 30% 40% / 0)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(77, 153, 153, 0);\
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
            \na {b: color.hwb($channels: 180 30% 40% / 0.4)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(77, 153, 153, 0.4);\
        \n}\
        \n"
    );
}
#[test]
fn no_alpha() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
            \na {b: color.hwb(180 30% 40%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #4d9999;\
        \n}\
        \n"
    );
}
