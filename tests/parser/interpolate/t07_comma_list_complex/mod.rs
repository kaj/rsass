//! Tests auto-converted from "sass-spec/spec/parser/interpolate/07_comma_list_complex"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/parser/interpolate/07_comma_list_complex/01_inline.hrx"
#[test]
fn t01_inline() {
    assert_eq!(
        rsass(
            ".result {\n  output: gamma, \"\'\"delta\"\'\";\n  output: #{gamma, \"\'\"delta\"\'\"};\n  output: \"[#{gamma, \"\'\"delta\"\'\"}]\";\n  output: \"#{gamma, \"\'\"delta\"\'\"}\";\n  output: \'#{gamma, \"\'\"delta\"\'\"}\';\n  output: \"[\'#{gamma, \"\'\"delta\"\'\"}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: gamma, \"\'\" delta \"\'\";\n  output: gamma, \' delta \';\n  output: \"[gamma, \' delta \']\";\n  output: \"gamma, \' delta \'\";\n  output: \"gamma, \' delta \'\";\n  output: \"[\'gamma, \' delta \'\']\";\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/07_comma_list_complex/02_variable.hrx"
#[test]
fn t02_variable() {
    assert_eq!(
        rsass(
            "$input: gamma, \"\'\"delta\"\'\";\n.result {\n  output: $input;\n  output: #{$input};\n  output: \"[#{$input}]\";\n  output: \"#{$input}\";\n  output: \'#{$input}\';\n  output: \"[\'#{$input}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: gamma, \"\'\" delta \"\'\";\n  output: gamma, \' delta \';\n  output: \"[gamma, \' delta \']\";\n  output: \"gamma, \' delta \'\";\n  output: \"gamma, \' delta \'\";\n  output: \"[\'gamma, \' delta \'\']\";\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/07_comma_list_complex/03_inline_double.hrx"
#[test]
fn t03_inline_double() {
    assert_eq!(
        rsass(
            ".result {\n  output: #{#{gamma, \"\'\"delta\"\'\"}};\n  output: #{\"[#{gamma, \"\'\"delta\"\'\"}]\"};\n  output: #{\"#{gamma, \"\'\"delta\"\'\"}\"};\n  output: #{\'#{gamma, \"\'\"delta\"\'\"}\'};\n  output: #{\"[\'#{gamma, \"\'\"delta\"\'\"}\']\"};\n}\n"
        )
        .unwrap(),
        ".result {\n  output: gamma, \' delta \';\n  output: [gamma, \' delta \'];\n  output: gamma, \' delta \';\n  output: gamma, \' delta \';\n  output: [\'gamma, \' delta \'\'];\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/07_comma_list_complex/04_variable_double.hrx"
#[test]
fn t04_variable_double() {
    assert_eq!(
        rsass(
            "$input: gamma, \"\'\"delta\"\'\";\n.result {\n  output: #{#{$input}};\n  output: #{\"[#{$input}]\"};\n  output: #{\"#{$input}\"};\n  output: #{\'#{$input}\'};\n  output: #{\"[\'#{$input}\']\"};\n}\n"
        )
        .unwrap(),
        ".result {\n  output: gamma, \' delta \';\n  output: [gamma, \' delta \'];\n  output: gamma, \' delta \';\n  output: gamma, \' delta \';\n  output: [\'gamma, \' delta \'\'];\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/07_comma_list_complex/05_variable_quoted_double.hrx"
#[test]
fn t05_variable_quoted_double() {
    assert_eq!(
        rsass(
            "$input: gamma, \"\'\"delta\"\'\";\n.result {\n  dquoted: \"#{#{$input}}\";\n  dquoted: \"#{\"[#{$input}]\"}\";\n  dquoted: \"#{\"#{$input}\"}\";\n  dquoted: \"#{\'#{$input}\'}\";\n  dquoted: \"#{\"[\'#{$input}\']\"}\";\n  squoted: \'#{#{$input}}\';\n  squoted: \'#{\"[#{$input}]\"}\';\n  squoted: \'#{\"#{$input}\"}\';\n  squoted: \'#{\'#{$input}\'}\';\n  squoted: \'#{\"[\'#{$input}\']\"}\';\n}\n"
        )
        .unwrap(),
        ".result {\n  dquoted: \"gamma, \' delta \'\";\n  dquoted: \"[gamma, \' delta \']\";\n  dquoted: \"gamma, \' delta \'\";\n  dquoted: \"gamma, \' delta \'\";\n  dquoted: \"[\'gamma, \' delta \'\']\";\n  squoted: \"gamma, \' delta \'\";\n  squoted: \"[gamma, \' delta \']\";\n  squoted: \"gamma, \' delta \'\";\n  squoted: \"gamma, \' delta \'\";\n  squoted: \"[\'gamma, \' delta \'\']\";\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/07_comma_list_complex/06_escape_interpolation.hrx"
#[test]
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: gamma, \"\'\"delta\"\'\";\n.result {\n  output: \"[\\#{gamma, \"\'\"delta\"\'\"}]\";\n  output: \"\\#{gamma, \"\'\"delta\"\'\"}\";\n  output: \'\\#{gamma, \"\'\"delta\"\'\"}\';\n  output: \"[\'\\#{gamma, \"\'\"delta\"\'\"}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"[#{gamma, \" \'\"delta\"\' \"}]\";\n  output: \"#{gamma, \" \'\"delta\"\' \"}\";\n  output: \'#{gamma, \"\' \"delta\" \'\"}\';\n  output: \"[\'#{gamma, \" \'\"delta\"\' \"}\']\";\n}\n"
    );
}
