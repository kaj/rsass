//! Tests auto-converted from "sass-spec/spec/core_functions/color/change_color/rgb.hrx"

#[test]
fn all() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color(black, $red: 12, $green: 24, $blue: 48)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #0c1830;\
        \n}\
        \n"
    );
}
#[test]
fn alpha_arg() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color(black, $red: 12, $green: 24, $blue: 48, $alpha: 0.3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(12, 24, 48, 0.3);\
        \n}\
        \n"
    );
}
#[test]
fn alpha_input() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color(rgba(black, 0.3), $red: 12, $green: 24, $blue: 48)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(12, 24, 48, 0.3);\
        \n}\
        \n"
    );
}
mod blue {
    #[test]
    fn high() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(black, $blue: 200)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #0000c8;\
        \n}\
        \n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(blue, $blue: 100)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #000064;\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(black, $blue: 255)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: blue;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(blue, $blue: 0)}\
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
mod green {
    #[test]
    fn high() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(black, $green: 200)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #00c800;\
        \n}\
        \n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(lime, $green: 100)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: darkgreen;\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(black, $green: 255)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: lime;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(lime, $green: 0)}\
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
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: change-color($color: black, $red: 12, $green: 24, $blue: 48)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #0c1830;\
        \n}\
        \n"
    );
}
mod red {
    #[test]
    fn high() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(black, $red: 200)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #c80000;\
        \n}\
        \n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(red, $red: 100)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #640000;\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: change-color(black, $red: 255)}\
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
                "a {b: change-color(red, $red: 0)}\
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
