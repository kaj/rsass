//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/21_escapes_double_quoted_uppercase/05_variable_quoted_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("05_variable_quoted_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "$input: \"\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\";\
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
             \n}\n"
        ),
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
         \n}\n"
    );
}
