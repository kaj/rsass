use super::check;

#[test]
fn t01_inline() {
    check(".result {\n  output: \"dquoted\";\n  output: #{\"dquoted\"};\n  \
           output: \"[#{\"dquoted\"}]\";\n  output: \"#{\"dquoted\"}\";\n  \
           output: '#{\"dquoted\"}';\n  output: \"['#{\"dquoted\"}']\";\n}\n",
          ".result {\n  output: \"dquoted\";\n  output: dquoted;\n  \
           output: \"[dquoted]\";\n  output: \"dquoted\";\n  \
           output: \"dquoted\";\n  output: \"['dquoted']\";\n}\n")
}

#[test]
fn t06_escape_interpolation() {
    check("$input: \"dquoted\";\n.result {\n  \
           output: \"[\\#{\"dquoted\"}]\";\n  \
           output: \"\\#{\"dquoted\"}\";\n  \
           output: '\\#{\"dquoted\"}';\n  \
           output: \"['\\#{\"dquoted\"}']\";\n}\n",
          ".result {\n  output: \"[#{\" dquoted \"}]\";\n  \
           output: \"#{\" dquoted \"}\";\n  output: '\\#{\"dquoted\"}';\n  \
           output: \"['#{\" dquoted \"}']\";\n}\n")
}
