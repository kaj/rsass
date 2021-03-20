//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/one_arg/alpha.hrx"

mod clamped {
    mod alpha {
        #[test]
        fn above() {
            assert_eq!(
                crate::rsass(
                    "a {b: rgba(0 0 0 / 1.1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: black;\
        \n}\
        \n"
            );
        }
        #[test]
        fn below() {
            assert_eq!(
                crate::rsass(
                    "a {b: rgba(0 0 0 / -0.1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(0, 0, 0, 0);\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn blue() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(0 0 9999 / 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 0, 255, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn green() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(0 -1 0 / 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 0, 0, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn red() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(256 0 0 / 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 0, 0, 0.5);\
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
                "a {b: rgba($channels: 0 255 127 / 0.3)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 255, 127, 0.3);\
        \n}\
        \n"
        );
    }
    #[test]
    fn opaque() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(190 173 237 / 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #beaded;\
        \n}\
        \n"
        );
    }
    #[test]
    fn parenthesized() {
        assert_eq!(
        crate::rsass(
            "// Extra parens shouldn\'t cause the slash to be forced into division.\
            \na {b: rgba(0 255 127 / 0.3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(0, 255, 127, 0.3);\
        \n}\
        \n"
    );
    }
    #[test]
    fn partial() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(18 52 86 / 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(18, 52, 86, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn percent() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(18 52 86 / 50%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(18, 52, 86, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            crate::rsass(
                "a {b: rgba(0 255 127 / 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(0, 255, 127, 0);\
        \n}\
        \n"
        );
    }
}
