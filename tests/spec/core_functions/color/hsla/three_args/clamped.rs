//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/three_args/clamped.hrx"

mod lightness {
    #[test]
    fn above() {
        assert_eq!(
            crate::rsass(
                "a {b: hsla(0, 100%, 500%)}\
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
                "a {b: hsla(0, 100%, -100%)}\
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
                "a {b: hsla(0, 500%, 50%)}\
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
                "a {b: hsla(0, -100%, 50%)}\
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
