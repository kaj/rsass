//! Tests auto-converted from "sass-spec/spec/core_functions/meta/feature_exists.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn at_error() {
    assert_eq!(
        runner().ok("a {b: feature-exists(at-error)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn custom_property() {
    assert_eq!(
        runner().ok("a {b: feature-exists(custom-property)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn dash_sensitive() {
    assert_eq!(
        runner().ok("a {b: feature-exists(at_error)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: feature-exists()}\n"),
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
            runner()
                .err("a {b: feature-exists(at-error, custom-property)}\n"),
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
            runner().err("a {b: feature-exists(1)}\n"),
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
        runner().ok("a {b: feature-exists(extend-selector-pseudoclass)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn global_variable_shadowing() {
    assert_eq!(
        runner().ok("a {b: feature-exists(global-variable-shadowing)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: feature-exists($feature: at-error)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn quote_insensitive() {
    assert_eq!(
        runner().ok("a {b: feature-exists(\"at-error\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn units_level_3() {
    assert_eq!(
        runner().ok("a {b: feature-exists(units-level-3)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn unknown() {
    assert_eq!(
        runner().ok("a {b: feature-exists(unknown)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
