//! Tests auto-converted from "sass-spec/spec/core_functions/meta/feature_exists.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("feature_exists")
}

#[test]
fn at_error() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.feature-exists(at-error)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn custom_property() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.feature-exists(custom-property)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn dash_sensitive() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.feature-exists(at_error)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
mod error {
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \na {b: meta.feature-exists()}\n"
            ),
            "Error: Missing argument $feature.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.feature-exists()}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function feature-exists($feature) {\
         \n  |           ======================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \na {b: meta.feature-exists(at-error, custom-property)}\n"
        ),
        "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.feature-exists(at-error, custom-property)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function feature-exists($feature) {\
         \n  |           ======================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \na {b: meta.feature-exists(1)}\n"
        ),
        "DEPRECATION WARNING [feature-exists]: The feature-exists() function is deprecated.\n\
         \nMore info: https://sass-lang.com/d/feature-exists\n\
         \n  ,\
         \n2 | a {b: meta.feature-exists(1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 2:7  root stylesheet\n\
         \nError: $feature: 1 is not a string.\
         \n  ,\
         \n2 | a {b: meta.feature-exists(1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
}
#[test]
#[ignore] // wrong result
fn extend_selector_pseudoclass() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.feature-exists(extend-selector-pseudoclass)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn global_variable_shadowing() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.feature-exists(global-variable-shadowing)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.feature-exists($feature: at-error)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn quote_insensitive() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.feature-exists(\"at-error\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn units_level_3() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.feature-exists(units-level-3)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn unknown() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.feature-exists(unknown)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
