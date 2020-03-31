//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials/01_inline.hrx"
#[test]
fn t01_inline() {
    assert_eq!(
        rsass(
            ".result {\
            \n  output: \\0_\\a_\\A;\
            \n  output: #{\\0_\\a_\\A};\
            \n  output: \"[#{\\0_\\a_\\A}]\";\
            \n  output: \"#{\\0_\\a_\\A}\";\
            \n  output: \'#{\\0_\\a_\\A}\';\
            \n  output: \"[\'#{\\0_\\a_\\A}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \\0 _\\a _\\a ;\
        \n  output: \\0 _\\a _\\a ;\
        \n  output: \"[\\\\0 _\\\\a _\\\\a ]\";\
        \n  output: \"\\\\0 _\\\\a _\\\\a \";\
        \n  output: \"\\\\0 _\\\\a _\\\\a \";\
        \n  output: \"[\'\\\\0 _\\\\a _\\\\a \']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials/02_variable.hrx"
#[test]
fn t02_variable() {
    assert_eq!(
        rsass(
            "$input: \\0_\\a_\\A;\
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
        \n  output: \\0 _\\a _\\a ;\
        \n  output: \\0 _\\a _\\a ;\
        \n  output: \"[\\\\0 _\\\\a _\\\\a ]\";\
        \n  output: \"\\\\0 _\\\\a _\\\\a \";\
        \n  output: \"\\\\0 _\\\\a _\\\\a \";\
        \n  output: \"[\'\\\\0 _\\\\a _\\\\a \']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials/03_inline_double.hrx"
#[test]
fn t03_inline_double() {
    assert_eq!(
        rsass(
            ".result {\
            \n  output: #{#{\\0_\\a_\\A}};\
            \n  output: #{\"[#{\\0_\\a_\\A}]\"};\
            \n  output: #{\"#{\\0_\\a_\\A}\"};\
            \n  output: #{\'#{\\0_\\a_\\A}\'};\
            \n  output: #{\"[\'#{\\0_\\a_\\A}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \\0 _\\a _\\a ;\
        \n  output: [\\0 _\\a _\\a ];\
        \n  output: \\0 _\\a _\\a ;\
        \n  output: \\0 _\\a _\\a ;\
        \n  output: [\'\\0 _\\a _\\a \'];\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials/04_variable_double.hrx"
#[test]
fn t04_variable_double() {
    assert_eq!(
        rsass(
            "$input: \\0_\\a_\\A;\
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
        \n  output: \\0 _\\a _\\a ;\
        \n  output: [\\0 _\\a _\\a ];\
        \n  output: \\0 _\\a _\\a ;\
        \n  output: \\0 _\\a _\\a ;\
        \n  output: [\'\\0 _\\a _\\a \'];\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials/05_variable_quoted_double.hrx"
#[test]
fn t05_variable_quoted_double() {
    assert_eq!(
        rsass(
            "$input: \\0_\\a_\\A;\
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
        \n  dquoted: \"\\\\0 _\\\\a _\\\\a \";\
        \n  dquoted: \"[\\\\0 _\\\\a _\\\\a ]\";\
        \n  dquoted: \"\\\\0 _\\\\a _\\\\a \";\
        \n  dquoted: \"\\\\0 _\\\\a _\\\\a \";\
        \n  dquoted: \"[\'\\\\0 _\\\\a _\\\\a \']\";\
        \n  squoted: \"\\\\0 _\\\\a _\\\\a \";\
        \n  squoted: \"[\\\\0 _\\\\a _\\\\a ]\";\
        \n  squoted: \"\\\\0 _\\\\a _\\\\a \";\
        \n  squoted: \"\\\\0 _\\\\a _\\\\a \";\
        \n  squoted: \"[\'\\\\0 _\\\\a _\\\\a \']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials/06_escape_interpolation"
#[test]
#[ignore] // wrong result
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: \\0_\\a_\\A;\
            \n.result {\
            \n  output: \"[\\#{\\0_\\a_\\A}]\";\
            \n  output: \"\\#{\\0_\\a_\\A}\";\
            \n  output: \'\\#{\\0_\\a_\\A}\';\
            \n  output: \"[\'\\#{\\0_\\a_\\A}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[\\#{\\0_\\a_\\A}]\";\
        \n  output: \"\\#{\\0_\\a_\\A}\";\
        \n  output: \'\\#{\\0_\\a_\\A}\';\
        \n  output: \"[\'\\#{\\0_\\a_\\A}\']\";\
        \n}\
        \n"
    );
}
