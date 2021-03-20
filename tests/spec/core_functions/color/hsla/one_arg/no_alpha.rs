//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/one_arg/no_alpha.hrx"

mod clamped {
    mod lightness {
        #[test]
        fn above() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsla(0 100% 500%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: white;\
        \n}\
        \n"
            );
        }
        #[test]
        fn below() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsla(0 100% -100%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: black;\
        \n}\
        \n"
            );
        }
    }
    mod saturation {
        #[test]
        fn above() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsla(0 500% 50%)}\
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
                    "a {b: hsla(0 -100% 50%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: gray;\
        \n}\
        \n"
            );
        }
    }
}
mod in_gamut {
    #[test]
    fn blue() {
        assert_eq!(
            crate::rsass(
                "a {b: hsla(240 100% 50%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: blue;\
        \n}\
        \n"
        );
    }
    mod grayish {
        #[test]
        fn yellow() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsla(60 60% 50%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #cccc33;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn red() {
        assert_eq!(
            crate::rsass(
                "a {b: hsla(0 100% 50%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: red;\
        \n}\
        \n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: hsla($channels: 0 100% 50%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: red;\
        \n}\
        \n"
    );
}
mod units {
    mod hue {
        #[test]
        fn deg() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsla(0deg 100% 50%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: red;\
        \n}\
        \n"
            );
        }
    }
    mod lightness {
        #[test]
        fn unitless() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsla(0 100% 50)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: red;\
        \n}\
        \n"
            );
        }
    }
    mod saturation {
        #[test]
        fn unitless() {
            assert_eq!(
                crate::rsass(
                    "a {b: hsla(0 50 50%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #bf4040;\
        \n}\
        \n"
            );
        }
    }
}
