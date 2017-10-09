//! Tests from `spec/parser/interpolate`

use super::check;

#[test]
fn t01_inline() {
    check(".result {\n  output: \"alpha\" 'beta';\n  \
           output: #{\"alpha\" 'beta'};\n  \
           output: \"[#{\"alpha\" 'beta'}]\";\n  \
           output: \"#{\"alpha\" 'beta'}\";\n  \
           output: '#{\"alpha\" 'beta'}';\n  \
           output: \"['#{\"alpha\" 'beta'}']\";\n}\n",
          ".result {\n  output: \"alpha\" 'beta';\n  \
           output: alpha beta;\n  output: \"[alpha beta]\";\n  \
           output: \"alpha beta\";\n  output: \"alpha beta\";\n  \
           output: \"['alpha beta']\";\n}\n")
}
