//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/17_escapes_literal_lowercase/01_inline.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("01_inline")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            ".result {\
             \n  output: \\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z;\
             \n  output: #{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z};\
             \n  output: \"[#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}]\";\
             \n  output: \"#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\";\
             \n  output: \'#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\';\
             \n  output: \"[\'#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\']\";\
             \n}\n"
        ),
        ".result {\
         \n  output: \\b \\c \\d \\e \\f ghijklmnopqrstuvwxyz;\
         \n  output: \\b \\c \\d \\e \\f ghijklmnopqrstuvwxyz;\
         \n  output: \"[\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz]\";\
         \n  output: \"\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\";\
         \n  output: \"\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\";\
         \n  output: \"[\'\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\']\";\
         \n}\n"
    );
}
