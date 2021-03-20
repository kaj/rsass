//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/19_escapes_single_quoted_lowercase/05_variable_quoted_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\';\
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
        \n  dquoted: \"\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz\";\
        \n  dquoted: \"[\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz]\";\
        \n  dquoted: \"\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz\";\
        \n  dquoted: \"\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz\";\
        \n  dquoted: \"[\'\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz\']\";\
        \n  squoted: \"\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz\";\
        \n  squoted: \"[\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz]\";\
        \n  squoted: \"\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz\";\
        \n  squoted: \"\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz\";\
        \n  squoted: \"[\'\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz\']\";\
        \n}\
        \n"
    );
}
