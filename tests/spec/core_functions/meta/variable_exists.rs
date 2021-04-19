//! Tests auto-converted from "sass-spec/spec/core_functions/meta/variable_exists.hrx"

#[test]
#[ignore] // wrong error
fn conflict() {
    assert_eq!(
        crate::rsass(
            "@use \"other1\" as *;\
             \n@use \"other2\" as *;\
             \n\
             \na {b: variable-exists(member)}\
             \n"
        )
        .unwrap_err(),
        "Error: This variable is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes variable\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes variable\
         \n... |\
         \n4   | a {b: variable-exists(member)}\
         \n    |       ^^^^^^^^^^^^^^^^^^^^^^^ variable use\
         \n    \'\
         \n  input.scss 4:7  root stylesheet",
    );
}
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
        #[test]
        fn too_few() {
            assert_eq!(
                crate::rsass(
                    "a {b: variable-exists()}\
             \n"
                )
                .unwrap_err(),
                "Error: Missing argument $name.\
         \n  ,--> input.scss\
         \n1 | a {b: variable-exists()}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function variable-exists($name) {\
         \n  |           ====================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn too_many() {
            assert_eq!(
                crate::rsass(
                    "a {b: variable-exists(foo, bar)}\
             \n"
                )
                .unwrap_err(),
                "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: variable-exists(foo, bar)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function variable-exists($name) {\
         \n  |           ====================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn test_type() {
            assert_eq!(
                crate::rsass(
                    "a {b: variable-exists(12px)}\
             \n"
                )
                .unwrap_err(),
                "Error: $name: 12px is not a string.\
         \n  ,\
         \n1 | a {b: variable-exists(12px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
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
