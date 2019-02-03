//! Tests auto-converted from "sass-spec/spec/libsass/features"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// Ignoring "at-error", not expected to work yet.

// Ignoring "extend-selector-pseudoclass", not expected to work yet.

/// From "sass-spec/spec/libsass/features/global-variable-shadowing"
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

/// From "sass-spec/spec/libsass/features/units-level-3"
#[test]
fn units_level_3() {
    assert_eq!(
        rsass("foo {\n  foo: feature-exists(\'units-level-3\');\n}\n")
            .unwrap(),
        "foo {\n  foo: true;\n}\n"
    );
}
