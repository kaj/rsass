//! Tests auto-converted from "sass-spec/spec/parser/interpolate/21_escapes_double_quoted_uppercase"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/parser/interpolate/21_escapes_double_quoted_uppercase/01_inline"
#[test]
fn t01_inline() {
    assert_eq!(
        rsass(
            ".result {\n  output: \"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\";\n  output: #{\"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\"};\n  output: \"[#{\"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\"}]\";\n  output: \"#{\"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\"}\";\n  output: \'#{\"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\"}\';\n  output: \"[\'#{\"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\"}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\";\n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\n  output: \"[\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ]\";\n  output: \"\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\";\n  output: \"\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\";\n  output: \"[\'\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\']\";\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/21_escapes_double_quoted_uppercase/02_variable"
#[test]
fn t02_variable() {
    assert_eq!(
        rsass(
            "$input: \"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\";\n.result {\n  output: $input;\n  output: #{$input};\n  output: \"[#{$input}]\";\n  output: \"#{$input}\";\n  output: \'#{$input}\';\n  output: \"[\'#{$input}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\";\n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\n  output: \"[\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ]\";\n  output: \"\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\";\n  output: \"\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\";\n  output: \"[\'\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\']\";\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/21_escapes_double_quoted_uppercase/03_inline_double"
#[test]
fn t03_inline_double() {
    assert_eq!(
        rsass(
            ".result {\n  output: #{#{\"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\"}};\n  output: #{\"[#{\"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\"}]\"};\n  output: #{\"#{\"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\"}\"};\n  output: #{\'#{\"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\"}\'};\n  output: #{\"[\'#{\"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\"}\']\"};\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\n  output: [\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ];\n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\n  output: [\'\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\'];\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/21_escapes_double_quoted_uppercase/04_variable_double"
#[test]
fn t04_variable_double() {
    assert_eq!(
        rsass(
            "$input: \"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\";\n.result {\n  output: #{#{$input}};\n  output: #{\"[#{$input}]\"};\n  output: #{\"#{$input}\"};\n  output: #{\'#{$input}\'};\n  output: #{\"[\'#{$input}\']\"};\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\n  output: [\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ];\n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\n  output: [\'\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\'];\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/21_escapes_double_quoted_uppercase/05_variable_quoted_double"
#[test]
fn t05_variable_quoted_double() {
    assert_eq!(
        rsass(
            "$input: \"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\";\n.result {\n  dquoted: \"#{#{$input}}\";\n  dquoted: \"#{\"[#{$input}]\"}\";\n  dquoted: \"#{\"#{$input}\"}\";\n  dquoted: \"#{\'#{$input}\'}\";\n  dquoted: \"#{\"[\'#{$input}\']\"}\";\n  squoted: \'#{#{$input}}\';\n  squoted: \'#{\"[#{$input}]\"}\';\n  squoted: \'#{\"#{$input}\"}\';\n  squoted: \'#{\'#{$input}\'}\';\n  squoted: \'#{\"[\'#{$input}\']\"}\';\n}\n"
        )
        .unwrap(),
        ".result {\n  dquoted: \"\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\";\n  dquoted: \"[\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ]\";\n  dquoted: \"\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\";\n  dquoted: \"\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\";\n  dquoted: \"[\'\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\']\";\n  squoted: \"\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\";\n  squoted: \"[\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ]\";\n  squoted: \"\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\";\n  squoted: \"\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\";\n  squoted: \"[\'\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\']\";\n}\n"
    );
}

// Ignoring "06_escape_interpolation", start_version is 3.7.
