//! Tests from `spec/parser/interpolate/24_escapes_double_quoted_specials`

use super::check;

#[test]
fn t01_inline() {
    check(
        ".result {\n  output: \"\\0_\\a_\\A\";\n  \
         output: #{\"\\0_\\a_\\A\"};\n  \
         output: \"[#{\"\\0_\\a_\\A\"}]\";\n  \
         output: \"#{\"\\0_\\a_\\A\"}\";\n  \
         output: '#{\"\\0_\\a_\\A\"}';\n  \
         output: \"['#{\"\\0_\\a_\\A\"}']\";\n}\n",
        "@charset \"UTF-8\";\n.result {\n  \
         output: \"\\0_\\a_\\A\";\n  output: \u{fffd}_ _ ;\n  \
         output: \"[\u{fffd}_\\a_\\a]\";\n  \
         output: \"\u{fffd}_\\a_\\a\";\n  \
         output: \"\u{fffd}_\\a_\\a\";\n  \
         output: \"['\u{fffd}_\\a_\\a']\";\n}\n",
    )
}

#[test]
fn t02_variable() {
    check(
        "$input: \"\\0_\\a_\\A\";\n\
         .result {\n  output: $input;\n  output: #{$input};\n  \
         output: \"[#{$input}]\";\n  output: \"#{$input}\";\n  \
         output: '#{$input}';\n  output: \"['#{$input}']\";\n}\n",
        "@charset \"UTF-8\";\n\
         .result {\n  output: \"\u{fffd}_\\a_\\a\";\n  \
         output: \u{fffd}_ _ ;\n  output: \"[\u{fffd}_\\a_\\a]\";\n  \
         output: \"\u{fffd}_\\a_\\a\";\n  output: \"\u{fffd}_\\a_\\a\";\n  \
         output: \"['\u{fffd}_\\a_\\a']\";\n}\n",
    )
}

#[test]
fn t03_inline_double() {
    check(
        ".result {\n  output: #{#{\"\\0_\\a_\\A\"}};\n  \
         output: #{\"[#{\"\\0_\\a_\\A\"}]\"};\n  \
         output: #{\"#{\"\\0_\\a_\\A\"}\"};\n  \
         output: #{'#{\"\\0_\\a_\\A\"}'};\n  \
         output: #{\"['#{\"\\0_\\a_\\A\"}']\"};\n}\n",
        "@charset \"UTF-8\";\n.result {\n  output: \u{fffd}_ _ ;\n  \
         output: [\u{fffd}_ _ ];\n  output: \u{fffd}_ _ ;\n  \
         output: \u{fffd}_ _ ;\n  output: ['\u{fffd}_ _ '];\n}\n",
    )
}
