//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/06_space_list_complex/02_variable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("02_variable")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: gamme \"\'\"delta\"\'\";\
             \n.result {\
             \n  output: $input;\
             \n  output: #{$input};\
             \n  output: \"[#{$input}]\";\
             \n  output: \"#{$input}\";\
             \n  output: \'#{$input}\';\
             \n  output: \"[\'#{$input}\']\";\
             \n}\n"),
        ".result {\
         \n  output: gamme \"\'\" delta \"\'\";\
         \n  output: gamme \' delta \';\
         \n  output: \"[gamme \' delta \']\";\
         \n  output: \"gamme \' delta \'\";\
         \n  output: \"gamme \' delta \'\";\
         \n  output: \"[\'gamme \' delta \'\']\";\
         \n}\n"
    );
}
