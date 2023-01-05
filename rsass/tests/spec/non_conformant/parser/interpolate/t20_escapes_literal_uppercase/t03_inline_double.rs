//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/20_escapes_literal_uppercase/03_inline_double.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("03_inline_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            ".result {\
             \n  output: #{#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}};\
             \n  output: #{\"[#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}]\"};\
             \n  output: #{\"#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}\"};\
             \n  output: #{\'#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}\'};\
             \n  output: #{\"[\'#{\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z}\']\"};\
             \n}\n"
        ),
        ".result {\
         \n  output: \\b \\c \\d \\e \\f GHIJKLMNOPQRSTUVWXYZ;\
         \n  output: [\\b \\c \\d \\e \\f GHIJKLMNOPQRSTUVWXYZ];\
         \n  output: \\b \\c \\d \\e \\f GHIJKLMNOPQRSTUVWXYZ;\
         \n  output: \\b \\c \\d \\e \\f GHIJKLMNOPQRSTUVWXYZ;\
         \n  output: [\'\\b \\c \\d \\e \\f GHIJKLMNOPQRSTUVWXYZ\'];\
         \n}\n"
    );
}
