//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/02_double_quoted"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/parser/interpolate/02_double_quoted/01_inline.hrx"
#[test]
fn t01_inline() {
    assert_eq!(
        rsass(
            ".result {\
            \n  output: \"dquoted\";\
            \n  output: #{\"dquoted\"};\
            \n  output: \"[#{\"dquoted\"}]\";\
            \n  output: \"#{\"dquoted\"}\";\
            \n  output: \'#{\"dquoted\"}\';\
            \n  output: \"[\'#{\"dquoted\"}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"dquoted\";\
        \n  output: dquoted;\
        \n  output: \"[dquoted]\";\
        \n  output: \"dquoted\";\
        \n  output: \"dquoted\";\
        \n  output: \"[\'dquoted\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/02_double_quoted/02_variable.hrx"
#[test]
fn t02_variable() {
    assert_eq!(
        rsass(
            "$input: \"dquoted\";\
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
        \n  output: \"dquoted\";\
        \n  output: dquoted;\
        \n  output: \"[dquoted]\";\
        \n  output: \"dquoted\";\
        \n  output: \"dquoted\";\
        \n  output: \"[\'dquoted\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/02_double_quoted/03_inline_double.hrx"
#[test]
fn t03_inline_double() {
    assert_eq!(
        rsass(
            ".result {\
            \n  output: #{#{\"dquoted\"}};\
            \n  output: #{\"[#{\"dquoted\"}]\"};\
            \n  output: #{\"#{\"dquoted\"}\"};\
            \n  output: #{\'#{\"dquoted\"}\'};\
            \n  output: #{\"[\'#{\"dquoted\"}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: dquoted;\
        \n  output: [dquoted];\
        \n  output: dquoted;\
        \n  output: dquoted;\
        \n  output: [\'dquoted\'];\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/02_double_quoted/04_variable_double.hrx"
#[test]
fn t04_variable_double() {
    assert_eq!(
        rsass(
            "$input: \"dquoted\";\
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
        \n  output: dquoted;\
        \n  output: [dquoted];\
        \n  output: dquoted;\
        \n  output: dquoted;\
        \n  output: [\'dquoted\'];\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/02_double_quoted/05_variable_quoted_double.hrx"
#[test]
fn t05_variable_quoted_double() {
    assert_eq!(
        rsass(
            "$input: \"dquoted\";\
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
        \n  dquoted: \"dquoted\";\
        \n  dquoted: \"[dquoted]\";\
        \n  dquoted: \"dquoted\";\
        \n  dquoted: \"dquoted\";\
        \n  dquoted: \"[\'dquoted\']\";\
        \n  squoted: \"dquoted\";\
        \n  squoted: \"[dquoted]\";\
        \n  squoted: \"dquoted\";\
        \n  squoted: \"dquoted\";\
        \n  squoted: \"[\'dquoted\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/02_double_quoted/06_escape_interpolation.hrx"
#[test]
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: \"dquoted\";\
            \n.result {\
            \n  output: \"[\\#{\"dquoted\"}]\";\
            \n  output: \"\\#{\"dquoted\"}\";\
            \n  output: \'\\#{\"dquoted\"}\';\
            \n  output: \"[\'\\#{\"dquoted\"}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[#{\" dquoted \"}]\";\
        \n  output: \"#{\" dquoted \"}\";\
        \n  output: \'#{\"dquoted\"}\';\
        \n  output: \"[\'#{\" dquoted \"}\']\";\
        \n}\
        \n"
    );
}
