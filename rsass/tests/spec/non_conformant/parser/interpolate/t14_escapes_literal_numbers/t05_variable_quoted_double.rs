//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/14_escapes_literal_numbers/05_variable_quoted_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("05_variable_quoted_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "$input: \\1\\2\\3\\4\\5\\6\\7\\8\\9;\
             \n.result {\
             \n  dquoted: \"#{#{$input}}\";\
             \n  dquoted: \"#{\"[#{$input}]\"}\";\
             \n  dquoted: \"#{\"#{$input}\"}\";\
             \n  dquoted: \"#{\'#{$input}\'}\";\
             \n  dquoted: \"#{\"[\'#{$input}\']\"}\";\
             \n  squoted: \'#{#{$input}}\';\
             \n  squoted: \'#{\"[#{$input}]\"}\';\
             \n  squoted: \'#{\"#{$input}\"}\';\
             \n  squoted: \'#{\'#{$input}\'}\';\
             \n  squoted: \'#{\"[\'#{$input}\']\"}\';\
             \n}\n"
        ),
        ".result {\
         \n  dquoted: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\
         \n  dquoted: \"[\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 ]\";\
         \n  dquoted: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\
         \n  dquoted: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\
         \n  dquoted: \"[\'\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \']\";\
         \n  squoted: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\
         \n  squoted: \"[\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 ]\";\
         \n  squoted: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\
         \n  squoted: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\
         \n  squoted: \"[\'\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \']\";\
         \n}\n"
    );
}
