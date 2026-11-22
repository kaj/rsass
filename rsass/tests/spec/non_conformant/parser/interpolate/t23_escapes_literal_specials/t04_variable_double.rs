//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials/04_variable_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("04_variable_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \\0_\\a_\\A;\
             \n.result {\
             \n  output: #{#{$input}};\
             \n  output: #{\"[#{$input}]\"};\
             \n  output: #{\"#{$input}\"};\
             \n  output: #{\'#{$input}\'};\
             \n  output: #{\"[\'#{$input}\']\"};\
             \n}\n"),
        "@charset \"UTF-8\";\
         \n.result {\
         \n  output: �_\\a _\\a ;\
         \n  output: [�_\\a _\\a ];\
         \n  output: �_\\a _\\a ;\
         \n  output: �_\\a _\\a ;\
         \n  output: [\'�_\\a _\\a \'];\
         \n}\n"
    );
}
