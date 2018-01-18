//! Tests from `spec/parser/interpolate/04_space_list_quoted`

use super::check;

#[test]
fn t01_inline() {
    check(
        ".result {\n  output: \"alpha\" 'beta';\n  \
         output: #{\"alpha\" 'beta'};\n  \
         output: \"[#{\"alpha\" 'beta'}]\";\n  \
         output: \"#{\"alpha\" 'beta'}\";\n  \
         output: '#{\"alpha\" 'beta'}';\n  \
         output: \"['#{\"alpha\" 'beta'}']\";\n}\n",
        ".result {\n  output: \"alpha\" 'beta';\n  \
         output: alpha beta;\n  output: \"[alpha beta]\";\n  \
         output: \"alpha beta\";\n  output: \"alpha beta\";\n  \
         output: \"['alpha beta']\";\n}\n",
    )
}

#[test]
fn t02_variable() {
    check(
        "$input: \"alpha\" 'beta';\n.result {\n  output: $input;\n  \
         output: #{$input};\n  output: \"[#{$input}]\";\n  \
         output: \"#{$input}\";\n  output: '#{$input}';\n  \
         output: \"['#{$input}']\";\n}\n",
        ".result {\n  output: \"alpha\" \"beta\";\n  \
         output: alpha beta;\n  output: \"[alpha beta]\";\n  \
         output: \"alpha beta\";\n  output: \"alpha beta\";\n  \
         output: \"['alpha beta']\";\n}\n",
    )
}
