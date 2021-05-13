//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/05_comma_list_quoted/02_variable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"alpha\", \'beta\';\
             \n.result {\
             \n  output: $input;\
             \n  output: #{$input};\
             \n  output: \"[#{$input}]\";\
             \n  output: \"#{$input}\";\
             \n  output: \'#{$input}\';\
             \n  output: \"[\'#{$input}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"alpha\", \"beta\";\
         \n  output: alpha, beta;\
         \n  output: \"[alpha, beta]\";\
         \n  output: \"alpha, beta\";\
         \n  output: \"alpha, beta\";\
         \n  output: \"[\'alpha, beta\']\";\
         \n}\n"
    );
}
