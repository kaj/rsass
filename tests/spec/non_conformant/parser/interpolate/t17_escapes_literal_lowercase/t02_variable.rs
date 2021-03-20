//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/17_escapes_literal_lowercase/02_variable.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z;\
            \n.result {\
            \n  output: $input;\
            \n  output: #{$input};\
            \n  output: \"[#{$input}]\";\
            \n  output: \"#{$input}\";\
            \n  output: \'#{$input}\';\
            \n  output: \"[\'#{$input}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \\b \\c \\d \\e \\f ghijklmnopqrstuvwxyz;\
        \n  output: \\b \\c \\d \\e \\f ghijklmnopqrstuvwxyz;\
        \n  output: \"[\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz]\";\
        \n  output: \"\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\";\
        \n  output: \"\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\";\
        \n  output: \"[\'\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\']\";\
        \n}\
        \n"
    );
}
