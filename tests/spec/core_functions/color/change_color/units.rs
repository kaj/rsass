//! Tests auto-converted from "sass-spec/spec/core_functions/color/change_color/units.hrx"

mod hue {
    #[test]
    fn angle() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $hue: 60rad)}\
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
                "a {b: change-color(red, $hue: 60deg)}\
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
                "a {b: change-color(red, $hue: 60)}\
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
                "a {b: change-color(red, $hue: 60in)}\
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
                "a {b: change-color(red, $lightness: 30%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #990000;\
        \n}\
        \n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $lightness: 30)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #990000;\
        \n}\
        \n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $lightness: 30in)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #990000;\
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
                "a {b: change-color(red, $saturation: 50%)}\
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
    fn unitless() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $saturation: 50)}\
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
                "a {b: change-color(red, $saturation: 50in)}\
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
