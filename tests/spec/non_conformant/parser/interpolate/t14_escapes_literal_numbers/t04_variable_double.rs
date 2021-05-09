//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/14_escapes_literal_numbers/04_variable_double.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \\1\\2\\3\\4\\5\\6\\7\\8\\9;\
             \n.result {\
             \n  output: #{#{$input}};\
             \n  output: #{\"[#{$input}]\"};\
             \n  output: #{\"#{$input}\"};\
             \n  output: #{\'#{$input}\'};\
             \n  output: #{\"[\'#{$input}\']\"};\
             \n}\n"),
        ".result {\
         \n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\
         \n  output: [\\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ];\
         \n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\
         \n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\
         \n  output: [\'\\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 \'];\
         \n}\n"
    );
}
