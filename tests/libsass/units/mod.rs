//! Tests auto-converted from "sass-spec/spec/libsass/units"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

mod conversion;

/// From "sass-spec/spec/libsass/units/feature-test"
#[test]
fn feature_test() {
    assert_eq!(
        rsass(
            "@if feature-exists(units-level-3) {\n  div {\n    feature: true;\n  }\n}\n"
        ).unwrap(),
        "div {\n  feature: true;\n}\n"
    );
}

// Ignoring "simple", not expected to work yet.
