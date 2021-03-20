//! Tests auto-converted from "sass-spec/spec/core_functions/meta/variable_exists.hrx"

// Ignoring "conflict", error tests are not supported yet.
mod dash_insensitive {
    #[test]
    fn dash_to_underscore() {
        assert_eq!(
            crate::rsass(
                "$a_b: null;\
            \n\
            \nc {d: variable-exists(a-b)}\
            \n"
            )
            .unwrap(),
            "c {\
        \n  d: true;\
        \n}\
        \n"
        );
    }
    #[test]
    fn underscore_to_dash() {
        assert_eq!(
            crate::rsass(
                "$a-b: null;\
            \n\
            \nc {d: variable-exists(a_b)}\
            \n"
            )
            .unwrap(),
            "c {\
        \n  d: true;\
        \n}\
        \n"
        );
    }
}
mod error {
    mod argument {

        // Ignoring "too_few", error tests are not supported yet.

        // Ignoring "too_many", error tests are not supported yet.

        // Ignoring "test_type", error tests are not supported yet.
    }
}
#[test]
fn global() {
    assert_eq!(
        crate::rsass(
            "$global-variable: null;\
            \n\
            \na {b: variable-exists(global-variable)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}
#[test]
fn keyword() {
    assert_eq!(
        crate::rsass(
            "a {b: variable-exists($name: foo)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
}
#[test]
fn local() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  $local-variable: null;\
            \n  b: variable-exists(local-variable);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}
#[test]
fn non_existent() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: variable-exists(non-existent);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
}
mod through_forward {
    #[test]
    #[ignore] // unexepected error
    fn test_as() {
        assert_eq!(
            crate::rsass(
                "@use \"midstream\" as *;\
            \na {\
            \n  with-prefix: variable-exists(b-c);\
            \n  without-prefix: variable-exists(c);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  with-prefix: true;\
        \n  without-prefix: false;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hide() {
        assert_eq!(
            crate::rsass(
                "@use \"midstream\" as *;\
            \na {\
            \n  hidden: variable-exists(b);\
            \n  not-hidden: variable-exists(c);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  hidden: false;\
        \n  not-hidden: true;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn show() {
        assert_eq!(
            crate::rsass(
                "@use \"midstream\" as *;\
            \na {\
            \n  shown: variable-exists(b);\
            \n  not-shown: variable-exists(c);\
            \n}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  shown: true;\
        \n  not-shown: false;\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn through_import() {
    assert_eq!(
        crate::rsass(
            "@import \"other\";\
            \na {b: variable-exists(global-variable)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn through_use() {
    assert_eq!(
        crate::rsass(
            "@use \"other\" as *;\
            \na {b: variable-exists(global-variable)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}
