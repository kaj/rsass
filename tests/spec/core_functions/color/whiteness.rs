//! Tests auto-converted from "sass-spec/spec/core_functions/color/whiteness.hrx"

mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
#[ignore] // wrong result
fn fraction() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
            \na {b: color.whiteness(color.hwb(0, 0.5%, 0%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.3921568627%;\
        \n}\
        \n"
    );
}
#[test]
fn max() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
            \na {b: color.whiteness(white)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 100%;\
        \n}\
        \n"
    );
}
mod middle {
    #[test]
    #[ignore] // wrong result
    fn half_blackness() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
            \na {b: color.whiteness(color.hwb(0, 50%, 50%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 50.1960784314%;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn high_blackness() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
            \na {b: color.whiteness(color.hwb(0, 70%, 70%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 50.1960784314%;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn zero_blackness() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
            \na {b: color.whiteness(color.hwb(0, 50%, 0%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 50.1960784314%;\
        \n}\
        \n"
        );
    }
}
#[test]
fn min() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
            \na {b: color.whiteness(black)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0%;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn named() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
            \na {b: color.whiteness($color: color.hwb(0, 42%, 0%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 41.9607843137%;\
        \n}\
        \n"
    );
}
