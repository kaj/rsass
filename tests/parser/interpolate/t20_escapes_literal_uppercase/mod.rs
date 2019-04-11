//! Tests auto-converted from "sass-spec/spec/parser/interpolate/20_escapes_literal_uppercase"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/parser/interpolate/20_escapes_literal_uppercase/01_inline.hrx"

// Ignoring "t01_inline", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/20_escapes_literal_uppercase/02_variable.hrx"

// Ignoring "t02_variable", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/20_escapes_literal_uppercase/03_inline_double.hrx"

// Ignoring "t03_inline_double", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/20_escapes_literal_uppercase/04_variable_double.hrx"

// Ignoring "t04_variable_double", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/20_escapes_literal_uppercase/05_variable_quoted_double.hrx"

// Ignoring "t05_variable_quoted_double", start_version is 3.7.

// From "sass-spec/spec/parser/interpolate/20_escapes_literal_uppercase/06_escape_interpolation.hrx"
#[test]
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: \\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z;\n.result {\n  output: \"[\\#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}]\";\n  output: \"\\#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}\";\n  output: \'\\#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}\';\n  output: \"[\'\\#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"[\\#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}]\";\n  output: \"\\#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}\";\n  output: \'\\#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}\';\n  output: \"[\'\\#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}\']\";\n}\n"
    );
}
