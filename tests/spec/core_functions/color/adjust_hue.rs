//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_hue.hrx"

#[test]
fn above_max() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-hue(red, 540)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: aqua;\
        \n}\
        \n"
    );
}
#[test]
fn alpha() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-hue(rgba(red, 0.1), 359)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(255, 0, 4, 0.1);\
        \n}\
        \n"
    );
}
mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
    mod test_type {

        // Ignoring "color", error tests are not supported yet.

        // Ignoring "hue", error tests are not supported yet.
    }
}
#[test]
fn fraction() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-hue(red, 0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #ff0200;\
        \n}\
        \n"
    );
}
#[test]
fn max() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-hue(red, 359)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #ff0004;\
        \n}\
        \n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-hue(red, 123)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #00ff0d;\
        \n}\
        \n"
    );
}
#[test]
fn min() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-hue(blue, 0)}\
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
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-hue($color: red, $degrees: 123)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #00ff0d;\
        \n}\
        \n"
    );
}
#[test]
fn negative() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-hue(red, -180)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: aqua;\
        \n}\
        \n"
    );
}
mod units {
    #[test]
    fn angle() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-hue(red, 60rad)}\
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
                "a {b: adjust-hue(red, 60deg)}\
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
                "a {b: adjust-hue(red, 60)}\
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
                "a {b: adjust-hue(red, 60in)}\
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
