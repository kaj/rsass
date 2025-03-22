//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/19_escapes_single_quoted_lowercase/04_variable_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("04_variable_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "$input: \'\\b\\c\\d\\e\\f\\g\\h\\i\\j\\k\\l\\m\\n\\o\\p\\q\\r\\s\\t\\u\\v\\w\\x\\y\\z\';\
             \n.result {\
             \n  output: #{#{$input}};\
             \n  output: #{\"[#{$input}]\"};\
             \n  output: #{\"#{$input}\"};\
             \n  output: #{\'#{$input}\'};\
             \n  output: #{\"[\'#{$input}\']\"};\
             \n}\n"
        ),
        ".result {\
         \n  output: \u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz;\
         \n  output: [\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz];\
         \n  output: \u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz;\
         \n  output: \u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz;\
         \n  output: [\'\u{b}\u{c}\r\u{e}\u{f}ghijklmnopqrstuvwxyz\'];\
         \n}\n"
    );
}
