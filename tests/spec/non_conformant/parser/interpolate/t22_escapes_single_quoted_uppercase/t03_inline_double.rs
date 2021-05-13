//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/22_escapes_single_quoted_uppercase/03_inline_double.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            ".result {\
             \n  output: #{#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}};\
             \n  output: #{\"[#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}]\"};\
             \n  output: #{\"#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}\"};\
             \n  output: #{\'#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}\'};\
             \n  output: #{\"[\'#{\'\\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z\'}\']\"};\
             \n}\n"
        ),
        ".result {\
         \n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\
         \n  output: [\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ];\
         \n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\
         \n  output: \u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ;\
         \n  output: [\'\u{b}\u{c}\r\u{e}\u{f}GHIJKLMNOPQRSTUVWXYZ\'];\
         \n}\n"
    );
}
