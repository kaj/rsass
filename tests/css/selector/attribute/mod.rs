//! Tests auto-converted from "sass-spec/spec/css/selector/attribute"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/css/selector/attribute/dash-dash"
#[test]
fn dash_dash() {
    assert_eq!(
        rsass(
            "// Attribute selector values are allowed to be unquoted as long as they\'re plain\n// CSS identifiers. However, IE 11 doesn\'t recognize custom-property-style\n// identifiers like `--foo` as identifiers, so they should always be quoted.\n\n[class=\"--foo\"], [class*=\"--foo\"] {\n  x: y;\n}\n"
        )
        .unwrap(),
        "[class=\"--foo\"], [class*=\"--foo\"] {\n  x: y;\n}\n"
    );
}
