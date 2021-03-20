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

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
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
