//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/04_space_list_quoted/02_variable.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("02_variable")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"alpha\" \'beta\';\
             \n.result {\
             \n  output: $input;\
             \n  output: #{$input};\
             \n  output: \"[#{$input}]\";\
             \n  output: \"#{$input}\";\
             \n  output: \'#{$input}\';\
             \n  output: \"[\'#{$input}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"alpha\" \"beta\";\
         \n  output: alpha beta;\
         \n  output: \"[alpha beta]\";\
         \n  output: \"alpha beta\";\
         \n  output: \"alpha beta\";\
         \n  output: \"[\'alpha beta\']\";\
         \n}\n"
    );
}
