//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/22_escapes_single_quoted_uppercase"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/parser/interpolate/22_escapes_single_quoted_uppercase/01_inline.hrx"
#[test]
#[ignore] // wrong result
fn t01_inline() {
    assert_eq!(
        rsass(
            ".result {\
            \n  output: \'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\';\
            \n  output: #{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'};\
            \n  output: \"[#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}]\";\
            \n  output: \"#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}\";\
            \n  output: \'#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}\';\
            \n  output: \"[\'#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\";\
        \n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\
        \n  output: \"[\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ]\";\
        \n  output: \"\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\";\
        \n  output: \"\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\";\
        \n  output: \"[\'\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/22_escapes_single_quoted_uppercase/02_variable.hrx"
#[test]
#[ignore] // wrong result
fn t02_variable() {
    assert_eq!(
        rsass(
            "$input: \'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\';\
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
        \n  output: \"\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\";\
        \n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\
        \n  output: \"[\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ]\";\
        \n  output: \"\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\";\
        \n  output: \"\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\";\
        \n  output: \"[\'\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/22_escapes_single_quoted_uppercase/03_inline_double.hrx"
#[test]
#[ignore] // wrong result
fn t03_inline_double() {
    assert_eq!(
        rsass(
            ".result {\
            \n  output: #{#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}};\
            \n  output: #{\"[#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}]\"};\
            \n  output: #{\"#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}\"};\
            \n  output: #{\'#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}\'};\
            \n  output: #{\"[\'#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\
        \n  output: [\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ];\
        \n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\
        \n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\
        \n  output: [\'\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\'];\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/22_escapes_single_quoted_uppercase/04_variable_double.hrx"
#[test]
#[ignore] // wrong result
fn t04_variable_double() {
    assert_eq!(
        rsass(
            "$input: \'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\';\
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
        \n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\
        \n  output: [\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ];\
        \n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\
        \n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\
        \n  output: [\'\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\'];\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/22_escapes_single_quoted_uppercase/05_variable_quoted_double.hrx"
#[test]
#[ignore] // wrong result
fn t05_variable_quoted_double() {
    assert_eq!(
        rsass(
            "$input: \'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\';\
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
        \n  dquoted: \"\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\";\
        \n  dquoted: \"[\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ]\";\
        \n  dquoted: \"\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\";\
        \n  dquoted: \"\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\";\
        \n  dquoted: \"[\'\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\']\";\
        \n  squoted: \"\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\";\
        \n  squoted: \"[\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ]\";\
        \n  squoted: \"\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\";\
        \n  squoted: \"\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\";\
        \n  squoted: \"[\'\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/interpolate/22_escapes_single_quoted_uppercase/06_escape_interpolation.hrx"
#[test]
#[ignore] // wrong result
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: \'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\';\
            \n.result {\
            \n  output: \"[\\#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}]\";\
            \n  output: \"\\#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}\";\
            \n  output: \'\\#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}\';\
            \n  output: \"[\'\\#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[#{\'\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\'}]\";\
        \n  output: \"#{\'\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\'}\";\
        \n  output: \"#{\" \\b \\c \\d \\e \\f GHIJKLMNOPQRSTUVWXYZ \"}\";\
        \n  output: \"[\'#{\'\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\'}\']\";\
        \n}\
        \n"
    );
}
