//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/20_escapes_literal_uppercase/06_escape_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("06_escape_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "$input: \\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z;\
             \n.result {\
             \n  output: \"[\\#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}]\";\
             \n  output: \"\\#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}\";\
             \n  output: \'\\#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}\';\
             \n  output: \"[\'\\#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}\']\";\
             \n}\n"
        ),
        ".result {\
         \n  output: \"[#{\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ}]\";\
         \n  output: \"#{\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ}\";\
         \n  output: \"#{\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ}\";\
         \n  output: \"[\'#{\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ}\']\";\
         \n}\n"
    );
}
