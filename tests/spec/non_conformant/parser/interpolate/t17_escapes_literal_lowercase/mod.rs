//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/17_escapes_literal_lowercase"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/parser/interpolate/17_escapes_literal_lowercase/01_inline.hrx"
#[test]
fn t01_inline() {
    assert_eq!(
        rsass(
            ".result {\
            \n  output: \\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z;\
            \n  output: #{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z};\
            \n  output: \"[#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}]\";\
            \n  output: \"#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\";\
            \n  output: \'#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\';\
            \n  output: \"[\'#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\']\";\
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

// From "sass-spec/spec/non_conformant/parser/interpolate/17_escapes_literal_lowercase/02_variable.hrx"
#[test]
fn t02_variable() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/non_conformant/parser/interpolate/17_escapes_literal_lowercase/03_inline_double.hrx"
#[test]
fn t03_inline_double() {
    assert_eq!(
        rsass(
            ".result {\
            \n  output: #{#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}};\
            \n  output: #{\"[#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}]\"};\
            \n  output: #{\"#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\"};\
            \n  output: #{\'#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\'};\
            \n  output: #{\"[\'#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \\b \\c \\d \\e \\f ghijklmnopqrstuvwxyz;\
        \n  output: [\\b \\c \\d \\e \\f ghijklmnopqrstuvwxyz];\
        \n  output: \\b \\c \\d \\e \\f ghijklmnopqrstuvwxyz;\
        \n  output: \\b \\c \\d \\e \\f ghijklmnopqrstuvwxyz;\
        \n  output: [\'\\b \\c \\d \\e \\f ghijklmnopqrstuvwxyz\'];\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/17_escapes_literal_lowercase/04_variable_double.hrx"
#[test]
fn t04_variable_double() {
    assert_eq!(
        rsass(
            "$input: \\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z;\
            \n.result {\
            \n  output: #{#{$input}};\
            \n  output: #{\"[#{$input}]\"};\
            \n  output: #{\"#{$input}\"};\
            \n  output: #{\'#{$input}\'};\
            \n  output: #{\"[\'#{$input}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \\b \\c \\d \\e \\f ghijklmnopqrstuvwxyz;\
        \n  output: [\\b \\c \\d \\e \\f ghijklmnopqrstuvwxyz];\
        \n  output: \\b \\c \\d \\e \\f ghijklmnopqrstuvwxyz;\
        \n  output: \\b \\c \\d \\e \\f ghijklmnopqrstuvwxyz;\
        \n  output: [\'\\b \\c \\d \\e \\f ghijklmnopqrstuvwxyz\'];\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/17_escapes_literal_lowercase/05_variable_quoted_double.hrx"
#[test]
fn t05_variable_quoted_double() {
    assert_eq!(
        rsass(
            "$input: \\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z;\
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
        \n  dquoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\";\
        \n  dquoted: \"[\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz]\";\
        \n  dquoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\";\
        \n  dquoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\";\
        \n  dquoted: \"[\'\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\']\";\
        \n  squoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\";\
        \n  squoted: \"[\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz]\";\
        \n  squoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\";\
        \n  squoted: \"\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\";\
        \n  squoted: \"[\'\\\\b \\\\c \\\\d \\\\e \\\\f ghijklmnopqrstuvwxyz\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/17_escapes_literal_lowercase/06_escape_interpolation.hrx"
#[test]
#[ignore] // wrong result
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: \\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z;\
            \n.result {\
            \n  output: \"[\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}]\";\
            \n  output: \"\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\";\
            \n  output: \'\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\';\
            \n  output: \"[\'\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[#{\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz}]\";\
        \n  output: \"#{\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz}\";\
        \n  output: \"#{\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz}\";\
        \n  output: \"[\'#{\\b\\c\\d\\e\\fghijklmnopqrstuvwxyz}\']\";\
        \n}\
        \n"
    );
}
