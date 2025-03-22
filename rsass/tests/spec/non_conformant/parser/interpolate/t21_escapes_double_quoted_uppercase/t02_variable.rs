//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/21_escapes_double_quoted_uppercase/02_variable.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("02_variable")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "$input: \"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\";\
             \n.result {\
             \n  output: $input;\
             \n  output: #{$input};\
             \n  output: \"[#{$input}]\";\
             \n  output: \"#{$input}\";\
             \n  output: \'#{$input}\';\
             \n  output: \"[\'#{$input}\']\";\
             \n}\n"
        ),
        ".result {\
         \n  output: \"\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\";\
         \n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\
         \n  output: \"[\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ]\";\
         \n  output: \"\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\";\
         \n  output: \"\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\";\
         \n  output: \"[\'\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ\']\";\
         \n}\n"
    );
}
