//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/07_comma_list_complex/02_variable.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("02_variable")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: gamma, \"\'\"delta\"\'\";\
             \n.result {\
             \n  output: $input;\
             \n  output: #{$input};\
             \n  output: \"[#{$input}]\";\
             \n  output: \"#{$input}\";\
             \n  output: \'#{$input}\';\
             \n  output: \"[\'#{$input}\']\";\
             \n}\n"),
        ".result {\
         \n  output: gamma, \"\'\" delta \"\'\";\
         \n  output: gamma, \' delta \';\
         \n  output: \"[gamma, \' delta \']\";\
         \n  output: \"gamma, \' delta \'\";\
         \n  output: \"gamma, \' delta \'\";\
         \n  output: \"[\'gamma, \' delta \'\']\";\
         \n}\n"
    );
}
