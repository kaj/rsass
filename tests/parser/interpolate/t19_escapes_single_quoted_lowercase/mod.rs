//! Tests auto-converted from "sass-spec/spec/parser/interpolate/19_escapes_single_quoted_lowercase"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/parser/interpolate/19_escapes_single_quoted_lowercase/01_inline"
#[test]
fn t01_inline() {
    assert_eq!(
        rsass(
            ".result {\n  output: \'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\';\n  output: #{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'};\n  output: \"[#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}]\";\n  output: \"#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\";\n  output: \'#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\';\n  output: \"[\'#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\';\n  output: \u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz;\n  output: \"[\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz]\";\n  output: \"\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\";\n  output: \"\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\";\n  output: \"[\'\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\']\";\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/19_escapes_single_quoted_lowercase/02_variable"
#[test]
fn t02_variable() {
    assert_eq!(
        rsass(
            "$input: \'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\';\n.result {\n  output: $input;\n  output: #{$input};\n  output: \"[#{$input}]\";\n  output: \"#{$input}\";\n  output: \'#{$input}\';\n  output: \"[\'#{$input}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\";\n  output: \u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz;\n  output: \"[\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz]\";\n  output: \"\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\";\n  output: \"\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\";\n  output: \"[\'\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\']\";\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/19_escapes_single_quoted_lowercase/03_inline_double"
#[test]
fn t03_inline_double() {
    assert_eq!(
        rsass(
            ".result {\n  output: #{#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}};\n  output: #{\"[#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}]\"};\n  output: #{\"#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\"};\n  output: #{\'#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\'};\n  output: #{\"[\'#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\']\"};\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz;\n  output: [\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz];\n  output: \u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz;\n  output: \u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz;\n  output: [\'\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\'];\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/19_escapes_single_quoted_lowercase/04_variable_double"
#[test]
fn t04_variable_double() {
    assert_eq!(
        rsass(
            "$input: \'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\';\n.result {\n  output: #{#{$input}};\n  output: #{\"[#{$input}]\"};\n  output: #{\"#{$input}\"};\n  output: #{\'#{$input}\'};\n  output: #{\"[\'#{$input}\']\"};\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz;\n  output: [\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz];\n  output: \u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz;\n  output: \u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz;\n  output: [\'\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\'];\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/19_escapes_single_quoted_lowercase/05_variable_quoted_double"
#[test]
fn t05_variable_quoted_double() {
    assert_eq!(
        rsass(
            "$input: \'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\';\n.result {\n  dquoted: \"#{#{$input}}\";\n  dquoted: \"#{\"[#{$input}]\"}\";\n  dquoted: \"#{\"#{$input}\"}\";\n  dquoted: \"#{\'#{$input}\'}\";\n  dquoted: \"#{\"[\'#{$input}\']\"}\";\n  squoted: \'#{#{$input}}\';\n  squoted: \'#{\"[#{$input}]\"}\';\n  squoted: \'#{\"#{$input}\"}\';\n  squoted: \'#{\'#{$input}\'}\';\n  squoted: \'#{\"[\'#{$input}\']\"}\';\n}\n"
        )
        .unwrap(),
        ".result {\n  dquoted: \"\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\";\n  dquoted: \"[\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz]\";\n  dquoted: \"\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\";\n  dquoted: \"\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\";\n  dquoted: \"[\'\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\']\";\n  squoted: \"\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\";\n  squoted: \"[\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz]\";\n  squoted: \"\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\";\n  squoted: \"\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\";\n  squoted: \"[\'\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\']\";\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/19_escapes_single_quoted_lowercase/06_escape_interpolation"
#[test]
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: \'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\';\n.result {\n  output: \"[\\#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}]\";\n  output: \"\\#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\";\n  output: \'\\#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\';\n  output: \"[\'\\#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"[\\#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}]\";\n  output: \"\\#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\";\n  output: \"#{\" \\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z \"}\";\n  output: \"[\'\\#{\'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\'}\']\";\n}\n"
    );
}
