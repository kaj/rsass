//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/16_escapes_single_quoted_numbers/05_variable_quoted_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \'\\1\\2\\3\\4\\5\\6\\7\\8\\9\';\
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
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  dquoted: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  dquoted: \"[\\1\\2\\3\\4\\5\\6\\7\\8 \t]\";\
        \n  dquoted: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  dquoted: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  dquoted: \"[\'\\1\\2\\3\\4\\5\\6\\7\\8 \t\']\";\
        \n  squoted: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  squoted: \"[\\1\\2\\3\\4\\5\\6\\7\\8 \t]\";\
        \n  squoted: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  squoted: \"\\1\\2\\3\\4\\5\\6\\7\\8 \t\";\
        \n  squoted: \"[\'\\1\\2\\3\\4\\5\\6\\7\\8 \t\']\";\
        \n}\
        \n"
    );
}
