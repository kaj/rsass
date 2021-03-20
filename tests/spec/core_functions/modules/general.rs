//! Tests auto-converted from "sass-spec/spec/core_functions/modules/general.hrx"

#[test]
fn test_as() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as m;\
            \na {b: m.round(0.7)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
mod error {

    // Ignoring "set_variable", error tests are not supported yet.
}
mod forward {
    #[test]
    #[ignore] // unexepected error
    fn test_as() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
            \na {b: other.s-round(0.7)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn bare() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
            \na {b: other.round(0.7)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
    mod error {

        // Ignoring "hide", error tests are not supported yet.

        // Ignoring "show", error tests are not supported yet.
    }
    #[test]
    #[ignore] // unexepected error
    fn hide() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
            \na {b: other.round(0.7)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn show() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
            \na {b: other.round(0.7)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
}
#[test]
fn global() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as *;\
            \na {b: compatible(1px, 1in)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}
