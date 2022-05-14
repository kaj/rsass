//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/14_escapes_literal_numbers/01_inline.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("01_inline")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            ".result {\
             \n  output: \\1\\2\\3\\4\\5\\6\\7\\8\\9;\
             \n  output: #{\\1\\2\\3\\4\\5\\6\\7\\8\\9};\
             \n  output: \"[#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}]\";\
             \n  output: \"#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\";\
             \n  output: \'#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\';\
             \n  output: \"[\'#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\']\";\
             \n}\n"
        ),
        ".result {\
         \n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\
         \n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\
         \n  output: \"[\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 ]\";\
         \n  output: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\
         \n  output: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\
         \n  output: \"[\'\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \']\";\
         \n}\n"
    );
}
