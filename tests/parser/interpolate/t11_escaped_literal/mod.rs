//! Tests auto-converted from "sass-spec/spec/parser/interpolate/11_escaped_literal"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// Ignoring "01_inline.hrx", start_version is 3.7.

// Ignoring "02_variable.hrx", start_version is 3.7.

// Ignoring "03_inline_double.hrx", start_version is 3.7.

// Ignoring "04_variable_double.hrx", start_version is 3.7.

// Ignoring "05_variable_quoted_double.hrx", start_version is 3.7.

/// From "sass-spec/spec/parser/interpolate/11_escaped_literal/06_escape_interpolation.hrx"
#[test]
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: l\\\\ite\\ral;\n.result {\n  output: \"[\\#{l\\\\ite\\ral}]\";\n  output: \"\\#{l\\\\ite\\ral}\";\n  output: \'\\#{l\\\\ite\\ral}\';\n  output: \"[\'\\#{l\\\\ite\\ral}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"[\\#{l\\\\ite\\ral}]\";\n  output: \"\\#{l\\\\ite\\ral}\";\n  output: \'\\#{l\\\\ite\\ral}\';\n  output: \"[\'\\#{l\\\\ite\\ral}\']\";\n}\n"
    );
}
