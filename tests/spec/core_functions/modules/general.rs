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
    #[test]
    #[ignore] // wrong error
    fn set_variable() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\";\
             \nmath.$a: b;\
             \n"
            )
            .unwrap_err(),
            "Error: Undefined variable.\
         \n  ,\
         \n2 | math.$a: b;\
         \n  | ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet",
        );
    }
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
        #[test]
        #[ignore] // wrong error
        fn hide() {
            assert_eq!(
                crate::rsass(
                    "@use \"other\";\
             \na {b: other.round(0.7)}\
             \n"
                )
                .unwrap_err(),
                "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: other.round(0.7)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn show() {
            assert_eq!(
                crate::rsass(
                    "@use \"other\";\
             \na {b: other.round(0.7)}\
             \n"
                )
                .unwrap_err(),
                "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: other.round(0.7)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
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
