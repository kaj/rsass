//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/03_single_quoted/04_variable_double.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("04_variable_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \'squoted\';\
             \n.result {\
             \n  output: #{#{$input}};\
             \n  output: #{\"[#{$input}]\"};\
             \n  output: #{\"#{$input}\"};\
             \n  output: #{\'#{$input}\'};\
             \n  output: #{\"[\'#{$input}\']\"};\
             \n}\n"),
        ".result {\
         \n  output: squoted;\
         \n  output: [squoted];\
         \n  output: squoted;\
         \n  output: squoted;\
         \n  output: [\'squoted\'];\
         \n}\n"
    );
}
