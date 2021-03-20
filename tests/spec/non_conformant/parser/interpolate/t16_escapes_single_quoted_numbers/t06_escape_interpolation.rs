//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/16_escapes_single_quoted_numbers/06_escape_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \'\\1\\2\\3\\4\\5\\6\\7\\8\\9\';\
            \n.result {\
            \n  output: \"[\\#{\'\\1\\2\\3\\4\\5\\6\\7\\8\\9\'}]\";\
            \n  output: \"\\#{\'\\1\\2\\3\\4\\5\\6\\7\\8\\9\'}\";\
            \n  output: \'\\#{\'\\1\\2\\3\\4\\5\\6\\7\\8\\9\'}\';\
            \n  output: \"[\'\\#{\'\\1\\2\\3\\4\\5\\6\\7\\8\\9\'}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[#{\'\\1\\2\\3\\4\\5\\6\\7\\8 \t\'}]\";\
        \n  output: \"#{\'\\1\\2\\3\\4\\5\\6\\7\\8 \t\'}\";\
        \n  output: \"#{\" \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9  \"}\";\
        \n  output: \"[\'#{\'\\1\\2\\3\\4\\5\\6\\7\\8 \t\'}\']\";\
        \n}\
        \n"
    );
}
