//! Tests auto-converted from "sass-spec/spec/parser/interpolate/17_escapes_literal_lowercase"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/parser/interpolate/17_escapes_literal_lowercase/01_inline.hrx"

// Ignoring "t01_inline", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/17_escapes_literal_lowercase/02_variable.hrx"

// Ignoring "t02_variable", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/17_escapes_literal_lowercase/03_inline_double.hrx"

// Ignoring "t03_inline_double", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/17_escapes_literal_lowercase/04_variable_double.hrx"

// Ignoring "t04_variable_double", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/17_escapes_literal_lowercase/05_variable_quoted_double.hrx"

// Ignoring "t05_variable_quoted_double", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/17_escapes_literal_lowercase/06_escape_interpolation.hrx"
#[test]
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: \\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z;\n.result {\n  output: \"[\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}]\";\n  output: \"\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\";\n  output: \'\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\';\n  output: \"[\'\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"[\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}]\";\n  output: \"\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\";\n  output: \'\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\';\n  output: \"[\'\\#{\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z}\']\";\n}\n"
    );
}
