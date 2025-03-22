//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/15_escapes_double_quoted_numbers/04_variable_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("04_variable_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"\\1\\2\\3\\4\\5\\6\\7\\8\\9\";\
             \n.result {\
             \n  output: #{#{$input}};\
             \n  output: #{\"[#{$input}]\"};\
             \n  output: #{\"#{$input}\"};\
             \n  output: #{\'#{$input}\'};\
             \n  output: #{\"[\'#{$input}\']\"};\
             \n}\n"),
        ".result {\
         \n  output: \u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t;\
         \n  output: [\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t];\
         \n  output: \u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t;\
         \n  output: \u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t;\
         \n  output: [\'\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t\'];\
         \n}\n"
    );
}
