use super::check;

#[test]
fn t01_inline() {
    check(
        ".result {\n  output: 'squoted';\n  output: #{'squoted'};\n  \
         output: \"[#{'squoted'}]\";\n  output: \"#{'squoted'}\";\n  \
         output: '#{'squoted'}';\n  output: \"['#{'squoted'}']\";\n}\n",
        ".result {\n  output: 'squoted';\n  output: squoted;\n  \
         output: \"[squoted]\";\n  output: \"squoted\";\n  \
         output: \"squoted\";\n  output: \"['squoted']\";\n}\n",
    )
}

#[test]
fn t02_variable() {
    check(
        "$input: 'squoted';\n.result {\n  output: $input;\n  \
         output: #{$input};\n  output: \"[#{$input}]\";\n  \
         output: \"#{$input}\";\n  output: '#{$input}';\n  \
         output: \"['#{$input}']\";\n}\n",
        ".result {\n  output: \"squoted\";\n  output: squoted;\n  \
         output: \"[squoted]\";\n  output: \"squoted\";\n  \
         output: \"squoted\";\n  output: \"['squoted']\";\n}\n",
    )
}

#[test]
fn t06_escape_interpolation() {
    check(
        "$input: 'squoted';\n.result {\n  output: \"[\\#{'squoted'}]\";\n  \
         output: \"\\#{'squoted'}\";\n  output: '\\#{'squoted'}';\n  \
         output: \"['\\#{'squoted'}']\";\n}\n",
        ".result {\n  output: \"[\\#{'squoted'}]\";\n  \
         output: \"\\#{'squoted'}\";\n  output: \"#{\" squoted \"}\";\n  \
         output: \"['\\#{'squoted'}']\";\n}\n",
    )
}
