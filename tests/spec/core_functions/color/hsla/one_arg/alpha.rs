//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/one_arg/alpha.hrx"

mod clamped {
    mod alpha {
        #[test]
        fn above() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsla(0 100% 50% / 1.1)}\
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
        fn below() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsla(0 100% 50% / -0.1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(255, 0, 0, 0);\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn lightness() {
        assert_eq!(
            crate::rsass(
                "a {b: hsla(0 100% 9999% / 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 255, 255, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn saturation() {
        assert_eq!(
            crate::rsass(
                "a {b: hsla(0 -0.1% 50% / 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(128, 128, 128, 0.5);\
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
                "a {b: hsla($channels: 180 60% 50% / 0.4)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(51, 204, 204, 0.4);\
        \n}\
        \n"
        );
    }
    #[test]
    fn opaque() {
        assert_eq!(
            crate::rsass(
                "a {b: hsla(180 60% 50% / 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #33cccc;\
        \n}\
        \n"
        );
    }
    #[test]
    fn parenthesized() {
        assert_eq!(
        crate::rsass(
            "// Extra parens shouldn\'t cause the slash to be forced into division.\
            \na {b: hsl(180 60% 50% / 0.4)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(51, 204, 204, 0.4);\
        \n}\
        \n"
    );
    }
    #[test]
    fn partial() {
        assert_eq!(
            crate::rsass(
                "a {b: hsla(180 60% 50% / 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(51, 204, 204, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn transparent() {
        assert_eq!(
            crate::rsass(
                "a {b: hsla(180 60% 50% / 0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(51, 204, 204, 0);\
        \n}\
        \n"
        );
    }
}
