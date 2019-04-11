//! Tests auto-converted from "sass-spec/spec/parser/interpolate/30_base_test"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/parser/interpolate/30_base_test/01_inline.hrx"
#[test]
fn t01_inline() {
    assert_eq!(
        rsass(
            ".result {\n  output: \"foo#{\'ba\' + \'r\'}baz\";\n  output: #{\"foo#{\'ba\' + \'r\'}baz\"};\n  output: \"[#{\"foo#{\'ba\' + \'r\'}baz\"}]\";\n  output: \"#{\"foo#{\'ba\' + \'r\'}baz\"}\";\n  output: \'#{\"foo#{\'ba\' + \'r\'}baz\"}\';\n  output: \"[\'#{\"foo#{\'ba\' + \'r\'}baz\"}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"foobarbaz\";\n  output: foobarbaz;\n  output: \"[foobarbaz]\";\n  output: \"foobarbaz\";\n  output: \"foobarbaz\";\n  output: \"[\'foobarbaz\']\";\n}\n"
    );
}

// From "sass-spec/spec/parser/interpolate/30_base_test/02_variable.hrx"
#[test]
fn t02_variable() {
    assert_eq!(
        rsass(
            "$input: \"foo#{\'ba\' + \'r\'}baz\";\n.result {\n  output: $input;\n  output: #{$input};\n  output: \"[#{$input}]\";\n  output: \"#{$input}\";\n  output: \'#{$input}\';\n  output: \"[\'#{$input}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"foobarbaz\";\n  output: foobarbaz;\n  output: \"[foobarbaz]\";\n  output: \"foobarbaz\";\n  output: \"foobarbaz\";\n  output: \"[\'foobarbaz\']\";\n}\n"
    );
}

// From "sass-spec/spec/parser/interpolate/30_base_test/03_inline_double.hrx"
#[test]
fn t03_inline_double() {
    assert_eq!(
        rsass(
            ".result {\n  output: #{#{\"foo#{\'ba\' + \'r\'}baz\"}};\n  output: #{\"[#{\"foo#{\'ba\' + \'r\'}baz\"}]\"};\n  output: #{\"#{\"foo#{\'ba\' + \'r\'}baz\"}\"};\n  output: #{\'#{\"foo#{\'ba\' + \'r\'}baz\"}\'};\n  output: #{\"[\'#{\"foo#{\'ba\' + \'r\'}baz\"}\']\"};\n}\n"
        )
        .unwrap(),
        ".result {\n  output: foobarbaz;\n  output: [foobarbaz];\n  output: foobarbaz;\n  output: foobarbaz;\n  output: [\'foobarbaz\'];\n}\n"
    );
}

// From "sass-spec/spec/parser/interpolate/30_base_test/04_variable_double.hrx"
#[test]
fn t04_variable_double() {
    assert_eq!(
        rsass(
            "$input: \"foo#{\'ba\' + \'r\'}baz\";\n.result {\n  output: #{#{$input}};\n  output: #{\"[#{$input}]\"};\n  output: #{\"#{$input}\"};\n  output: #{\'#{$input}\'};\n  output: #{\"[\'#{$input}\']\"};\n}\n"
        )
        .unwrap(),
        ".result {\n  output: foobarbaz;\n  output: [foobarbaz];\n  output: foobarbaz;\n  output: foobarbaz;\n  output: [\'foobarbaz\'];\n}\n"
    );
}

// From "sass-spec/spec/parser/interpolate/30_base_test/05_variable_quoted_double.hrx"
#[test]
fn t05_variable_quoted_double() {
    assert_eq!(
        rsass(
            "$input: \"foo#{\'ba\' + \'r\'}baz\";\n.result {\n  dquoted: \"#{#{$input}}\";\n  dquoted: \"#{\"[#{$input}]\"}\";\n  dquoted: \"#{\"#{$input}\"}\";\n  dquoted: \"#{\'#{$input}\'}\";\n  dquoted: \"#{\"[\'#{$input}\']\"}\";\n  squoted: \'#{#{$input}}\';\n  squoted: \'#{\"[#{$input}]\"}\';\n  squoted: \'#{\"#{$input}\"}\';\n  squoted: \'#{\'#{$input}\'}\';\n  squoted: \'#{\"[\'#{$input}\']\"}\';\n}\n"
        )
        .unwrap(),
        ".result {\n  dquoted: \"foobarbaz\";\n  dquoted: \"[foobarbaz]\";\n  dquoted: \"foobarbaz\";\n  dquoted: \"foobarbaz\";\n  dquoted: \"[\'foobarbaz\']\";\n  squoted: \"foobarbaz\";\n  squoted: \"[foobarbaz]\";\n  squoted: \"foobarbaz\";\n  squoted: \"foobarbaz\";\n  squoted: \"[\'foobarbaz\']\";\n}\n"
    );
}

// From "sass-spec/spec/parser/interpolate/30_base_test/06_escape_interpolation-4.0.hrx"

// Ignoring "t06_escape_interpolation_4_0", start_version is 4.0.

// From "sass-spec/spec/parser/interpolate/30_base_test/06_escape_interpolation.hrx"

// Ignoring "t06_escape_interpolation", end_version is 3.5.
