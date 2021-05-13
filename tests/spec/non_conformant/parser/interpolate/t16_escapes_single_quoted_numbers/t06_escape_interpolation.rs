//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/16_escapes_single_quoted_numbers/06_escape_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \'\\1\\2\\3\\4\\5\\6\\7\\8\\9\';\
             \n.result {\
             \n  output: \"[\\#{\'\\1\\2\\3\\4\\5\\6\\7\\8\\9\'}]\";\
             \n  output: \"\\#{\'\\1\\2\\3\\4\\5\\6\\7\\8\\9\'}\";\
             \n  output: \'\\#{\'\\1\\2\\3\\4\\5\\6\\7\\8\\9\'}\';\
             \n  output: \"[\'\\#{\'\\1\\2\\3\\4\\5\\6\\7\\8\\9\'}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"[#{\'\\1\\2\\3\\4\\5\\6\\7\\8 \t\'}]\";\
         \n  output: \"#{\'\\1\\2\\3\\4\\5\\6\\7\\8 \t\'}\";\
         \n  output: \"#{\" \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9  \"}\";\
         \n  output: \"[\'#{\'\\1\\2\\3\\4\\5\\6\\7\\8 \t\'}\']\";\
         \n}\n"
    );
}
