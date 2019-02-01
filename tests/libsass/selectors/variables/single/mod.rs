//! Tests auto-converted from "sass-spec/spec/libsass/selectors/variables/single"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::precision;

/// From "sass-spec/spec/libsass/selectors/variables/single/bare"
#[test]
fn bare() {
    assert_eq!(
        rsass(".foo {\n  $bar: &;\n  content: $bar;\n}").unwrap(),
        ".foo {\n  content: .foo;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/selectors/variables/single/interpolated"
#[test]
fn interpolated() {
    assert_eq!(
        rsass(".foo {\n  $bar: &;\n  content: #{$bar};\n}").unwrap(),
        ".foo {\n  content: .foo;\n}\n"
    );
}
