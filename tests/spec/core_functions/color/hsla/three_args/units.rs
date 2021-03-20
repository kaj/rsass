//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/three_args/units.hrx"

mod hue {
    #[test]
    fn angle() {
        assert_eq!(
            crate::rsass(
                "a {b: hsla(60rad, 100%, 50%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: yellow;\
        \n}\
        \n"
        );
    }
    #[test]
    fn deg() {
        assert_eq!(
            crate::rsass(
                "a {b: hsla(0deg, 100%, 50%)}\
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
    fn unitless() {
        assert_eq!(
            crate::rsass(
                "a {b: hsla(60, 100%, 50%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: yellow;\
        \n}\
        \n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            crate::rsass(
                "a {b: hsla(60in, 100%, 50%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: yellow;\
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
                "a {b: hsla(0, 100%, 50)}\
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
    fn unknown() {
        assert_eq!(
            crate::rsass(
                "a {b: hsla(0, 100%, 50in)}\
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
                "a {b: hsla(0, 50, 50%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #bf4040;\
        \n}\
        \n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            crate::rsass(
                "a {b: hsla(0, 50in, 50%)}\
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
