//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/16_escapes_single_quoted_numbers/02_variable.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("02_variable")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \'\\1\\2\\3\\4\\5\\6\\7\\8\\9\';\
             \n.result {\
             \n  output: $input;\
             \n  output: #{$input};\
             \n  output: \"[#{$input}]\";\
             \n  output: \"#{$input}\";\
             \n  output: \'#{$input}\';\
             \n  output: \"[\'#{$input}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
         \n  output: \u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t;\
         \n  output: \"[\\1\\2\\3\\4\\5\\6\\7\\8 \t]\";\
         \n  output: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
         \n  output: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
         \n  output: \"[\'\\1\\2\\3\\4\\5\\6\\7\\8 \t\']\";\
         \n}\n"
    );
}
