//! Tests auto-converted from "sass-spec/spec/parser/interpolate/14_escapes_literal_numbers"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// Ignoring "01_inline.hrx", start_version is 3.7.

// Ignoring "02_variable.hrx", start_version is 3.7.

// Ignoring "03_inline_double.hrx", start_version is 3.7.

// Ignoring "04_variable_double.hrx", start_version is 3.7.

// Ignoring "05_variable_quoted_double.hrx", start_version is 3.7.

/// From "sass-spec/spec/parser/interpolate/14_escapes_literal_numbers/06_escape_interpolation.hrx"
#[test]
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: \\1\\2\\3\\4\\5\\6\\7\\8\\9;\n.result {\n  output: \"[\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}]\";\n  output: \"\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\";\n  output: \'\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\';\n  output: \"[\'\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"[\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}]\";\n  output: \"\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\";\n  output: \'\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\';\n  output: \"[\'\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\']\";\n}\n"
    );
}
