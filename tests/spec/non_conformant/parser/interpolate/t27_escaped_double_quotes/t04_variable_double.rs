//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/27_escaped_double_quotes/04_variable_double.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"\\\"\";\
             \n.result {\
             \n  output: #{#{$input}};\
             \n  output: #{\"[#{$input}]\"};\
             \n  output: #{\"#{$input}\"};\
             \n  output: #{\'#{$input}\'};\
             \n  output: #{\"[\'#{$input}\']\"};\
             \n}\n"),
        ".result {\
         \n  output: \";\
         \n  output: [\"];\
         \n  output: \";\
         \n  output: \";\
         \n  output: [\'\"\'];\
         \n}\n"
    );
}
