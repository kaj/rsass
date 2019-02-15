//! Tests auto-converted from "sass-spec/spec/libsass/warn-directive-nested"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// Ignoring "function.hrx", not expected to work yet.

// Ignoring "inline.hrx", error tests are not supported yet.

/// From "sass-spec/spec/libsass/warn-directive-nested/mixin.hrx"
#[test]
#[ignore] // failing
fn mixin() {
    assert_eq!(
        rsass(
            "@mixin c() {\n  @warn test;\n  c: d;\n}\n\na {\n  b: {\n    @include c();\n  }\n}\n"
        )
        .unwrap(),
        "a {\n  b-c: d;\n}\n"
    );
}
