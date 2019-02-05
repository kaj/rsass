//! Tests auto-converted from "sass-spec/spec/libsass/selectors/variables/nested"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/libsass/selectors/variables/nested/bare"
#[test]
#[ignore] // failing
fn bare() {
    assert_eq!(
        rsass(
            ".foo a,\n.bar p {\n\n  .baz {\n    $bar: &;\n    content: $bar;\n  }\n\n}"
        )
        .unwrap(),
        ".foo a .baz,\n.bar p .baz {\n  content: .foo a .baz, .bar p .baz;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/selectors/variables/nested/interpolated"
#[test]
#[ignore] // failing
fn interpolated() {
    assert_eq!(
        rsass(
            ".foo a,\n.bar p {\n\n  .baz {\n    $bar: &;\n    content: #{$bar};\n  }\n\n}"
        )
        .unwrap(),
        ".foo a .baz,\n.bar p .baz {\n  content: .foo a .baz, .bar p .baz;\n}\n"
    );
}
