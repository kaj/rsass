//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/13_escaped_single_quoted/05_variable_quoted_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("05_variable_quoted_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \'l\\\\ite\\ral\';\
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
             \n}\n"),
        ".result {\
         \n  dquoted: \"l\\\\iteral\";\
         \n  dquoted: \"[l\\\\iteral]\";\
         \n  dquoted: \"l\\\\iteral\";\
         \n  dquoted: \"l\\\\iteral\";\
         \n  dquoted: \"[\'l\\\\iteral\']\";\
         \n  squoted: \"l\\\\iteral\";\
         \n  squoted: \"[l\\\\iteral]\";\
         \n  squoted: \"l\\\\iteral\";\
         \n  squoted: \"l\\\\iteral\";\
         \n  squoted: \"[\'l\\\\iteral\']\";\
         \n}\n"
    );
}
