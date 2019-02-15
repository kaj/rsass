//! Tests auto-converted from "sass-spec/spec/css/selector"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

mod attribute;

// Ignoring "reference_combinator.hrx", start_version is 4.0.

/// From "sass-spec/spec/css/selector/slotted.hrx"
#[test]
#[ignore] // failing
fn slotted() {
    assert_eq!(
        rsass(
            "::slotted(.a) {x: y}\n\n::slotted(.c.d) {x: y}\n.e {@extend .c}\n\n::slotted(.f) {x: y}\n::slotted(.g) {@extend .f}\n"
        )
        .unwrap(),
        "::slotted(.a) {\n  x: y;\n}\n::slotted(.c.d, .d.e) {\n  x: y;\n}\n::slotted(.f, ::slotted(.g)) {\n  x: y;\n}\n"
    );
}
