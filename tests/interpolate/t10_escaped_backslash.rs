//! Tests from `spec/parser/interpolate/10_escaped_backslash`

use super::check;

#[test]
fn t01_inline() {
    check(
        ".result {\n  output: \\\\;\n  output: #{\\\\};\n  \
         output: \"[#{\\\\}]\";\n  output: \"#{\\\\}\";\n  \
         output: '#{\\\\}';\n  output: \"['#{\\\\}']\";\n}\n",
        ".result {\n  output: \\\\;\n  output: \\\\;\n  \
         output: \"[\\\\\\\\]\";\n  output: \"\\\\\\\\\";\n  \
         output: \"\\\\\\\\\";\n  output: \"['\\\\\\\\']\";\n}\n",
    )
}

#[test]
fn t02_variable() {
    check(
        "$input: \\\\;\n.result {\n  output: $input;\n  \
         output: #{$input};\n  output: \"[#{$input}]\";\n  \
         output: \"#{$input}\";\n  output: '#{$input}';\n  \
         output: \"['#{$input}']\";\n}\n",
        ".result {\n  output: \\\\;\n  output: \\\\;\n  \
         output: \"[\\\\\\\\]\";\n  output: \"\\\\\\\\\";\n  \
         output: \"\\\\\\\\\";\n  output: \"['\\\\\\\\']\";\n}\n",
    )
}

#[test]
fn t03_inline_double() {
    check(
        ".result {\n  output: #{#{\\\\}};\n  output: #{\"[#{\\\\}]\"};\n  \
         output: #{\"#{\\\\}\"};\n  output: #{'#{\\\\}'};\n  \
         output: #{\"['#{\\\\}']\"};\n}\n",
        ".result {\n  output: \\\\;\n  output: [\\\\];\n  output: \\\\;\n  \
         output: \\\\;\n  output: ['\\\\'];\n}\n",
    )
}
