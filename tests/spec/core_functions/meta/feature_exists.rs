//! Tests auto-converted from "sass-spec/spec/core_functions/meta/feature_exists.hrx"

#[test]
fn at_error() {
    assert_eq!(
        crate::rsass(
            "a {b: feature-exists(at-error)}\
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
#[ignore] // wrong result
fn custom_property() {
    assert_eq!(
        crate::rsass(
            "a {b: feature-exists(custom-property)}\
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
fn dash_sensitive() {
    assert_eq!(
        crate::rsass(
            "a {b: feature-exists(at_error)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
}
mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: feature-exists()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $feature.\
         \n  ,--> input.scss\
         \n1 | a {b: feature-exists()}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function feature-exists($feature) {\
         \n  |           ======================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: feature-exists(at-error, custom-property)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: feature-exists(at-error, custom-property)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function feature-exists($feature) {\
         \n  |           ======================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: feature-exists(1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $feature: 1 is not a string.\
         \n  ,\
         \n1 | a {b: feature-exists(1)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
#[ignore] // wrong result
fn extend_selector_pseudoclass() {
    assert_eq!(
        crate::rsass(
            "a {b: feature-exists(extend-selector-pseudoclass)}\
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
fn global_variable_shadowing() {
    assert_eq!(
        crate::rsass(
            "a {b: feature-exists(global-variable-shadowing)}\
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
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: feature-exists($feature: at-error)}\
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
fn quote_insensitive() {
    assert_eq!(
        crate::rsass(
            "a {b: feature-exists(\"at-error\")}\
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
fn units_level_3() {
    assert_eq!(
        crate::rsass(
            "a {b: feature-exists(units-level-3)}\
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
fn unknown() {
    assert_eq!(
        crate::rsass(
            "a {b: feature-exists(unknown)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
}
