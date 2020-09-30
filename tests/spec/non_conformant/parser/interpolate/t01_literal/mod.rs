//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/01_literal"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/parser/interpolate/01_literal/01_inline.hrx"
#[test]
fn t01_inline() {
    assert_eq!(
        rsass(
            ".result {\
            \n  output: literal;\
            \n  output: #{literal};\
            \n  output: \"[#{literal}]\";\
            \n  output: \"#{literal}\";\
            \n  output: \'#{literal}\';\
            \n  output: \"[\'#{literal}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: literal;\
        \n  output: literal;\
        \n  output: \"[literal]\";\
        \n  output: \"literal\";\
        \n  output: \"literal\";\
        \n  output: \"[\'literal\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/01_literal/02_variable.hrx"
#[test]
fn t02_variable() {
    assert_eq!(
        rsass(
            "$input: literal;\
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
        \n  output: literal;\
        \n  output: literal;\
        \n  output: \"[literal]\";\
        \n  output: \"literal\";\
        \n  output: \"literal\";\
        \n  output: \"[\'literal\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/01_literal/03_inline_double.hrx"
#[test]
fn t03_inline_double() {
    assert_eq!(
        rsass(
            ".result {\
            \n  output: #{#{literal}};\
            \n  output: #{\"[#{literal}]\"};\
            \n  output: #{\"#{literal}\"};\
            \n  output: #{\'#{literal}\'};\
            \n  output: #{\"[\'#{literal}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: literal;\
        \n  output: [literal];\
        \n  output: literal;\
        \n  output: literal;\
        \n  output: [\'literal\'];\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/01_literal/04_variable_double.hrx"
#[test]
fn t04_variable_double() {
    assert_eq!(
        rsass(
            "$input: literal;\
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
        \n  output: literal;\
        \n  output: [literal];\
        \n  output: literal;\
        \n  output: literal;\
        \n  output: [\'literal\'];\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/01_literal/05_variable_quoted_double.hrx"
#[test]
fn t05_variable_quoted_double() {
    assert_eq!(
        rsass(
            "$input: literal;\
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
        \n  dquoted: \"literal\";\
        \n  dquoted: \"[literal]\";\
        \n  dquoted: \"literal\";\
        \n  dquoted: \"literal\";\
        \n  dquoted: \"[\'literal\']\";\
        \n  squoted: \"literal\";\
        \n  squoted: \"[literal]\";\
        \n  squoted: \"literal\";\
        \n  squoted: \"literal\";\
        \n  squoted: \"[\'literal\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/01_literal/06_escape_interpolation.hrx"
#[test]
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: literal;\
            \n.result {\
            \n  output: \"[\\#{literal}]\";\
            \n  output: \"\\#{literal}\";\
            \n  output: \'\\#{literal}\';\
            \n  output: \"[\'\\#{literal}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[#{literal}]\";\
        \n  output: \"#{literal}\";\
        \n  output: \"#{literal}\";\
        \n  output: \"[\'#{literal}\']\";\
        \n}\
        \n"
    );
}
