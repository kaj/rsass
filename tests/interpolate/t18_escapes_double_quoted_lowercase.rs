//! Tests from `spec/parser/interpolate/18_escapes_double_quoted_lowercase`

use super::check;

#[test]
fn t01_inline() {
    check(
        ".result {\n  \
         output: \"\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\
         \\r\\s\\t\\u\\v\\w\\x\\y\\z\";\n  \
         output: #{\"\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\
         \\r\\s\\t\\u\\v\\w\\x\\y\\z\"};\n  \
         output: \"[#{\"\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\
         \\r\\s\\t\\u\\v\\w\\x\\y\\z\"}]\";\n  \
         output: \"#{\"\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\
         \\r\\s\\t\\u\\v\\w\\x\\y\\z\"}\";\n  \
         output: '#{\"\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\
         \\r\\s\\t\\u\\v\\w\\x\\y\\z\"}';\n  \
         output: \"['#{\"\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\
         \\r\\s\\t\\u\\v\\w\\x\\y\\z\"}']\";\n}\n",
        ".result {\n  \
         output: \"\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\
         \\r\\s\\t\\u\\v\\w\\x\\y\\z\";\n  \
         output: \u{b}\u{c}\u{d}\u{e}\u{f}ghijklmnopqrstuvwxyz;\n  \
         output: \"[\u{b}\u{c}\u{d}\u{e}\u{f}ghijklmnopqrstuvwxyz]\";\n  \
         output: \"\u{b}\u{c}\u{d}\u{e}\u{f}ghijklmnopqrstuvwxyz\";\n  \
         output: \"\u{b}\u{c}\u{d}\u{e}\u{f}ghijklmnopqrstuvwxyz\";\n  \
         output: \"['\u{b}\u{c}\u{d}\u{e}\u{f}ghijklmnopqrstuvwxyz']\";\n}\n",
    )
}

#[test]
fn t02_variable() {
    check(
        "$input: \"\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\
         \\r\\s\\t\\u\\v\\w\\x\\y\\z\";\n\
         .result {\n  output: $input;\n  output: #{$input};\n  \
         output: \"[#{$input}]\";\n  output: \"#{$input}\";\n  \
         output: '#{$input}';\n  output: \"['#{$input}']\";\n}\n",
        ".result {\n  \
         output: \"\u{b}\u{c}\u{d}\u{e}\u{f}ghijklmnopqrstuvwxyz\";\n  \
         output: \u{b}\u{c}\u{d}\u{e}\u{f}ghijklmnopqrstuvwxyz;\n  \
         output: \"[\u{b}\u{c}\u{d}\u{e}\u{f}ghijklmnopqrstuvwxyz]\";\n  \
         output: \"\u{b}\u{c}\u{d}\u{e}\u{f}ghijklmnopqrstuvwxyz\";\n  \
         output: \"\u{b}\u{c}\u{d}\u{e}\u{f}ghijklmnopqrstuvwxyz\";\n  \
         output: \"['\u{b}\u{c}\u{d}\u{e}\u{f}ghijklmnopqrstuvwxyz']\";\n}\n",
    )
}

#[test]
fn t03_inline_double() {
    check(
        ".result {\n  \
         output: #{#{\"\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\
         \\r\\s\\t\\u\\v\\w\\x\\y\\z\"}};\n  \
         output: #{\"[#{\"\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\
         \\r\\s\\t\\u\\v\\w\\x\\y\\z\"}]\"};\n  \
         output: #{\"#{\"\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\
         \\r\\s\\t\\u\\v\\w\\x\\y\\z\"}\"};\n  \
         output: #{'#{\"\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\
         \\r\\s\\t\\u\\v\\w\\x\\y\\z\"}'};\n  \
         output: #{\"['#{\"\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\
         \\r\\s\\t\\u\\v\\w\\x\\y\\z\"}']\"};\n}\n",
        ".result {\n  \
         output: \u{b}\u{c}\u{d}\u{e}\u{f}ghijklmnopqrstuvwxyz;\n  \
         output: [\u{b}\u{c}\u{d}\u{e}\u{f}ghijklmnopqrstuvwxyz];\n  \
         output: \u{b}\u{c}\u{d}\u{e}\u{f}ghijklmnopqrstuvwxyz;\n  \
         output: \u{b}\u{c}\u{d}\u{e}\u{f}ghijklmnopqrstuvwxyz;\n  \
         output: ['\u{b}\u{c}\u{d}\u{e}\u{f}ghijklmnopqrstuvwxyz'];\n}\n",
    )
}
