//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/20_escapes_literal_uppercase/06_escape_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z;\
            \n.result {\
            \n  output: \"[\\#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}]\";\
            \n  output: \"\\#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}\";\
            \n  output: \'\\#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}\';\
            \n  output: \"[\'\\#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[#{\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ}]\";\
        \n  output: \"#{\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ}\";\
        \n  output: \"#{\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ}\";\
        \n  output: \"[\'#{\\b\\c\\d\\e\\fGHIJKLMNOPQRSTUVWXYZ}\']\";\
        \n}\
        \n"
    );
}
