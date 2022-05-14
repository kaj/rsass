//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/10_escaped_backslash/04_variable_double.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("04_variable_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \\\\;\
             \n.result {\
             \n  output: #{#{$input}};\
             \n  output: #{\"[#{$input}]\"};\
             \n  output: #{\"#{$input}\"};\
             \n  output: #{\'#{$input}\'};\
             \n  output: #{\"[\'#{$input}\']\"};\
             \n}\n"),
        ".result {\
         \n  output: \\\\;\
         \n  output: [\\\\];\
         \n  output: \\\\;\
         \n  output: \\\\;\
         \n  output: [\'\\\\\'];\
         \n}\n"
    );
}
