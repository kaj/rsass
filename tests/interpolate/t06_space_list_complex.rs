//! Tests from `spec/parser/interpolate/06_space_list_complex`

use super::check;

#[test]
fn t01_inline() {
    check(
        ".result {\n  output: gamme \"'\"delta\"'\";\n  \
         output: #{gamme \"'\"delta\"'\"};\n  \
         output: \"[#{gamme \"'\"delta\"'\"}]\";\n  \
         output: \"#{gamme \"'\"delta\"'\"}\";\n  \
         output: '#{gamme \"'\"delta\"'\"}';\n  \
         output: \"['#{gamme \"'\"delta\"'\"}']\";\n}\n",
        ".result {\n  output: gamme \"'\" delta \"'\";\n  \
         output: gamme ' delta ';\n  output: \"[gamme ' delta ']\";\n  \
         output: \"gamme ' delta '\";\n  output: \"gamme ' delta '\";\n  \
         output: \"['gamme ' delta '']\";\n}\n",
    )
}

#[test]
fn t02_variable() {
    check(
        "$input: gamme \"'\"delta\"'\";\n\
         .result {\n  output: $input;\n  output: #{$input};\n  \
         output: \"[#{$input}]\";\n  output: \"#{$input}\";\n  \
         output: '#{$input}';\n  output: \"['#{$input}']\";\n}\n",
        ".result {\n  output: gamme \"'\" delta \"'\";\n  \
         output: gamme ' delta ';\n  output: \"[gamme ' delta ']\";\n  \
         output: \"gamme ' delta '\";\n  output: \"gamme ' delta '\";\n  \
         output: \"['gamme ' delta '']\";\n}\n",
    )
}

#[test]
fn t03_inline_double() {
    check(
        ".result {\n  output: #{#{gamme \"'\"delta\"'\"}};\n  \
         output: #{\"[#{gamme \"'\"delta\"'\"}]\"};\n  \
         output: #{\"#{gamme \"'\"delta\"'\"}\"};\n  \
         output: #{'#{gamme \"'\"delta\"'\"}'};\n  \
         output: #{\"['#{gamme \"'\"delta\"'\"}']\"};\n}\n",
        ".result {\n  output: gamme ' delta ';\n  \
         output: [gamme ' delta '];\n  output: gamme ' delta ';\n  \
         output: gamme ' delta ';\n  output: ['gamme ' delta ''];\n}\n",
    )
}
