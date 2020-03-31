//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/25_escapes_single_quoted_specials"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/parser/interpolate/25_escapes_single_quoted_specials/01_inline"
#[test]
#[ignore] // wrong result
fn t01_inline() {
    assert_eq!(
        rsass(
            ".result {\
            \n  output: \'\\0_\\a_\\A\';\
            \n  output: #{\'\\0_\\a_\\A\'};\
            \n  output: \"[#{\'\\0_\\a_\\A\'}]\";\
            \n  output: \"#{\'\\0_\\a_\\A\'}\";\
            \n  output: \'#{\'\\0_\\a_\\A\'}\';\
            \n  output: \"[\'#{\'\\0_\\a_\\A\'}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \n.result {\
        \n  output: \'\\0_\\a_\\A\';\
        \n  output: �_ _ ;\
        \n  output: \"[�_\\a_\\a]\";\
        \n  output: \"�_\\a_\\a\";\
        \n  output: \"�_\\a_\\a\";\
        \n  output: \"[\'�_\\a_\\a\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/25_escapes_single_quoted_specials/02_variable"
#[test]
#[ignore] // wrong result
fn t02_variable() {
    assert_eq!(
        rsass(
            "$input: \'\\0_\\a_\\A\';\
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
        "@charset \"UTF-8\";\
        \n.result {\
        \n  output: \"�_\\a_\\a\";\
        \n  output: �_ _ ;\
        \n  output: \"[�_\\a_\\a]\";\
        \n  output: \"�_\\a_\\a\";\
        \n  output: \"�_\\a_\\a\";\
        \n  output: \"[\'�_\\a_\\a\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/25_escapes_single_quoted_specials/03_inline_double"
#[test]
#[ignore] // wrong result
fn t03_inline_double() {
    assert_eq!(
        rsass(
            ".result {\
            \n  output: #{#{\'\\0_\\a_\\A\'}};\
            \n  output: #{\"[#{\'\\0_\\a_\\A\'}]\"};\
            \n  output: #{\"#{\'\\0_\\a_\\A\'}\"};\
            \n  output: #{\'#{\'\\0_\\a_\\A\'}\'};\
            \n  output: #{\"[\'#{\'\\0_\\a_\\A\'}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \n.result {\
        \n  output: �_ _ ;\
        \n  output: [�_ _ ];\
        \n  output: �_ _ ;\
        \n  output: �_ _ ;\
        \n  output: [\'�_ _ \'];\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/25_escapes_single_quoted_specials/04_variable_double"
#[test]
#[ignore] // wrong result
fn t04_variable_double() {
    assert_eq!(
        rsass(
            "$input: \'\\0_\\a_\\A\';\
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
        "@charset \"UTF-8\";\
        \n.result {\
        \n  output: �_ _ ;\
        \n  output: [�_ _ ];\
        \n  output: �_ _ ;\
        \n  output: �_ _ ;\
        \n  output: [\'�_ _ \'];\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/25_escapes_single_quoted_specials/06_escape_interpolation"
#[test]
#[ignore] // wrong result
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: \'\\0_\\a_\\A\';\
            \n.result {\
            \n  output: \"[\\#{\'\\0_\\a_\\A\'}]\";\
            \n  output: \"\\#{\'\\0_\\a_\\A\'}\";\
            \n  output: \'\\#{\'\\0_\\a_\\A\'}\';\
            \n  output: \"[\'\\#{\'\\0_\\a_\\A\'}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[\\#{\'\\0_\\a_\\A\'}]\";\
        \n  output: \"\\#{\'\\0_\\a_\\A\'}\";\
        \n  output: \"#{\" \\0 _\\a _\\a  \"}\";\
        \n  output: \"[\'\\#{\'\\0_\\a_\\A\'}\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/25_escapes_single_quoted_specials/todo_05_variable_quoted_double"
#[test]
#[ignore] // wrong result
fn todo_05_variable_quoted_double() {
    assert_eq!(
        rsass(
            "$input: \'\\0_\\a_\\A\';\
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
        "@charset \"UTF-8\";\
        \n.result {\
        \n  dquoted: \"�_\\a_\\a\";\
        \n  dquoted: \"[�_\\a_\\a]\";\
        \n  dquoted: \"�_\\a_\\a\";\
        \n  dquoted: \"�_\\a_\\a\";\
        \n  dquoted: \"[\'�_\\a_\\a\']\";\
        \n  squoted: \"�_\\a_\\a\";\
        \n  squoted: \"[�_\\a_\\a]\";\
        \n  squoted: \"�_\\a_\\a\";\
        \n  squoted: \"�_\\a_\\a\";\
        \n  squoted: \"[\'�_\\a_\\a\']\";\
        \n}\
        \n"
    );
}
