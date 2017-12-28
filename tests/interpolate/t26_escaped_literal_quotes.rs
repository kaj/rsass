//! Tests from `spec/parser/interpolate/26_escaped_literal_quotes`

use super::check;

#[test]
fn t01_inline() {
    check(".result {\n  output: \\\"\\';\n  output: #{\\\"\\'};\n  \
           output: \"[#{\\\"\\'}]\";\n  output: \"#{\\\"\\'}\";\n  \
           output: '#{\\\"\\'}';\n  output: \"['#{\\\"\\'}']\";\n}\n",
          ".result {\n  output: \\\"\\';\n  output: \\\"\\';\n  \
           output: \"[\\\\\\\"\\\\']\";\n  output: \"\\\\\\\"\\\\'\";\n  \
           output: \"\\\\\\\"\\\\'\";\n  output: \"['\\\\\\\"\\\\'']\";\n}\n")
}

#[test]
fn t06_escape_interpolation() {
    check("$input: \\\"\\';\n.result {\n  \
           output: \"[\\#{\\\"\\'}]\";\n  output: \"\\#{\\\"\\'}\";\n  \
           output: '\\#{\\\"\\'}';\n  output: \"['\\#{\\\"\\'}']\";\n}\n",
          ".result {\n  \
           output: \"[\\#{\\\"\\'}]\";\n  output: \"\\#{\\\"\\'}\";\n  \
           output: '\\#{\\\"\\'}';\n  output: \"['\\#{\\\"\\'}']\";\n}\n")
}
