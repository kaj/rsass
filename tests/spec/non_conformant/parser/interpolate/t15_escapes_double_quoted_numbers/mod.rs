//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/15_escapes_double_quoted_numbers"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/parser/interpolate/15_escapes_double_quoted_numbers/01_inline.hrx"
#[test]
#[ignore] // wrong result
fn t01_inline() {
    assert_eq!(
        rsass(
            ".result {\
            \n  output: \"\\1\\2\\3\\4\\5\\6\\7\\8\\9\";\
            \n  output: #{\"\\1\\2\\3\\4\\5\\6\\7\\8\\9\"};\
            \n  output: \"[#{\"\\1\\2\\3\\4\\5\\6\\7\\8\\9\"}]\";\
            \n  output: \"#{\"\\1\\2\\3\\4\\5\\6\\7\\8\\9\"}\";\
            \n  output: \'#{\"\\1\\2\\3\\4\\5\\6\\7\\8\\9\"}\';\
            \n  output: \"[\'#{\"\\1\\2\\3\\4\\5\\6\\7\\8\\9\"}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  output: \u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t;\
        \n  output: \"[\\1\\2\\3\\4\\5\\6\\7\\8 \t]\";\
        \n  output: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  output: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  output: \"[\'\\1\\2\\3\\4\\5\\6\\7\\8 \t\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/15_escapes_double_quoted_numbers/02_variable.hrx"
#[test]
#[ignore] // wrong result
fn t02_variable() {
    assert_eq!(
        rsass(
            "$input: \"\\1\\2\\3\\4\\5\\6\\7\\8\\9\";\
            \n.result {\
            \n  output: $input;\
            \n  output: #{$input};\
            \n  output: \"[#{$input}]\";\
            \n  output: \"#{$input}\";\
            \n  output: \'#{$input}\';\
            \n  output: \"[\'#{$input}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  output: \u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t;\
        \n  output: \"[\\1\\2\\3\\4\\5\\6\\7\\8 \t]\";\
        \n  output: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  output: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  output: \"[\'\\1\\2\\3\\4\\5\\6\\7\\8 \t\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/15_escapes_double_quoted_numbers/03_inline_double.hrx"
#[test]
#[ignore] // wrong result
fn t03_inline_double() {
    assert_eq!(
        rsass(
            ".result {\
            \n  output: #{#{\"\\1\\2\\3\\4\\5\\6\\7\\8\\9\"}};\
            \n  output: #{\"[#{\"\\1\\2\\3\\4\\5\\6\\7\\8\\9\"}]\"};\
            \n  output: #{\"#{\"\\1\\2\\3\\4\\5\\6\\7\\8\\9\"}\"};\
            \n  output: #{\'#{\"\\1\\2\\3\\4\\5\\6\\7\\8\\9\"}\'};\
            \n  output: #{\"[\'#{\"\\1\\2\\3\\4\\5\\6\\7\\8\\9\"}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t;\
        \n  output: [\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t];\
        \n  output: \u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t;\
        \n  output: \u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t;\
        \n  output: [\'\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t\'];\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/15_escapes_double_quoted_numbers/04_variable_double.hrx"
#[test]
#[ignore] // wrong result
fn t04_variable_double() {
    assert_eq!(
        rsass(
            "$input: \"\\1\\2\\3\\4\\5\\6\\7\\8\\9\";\
            \n.result {\
            \n  output: #{#{$input}};\
            \n  output: #{\"[#{$input}]\"};\
            \n  output: #{\"#{$input}\"};\
            \n  output: #{\'#{$input}\'};\
            \n  output: #{\"[\'#{$input}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t;\
        \n  output: [\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t];\
        \n  output: \u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t;\
        \n  output: \u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t;\
        \n  output: [\'\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t\'];\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/15_escapes_double_quoted_numbers/05_variable_quoted_double.hrx"
#[test]
#[ignore] // wrong result
fn t05_variable_quoted_double() {
    assert_eq!(
        rsass(
            "$input: \"\\1\\2\\3\\4\\5\\6\\7\\8\\9\";\
            \n.result {\
            \n  dquoted: \"#{#{$input}}\";\
            \n  dquoted: \"#{\"[#{$input}]\"}\";\
            \n  dquoted: \"#{\"#{$input}\"}\";\
            \n  dquoted: \"#{\'#{$input}\'}\";\
            \n  dquoted: \"#{\"[\'#{$input}\']\"}\";\
            \n  squoted: \'#{#{$input}}\';\
            \n  squoted: \'#{\"[#{$input}]\"}\';\
            \n  squoted: \'#{\"#{$input}\"}\';\
            \n  squoted: \'#{\'#{$input}\'}\';\
            \n  squoted: \'#{\"[\'#{$input}\']\"}\';\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  dquoted: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  dquoted: \"[\\1\\2\\3\\4\\5\\6\\7\\8 \t]\";\
        \n  dquoted: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  dquoted: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  dquoted: \"[\'\\1\\2\\3\\4\\5\\6\\7\\8 \t\']\";\
        \n  squoted: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  squoted: \"[\\1\\2\\3\\4\\5\\6\\7\\8 \t]\";\
        \n  squoted: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  squoted: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  squoted: \"[\'\\1\\2\\3\\4\\5\\6\\7\\8 \t\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/15_escapes_double_quoted_numbers/06_escape_interpolation.hrx"
#[test]
#[ignore] // wrong result
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: \"\\1\\2\\3\\4\\5\\6\\7\\8\\9\";\
            \n.result {\
            \n  output: \"[\\#{\"\\1\\2\\3\\4\\5\\6\\7\\8\\9\"}]\";\
            \n  output: \"\\#{\"\\1\\2\\3\\4\\5\\6\\7\\8\\9\"}\";\
            \n  output: \'\\#{\"\\1\\2\\3\\4\\5\\6\\7\\8\\9\"}\';\
            \n  output: \"[\'\\#{\"\\1\\2\\3\\4\\5\\6\\7\\8\\9\"}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[#{\" \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9  \"}]\";\
        \n  output: \"#{\" \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9  \"}\";\
        \n  output: \'#{\"\\1\\2\\3\\4\\5\\6\\7\\8 \t\"}\';\
        \n  output: \"[\'#{\" \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9  \"}\']\";\
        \n}\
        \n"
    );
}
