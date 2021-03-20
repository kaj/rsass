//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/19_escapes_single_quoted_lowercase/06_escape_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\';\
            \n.result {\
            \n  output: \"[\\#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}]\";\
            \n  output: \"\\#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\";\
            \n  output: \'\\#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\';\
            \n  output: \"[\'\\#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[#{\'\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz\'}]\";\
        \n  output: \"#{\'\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz\'}\";\
        \n  output: \"#{\" \\b \\c \\d \\e \\f ghijklmnopqrstuvwxyz \"}\";\
        \n  output: \"[\'#{\'\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz\'}\']\";\
        \n}\
        \n"
    );
}
