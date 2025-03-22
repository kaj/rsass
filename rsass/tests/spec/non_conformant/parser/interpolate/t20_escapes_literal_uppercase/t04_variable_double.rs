//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/20_escapes_literal_uppercase/04_variable_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("04_variable_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "$input: \\B\\C\\D\\E\\F\\G\\H\\I\\J\\K\\L\\M\\N\\O\\P\\Q\\R\\S\\T\\U\\V\\W\\X\\Y\\Z;\
             \n.result {\
             \n  output: #{#{$input}};\
             \n  output: #{\"[#{$input}]\"};\
             \n  output: #{\"#{$input}\"};\
             \n  output: #{\'#{$input}\'};\
             \n  output: #{\"[\'#{$input}\']\"};\
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
