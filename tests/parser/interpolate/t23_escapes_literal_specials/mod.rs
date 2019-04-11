//! Tests auto-converted from "sass-spec/spec/parser/interpolate/23_escapes_literal_specials"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/parser/interpolate/23_escapes_literal_specials/01_inline.hrx"

// Ignoring "t01_inline", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/23_escapes_literal_specials/02_variable.hrx"

// Ignoring "t02_variable", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/23_escapes_literal_specials/03_inline_double.hrx"

// Ignoring "t03_inline_double", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/23_escapes_literal_specials/04_variable_double.hrx"

// Ignoring "t04_variable_double", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/23_escapes_literal_specials/05_variable_quoted_double.hrx"

// Ignoring "t05_variable_quoted_double", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/23_escapes_literal_specials/06_escape_interpolation"
#[test]
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: \\0_\\a_\\A;\n.result {\n  output: \"[\\#{\\0_\\a_\\A}]\";\n  output: \"\\#{\\0_\\a_\\A}\";\n  output: \'\\#{\\0_\\a_\\A}\';\n  output: \"[\'\\#{\\0_\\a_\\A}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"[\\#{\\0_\\a_\\A}]\";\n  output: \"\\#{\\0_\\a_\\A}\";\n  output: \'\\#{\\0_\\a_\\A}\';\n  output: \"[\'\\#{\\0_\\a_\\A}\']\";\n}\n"
    );
}
