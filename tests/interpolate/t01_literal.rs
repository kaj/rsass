use super::check;

#[test]
fn t01_inline() {
    check(
        ".result {\n  output: literal;\n  output: #{literal};\n  \
         output: \"[#{literal}]\";\n  output: \"#{literal}\";\n  \
         output: '#{literal}';\n  output: \"['#{literal}']\";\n}\n",
        ".result {\n  output: literal;\n  output: literal;\n  \
         output: \"[literal]\";\n  output: \"literal\";\n  \
         output: \"literal\";\n  output: \"['literal']\";\n}\n",
    )
}

#[test]
fn t02_variable() {
    check(
        "$input: literal;\n\
         .result {\n  output: $input;\n  output: #{$input};\n  \
         output: \"[#{$input}]\";\n  output: \"#{$input}\";\n  \
         output: '#{$input}';\n  output: \"['#{$input}']\";\n}\n",
        ".result {\n  output: literal;\n  output: literal;\n  \
         output: \"[literal]\";\n  output: \"literal\";\n  \
         output: \"literal\";\n  output: \"['literal']\";\n}\n",
    )
}

#[test]
fn t03() {
    check(
        ".result {\n  \
         output: #{#{literal}};\n  output: #{\"[#{literal}]\"};\n  \
         output: #{\"#{literal}\"};\n  output: #{'#{literal}'};\n  \
         output: #{\"['#{literal}']\"};\n}\n",
        ".result {\n  output: literal;\n  output: [literal];\n  \
         output: literal;\n  output: literal;\n  output: ['literal'];\n}\n",
    )
}

#[test]
fn t06_escape_interpolation() {
    check(
        "$input: literal;\n\
         .result {\n  output: \"[\\#{literal}]\";\n  \
         output: \"\\#{literal}\";\n  output: '\\#{literal}';\n  \
         output: \"['\\#{literal}']\";\n}\n",
        ".result {\n  output: \"[\\#{literal}]\";\n  \
         output: \"\\#{literal}\";\n  output: '\\#{literal}';\n  \
         output: \"['\\#{literal}']\";\n}\n",
    )
}
