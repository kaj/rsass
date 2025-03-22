//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/17_escapes_literal_lowercase/06_escape_interpolation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("06_escape_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "$input: \\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z;\
             \n.result {\
             \n  output: \"[\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}]\";\
             \n  output: \"\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\";\
             \n  output: \'\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\';\
             \n  output: \"[\'\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\']\";\
             \n}\n"
        ),
        ".result {\
         \n  output: \"[#{\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz}]\";\
         \n  output: \"#{\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz}\";\
         \n  output: \"#{\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz}\";\
         \n  output: \"[\'#{\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz}\']\";\
         \n}\n"
    );
}
