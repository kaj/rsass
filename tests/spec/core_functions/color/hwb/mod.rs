//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb"
#[allow(unused)]
use super::rsass;

mod error;

// From "sass-spec/spec/core_functions/color/hwb/four_args.hrx"
mod four_args {
    #[allow(unused)]
    use super::rsass;
    mod alpha {
        #[allow(unused)]
        use super::rsass;
        mod percent {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn above_max() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
                    rsass(
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
                    rsass(
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
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn above_max() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
                    rsass(
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
                    rsass(
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
                    rsass(
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
        rsass(
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
}

// From "sass-spec/spec/core_functions/color/hwb/one_arg.hrx"
mod one_arg {
    #[allow(unused)]
    use super::rsass;
    mod alpha {
        #[allow(unused)]
        use super::rsass;
        mod clamped {
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn above() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
            #[allow(unused)]
            use super::rsass;
            #[test]
            fn named() {
                assert_eq!(
                    rsass(
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
                    rsass(
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
        rsass(
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
                    rsass(
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
                    rsass(
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
            rsass(
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
            rsass(
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
}

mod three_args;
