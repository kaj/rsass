//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/29_binary_operation/04_variable_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("04_variable_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"foo#{\'ba\' + \'r\'}baz\";\
             \n.result {\
             \n  output: #{#{$input}};\
             \n  output: #{\"[#{$input}]\"};\
             \n  output: #{\"#{$input}\"};\
             \n  output: #{\'#{$input}\'};\
             \n  output: #{\"[\'#{$input}\']\"};\
             \n}\n"),
        ".result {\
         \n  output: foobarbaz;\
         \n  output: [foobarbaz];\
         \n  output: foobarbaz;\
         \n  output: foobarbaz;\
         \n  output: [\'foobarbaz\'];\
         \n}\n"
    );
}
