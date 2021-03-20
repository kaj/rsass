//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/22_escapes_single_quoted_uppercase/02_variable.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
