//! Tests auto-converted from "sass-spec/spec/libsass/features"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/libsass/features/at-error.hrx"
#[test]
#[ignore] // failing
fn at_error() {
    assert_eq!(
        rsass("foo {\n  foo: feature-exists(\'at-error\');\n}\n").unwrap(),
        "foo {\n  foo: true;\n}\n"
    );
}

// From "sass-spec/spec/libsass/features/extend-selector-pseudoclass.hrx"
#[test]
#[ignore] // failing
fn extend_selector_pseudoclass() {
    assert_eq!(
        rsass(
            "foo {\n  foo: feature-exists(\'extend-selector-pseudoclass\');\n}\n"
        )
        .unwrap(),
        "foo {\n  foo: true;\n}\n"
    );
}

// From "sass-spec/spec/libsass/features/global-variable-shadowing.hrx"
#[test]
fn global_variable_shadowing() {
    assert_eq!(
        rsass(
            "foo {\n  foo: feature-exists(\'global-variable-shadowing\');\n}\n"
        )
        .unwrap(),
        "foo {\n  foo: true;\n}\n"
    );
}

// From "sass-spec/spec/libsass/features/units-level-3.hrx"
#[test]
fn units_level_3() {
    assert_eq!(
        rsass("foo {\n  foo: feature-exists(\'units-level-3\');\n}\n")
            .unwrap(),
        "foo {\n  foo: true;\n}\n"
    );
}
