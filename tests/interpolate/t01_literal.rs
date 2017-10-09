use super::check;

#[test]
fn t06_escape_interpolation() {
    check("$input: literal;\n\
           .result {\n  output: \"[\\#{literal}]\";\n  \
           output: \"\\#{literal}\";\n  output: '\\#{literal}';\n  \
           output: \"['\\#{literal}']\";\n}\n",
          ".result {\n  output: \"[\\#{literal}]\";\n  \
           output: \"\\#{literal}\";\n  output: '\\#{literal}';\n  \
           output: \"['\\#{literal}']\";\n}\n")
}
