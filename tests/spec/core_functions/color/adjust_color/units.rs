//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_color/units.hrx"

mod hue {
    #[test]
    fn angle() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(red, $hue: 60rad)}\
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
                "a {b: adjust-color(red, $hue: 60deg)}\
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
    fn unitless() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(red, $hue: 60)}\
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
                "a {b: adjust-color(red, $hue: 60in)}\
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
    fn percent() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(red, $lightness: 10%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #ff3333;\
        \n}\
        \n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(red, $lightness: 10)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #ff3333;\
        \n}\
        \n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(red, $lightness: 10in)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #ff3333;\
        \n}\
        \n"
        );
    }
}
mod saturation {
    #[test]
    fn percent() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(red, $saturation: -10%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #f20d0d;\
        \n}\
        \n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(red, $saturation: -10)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #f20d0d;\
        \n}\
        \n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-color(red, $saturation: -10in)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #f20d0d;\
        \n}\
        \n"
        );
    }
}
