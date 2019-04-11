//! Tests auto-converted from "sass-spec/spec/parser/interpolate/11_escaped_literal"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/parser/interpolate/11_escaped_literal/01_inline.hrx"

// Ignoring "t01_inline", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/11_escaped_literal/02_variable.hrx"

// Ignoring "t02_variable", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/11_escaped_literal/03_inline_double.hrx"

// Ignoring "t03_inline_double", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/11_escaped_literal/04_variable_double.hrx"

// Ignoring "t04_variable_double", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/11_escaped_literal/05_variable_quoted_double.hrx"

// Ignoring "t05_variable_quoted_double", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/11_escaped_literal/06_escape_interpolation.hrx"
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
