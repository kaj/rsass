//! Tests auto-converted from "sass-spec/spec/core_functions/content-exists"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/core_functions/content-exists/basic"
#[test]
fn basic() {
    assert_eq!(
        rsass(
            "@mixin check-for-content {\n  content-exists: content-exists();\n  @if false {\n    @content;\n  }\n}\n\n.should-be-true {\n  @include check-for-content {\n    content-given: yes;\n  }\n}\n\n.should-be-false {\n  @include check-for-content;\n}\n"
        )
        .unwrap(),
        ".should-be-true {\n  content-exists: true;\n}\n.should-be-false {\n  content-exists: false;\n}\n"
    );
}

mod errors;
