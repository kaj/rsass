//! Tests auto-converted from "sass-spec/spec/parser/interpolate/27_escaped_double_quotes"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/parser/interpolate/27_escaped_double_quotes/01_inline.hrx"
#[test]
fn t01_inline() {
    assert_eq!(
        rsass(
            ".result {\n  output: \"\\\"\";\n  output: #{\"\\\"\"};\n  output: \"[#{\"\\\"\"}]\";\n  output: \"#{\"\\\"\"}\";\n  output: \'#{\"\\\"\"}\';\n  output: \"[\'#{\"\\\"\"}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"\\\"\";\n  output: \";\n  output: \'[\"]\';\n  output: \'\"\';\n  output: \'\"\';\n  output: \"[\'\\\"\']\";\n}\n"
    );
}

// From "sass-spec/spec/parser/interpolate/27_escaped_double_quotes/02_variable.hrx"
#[test]
fn t02_variable() {
    assert_eq!(
        rsass(
            "$input: \"\\\"\";\n.result {\n  output: $input;\n  output: #{$input};\n  output: \"[#{$input}]\";\n  output: \"#{$input}\";\n  output: \'#{$input}\';\n  output: \"[\'#{$input}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \'\"\';\n  output: \";\n  output: \'[\"]\';\n  output: \'\"\';\n  output: \'\"\';\n  output: \"[\'\\\"\']\";\n}\n"
    );
}

// From "sass-spec/spec/parser/interpolate/27_escaped_double_quotes/03_inline_double.hrx"
#[test]
fn t03_inline_double() {
    assert_eq!(
        rsass(
            ".result {\n  output: #{#{\"\\\"\"}};\n  output: #{\"[#{\"\\\"\"}]\"};\n  output: #{\"#{\"\\\"\"}\"};\n  output: #{\'#{\"\\\"\"}\'};\n  output: #{\"[\'#{\"\\\"\"}\']\"};\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \";\n  output: [\"];\n  output: \";\n  output: \";\n  output: [\'\"\'];\n}\n"
    );
}

// From "sass-spec/spec/parser/interpolate/27_escaped_double_quotes/04_variable_double.hrx"
#[test]
fn t04_variable_double() {
    assert_eq!(
        rsass(
            "$input: \"\\\"\";\n.result {\n  output: #{#{$input}};\n  output: #{\"[#{$input}]\"};\n  output: #{\"#{$input}\"};\n  output: #{\'#{$input}\'};\n  output: #{\"[\'#{$input}\']\"};\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \";\n  output: [\"];\n  output: \";\n  output: \";\n  output: [\'\"\'];\n}\n"
    );
}

// From "sass-spec/spec/parser/interpolate/27_escaped_double_quotes/05_variable_quoted_double.hrx"
#[test]
fn t05_variable_quoted_double() {
    assert_eq!(
        rsass(
            "$input: \"\\\"\";\n.result {\n  dquoted: \"#{#{$input}}\";\n  dquoted: \"#{\"[#{$input}]\"}\";\n  dquoted: \"#{\"#{$input}\"}\";\n  dquoted: \"#{\'#{$input}\'}\";\n  dquoted: \"#{\"[\'#{$input}\']\"}\";\n  squoted: \'#{#{$input}}\';\n  squoted: \'#{\"[#{$input}]\"}\';\n  squoted: \'#{\"#{$input}\"}\';\n  squoted: \'#{\'#{$input}\'}\';\n  squoted: \'#{\"[\'#{$input}\']\"}\';\n}\n"
        )
        .unwrap(),
        ".result {\n  dquoted: \'\"\';\n  dquoted: \'[\"]\';\n  dquoted: \'\"\';\n  dquoted: \'\"\';\n  dquoted: \"[\'\\\"\']\";\n  squoted: \'\"\';\n  squoted: \'[\"]\';\n  squoted: \'\"\';\n  squoted: \'\"\';\n  squoted: \"[\'\\\"\']\";\n}\n"
    );
}

// From "sass-spec/spec/parser/interpolate/27_escaped_double_quotes/06_escape_interpolation.hrx"
#[test]
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: \"\\\"\";\n.result {\n  output: \"[\\#{\"\\\"\"}]\";\n  output: \"\\#{\"\\\"\"}\";\n  output: \'\\#{\"\\\"\"}\';\n  output: \"[\'\\#{\"\\\"\"}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"[#{\" \\\" \"}]\";\n  output: \"#{\" \\\" \"}\";\n  output: \'\\#{\"\\\"\"}\';\n  output: \"[\'#{\" \\\" \"}\']\";\n}\n"
    );
}
